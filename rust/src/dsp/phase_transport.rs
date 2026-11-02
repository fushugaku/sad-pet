//! Prepared phaseable windows for signals without a native playback phase.
use super::{
    SampleBuffer,
    interaction::{GLOBAL, LANES},
    sample::LiveInput,
};
use crate::{
    Stereo,
    interaction::{Node, Transport},
    preset::Preset,
    resynth::{Model, Waveform},
};

const BUDGET: usize = 64 * 1024 * 1024;
struct Window {
    node: usize,
    lane: usize,
    identity: u32,
    settings: Transport,
    live: LiveInput,
    noise: SampleBuffer,
    stamp: u64,
    waveform: Waveform,
    pending: Stereo,
    contributors: u64,
}
pub struct Runtime {
    windows: Vec<Window>,
    map: Box<[[i16; LANES]]>,
}
impl Runtime {
    pub fn contains(&self, node: usize, lane: usize) -> bool {
        self.index(node, lane).is_some()
    }
    pub fn prepare(p: &Preset, rate: u32) -> Result<Option<Box<Self>>, String> {
        let mut specifications = Vec::new();
        // Every addressable target reserves its optional phase transport:
        // DAW automation can enable Sync or change waveform on the audio thread.
        let targeted = |node: Node| p.interactions.routes.iter().any(|r| r.target == node);
        let mut add =
            |node: usize, lane: usize, id: u32, o: crate::resynth::Oscillator, needed: bool| {
                if (o.model == Model::Radias || o.waveform == Waveform::AudioIn)
                    && (needed || o.transport.enabled)
                {
                    specifications.push((node, lane, id, o.transport, o.waveform));
                }
            };
        if let Some(s) = &p.oscillators {
            for (i, slot) in s.slots.iter().enumerate() {
                if slot.id != 0 && slot.kind == crate::oscillators::Kind::Radias {
                    add(
                        i,
                        GLOBAL,
                        slot.id,
                        slot.controls.native,
                        targeted(Node::Osc { id: slot.id }),
                    );
                }
            }
        }
        if let Some(s) = &p.synth {
            for (i, &o) in s.oscillators.iter().enumerate().take(s.count as usize) {
                add(
                    8 + i,
                    GLOBAL,
                    0,
                    o,
                    targeted(Node::Synth { index: i as u8 }),
                );
            }
        }
        for (i, &o) in p
            .resynth
            .oscillators
            .iter()
            .enumerate()
            .take(p.resynth.count as usize)
        {
            for band in 0..crate::resynth::BANDS {
                add(
                    16 + i,
                    super::VOICES + band,
                    0,
                    o,
                    targeted(Node::Resynth { index: i as u8 }),
                );
            }
        }
        if let Some(s) = &p.tb303 {
            for (i, &o) in s.oscillators.iter().enumerate().take(s.count as usize) {
                add(
                    20 + i,
                    GLOBAL,
                    0,
                    o,
                    targeted(Node::Tb303 { index: i as u8 }),
                );
            }
        }
        if specifications.is_empty() {
            return Ok(None);
        }
        let bytes = specifications
            .iter()
            .map(|(_, _, _, t, _)| {
                (t.window_seconds.clamp(0.005, 0.25) * rate as f32).ceil() as usize * 40
            })
            .sum::<usize>();
        if bytes > BUDGET {
            return Err("Phase transport windows exceed 64 MiB; shorten their windows".into());
        }
        let mut map = vec![[-1; LANES]; crate::interaction::NODES].into_boxed_slice();
        let mut windows = Vec::with_capacity(specifications.len());
        for (node, lane, identity, mut settings, waveform) in specifications {
            settings.sanitize();
            let mut live = LiveInput::for_transport(rate, settings.window_seconds);
            let frames = live.sample.len();
            let mut seed = settings.seed ^ ((node as u32 + 1) * 0x12345) ^ (lane as u32);
            let data = (0..frames)
                .map(|_| {
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    let x = (seed as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32 * 0.8;
                    [x; 2]
                })
                .collect::<Vec<_>>();
            if waveform == Waveform::Noise {
                for &sample in &data {
                    live.push(sample, false);
                }
            }
            let noise = SampleBuffer::new("Seeded phase segment".into(), rate, data)?;
            map[node][lane] = windows.len() as i16;
            windows.push(Window {
                node,
                lane,
                identity,
                settings,
                live,
                noise,
                stamp: u64::MAX,
                waveform,
                pending: [0.0; 2],
                contributors: 0,
            });
        }
        Ok(Some(Box::new(Self { windows, map })))
    }
    pub fn inherit(&mut self, old: &mut Self) {
        for window in &mut self.windows {
            let i = old.map[window.node][window.lane];
            if i < 0 {
                continue;
            }
            let previous = &mut old.windows[i as usize];
            if previous.identity == window.identity
                && previous.settings == window.settings
                && previous.waveform == window.waveform
                && previous.live.sample.sample_rate == window.live.sample.sample_rate
            {
                std::mem::swap(&mut window.live, &mut previous.live);
                std::mem::swap(&mut window.noise, &mut previous.noise);
                window.stamp = previous.stamp;
                window.pending = previous.pending;
                window.contributors = previous.contributors;
            }
        }
    }
    fn index(&self, node: usize, lane: usize) -> Option<usize> {
        let at = *self.map.get(node)?.get(lane)?;
        let at = if at >= 0 { at } else { self.map[node][GLOBAL] };
        (at >= 0).then_some(at as usize)
    }
    pub fn capture(&mut self, node: usize, lane: usize, frame: Stereo, stamp: u64) {
        let Some(index) = self.index(node, lane) else {
            return;
        };
        let window = &mut self.windows[index];
        if window.stamp != stamp {
            window.stamp = stamp;
            window.pending = [0.0; 2];
            window.contributors = 0;
        }
        let bit = 1u64 << lane;
        if window.contributors & bit != 0 {
            return;
        }
        window.contributors |= bit;
        for (pending, sample) in window.pending.iter_mut().zip(frame) {
            *pending += sample;
        }
    }
    /// All voices read the same frozen host-frame snapshot. Commit only after
    /// every oscillator owner has rendered, so voice order cannot change it.
    pub fn finish_frame(&mut self) {
        for window in &mut self.windows {
            let count = window.contributors.count_ones();
            if count == 0 {
                continue;
            }
            let frozen = !window.settings.rolling && window.live.window_full();
            window
                .live
                .push(window.pending.map(|v| v / count as f32), frozen);
            window.contributors = 0;
            window.pending = [0.0; 2];
        }
    }
    pub fn read(
        &self,
        node: usize,
        lane: usize,
        phase: f64,
        frequency: f64,
        rate: f64,
        waveform: Waveform,
    ) -> Option<Stereo> {
        let window = &self.windows[self.index(node, lane)?];
        let (sample, range) = if waveform == Waveform::Noise && !window.settings.rolling {
            (&window.noise, 0..window.noise.len())
        } else {
            (&window.live.sample, window.live.range())
        };
        let speed = frequency * range.len() as f64 / rate * window.settings.rate as f64;
        let mip = sample.mip_for_rate(speed, true);
        let phase = phase.rem_euclid(1.0);
        let coordinate = 0.05 + phase * 0.95;
        let position = range.start as f64 + coordinate * range.len() as f64;
        let mut out = sample.read_region_mip(position, mip, &range);
        // Only the window seam is blended. Hard-reset discontinuities are
        // handled by the carrier's high-rate path, not hidden by this fade.
        if coordinate > 0.95 {
            let t = ((coordinate - 0.95) / 0.05) as f32;
            let blend = t * t * (3.0 - 2.0 * t);
            let other = sample.read_region_mip(
                range.start as f64 + (coordinate - 0.95) * range.len() as f64,
                mip,
                &range,
            );
            for c in 0..2 {
                out[c] += (other[c] - out[c]) * blend;
            }
        }
        Some(out)
    }
}

#[derive(Default)]
pub struct Prepared(pub Option<Box<Runtime>>, pub Option<String>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedPhaseTransport")
            .field("ready", &self.0.is_some())
            .field("error", &self.1)
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, preset: &Preset, rate: u32) -> Result<(), String> {
        self.1 = None;
        match Runtime::prepare(preset, rate) {
            Ok(r) => {
                self.0 = r;
                Ok(())
            }
            Err(e) => {
                self.1 = Some(e.clone());
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn patch() -> Preset {
        let mut p = crate::timbres::Kind::Oscillators.preset().unwrap();
        let o = &mut p.oscillators.as_mut().unwrap().slots[0].controls.native;
        o.waveform = Waveform::AudioIn;
        o.transport = Transport {
            enabled: true,
            rolling: false,
            window_seconds: 0.1,
            ..Default::default()
        };
        p
    }
    #[test]
    fn polyphonic_capture_commits_one_snapshot_and_freezes_at_an_integer_capacity() {
        let mut r = Runtime::prepare(&patch(), 8000).unwrap().unwrap();
        for stamp in 0..1000 {
            let before = r.windows[0].live.head();
            r.capture(0, 1, [0.25; 2], stamp);
            r.capture(0, 2, [0.75; 2], stamp);
            assert_eq!(
                r.windows[0].live.head(),
                before,
                "a voice changed another voice's read window"
            );
            r.finish_frame();
        }
        let w = &r.windows[0];
        assert!(w.live.window_full());
        assert_eq!(w.live.head(), 0);
        assert!(w.live.sample.frames().iter().all(|x| x == &[0.5; 2]));
    }
    #[test]
    fn seeded_segment_seam_meets_the_next_cycle_and_sync_automation_is_prepared() {
        let mut p = patch();
        p.oscillators.as_mut().unwrap().slots[0]
            .controls
            .native
            .waveform = Waveform::Noise;
        let r = Runtime::prepare(&p, 8000).unwrap().unwrap();
        let a = r.read(0, 1, 0.0, 1.0, 8000.0, Waveform::Noise).unwrap();
        let b = r
            .read(0, 1, 1.0 - 1e-9, 1.0, 8000.0, Waveform::Noise)
            .unwrap();
        assert!((a[0] - b[0]).abs() < 1e-5);
        p.oscillators.as_mut().unwrap().slots[0]
            .controls
            .native
            .transport
            .enabled = false;
        p.interactions.routes.push(crate::interaction::Route {
            source: Node::Group { index: 0 },
            target: Node::Osc { id: 1 },
            ring: 1.0,
            ..Default::default()
        });
        assert!(Runtime::prepare(&p, 8000).unwrap().unwrap().contains(0, 1));
    }
}

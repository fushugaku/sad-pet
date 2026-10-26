//! The advanced grain mode's per-grain state: which group a grain joined, its
//! two envelopes over its own life and its own stereo filter.
use super::{MAX_GRAINS, fast, filter};
use crate::{
    Stereo,
    grain_groups::{Envelope, FilterType, GROUPS, Group, Settings},
    preset::{self, FilterRouting},
    tables::{self, Random},
};

/// A filtering grain's coefficients follow its envelope this often, in samples.
const REFRESH: u32 = 16;
/// Steps of the cutoff → SVF gain table, which spares every refresh a powf
/// and a tan. Neighbours differ by 1.4%, so interpolation stays within 2e-5.
const CUTOFF_STEPS: usize = 512;

/// An envelope's segment boundaries in samples of one grain's life, which
/// ends at zero on its last sample as the engine's window does.
#[derive(Clone, Copy, Debug, Default)]
struct Segments {
    attack_end: f32,
    decay_end: f32,
    release_start: f32,
    end: f32,
    sustain: f32,
    attack_scale: f32,
    decay_scale: f32,
    release_scale: f32,
}
impl Segments {
    fn new(envelope: &Envelope, duration: u32) -> Self {
        let end = duration.saturating_sub(1) as f32;
        let total = envelope.attack + envelope.decay + envelope.release;
        let scale = end / total.max(1.0);
        let [attack, decay, release] =
            [envelope.attack, envelope.decay, envelope.release].map(|share| share * scale);
        let inverse = |length: f32| if length > 0.0 { 1.0 / length } else { 0.0 };
        Self {
            attack_end: attack,
            decay_end: attack + decay,
            // Without a release the level holds to the grain's last sample.
            release_start: if release > 0.0 {
                end - release
            } else {
                f32::INFINITY
            },
            end,
            sustain: envelope.sustain,
            attack_scale: inverse(attack),
            decay_scale: inverse(decay),
            release_scale: inverse(release),
        }
    }
    /// The level at `age`: `rise` bends the attack's 0..1 ramp, `fall` the
    /// share of decay and release still to go.
    #[inline(always)]
    fn value(&self, age: f32, rise: impl Fn(f32) -> f32, fall: impl Fn(f32) -> f32) -> f32 {
        if age < self.attack_end {
            rise(age * self.attack_scale)
        } else if age < self.decay_end {
            let t = 1.0 - (age - self.attack_end) * self.decay_scale;
            self.sustain + (1.0 - self.sustain) * fall(t)
        } else if age < self.release_start {
            self.sustain
        } else {
            self.sustain * fall(((self.end - age) * self.release_scale).clamp(0.0, 1.0))
        }
    }
    /// The amp envelope: every segment on the window's curve.
    #[inline(always)]
    fn shaped(&self, age: f32, shape: f32) -> f32 {
        let curve = |x: f32| tables::window_curve(x, shape);
        self.value(age, curve, curve)
    }
    /// The filter envelope: a straight attack, then decay and release on
    /// squared curves.
    #[inline(always)]
    fn swept(&self, age: f32) -> f32 {
        self.value(age, |x| x, |x| x * x)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Grain {
    /// Started in advanced mode; other grains pass through untouched.
    grouped: bool,
    group: u8,
    shape: f32,
    /// This grain's own random cutoff offset and level.
    cutoff_offset: f32,
    level: f32,
    /// False for a filter that stayed open across the whole grain's
    /// envelope as it started, or for Filter 2 routed off, so the grain
    /// skips it.
    filtering: [bool; 2],
    amp: Segments,
    filter_env: Segments,
    filters: [Svf; 2],
}

/// A grain's stereo state-variable filter, as the engine's legacy one.
#[derive(Clone, Copy, Debug, Default)]
struct Svf {
    state: [[f32; 2]; 2],
    coefficients: [f32; 3],
    k: f32,
    band_gain: f32,
}
impl Svf {
    fn tune(&mut self, g: f32, resonance: f32) {
        let k = 2.0 - 1.94 * resonance;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        self.coefficients = [a1, a2, g * a2];
        self.k = k;
        self.band_gain = k.sqrt();
    }
    /// An open low pass passes the frame and keeps its state running.
    #[inline(always)]
    fn process(&mut self, frame: Stereo, kind: FilterType, open: bool) -> Stereo {
        let (k, band_gain) = (self.k, self.band_gain);
        let [a1, a2, a3] = self.coefficients;
        std::array::from_fn(|c| {
            let [ic1, ic2] = self.state[c];
            let v3 = frame[c] - ic2;
            let band = a1 * ic1 + a2 * v3;
            let low = ic2 + a2 * ic1 + a3 * v3;
            self.state[c] = [fast::flush(2.0 * band - ic1), fast::flush(2.0 * low - ic2)];
            if open {
                frame[c]
            } else {
                match kind {
                    FilterType::LowPass => low,
                    FilterType::BandPass => band * band_gain,
                    FilterType::HighPass => frame[c] - k * band - low,
                }
            }
        })
    }
}

pub(crate) struct Runtime {
    /// tan(pi f / fs) at each step of the 0..1 cutoff scale.
    gains: [f32; CUTOFF_STEPS + 1],
    grains: [Grain; MAX_GRAINS],
    /// Smooth weighted round robin: each deal adds every share, the richest
    /// group takes the grain and pays the total back.
    credit: [f32; GROUPS],
    /// The groups' own randomness, apart from the engine's, so that grains
    /// outside them keep the engine's random sequence.
    pub(crate) random: Random,
}
impl Runtime {
    pub(crate) fn new(sample_rate: u32) -> Self {
        let rate = sample_rate as f32;
        Self {
            gains: std::array::from_fn(|i| {
                let cutoff = i as f32 / CUTOFF_STEPS as f32;
                let hz = filter::cutoff_hz(preset::FilterType::LowPass, cutoff).min(rate * 0.45);
                (std::f32::consts::PI * hz / rate).tan()
            }),
            grains: [Grain::default(); MAX_GRAINS],
            credit: [0.0; GROUPS],
            random: Random(0x6772_6169_6e5f_6772),
        }
    }
    pub(crate) fn reset(&mut self) {
        self.credit = [0.0; GROUPS];
    }
    /// The group the next grain joins, None when every share is zero.
    /// Equal shares take turns A, B, C, D.
    pub(crate) fn deal(&mut self, settings: &Settings) -> Option<usize> {
        let total: f32 = settings.groups.iter().map(|g| g.share).sum();
        if total <= 0.0 {
            return None;
        }
        let mut chosen = None;
        for (i, group) in settings.groups.iter().enumerate() {
            if group.share <= 0.0 {
                // A group switched off keeps no claim on later grains.
                self.credit[i] = 0.0;
                continue;
            }
            self.credit[i] = (self.credit[i] + group.share).clamp(-total, total);
            if chosen.is_none_or(|c: usize| self.credit[i] > self.credit[c]) {
                chosen = Some(i);
            }
        }
        let chosen = chosen?;
        self.credit[chosen] -= total;
        Some(chosen)
    }
    pub(crate) fn start(&mut self, slot: usize, index: usize, group: &Group, duration: u32) {
        let cutoff_offset = if group.random_cutoff > 0.0 {
            self.random.bipolar() * group.random_cutoff
        } else {
            0.0
        };
        let level = if group.random_level > 0.0 {
            1.0 - self.random.unit() * group.random_level
        } else {
            1.0
        };
        self.grains[slot] = Grain {
            grouped: true,
            group: index as u8,
            shape: group.shape.clamp(0.0, 1.0),
            cutoff_offset,
            level,
            filtering: [
                !(group.filter_type == FilterType::LowPass
                    && group.cutoff + cutoff_offset >= 0.999
                    && group.env_depth >= 0.0),
                group.filter_routing != FilterRouting::Single
                    && !(group.filter2_type == FilterType::LowPass
                        && group.filter2_cutoff + cutoff_offset >= 0.999
                        && group.filter2_env_depth >= 0.0),
            ],
            amp: Segments::new(&group.amp, duration),
            filter_env: Segments::new(&group.filter_env, duration),
            ..Grain::default()
        };
    }
    /// A grain started outside advanced mode.
    pub(crate) fn release(&mut self, slot: usize) {
        self.grains[slot].grouped = false;
    }
    pub(crate) fn group(&self, slot: usize) -> Option<u8> {
        let grain = &self.grains[slot];
        grain.grouped.then_some(grain.group)
    }
    /// A grain's group and one frame of it through its filter, amp envelope
    /// and level, or None for a grain outside the groups, which keeps the
    /// engine's window. Filter and level follow the group's current settings
    /// while it plays.
    #[inline]
    pub(crate) fn process(
        &mut self,
        slot: usize,
        age: u32,
        frame: Stereo,
        settings: &Settings,
    ) -> Option<(usize, Stereo)> {
        let grain = &mut self.grains[slot];
        if !grain.grouped {
            return None;
        }
        let index = grain.group as usize;
        let group = &settings.groups[index];
        let age_f = age as f32;
        let gain = grain.amp.shaped(age_f, grain.shape) * group.level * grain.level;
        let [first, second] = grain.filtering;
        let routing = group.filter_routing;
        let second = second && routing != FilterRouting::Single;
        if !first && !second {
            return Some((index, frame.map(|x| x * gain)));
        }
        // Both cutoffs follow the one filter envelope, each at its depth.
        let sweep = grain.filter_env.swept(age_f);
        let cutoffs = [
            group.cutoff + grain.cutoff_offset + group.env_depth * sweep,
            group.filter2_cutoff + grain.cutoff_offset + group.filter2_env_depth * sweep,
        ];
        if age.is_multiple_of(REFRESH) {
            for (n, filter) in grain.filters.iter_mut().enumerate() {
                if [first, second][n] {
                    let step = cutoffs[n].clamp(0.0, 1.0) * CUTOFF_STEPS as f32;
                    let i = (step as usize).min(CUTOFF_STEPS - 1);
                    let g = self.gains[i] + (self.gains[i + 1] - self.gains[i]) * (step - i as f32);
                    filter.tune(g, [group.resonance, group.filter2_resonance][n]);
                }
            }
        }
        let kinds = [group.filter_type, group.filter2_type];
        let open = |n: usize| kinds[n] == FilterType::LowPass && cutoffs[n] >= 0.999;
        let [one, two] = &mut grain.filters;
        let filtered = if first {
            one.process(frame, kinds[0], open(0))
        } else {
            frame
        };
        // A Filter 2 left open passes its input: after Filter 1 that is
        // Filter 1's output, beside it the grain itself.
        let out = match routing {
            FilterRouting::Single => filtered,
            FilterRouting::Serial if second => two.process(filtered, kinds[1], open(1)),
            FilterRouting::Serial => filtered,
            FilterRouting::Parallel => {
                let beside = if second {
                    two.process(frame, kinds[1], open(1))
                } else {
                    frame
                };
                std::array::from_fn(|c| (filtered[c] + beside[c]) * 0.5)
            }
        };
        Some((index, out.map(|x| x * gain)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_shares_take_turns_and_zero_shares_sit_out() {
        let mut runtime = Runtime::new(48_000);
        let mut settings = Settings::default();
        let dealt: Vec<_> = (0..8).map(|_| runtime.deal(&settings).unwrap()).collect();
        assert_eq!(dealt, [0, 1, 2, 3, 0, 1, 2, 3]);
        settings.groups[1].share = 0.0;
        settings.groups[3].share = 0.5;
        runtime.reset();
        let mut counts = [0; GROUPS];
        for _ in 0..500 {
            counts[runtime.deal(&settings).unwrap()] += 1;
        }
        assert_eq!(counts, [200, 0, 200, 100]);
        settings.groups.iter_mut().for_each(|g| g.share = 0.0);
        assert_eq!(runtime.deal(&settings), None);
    }

    #[test]
    fn envelopes_fit_inside_the_grain() {
        let flat = Segments::new(
            &Envelope {
                attack: 0.0,
                decay: 0.0,
                sustain: 1.0,
                release: 0.0,
            },
            100,
        );
        assert!((0..100).all(|age| flat.swept(age as f32) == 1.0));
        let pluck = Segments::new(
            &Envelope {
                attack: 0.1,
                decay: 0.4,
                sustain: 0.25,
                release: 0.2,
            },
            1001,
        );
        assert_eq!(pluck.swept(0.0), 0.0);
        assert!((pluck.swept(100.0) - 1.0).abs() < 1e-6);
        assert!((pluck.swept(600.0) - 0.25).abs() < 1e-6);
        assert!(pluck.swept(999.0) < 0.01);
        assert_eq!(pluck.swept(1000.0), 0.0);
        // Shares adding up to more than the grain are scaled down together.
        let long = Segments::new(
            &Envelope {
                attack: 1.0,
                decay: 0.0,
                sustain: 1.0,
                release: 1.0,
            },
            1001,
        );
        assert!((long.swept(500.0) - 1.0).abs() < 1e-3);
        assert!(long.swept(250.0) > 0.49 && long.swept(250.0) < 0.51);
    }

    #[test]
    fn each_grain_draws_its_own_cutoff_and_level() {
        let mut runtime = Runtime::new(48_000);
        let group = Group {
            random_cutoff: 0.3,
            random_level: 0.5,
            ..Group::default()
        };
        let (mut offsets, mut levels) = (Vec::new(), Vec::new());
        for slot in 0..MAX_GRAINS {
            runtime.start(slot, 0, &group, 1_000);
            offsets.push(runtime.grains[slot].cutoff_offset);
            levels.push(runtime.grains[slot].level);
        }
        assert!(offsets.iter().all(|o| o.abs() <= 0.3));
        assert!(offsets.iter().any(|&o| o < -0.15) && offsets.iter().any(|&o| o > 0.15));
        assert!(levels.iter().all(|&l| (0.5..=1.0).contains(&l)));
        assert!(levels.iter().any(|&l| l < 0.6) && levels.iter().any(|&l| l > 0.9));
        // A random cutoff below the top keeps a filter open at 100% working.
        assert!(runtime.grains.iter().any(|g| g.filtering[0]));
        runtime.start(0, 0, &Group::default(), 1_000);
        assert_eq!(
            (runtime.grains[0].cutoff_offset, runtime.grains[0].level),
            (0.0, 1.0)
        );
        assert_eq!(runtime.grains[0].filtering, [false; 2]);
    }

    #[test]
    fn the_default_amp_envelope_is_the_engines_window() {
        let duration = 4_801;
        let window = Segments::new(&Envelope::default(), duration);
        for shape in [0.0, 0.4, 0.9, 1.0] {
            for age in (0..duration).step_by(37) {
                let phase = 2.0 * age as f32 / (duration - 1) as f32;
                let engine = tables::grain_window(phase, shape);
                let group = window.shaped(age as f32, shape);
                assert!(
                    (engine - group).abs() < 1e-4,
                    "{shape} {age}: {engine} {group}"
                );
            }
        }
    }
}

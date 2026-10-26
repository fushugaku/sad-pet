//! The advanced grain mode's per-grain state: which group a grain joined, its
//! two envelopes over its own life and its own stereo filter.
use super::{MAX_GRAINS, fast, filter};
use crate::{
    Stereo,
    grain_groups::{Envelope, FilterType, GROUPS, Group, Settings},
    preset,
};

/// A filtering grain's coefficients follow its envelope this often, in samples.
const REFRESH: u32 = 16;

/// An envelope's segment boundaries in samples of one grain's life.
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
        let total = envelope.attack + envelope.decay + envelope.release;
        let scale = duration as f32 / total.max(1.0);
        let [attack, decay, release] =
            [envelope.attack, envelope.decay, envelope.release].map(|share| share * scale);
        let inverse = |length: f32| if length > 0.0 { 1.0 / length } else { 0.0 };
        let end = duration as f32;
        Self {
            attack_end: attack,
            decay_end: attack + decay,
            release_start: end - release,
            end,
            sustain: envelope.sustain,
            attack_scale: inverse(attack),
            decay_scale: inverse(decay),
            release_scale: inverse(release),
        }
    }
    /// Linear attack, then decay and release on squared curves.
    #[inline(always)]
    fn value(&self, age: f32) -> f32 {
        if age < self.attack_end {
            age * self.attack_scale
        } else if age < self.decay_end {
            let t = 1.0 - (age - self.attack_end) * self.decay_scale;
            self.sustain + (1.0 - self.sustain) * t * t
        } else if age < self.release_start {
            self.sustain
        } else {
            let t = ((self.end - age) * self.release_scale).clamp(0.0, 1.0);
            self.sustain * t * t
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Grain {
    /// Started in advanced mode; other grains pass through untouched.
    grouped: bool,
    group: u8,
    /// False when the group's filter stayed open across the whole grain's
    /// envelope as it started, so the grain skips filtering.
    filtering: bool,
    amp: Segments,
    filter_env: Segments,
    state: [[f32; 2]; 2],
    coefficients: [f32; 3],
    k: f32,
}

pub(crate) struct Runtime {
    sample_rate: f32,
    grains: [Grain; MAX_GRAINS],
    /// Smooth weighted round robin: each deal adds every share, the richest
    /// group takes the grain and pays the total back.
    credit: [f32; GROUPS],
}
impl Runtime {
    pub(crate) fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            grains: [Grain::default(); MAX_GRAINS],
            credit: [0.0; GROUPS],
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
        self.grains[slot] = Grain {
            grouped: true,
            group: index as u8,
            filtering: !(group.filter_type == FilterType::LowPass
                && group.cutoff >= 0.999
                && group.env_depth >= 0.0),
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
    /// One frame of a grain through its filter, envelope and level. Filter
    /// and level follow the group's current settings while the grain plays.
    #[inline]
    pub(crate) fn process(
        &mut self,
        slot: usize,
        age: u32,
        frame: Stereo,
        settings: &Settings,
    ) -> Stereo {
        let grain = &mut self.grains[slot];
        if !grain.grouped {
            return frame;
        }
        let group = &settings.groups[grain.group as usize];
        let age_f = age as f32;
        let gain = grain.amp.value(age_f) * group.level;
        if !grain.filtering {
            return frame.map(|x| x * gain);
        }
        let cutoff = group.cutoff + group.env_depth * grain.filter_env.value(age_f);
        let open = group.filter_type == FilterType::LowPass && cutoff >= 0.999;
        if age.is_multiple_of(REFRESH) {
            let hz = filter::cutoff_hz(preset::FilterType::LowPass, cutoff)
                .min(self.sample_rate * 0.45);
            let g = (std::f32::consts::PI * hz / self.sample_rate).tan();
            let k = 2.0 - 1.94 * group.resonance;
            let a1 = 1.0 / (1.0 + g * (g + k));
            let a2 = g * a1;
            grain.coefficients = [a1, a2, g * a2];
            grain.k = k;
        }
        let k = grain.k;
        let [a1, a2, a3] = grain.coefficients;
        std::array::from_fn(|c| {
            let [ic1, ic2] = grain.state[c];
            let v3 = frame[c] - ic2;
            let band = a1 * ic1 + a2 * v3;
            let low = ic2 + a2 * ic1 + a3 * v3;
            grain.state[c] = [fast::flush(2.0 * band - ic1), fast::flush(2.0 * low - ic2)];
            let out = if open {
                frame[c]
            } else {
                match group.filter_type {
                    FilterType::LowPass => low,
                    FilterType::BandPass => band * k.sqrt(),
                    FilterType::HighPass => frame[c] - k * band - low,
                }
            };
            out * gain
        })
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
        let flat = Segments::new(&Envelope::default(), 100);
        assert!((0..100).all(|age| flat.value(age as f32) == 1.0));
        let pluck = Segments::new(
            &Envelope {
                attack: 0.1,
                decay: 0.4,
                sustain: 0.25,
                release: 0.2,
            },
            1000,
        );
        assert_eq!(pluck.value(0.0), 0.0);
        assert!((pluck.value(100.0) - 1.0).abs() < 1e-6);
        assert!((pluck.value(600.0) - 0.25).abs() < 1e-6);
        assert!(pluck.value(999.0) < 0.01);
        // Shares adding up to more than the grain are scaled down together.
        let long = Segments::new(
            &Envelope {
                attack: 1.0,
                decay: 0.0,
                sustain: 1.0,
                release: 1.0,
            },
            1000,
        );
        assert!((long.value(500.0) - 1.0).abs() < 1e-3);
        assert!(long.value(250.0) > 0.49 && long.value(250.0) < 0.51);
    }
}

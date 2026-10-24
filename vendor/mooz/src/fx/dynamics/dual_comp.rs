//! DualComp (DUAL_CMP.ZDL, callback 0..0x880) with its decoded editors and ROM
//! image (`init_tbl` 0x80000678).
//!
//! `mono = (L + R) / 2` splits into a first-order high-pass band (c5..c7) and
//! a first-order low-pass band (c14..c16). Each band is a feedback compressor:
//! the band times its previous gain is clipped, scaled by its sensitivity
//! (c8 / c17), clipped and rectified, smoothed twice (c9/c10 then c11/c12 for
//! the high band, c18/c19 then c20/c21 for the low band) and the new gain is
//! `clip(1 - clip(e c13))` (c22 for the low band). The bands are summed with
//! their auto-level gains (c23 / c25), pass the Tone filter (c28..c32), the
//! fixed c27 = 2 and Level c2, and replace both channels.
//!
//! Editors: Hi → c8 = `Sense_tbl[Hi]` and c23 = `autolvl_tbl[Hi]`, Lo → c17
//! and c25 from the same tables (Hi/Lo 0 switch a band's compression off),
//! all through 16 ms glides. Freq copies `hpf_tbl`/`lpf_tbl`: bilinear
//! first-order sections at 300..1500 Hz (high band) and 200..1500 Hz (low band,
//! one step lower below 1 kHz), designed here at the host rate. Tone asks
//! Main's filter designer (c00bc2e0 type 8) for a 4 kHz first-order high
//! shelf of `2 Tone` dB, also designed at the host rate. Level → c2 = `v / 100`.
use super::common::{self, FirstOrder, Glide, clip, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dual_comp",
    name: "DualComp",
    category: Category::Dynamics,
    zdl: "DUAL_CMP.ZDL",
    description: "Two-band compressor with adjustable crossover",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Hi",
            max: 50,
            default: 24,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Lo",
            max: 50,
            default: 15,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Freq",
            max: 9,
            default: 9,
            display: Display::labels(&[
                "300Hz", "400Hz", "500Hz", "600Hz", "700Hz", "800Hz", "900Hz", "1.0kHz", "1.2kHz",
                "1.5kHz",
            ]),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 2,
            display: Display::INT,
        },
    ],
};

/// Hi/Lo → c8/c17 (Fx_DYN_DualComp_Sense_tbl) (DUAL_CMP 0x800004d8).
const SENSE: [u32; 51] = [
    0x00000000, 0x3fcd0000, 0x3fd6a000, 0x3fe0f000, 0x3febc000, 0x3ff72000, 0x40018000, 0x4007c000,
    0x400e4800, 0x40152000, 0x401c4800, 0x4023d000, 0x402bb000, 0x4033f000, 0x403ca000, 0x4045b000,
    0x404f3000, 0x40592800, 0x4063a000, 0x406e9000, 0x407a1000, 0x40830c00, 0x40895800, 0x408ff400,
    0x4096e000, 0x409e2400, 0x40a5c000, 0x40adb800, 0x40b61400, 0x40bed800, 0x40c80800, 0x40d1a800,
    0x40dbbc00, 0x40e65000, 0x40f16400, 0x40fd0000, 0x41049600, 0x410af800, 0x4111a800, 0x4118aa00,
    0x41200200, 0x4127b600, 0x412fc600, 0x41383c00, 0x41411a00, 0x414a6400, 0x41542000, 0x415e5600,
    0x41690800, 0x41743e00, 0x417ffe00,
];
/// Hi/Lo → c23/c25 (Fx_DYN_DualComp_autolvl_tbl) (DUAL_CMP 0x800005a8).
const AUTO_LEVEL: [u32; 51] = [
    0x3f810000, 0x3f810000, 0x3f810000, 0x3f912000, 0x3f912000, 0x3f912000, 0x3f912000, 0x3fa14000,
    0x3fa14000, 0x3fa14000, 0x3fb16000, 0x3fb16000, 0x3fc18000, 0x3fc18000, 0x3fc18000, 0x3fd1a000,
    0x3fd1a000, 0x3fe1c000, 0x3fe1c000, 0x3ff1e000, 0x3ff1e000, 0x40010000, 0x40010000, 0x40091000,
    0x40091000, 0x40112000, 0x40112000, 0x40193000, 0x40214000, 0x40214000, 0x40295000, 0x40316000,
    0x40316000, 0x40397000, 0x40418000, 0x40499000, 0x40499000, 0x4051a000, 0x4059b000, 0x4061c000,
    0x4069d000, 0x4071e000, 0x4079f000, 0x40810000, 0x40850800, 0x40891000, 0x408d1800, 0x40912000,
    0x40993000, 0x409d3800, 0x40a14000,
];
/// Corners of `hpf_tbl` and `lpf_tbl` (verified against the ROM words).
const HIGH_BAND_HZ: [f32; 10] = [
    300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1200.0, 1500.0,
];
const LOW_BAND_HZ: [f32; 10] = [
    200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 1000.0, 1200.0, 1500.0,
];
/// Tone: Main designer type 8 at 4000 Hz, gain curve(20, 0, 11, Tone) dB.
const TONE_HZ: f32 = 4000.0;
/// ROM detector words: c9..c13 (high band) and c18..c22 (low band).
const HIGH_DETECTOR: [u32; 5] = [0x3c02f001, 0x3f7fbe88, 0x3ff9001d, 0x3f3fbde8, 0x3f7ffe00];
const LOW_DETECTOR: [u32; 5] = [0x3b82f001, 0x3f7fbe88, 0x3e892f55, 0x3f732390, 0x3f780000];
/// ROM c27.
const WET: f32 = 2.0;

/// One band's detector at the host rate.
#[derive(Debug, Clone, Copy)]
struct Detector {
    first: (f32, f32),
    second: (f32, f32),
    depth: f32,
}

impl Detector {
    fn new(words: [u32; 5], sample_rate: f32) -> Self {
        let c = words.map(f32::from_bits);
        Self {
            first: one_pole(c[0], c[1], sample_rate),
            second: one_pole(c[2], c[3], sample_rate),
            depth: c[4],
        }
    }
}

struct Band {
    filter: FirstOrder,
    detector: Detector,
    sense: Glide,
    auto_level: Glide,
    first: f32,
    second: f32,
    gain: f32,
}

impl Band {
    fn new(filter: FirstOrder, detector: Detector, sample_rate: f32) -> Self {
        Self {
            filter,
            detector,
            sense: Glide::new(0.0, sample_rate),
            auto_level: Glide::new(1.0, sample_rate),
            first: 0.0,
            second: 0.0,
            gain: 1.0,
        }
    }

    fn set(&mut self, index: usize) {
        self.sense.set(f32::from_bits(SENSE[index]));
        self.auto_level.set(f32::from_bits(AUTO_LEVEL[index]));
    }

    #[inline]
    fn process(&mut self, mono: f32) -> f32 {
        let band = self.filter.process(mono) * self.gain;
        let d = self.detector;
        let rectified = clip(clip(band) * self.sense.tick()).abs();
        self.first = flush(d.first.1 * self.first + rectified * d.first.0);
        self.second = flush(d.second.1 * self.second + clip(self.first) * d.second.0);
        self.gain = clip(1.0 - clip(clip(self.second) * d.depth));
        band * self.auto_level.tick()
    }

    fn clear(&mut self) {
        self.filter.clear();
        self.first = 0.0;
        self.second = 0.0;
        self.gain = 1.0;
        self.sense.snap();
        self.auto_level.snap();
    }
}

struct DualComp {
    sample_rate: f32,
    high: Band,
    low: Band,
    tone: FirstOrder,
    level: Ramp,
    cache: [usize; 4],
    primed: bool,
}

impl Effect for DualComp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let hi = common::index(values[0], 50);
        let lo = common::index(values[1], 50);
        let freq = common::index(values[2], 9);
        let tone = common::index(values[4], 10);
        if hi != self.cache[0] {
            self.high.set(hi);
        }
        if lo != self.cache[1] {
            self.low.set(lo);
        }
        if freq != self.cache[2] {
            let rate = self.sample_rate;
            self.high
                .filter
                .set_coefficients(FirstOrder::highpass(rate, HIGH_BAND_HZ[freq]));
            self.low
                .filter
                .set_coefficients(FirstOrder::lowpass(rate, LOW_BAND_HZ[freq]));
        }
        if tone != self.cache[3] {
            let gain_db = 2.0 * tone as f32;
            self.tone
                .set_coefficients(FirstOrder::high_shelf(self.sample_rate, TONE_HZ, gain_db));
        }
        self.level.set(common::level(values[3]));
        self.cache = [hi, lo, freq, tone];
        if !self.primed {
            self.primed = true;
            for band in [&mut self.high, &mut self.low] {
                band.sense.snap();
                band.auto_level.snap();
            }
            common::settle(&mut self.level);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (left + right);
        let mixed = self.high.process(mono) + self.low.process(mono);
        let wet = self.tone.process(mixed) * WET * self.level.tick();
        [wet, wet]
    }

    fn clear(&mut self) {
        self.high.clear();
        self.low.clear();
        self.tone.clear();
        common::settle(&mut self.level);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(build(sample_rate))
}

fn build(sample_rate: f32) -> DualComp {
    DualComp {
        sample_rate,
        high: Band::new(
            FirstOrder::highpass(sample_rate, HIGH_BAND_HZ[9]),
            Detector::new(HIGH_DETECTOR, sample_rate),
            sample_rate,
        ),
        low: Band::new(
            FirstOrder::lowpass(sample_rate, LOW_BAND_HZ[9]),
            Detector::new(LOW_DETECTOR, sample_rate),
            sample_rate,
        ),
        tone: FirstOrder::IDENTITY,
        level: common::level_ramp(100.0, sample_rate),
        cache: [usize::MAX; 4],
        primed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    fn steady(effect: &mut DualComp, amplitude: f32, hz: f32) -> f32 {
        let rate = effect.sample_rate;
        let frames = rate as usize;
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (i as f32 * std::f32::consts::TAU * hz / rate).sin();
            let [l, r] = effect.process([x, x]);
            assert_eq!(l, r);
            if i > frames / 2 {
                peak = peak.max(l.abs());
            }
        }
        peak
    }

    fn values(hi: f32, lo: f32, freq: f32, tone: f32) -> Values {
        [hi, lo, freq, 100.0, tone, 0.0, 0.0, 0.0, 0.0]
    }

    #[test]
    fn rom_crossover_matches_the_designed_sections() {
        // hpf_tbl[0] and lpf_tbl[0] words of DUAL_CMP 0x80000700 / 0x80000778.
        let high = FirstOrder::highpass(44_100.0, 300.0);
        assert!((high.b0 - 0.979_072_7).abs() < 1e-6);
        let low = FirstOrder::lowpass(44_100.0, 200.0);
        assert!((low.b0 - 0.014_048_38).abs() < 1e-6);
        assert!((low.a1 - 0.971_903_3).abs() < 1e-6);
    }

    #[test]
    fn bands_off_pass_with_the_fixed_gain() {
        let mut effect = build(48_000.0);
        effect.set(&values(0.0, 0.0, 9.0, 0.0), 120.0);
        // Hi/Lo 0: no compression, auto level 1.0078 per band, c27 = 2.
        let out = steady(&mut effect, 0.1, 100.0);
        assert!((out / 0.1 - 2.0 * 1.0078).abs() < 0.1, "{out}");
    }

    #[test]
    fn sensitivity_compresses_its_band() {
        let mut effect = build(44_100.0);
        effect.set(&values(0.0, 40.0, 9.0, 0.0), 120.0);
        let a = steady(&mut effect, 0.05, 100.0);
        let b = steady(&mut effect, 0.5, 100.0);
        let slope = db(b / a) / 20.0;
        assert!(slope < 0.5, "{a} {b} {slope}");
    }

    #[test]
    fn tone_is_a_four_kilohertz_shelf() {
        let shelf = FirstOrder::high_shelf(44_100.0, TONE_HZ, 20.0);
        assert!((shelf.magnitude(44_100.0, 1.0) - 1.0).abs() < 1e-3);
        assert!(db(shelf.magnitude(44_100.0, 16_000.0)) > 14.0);
    }
}

//! The parametric and graphic EQ families. Every band is designed by the Main
//! designer (`common::design`) exactly as the original editors call it, and the
//! bands run in the shared `Equalizer` callback.
use super::common::{Equalizer, Kind, design};
use crate::{Effect, MAX_PARAMS, Values, catalog::Display};

/// Q table of PEQ.ZDL 0x80000728 / B_PEQ.ZDL 0x800007b8.
pub const Q_TABLE: [f32; 6] = [0.5, 1.0, 2.0, 4.0, 8.0, 16.0];
pub const Q_LABELS: Display = Display::labels(&["0.5", "1", "2", "4", "8", "16"]);

fn pick(table: &[f32], value: f32) -> f32 {
    table[(value.round().max(0.0) as usize).min(table.len() - 1)]
}

/// ParaEQ / Ba PEQ: two Zölzer peaks (Main type 10) on the mono sum. Each
/// editor reads the frequency (stride-2 ROM table) and Q tables and passes
/// `gain − offset` dB; values are `[f1, q1, g1, f2, q2, g2, level]`.
pub struct Parametric {
    sample_rate: f32,
    frequencies: &'static [f32],
    offset: f32,
    eq: Equalizer<2>,
    previous: Values,
}

impl Parametric {
    pub fn new(
        frequencies: &'static [f32],
        offset: f32,
        defaults: Values,
        sample_rate: f32,
    ) -> Self {
        let mut effect = Self {
            sample_rate,
            frequencies,
            offset,
            eq: Equalizer::new(false, sample_rate),
            previous: [f32::NAN; MAX_PARAMS],
        };
        effect.set(&defaults, 120.0);
        effect.eq.jump_level(defaults[6]);
        effect
    }

    #[cfg(test)]
    pub fn band(&self, band: usize) -> crate::dsp::Coefficients {
        self.eq.band(band)
    }
}

impl Effect for Parametric {
    fn set(&mut self, values: &Values, _bpm: f32) {
        for band in 0..2 {
            let at = band * 3;
            if values[at..at + 3] != self.previous[at..at + 3] {
                let coefficients = design(
                    Kind::Peak,
                    self.sample_rate,
                    pick(self.frequencies, values[at]),
                    values[at + 2] - self.offset,
                    pick(&Q_TABLE, values[at + 1]),
                );
                self.eq.set_band(band, coefficients);
            }
        }
        self.eq.set_level(values[6]);
        self.previous = *values;
    }
    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.eq.process(frame)
    }
    fn clear(&mut self) {
        self.eq.clear();
    }
}

/// One graphic band as the `_Set` tables of B_GEQ/ST_G_GEQ/ST_B_GEQ store it.
#[derive(Debug, Clone, Copy)]
pub struct GraphicBand {
    pub hz: f32,
    /// dB at the top (24) and bottom (0) of the 0..24 range.
    pub boost: f32,
    pub cut: f32,
    pub q: f32,
    pub kind: Kind,
}

impl GraphicBand {
    /// `_Set`: `curve(cut, 0, 13, v)` below 12, `curve(boost, 0, 13, v − 12)` above.
    pub fn gain_db(&self, value: f32) -> f32 {
        let value = value.clamp(0.0, 24.0);
        if value < 12.0 {
            self.cut * (1.0 - value / 12.0)
        } else {
            self.boost * (value - 12.0) / 12.0
        }
    }
}

/// The guitar bands (ST_G_GEQ 0x80000568; GEQ.ZDL uses the same frequencies,
/// Q √2, ±12 dB and a type-8 shelf at 12 kHz from its ROM table).
pub const GUITAR: [GraphicBand; 6] = {
    const Q: f32 = std::f32::consts::SQRT_2;
    let hz = [160.0, 400.0, 800.0, 3200.0, 6400.0, 12_000.0];
    let mut bands = [GraphicBand {
        hz: 0.0,
        boost: 12.0,
        cut: -12.0,
        q: Q,
        kind: Kind::Peak,
    }; 6];
    let mut i = 0;
    while i < 6 {
        bands[i].hz = hz[i];
        i += 1;
    }
    bands[5].kind = Kind::HighShelf1;
    bands
};

/// The bass bands (B_GEQ 0x80000578.., ST_B_GEQ 0x80000610..): the knobs read
/// 50/120/400/500/800/4.5k/10k but the bands sit at 50/125/390/520/800/4.5k/
/// 10k Hz with per-band ranges.
pub const BASS: [GraphicBand; 7] = {
    let hz = [50.0, 125.0, 390.0, 520.0, 800.0, 4500.0, 10_000.0];
    let boost = [17.5, 17.0, 13.0, 11.0, 14.0, 15.5, 14.0];
    let cut = [-17.0, -16.5, -13.0, -11.0, -13.0, -15.0, -14.0];
    let q = [2.0, 2.5, 5.5, 5.0, 5.0, 5.0, 1.0];
    let mut bands = [GraphicBand {
        hz: 0.0,
        boost: 0.0,
        cut: 0.0,
        q: 1.0,
        kind: Kind::Peak,
    }; 7];
    let mut i = 0;
    while i < 7 {
        bands[i] = GraphicBand {
            hz: hz[i],
            boost: boost[i],
            cut: cut[i],
            q: q[i],
            kind: Kind::Peak,
        };
        i += 1;
    }
    bands[6].kind = Kind::HighShelf1;
    bands
};

/// GraphicEQ / Ba GEQ (mono) and St Gt GEQ / St Bs GEQ (stereo); values are
/// `[band 0, …, band N − 1, level]`.
pub struct Graphic<const N: usize> {
    sample_rate: f32,
    bands: &'static [GraphicBand; N],
    eq: Equalizer<N>,
    previous: [f32; N],
}

impl<const N: usize> Graphic<N> {
    pub fn new(
        bands: &'static [GraphicBand; N],
        stereo: bool,
        defaults: Values,
        sample_rate: f32,
    ) -> Self {
        let mut effect = Self {
            sample_rate,
            bands,
            eq: Equalizer::new(stereo, sample_rate),
            previous: [f32::NAN; N],
        };
        effect.set(&defaults, 120.0);
        effect.eq.jump_level(defaults[N]);
        effect
    }

    #[cfg(test)]
    pub fn magnitude(&self, hz: f32) -> f32 {
        self.eq.magnitude(self.sample_rate, hz)
    }
}

impl<const N: usize> Effect for Graphic<N> {
    fn set(&mut self, values: &Values, _bpm: f32) {
        for (i, band) in self.bands.iter().enumerate() {
            if values[i] != self.previous[i] {
                self.previous[i] = values[i];
                let coefficients = design(
                    band.kind,
                    self.sample_rate,
                    band.hz,
                    band.gain_db(values[i]),
                    band.q,
                );
                self.eq.set_band(i, coefficients);
            }
        }
        self.eq.set_level(values[N]);
    }
    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.eq.process(frame)
    }
    fn clear(&mut self) {
        self.eq.clear();
    }
}

/// The ±12 dB display of the graphic bands (`GetString_offset_minus12`).
pub const BAND_DB: Display = Display::number(-12.0, 1.0, 0, " dB");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn graphic_bands_hit_their_gains() {
        let fs = 48_000.0;
        let mut values = [12.0; MAX_PARAMS];
        values[6] = 100.0;
        values[2] = 24.0;
        let eq = Graphic::new(&GUITAR, false, values, fs);
        assert!((gain_to_db(eq.magnitude(800.0)) - 12.0).abs() < 0.5);
        assert!(gain_to_db(eq.magnitude(50.0)).abs() < 0.3);
        values[2] = 12.0;
        values[5] = 0.0;
        let eq = Graphic::new(&GUITAR, false, values, fs);
        assert!((gain_to_db(eq.magnitude(20_000.0)) + 12.0).abs() < 1.0);
    }

    #[test]
    fn bass_bands_use_their_own_ranges() {
        assert_eq!(BASS[0].gain_db(24.0), 17.5);
        assert_eq!(BASS[0].gain_db(0.0), -17.0);
        assert_eq!(BASS[3].gain_db(18.0), 5.5);
        assert_eq!(BASS[2].gain_db(12.0), 0.0);
    }
}

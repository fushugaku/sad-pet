//! One owner for oscillator backends and their shared directed interactions.
//! Legacy source groups keep their original kernels and render arithmetic.
use super::{interaction, resynth, synth, tb303, wavetable};

pub(super) struct OscillatorEngine {
    pub synth: Option<Box<synth::Runtime>>,
    pub wavetable: Option<Box<wavetable::Runtime>>,
    pub resynth: Box<resynth::Resynth>,
    pub tb303: Option<Box<tb303::Runtime>>,
    pub graph: Box<interaction::Runtime>,
    pub mixed: Option<Box<wavetable::mixed::Runtime>>,
    pub phase_adapters: Option<Box<super::phase_transport::Runtime>>,
}

#[derive(Default)]
pub struct Prepared(
    pub Option<Box<wavetable::mixed::Runtime>>,
    pub Option<String>,
);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedOscillators")
            .field("ready", &self.0.is_some())
            .field("error", &self.1)
            .finish()
    }
}
impl Prepared {
    pub fn prepare(
        &mut self,
        settings: Option<&crate::oscillators::Settings>,
        assets: &crate::wavetable::assets::Pool,
        rate: u32,
    ) -> Result<(), String> {
        self.prepare_reusing(settings, assets, rate, None)
    }
    pub(crate) fn prepare_reusing(
        &mut self,
        settings: Option<&crate::oscillators::Settings>,
        assets: &crate::wavetable::assets::Pool,
        rate: u32,
        previous: Option<&wavetable::mixed::Runtime>,
    ) -> Result<(), String> {
        self.1 = None;
        self.0 = match settings {
            Some(s) => match previous.map_or_else(
                || wavetable::mixed::Runtime::new(rate, s, assets),
                |old| wavetable::mixed::Runtime::prepare(rate, s, assets, Some(old)),
            ) {
                Ok(runtime) => Some(Box::new(runtime)),
                Err(error) => {
                    self.1 = Some(error.clone());
                    return Err(error);
                }
            },
            None => None,
        };
        Ok(())
    }
}

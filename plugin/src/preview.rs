//! A short raw-source audition, independent of the scene and its saved state.
use super::*;

pub(super) struct Preview {
    wavetable: Option<Box<sad_pet::dsp::wavetable::Audition>>,
    sample: Arc<SampleBuffer>,
    position: f64,
    step: f64,
    end: f64,
}
impl Preview {
    fn new(sample: Arc<SampleBuffer>, rate: u32) -> Self {
        let end = (sample.sample_rate as f64 * 8.0).min(sample.len() as f64);
        Self {
            wavetable: None,
            step: sample.sample_rate as f64 / rate as f64,
            sample,
            position: 0.0,
            end,
        }
    }
    pub(super) fn stop(&mut self) {
        if let Some(wavetable) = &mut self.wavetable {
            wavetable.stop();
            return;
        }
        self.end = self
            .end
            .min(self.position + self.sample.sample_rate as f64 * 0.015);
    }
    pub(super) fn next(&mut self) -> Stereo {
        if let Some(wavetable) = &mut self.wavetable {
            return wavetable.process_frame();
        }
        if self.position >= self.end {
            return [0.0; 2];
        }
        let rate = self.sample.sample_rate as f64;
        let fade = (self.position / (rate * 0.008)).min(1.0)
            * ((self.end - self.position) / (rate * 0.02)).min(1.0);
        let frame = self
            .sample
            .read(self.position, self.step, true)
            .map(|v| v * fade as f32 * 0.5);
        self.position += self.step;
        // Keep the allocation until a control-thread command retires it.
        frame
    }
}

impl Controller {
    pub(super) fn wavetable_audition(&mut self, data: &Value) -> Result<Value, String> {
        let generation = data["generation"]
            .as_u64()
            .ok_or("Missing audition generation")?;
        if generation <= self.preview_generation {
            return Ok(json!({"seconds":0}));
        }
        self.preview_generation = generation;
        let Some(id) = data["id"].as_str() else {
            self.enqueue(Command::PreviewStop)?;
            return Ok(json!({"seconds":0}));
        };
        let current = if self.active == 0 {
            &*self
        } else {
            self.parts
                .iter()
                .find(|p| p.meta.id == self.active)
                .ok_or("Unknown timbre")?
        };
        let audition =
            sad_pet::dsp::wavetable::Audition::new(id, &current.preset.wavetable_pool, self.rate)?;
        let mut preview = Preview::new(self.sample.clone(), self.rate);
        preview.wavetable = Some(Box::new(audition));
        self.enqueue(Command::Preview(Some(Box::new(preview))))?;
        Ok(json!({"seconds":0.75}))
    }
    pub(super) fn preview_request(&mut self, data: &Value) -> Result<Value, String> {
        if self.standalone && data.get("base64").is_some() {
            return Err("The app previews audio natively.".into());
        }
        let generation = data["generation"]
            .as_u64()
            .ok_or("Missing preview generation")?;
        if generation <= self.preview_generation {
            return Ok(json!({"seconds":0}));
        }
        self.preview_generation = generation;
        let Some(id) = data["id"].as_str() else {
            self.enqueue(Command::PreviewStop)?;
            return Ok(json!({"seconds":0}));
        };
        let sample = if let Some(track) = id
            .strip_prefix("factory:")
            .and_then(|s| s.strip_suffix(":v1"))
        {
            sad_pet::samples::get(track).ok_or("Unknown factory sample")?
        } else if self.standalone {
            match self.files.read(id) {
                Ok(sample) => Arc::new(sample),
                Err(error) => {
                    let current = if self.active == 0 {
                        Some(&*self)
                    } else {
                        self.parts.iter().find(|p| p.meta.id == self.active)
                    };
                    match current.filter(|p| p.sample.library_id.as_deref() == Some(id)) {
                        Some(part) => part.sample.clone(),
                        None => return Err(error),
                    }
                }
            }
        } else {
            let encoded = data["base64"].as_str().ok_or("Missing preview audio")?;
            if encoded.len() > MAX_BYTES.div_ceil(3) * 4 {
                return Err("File exceeds 128 MB".into());
            }
            let bytes = BASE64.decode(encoded).map_err(|e| e.to_string())?;
            Arc::new(wav::decode(Cursor::new(bytes), "Preview".into())?)
        };
        let seconds = sample.duration().min(8.0);
        self.enqueue(Command::Preview(Some(Box::new(Preview::new(
            sample, self.rate,
        )))))?;
        Ok(json!({"seconds":seconds}))
    }
}

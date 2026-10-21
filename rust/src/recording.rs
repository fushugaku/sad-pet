//! Capture the actual output, independently of transport and offline rendering.
use crate::Stereo;
use serde::Serialize;
use std::sync::Arc;

// Keep a take below 512 MiB; also stop after 30 minutes at lower sample rates.
const MAX_FRAMES: usize = (512 * 1024 * 1024 - 4096) / std::mem::size_of::<Stereo>();

#[derive(Clone, Default, Serialize)]
pub struct View {
    pub active: bool,
    pub id: Option<String>,
    pub seconds: f64,
    pub available: bool,
    pub error: Option<String>,
}

pub struct Take {
    pub id: String,
    pub sample_rate: u32,
    pub frames: Vec<Stereo>,
}

pub struct Recorder {
    sample_rate: u32,
    id: Option<Arc<str>>,
    frames: Vec<Stereo>,
    take: Option<Arc<Take>>,
    error: Option<Arc<str>>,
}

/// Share recording metadata with the UI without copying strings on the DSP thread.
pub(crate) struct Feedback {
    id: Option<Arc<str>>,
    seconds: f64,
    error: Option<Arc<str>>,
    pub take: Option<Arc<Take>>,
}
impl Feedback {
    pub fn view(&self) -> View {
        View {
            active: self.id.is_some(),
            id: self
                .id
                .as_ref()
                .map(|id| id.to_string())
                .or_else(|| self.take.as_ref().map(|take| take.id.clone())),
            seconds: self.seconds,
            available: self.id.is_none() && self.take.is_some(),
            error: self.error.as_ref().map(|error| error.to_string()),
        }
    }
}
impl Recorder {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            id: None,
            frames: Vec::new(),
            take: None,
            error: None,
        }
    }
    pub fn start(&mut self) -> Result<(), String> {
        if self.id.is_some() {
            return Err("Recording is already running.".into());
        }
        self.frames.clear();
        self.frames
            .try_reserve(self.sample_rate as usize * 10)
            .map_err(|e| e.to_string())?;
        self.error = None;
        self.id = Some(
            format!(
                "{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            )
            .into(),
        );
        Ok(())
    }
    pub fn stop(&mut self) -> Result<Arc<Take>, String> {
        if let Some(id) = self.id.take() {
            self.take = Some(Arc::new(Take {
                id: id.to_string(),
                sample_rate: self.sample_rate,
                frames: std::mem::take(&mut self.frames),
            }));
        }
        self.take
            .clone()
            .ok_or_else(|| "There is no recording yet.".into())
    }
    pub fn capture(&mut self, frames: &[Stereo]) {
        if self.id.is_none() {
            return;
        }
        let limit = MAX_FRAMES.min(self.sample_rate as usize * 60 * 30);
        let count = frames.len().min(limit.saturating_sub(self.frames.len()));
        if self.frames.capacity() - self.frames.len() < count
            && let Err(error) = self.frames.try_reserve(self.sample_rate as usize * 10)
        {
            self.error = Some(format!("Recording stopped: {error}").into());
            let _ = self.stop();
            return;
        }
        self.frames.extend_from_slice(&frames[..count]);
        if self.frames.len() == limit {
            self.error = Some("Recording reached the take size limit and was saved.".into());
            let _ = self.stop();
        }
    }
    pub fn take(&self) -> Option<Arc<Take>> {
        self.take.clone()
    }
    pub fn view(&self) -> View {
        self.feedback().view()
    }
    pub(crate) fn feedback(&self) -> Feedback {
        Feedback {
            id: self.id.clone(),
            seconds: if self.id.is_some() {
                self.frames.len() as f64 / self.sample_rate as f64
            } else {
                self.take.as_ref().map_or(0.0, |take| {
                    take.frames.len() as f64 / self.sample_rate as f64
                })
            },
            error: self.error.clone(),
            take: self.take.clone(),
        }
    }
}

pub fn mp3(sample_rate: u32, frames: &[Stereo]) -> Result<Vec<u8>, String> {
    use mp3lame_encoder::{Bitrate, Builder, DualPcm, FlushNoGap, Quality};
    let mut encoder = Builder::new()
        .ok_or("Cannot create MP3 encoder")?
        .with_num_channels(2)
        .map_err(|e| format!("MP3: {e:?}"))?
        .with_sample_rate(sample_rate)
        .map_err(|e| format!("MP3: {e:?}"))?
        .with_brate(Bitrate::Kbps192)
        .map_err(|e| format!("MP3: {e:?}"))?
        .with_quality(Quality::Best)
        .map_err(|e| format!("MP3: {e:?}"))?
        .build()
        .map_err(|e| format!("MP3: {e:?}"))?;
    let mut output = Vec::new();
    let mut left = [0.0f32; 1152];
    let mut right = [0.0f32; 1152];
    for block in frames.chunks(1152) {
        for (i, frame) in block.iter().enumerate() {
            left[i] = frame[0];
            right[i] = frame[1];
        }
        output.reserve(mp3lame_encoder::max_required_buffer_size(block.len()));
        let written = encoder
            .encode(
                DualPcm {
                    left: &left[..block.len()],
                    right: &right[..block.len()],
                },
                output.spare_capacity_mut(),
            )
            .map_err(|e| format!("MP3: {e:?}"))?;
        // LAME initialized exactly `written` bytes inside the reserved capacity.
        unsafe {
            output.set_len(output.len() + written);
        }
    }
    output.reserve(mp3lame_encoder::max_required_buffer_size(0));
    let written = encoder
        .flush::<FlushNoGap>(output.spare_capacity_mut())
        .map_err(|e| format!("MP3: {e:?}"))?;
    // The encoder's flush has initialized this tail of the output buffer.
    unsafe {
        output.set_len(output.len() + written);
    }
    Ok(output)
}

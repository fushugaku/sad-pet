use super::*;
use sad_pet::dsp::source_fx::{self, ProcessedSource, Recipe};

pub(super) struct Stage {
    cache: Option<Arc<ProcessedSource>>,
    recipe: Recipe,
    lengths: Vec<usize>,
    level: usize,
    offset: usize,
}
#[derive(Deserialize)]
struct Begin {
    recipe: Recipe,
    header: Option<Prepared>,
    generation: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_source_prepare() -> u32 {
    host(|h| {
        let result = serde_json::from_slice::<Recipe>(&h.input)
            .map_err(|e| e.to_string())
            .and_then(|r| {
                source_fx::prepare(
                    h.engine.sample.clone(),
                    r.region,
                    r.slot,
                    r.bpm,
                    r.root,
                    || false,
                )
                .map(|cache| cache.map_or_else(Vec::new, |c| pack(&c.sample)))
            });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_source_begin() -> u32 {
    host(|h| {
        let result = serde_json::from_slice::<Begin>(&h.input)
            .map_err(|e| e.to_string())
            .and_then(|begin| {
                if begin.generation != h.file_generation
                    || begin.recipe != Recipe::from_engine(&h.engine)
                    || h.engine.live_input.is_some()
                {
                    return Ok(json!({"started":false}));
                }
                let abandoned = h.source_stage.take().and_then(|s| s.cache);
                let mut reusable = abandoned
                    .or_else(|| h.retired_source.take())
                    .or_else(|| h.source_pool.pop());
                let cache = if let Some(header) = &begin.header {
                    let range = begin.recipe.region.bounds(h.engine.sample.len())?;
                    if header.lengths.first().copied() != Some(range.len())
                        || header.sample_rate != h.engine.sample.sample_rate
                    {
                        return Err("Source cache length changed".into());
                    }
                    let old = reusable
                        .as_mut()
                        .and_then(Arc::get_mut)
                        .ok_or("No source buffer available")?;
                    old.sample
                        .reset_prepared(&header.lengths)
                        .map_err(str::to_owned)?;
                    old.original = h.engine.sample.clone();
                    old.range = range;
                    old.slot = begin.recipe.slot.ok_or("Missing source effect")?;
                    old.bpm = begin.recipe.bpm;
                    old.root = begin.recipe.root;
                    h.engine.source_fx_error = false;
                    reusable
                } else {
                    if let Some(cache) = reusable {
                        h.source_pool.push(cache);
                    }
                    None
                };
                h.source_stage = Some(Stage {
                    cache,
                    recipe: begin.recipe,
                    lengths: begin.header.map_or_else(Vec::new, |v| v.lengths),
                    level: 0,
                    offset: 0,
                });
                Ok(json!({"started":true}))
            });
        h.finish(result.and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string())))
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_source_generation(generation: u32) {
    host(|h| h.file_generation = generation);
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_source_input() -> *mut Stereo {
    host(|h| h.source_upload.as_mut_ptr())
}
/// 0=pending, 1=published, 2=obsolete, 3=invalid. Bounded copying does not allocate.
#[unsafe(no_mangle)]
pub extern "C" fn sad_source_push(count: usize) -> u32 {
    host(|h| {
        let Some(stage) = &mut h.source_stage else {
            return 2;
        };
        if count > BLOCK {
            return 3;
        }
        let mut copied = 0;
        while copied < count {
            if stage.level >= stage.lengths.len() {
                return 3;
            }
            let n = (stage.lengths[stage.level] - stage.offset).min(count - copied);
            let Some(cache) = stage.cache.as_mut().and_then(Arc::get_mut) else {
                return 3;
            };
            if cache
                .sample
                .write_prepared(
                    stage.level,
                    stage.offset,
                    &h.source_upload[copied..copied + n],
                    stage.lengths[0],
                )
                .is_err()
            {
                return 3;
            }
            copied += n;
            stage.offset += n;
            if stage.offset == stage.lengths[stage.level] {
                stage.level += 1;
                stage.offset = 0;
            }
        }
        if stage.level == stage.lengths.len() {
            if stage.recipe == Recipe::from_engine(&h.engine)
                && h.engine.live_input.is_none()
                && stage
                    .cache
                    .as_ref()
                    .is_none_or(|c| Arc::ptr_eq(&c.original, &h.engine.sample))
            {
                h.engine.exchange_processed_source(&mut stage.cache);
                std::mem::swap(&mut h.retired_source, &mut stage.cache);
                1
            } else {
                std::mem::swap(&mut h.retired_source, &mut stage.cache);
                2
            }
        } else {
            0
        }
    })
}

impl Host {
    pub(super) fn reserve_source_buffers(&mut self) {
        self.engine.clear_processed_sources();
        self.source_stage = None;
        self.retired_source = None;
        self.source_pool.clear();
        let lengths: Vec<_> = self
            .engine
            .sample
            .prepared_levels()
            .iter()
            .map(Vec::len)
            .collect();
        for _ in 0..3 {
            self.source_pool.push(Arc::new(ProcessedSource {
                original: self.engine.sample.clone(),
                sample: SampleBuffer::blank_prepared(
                    self.engine.sample.name.clone(),
                    self.engine.sample.sample_rate,
                    &lengths,
                )
                .expect("valid source capacity"),
                range: 0..self.engine.sample.len(),
                slot: Default::default(),
                bpm: 120.0,
                root: 60.0,
            }));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_source_current_generation() -> u32 {
    host(|h| h.file_generation)
}

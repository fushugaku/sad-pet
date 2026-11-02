//! Heavy compilation stays in the utility instance. The AudioWorklet opts
//! into cache-only resolution and receives ready banks in bounded chunks.
use super::*;
use sad_pet::dsp::wavetable::{Bank, BankHeader, BankStage};

pub(super) fn document_assets(
    document: &Value,
) -> Result<sad_pet::wavetable::assets::Pool, String> {
    let mut pool = sad_pet::wavetable::assets::Pool::default();
    fn visit(value: &Value, pool: &mut sad_pet::wavetable::assets::Pool) -> Result<(), String> {
        if let Some(manifest) = value.get("wavetable_assets") {
            let assets = serde_json::from_value::<Vec<Arc<sad_pet::wavetable::assets::Asset>>>(
                manifest.clone(),
            )
            .map_err(|e| e.to_string())?;
            pool.merge(&sad_pet::wavetable::assets::Pool::from_manifest(&assets)?)?;
        }
        for part in value["timbres"].as_array().into_iter().flatten() {
            visit(part, pool)?;
        }
        Ok(())
    }
    visit(document, &mut pool)?;
    Ok(pool)
}
pub(super) fn hoist_document_assets(document: &mut Value) {
    fn take(value: &mut Value, assets: &mut std::collections::BTreeMap<String, Value>) {
        if let Some(manifest) = value
            .as_object_mut()
            .and_then(|o| o.remove("wavetable_assets"))
        {
            for asset in manifest.as_array().into_iter().flatten() {
                if let Some(id) = asset["id"].as_str() {
                    assets.entry(id.into()).or_insert_with(|| asset.clone());
                }
            }
        }
        for part in value["timbres"].as_array_mut().into_iter().flatten() {
            take(part, assets);
        }
    }
    let mut assets = std::collections::BTreeMap::new();
    take(document, &mut assets);
    if !assets.is_empty() {
        document["wavetable_assets"] = json!(assets.into_values().collect::<Vec<_>>());
    }
}

/// Metadata-only startup: a Wavetable Rack does not need a sample/FX scene.
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_default_preset() -> u32 {
    registry(|r| {
        let i = r.index();
        let preset = sad_pet::timbres::Kind::Wavetable.rack_instrument().unwrap();
        let result = serde_json::to_vec(&preset).map_err(|e| e.to_string());
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_realtime_init() {
    sad_pet::dsp::wavetable::enter_realtime();
}
fn compile_bank(r: &mut timbres::Registry) -> Result<Bank, String> {
    let data: Value =
        serde_json::from_slice(&r.parts[r.index()].host.input).map_err(|e| e.to_string())?;
    let asset = r
        .wavetable_assets
        .resolve(data["id"].as_str().ok_or("Choose a wavetable")?)?;
    Bank::compile_reusing(&asset, std::mem::take(&mut r.wavetable_compile_buffer))
}
/// The worker copies the bank straight into its transferable JS packet.
/// A second bank-sized Rust serialization buffer is unnecessary.
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prepare_view() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = (|| {
            let bank = compile_bank(r)?;
            let header = serde_json::to_vec(&bank.transport_header()).map_err(|e| e.to_string())?;
            r.wavetable_compile_buffer = bank.into_compile_buffer();
            Ok(header)
        })();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prepared_pointer() -> *const f32 {
    registry(|r| r.wavetable_compile_buffer.as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prepared_len() -> usize {
    registry(|r| r.wavetable_compile_buffer.len() * size_of::<f32>())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prepare() -> u32 {
    registry(|r| {
        let i = r.index();
        // Reuse the previous export allocation before compiling the next bank.
        // No duplicate bank-sized transport Vec or retained previous output.
        let mut output = std::mem::take(&mut r.parts[i].host.output);
        output.clear();
        let result = (|| {
            let bank = compile_bank(r)?;
            let header = serde_json::to_vec(&bank.transport_header()).map_err(|e| e.to_string())?;
            output.reserve(4 + header.len() + bank.bytes());
            output.extend_from_slice(&(header.len() as u32).to_le_bytes());
            output.extend_from_slice(&header);
            bank.append_transport_bytes(&mut output);
            r.wavetable_compile_buffer = bank.into_compile_buffer();
            Ok(output)
        })();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prune() {
    registry(|r| {
        r.wavetable_stage = None;
        for part in &mut r.parts {
            let mut retired = [const { None }; sad_pet::wavetable::OSCILLATORS];
            part.host.engine.retire_wavetable_banks(&mut retired);
            let mut retired_mixed = [const { None }; sad_pet::oscillators::SLOTS];
            part.host.engine.retire_oscillator_banks(&mut retired_mixed);
        }
        r.wavetable_banks.retain(|bank| Arc::strong_count(bank) > 1);
    });
}
/// A control-side cache probe avoids re-compiling and exporting unchanged
/// banks for repeated preset edits. It never creates a staging allocation.
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_cached() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = (|| {
            let header: BankHeader =
                serde_json::from_slice(&r.parts[i].host.input).map_err(|e| e.to_string())?;
            header.bytes()?;
            let cached = Bank::cached(&header.content_hash)?
                .is_some_and(|bank| bank.frames == header.frames);
            serde_json::to_vec(&json!({"cached":cached})).map_err(|e| e.to_string())
        })();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_stage_begin() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = (|| {
            let header: BankHeader =
                serde_json::from_slice(&r.parts[i].host.input).map_err(|e| e.to_string())?;
            header.bytes()?;
            r.wavetable_stage = None;
            if let Some(bank) = Bank::cached(&header.content_hash)? {
                if !r.wavetable_banks.iter().any(|b| Arc::ptr_eq(b, &bank)) {
                    if r.wavetable_banks.len() == r.wavetable_banks.capacity() {
                        return Err("Wavetable bank limit reached".into());
                    }
                    r.wavetable_banks.push(bank);
                }
                return serde_json::to_vec(&json!({"cached":true})).map_err(|e| e.to_string());
            }
            if r.wavetable_banks.len() == r.wavetable_banks.capacity() {
                return Err("Wavetable bank limit reached".into());
            }
            r.wavetable_stage = Some(BankStage::begin(header)?);
            serde_json::to_vec(&json!({"cached":false})).map_err(|e| e.to_string())
        })();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_upload() -> *mut u8 {
    registry(|r| r.wavetable_upload.as_mut_ptr())
}
/// 0 = more chunks, 1 = ready, 2 = rejected. No serialization in the normal
/// per-quantum copy. Partial/failed stages never replace the sounding bank.
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_stage_chunk(length: u32) -> u32 {
    registry(|r| {
        let result = (|| {
            let bytes = r
                .wavetable_upload
                .get(..length as usize)
                .ok_or("Invalid wavetable transfer length")?;
            let done = r
                .wavetable_stage
                .as_mut()
                .ok_or("No wavetable transfer is pending")?
                .append(bytes)?;
            Ok::<_, String>(done)
        })();
        match result {
            Ok(false) => 0,
            Ok(true) => 1,
            Err(error) => {
                r.wavetable_stage = None;
                let i = r.index();
                r.parts[i].host.finish(Err(error));
                2
            }
        }
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_stage_finish() -> u32 {
    registry(|r| {
        let result = (|| {
            let bank = r
                .wavetable_stage
                .take()
                .ok_or("No wavetable transfer is pending")?
                .finish()?;
            r.wavetable_banks.push(bank);
            serde_json::to_vec(&json!({"ready":true})).map_err(|e| e.to_string())
        })();
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_stage_cancel() {
    registry(|r| r.wavetable_stage = None);
}

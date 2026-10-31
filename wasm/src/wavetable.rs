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

#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_realtime_init() {
    sad_pet::dsp::wavetable::enter_realtime();
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prepare() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = (|| {
            let data: Value =
                serde_json::from_slice(&r.parts[i].host.input).map_err(|e| e.to_string())?;
            let bank = Bank::resolve(
                data["id"].as_str().ok_or("Choose a wavetable")?,
                &r.wavetable_assets,
            )?;
            let header = serde_json::to_vec(&bank.transport_header()).map_err(|e| e.to_string())?;
            let mut bytes = Vec::with_capacity(4 + header.len() + bank.bytes());
            bytes.extend_from_slice(&(header.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&header);
            bytes.extend_from_slice(&bank.transport_bytes());
            Ok(bytes)
        })();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_wavetable_prune() {
    registry(|r| {
        r.wavetable_stage = None;
        r.wavetable_banks.retain(|bank| Arc::strong_count(bank) > 1);
    });
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

//! Control-side table browser shared by native and the browser utility worker.
use super::{
    assets::{Asset, Pool},
    factory, import,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

pub fn record(asset: &Asset) -> Value {
    json!({"id":asset.id,"name":asset.name,"frames":asset.frame_count,
        "content_hash":asset.content_hash,"author":asset.author,"provenance":asset.provenance,
        "pack":"user","pack_name":"My Wavetables"})
}
pub fn command(path: &str, data: &Value, pool: &mut Pool) -> Result<Value, String> {
    match path {
        "/api/wavetables/presets" => Ok(json!(
            super::presets::INSTRUMENTS
                .iter()
                .map(|p| p.record())
                .collect::<Vec<_>>()
        )),
        "/api/wavetables/preset" => {
            super::presets::find(data["id"].as_str().ok_or("Choose an instrument preset")?)
                .ok_or("Unknown factory wavetable instrument")?
                .document()
        }
        "/api/wavetables" => {
            let mut tables: Vec<_> = factory::FACTORY.iter().map(|f| json!({
                "id":f.id,"name":f.name,"pack":f.pack,"pack_name":f.pack_name,
                "frames":f.frames,"content_hash":f.content_hash,
                "author":"SAD PET","provenance":"Original procedural waves; no third-party recordings or wavetable data"
            })).collect();
            tables.extend(pool.user_assets().map(|a| record(a)));
            tables.extend(pool.disk_records());
            Ok(
                json!({"tables":tables,"frame_sizes":import::PERIODS,"max_frames":super::MAX_FRAMES,"max_bytes":import::MAX_BYTES}),
            )
        }
        "/api/wavetables/preview" => {
            let asset = pool.resolve(data["id"].as_str().ok_or("Choose a wavetable")?)?;
            let mut value = record(&asset);
            value["waves"] = json!(asset.preview(128));
            if let Some(factory) = factory::find(&asset.id) {
                value["pack"] = json!(factory.pack);
                value["pack_name"] = json!(factory.pack_name);
            }
            Ok(value)
        }
        "/api/wavetables/import" => {
            let text = data["wav"].as_str().ok_or("Choose a WAV file")?;
            if text.len() > import::MAX_BYTES.div_ceil(3) * 4 {
                return Err("The wavetable WAV exceeds 16 MiB".into());
            }
            let bytes = STANDARD.decode(text).map_err(|_| "Invalid WAV upload")?;
            if data["inspect"] == true {
                return serde_json::to_value(import::inspect(&bytes)?).map_err(|e| e.to_string());
            }
            let channel = data
                .get("channel")
                .filter(|v| !v.is_null())
                .map(|v| serde_json::from_value(v.clone()))
                .transpose()
                .map_err(|_| "Choose Left, Right or Mix")?;
            let asset = import::decode(
                &bytes,
                data["name"].as_str().unwrap_or("Imported Wavetable").into(),
                data["frame_size"].as_u64().map(|n| n as usize),
                channel,
            )?;
            if data["preview"] == true {
                let asset = pool.preview(data["generation"].as_u64().unwrap_or(0), asset)?;
                let mut value = record(&asset);
                value["waves"] = json!(asset.preview(128));
                return Ok(value);
            }
            let existing = pool.contains(&asset.id);
            pool.admit(&asset)?;
            pool.persist(&asset)?;
            let asset = pool.insert(asset)?;
            let mut value = record(&asset);
            value["waves"] = json!(asset.preview(128));
            value["already_present"] = json!(existing);
            Ok(value)
        }
        "/api/wavetables/make" => {
            let text = data["wav"].as_str().ok_or("Choose or capture audio")?;
            if text.len() > import::MAX_BYTES.div_ceil(3) * 4 {
                return Err("The audio WAV exceeds 16 MiB".into());
            }
            let bytes = STANDARD.decode(text).map_err(|_| "Invalid audio upload")?;
            let settings: super::maker::Settings =
                serde_json::from_value(data["settings"].clone()).map_err(|e| e.to_string())?;
            let asset = super::maker::make(
                &bytes,
                data["name"].as_str().unwrap_or("Captured Wavetable").into(),
                &settings,
            )?;
            let asset = pool.preview(
                data["generation"]
                    .as_u64()
                    .ok_or("Missing preview generation")?,
                asset,
            )?;
            let mut value = record(&asset);
            value["waves"] = json!(asset.preview(128));
            Ok(value)
        }
        "/api/wavetables/edit" => {
            let source = pool.resolve(data["id"].as_str().ok_or("Choose a table to edit")?)?;
            let settings: super::edit::Settings =
                serde_json::from_value(data["settings"].clone()).map_err(|e| e.to_string())?;
            let asset = super::edit::edit(
                &source,
                data["name"].as_str().unwrap_or(&source.name).into(),
                &settings,
            )?;
            let asset = pool.preview(
                data["generation"]
                    .as_u64()
                    .ok_or("Missing preview generation")?,
                asset,
            )?;
            let mut value = record(&asset);
            value["waves"] = json!(asset.preview(128));
            Ok(value)
        }
        "/api/wavetables/commit" => {
            let (asset, existing) = pool.commit_preview(
                data["generation"]
                    .as_u64()
                    .ok_or("Missing preview generation")?,
                data["id"].as_str().ok_or("Choose a preview")?,
            )?;
            let mut value = record(&asset);
            value["waves"] = json!(asset.preview(128));
            value["already_present"] = json!(existing);
            Ok(value)
        }
        "/api/wavetables/export" => {
            let asset = pool.resolve(data["id"].as_str().ok_or("Choose a wavetable")?)?;
            Ok(
                json!({"name":format!("{}.wav",asset.name),"wav":STANDARD.encode(import::encode(&asset)?)}),
            )
        }
        "/api/wavetables/assets" => Ok(json!(
            pool.user_assets()
                .filter(|a| data["id"].as_str().is_none_or(|id| a.id == id))
                .collect::<Vec<_>>()
        )),
        "/api/wavetables/discard" => {
            pool.remove(data["id"].as_str().ok_or("Choose a wavetable")?);
            Ok(json!({"ok":true}))
        }
        "/api/wavetables/preview-clear" => {
            pool.clear_preview(data["generation"].as_u64().unwrap_or(0));
            Ok(json!({"ok":true}))
        }
        "/api/wavetables/restore" => {
            let assets = serde_json::from_value::<Vec<std::sync::Arc<Asset>>>(data.clone())
                .map_err(|e| e.to_string())?;
            let incoming = Pool::from_manifest(&assets)?;
            pool.merge(&incoming)?;
            Ok(json!({"ok":true}))
        }
        _ => Err("Unknown wavetable action".into()),
    }
}

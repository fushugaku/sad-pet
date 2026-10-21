//! Small portable effect patches, shared by all native instances and racks.
use super::preset_library::{name, timestamp, unique_id, valid_id};
use super::{BASE64, Controller};
use base64::Engine as _;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use sad_pet::{
    color::{ColorSettings, ColorSlot},
    preset::SaturatorType,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

const MAGIC: &[u8; 8] = b"SADFX\x01\0\0";
const MAX_FILE: usize = 64 * 1024;
const MAX_DATA: u64 = 1024 * 1024;

#[derive(Clone, Serialize, Deserialize)]
struct Patch {
    format: String,
    name: String,
    kind: SaturatorType,
    mix: f32,
    settings: ColorSettings,
}
impl Patch {
    fn validate(&mut self) -> Result<(), String> {
        if self.format != "sad-pet-effect-v1" {
            return Err("Use a SAD PET effect preset (.sadfx).".into());
        }
        self.name = name(&self.name)?;
        let mut slot = ColorSlot {
            kind: self.kind,
            mix: self.mix,
            settings: self.settings,
            ..Default::default()
        };
        slot.sanitize();
        self.mix = slot.mix;
        self.settings = slot.settings;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Record {
    id: String,
    created_at: u64,
    modified_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trashed_at: Option<u64>,
    #[serde(flatten)]
    patch: Patch,
}
fn decode(bytes: &[u8]) -> Result<Record, String> {
    if bytes.len() > MAX_FILE || !bytes.starts_with(MAGIC) {
        return Err("Use a SAD PET effect preset (.sadfx, up to 64 KB).".into());
    }
    let mut data = Vec::new();
    GzDecoder::new(&bytes[8..])
        .take(MAX_DATA + 1)
        .read_to_end(&mut data)
        .map_err(|_| "The effect preset is damaged or incomplete.")?;
    if data.len() as u64 > MAX_DATA {
        return Err("The effect preset is too large.".into());
    }
    let mut record: Record =
        serde_json::from_slice(&data).map_err(|_| "Invalid effect preset data.")?;
    if !valid_id(&record.id) {
        return Err("Invalid effect preset identity.".into());
    }
    record.patch.validate()?;
    Ok(record)
}
fn encode(record: &Record) -> Result<Vec<u8>, String> {
    let mut compressed = GzEncoder::new(MAGIC.to_vec(), Compression::new(6));
    serde_json::to_writer(&mut compressed, record).map_err(|e| e.to_string())?;
    let bytes = compressed.finish().map_err(|e| e.to_string())?;
    if bytes.len() > MAX_FILE {
        return Err("The effect preset is too large.".into());
    }
    Ok(bytes)
}
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX_FILE as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_FILE {
        return Err("The effect preset exceeds 64 KB.".into());
    }
    Ok(bytes)
}
impl Controller {
    fn effect_root(&self) -> Result<PathBuf, String> {
        Ok(self.library.root()?.join("Effects"))
    }
    fn effect_path(&self, id: &str) -> Result<PathBuf, String> {
        if !valid_id(id) {
            return Err("Unknown effect preset.".into());
        }
        Ok(self.effect_root()?.join(format!("{id}.sadfx")))
    }
    fn effect_list(&self) -> Result<Vec<Record>, String> {
        let root = self.effect_root()?;
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut records = Vec::new();
        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "sadfx") {
                continue;
            }
            if let Ok(bytes) = read_file(&path)
                && let Ok(record) = decode(&bytes)
                && path.file_stem().and_then(|n| n.to_str()) == Some(&record.id)
            {
                records.push(record);
            }
        }
        records.sort_by(|a, b| {
            a.patch
                .name
                .to_lowercase()
                .cmp(&b.patch.name.to_lowercase())
                .then(a.id.cmp(&b.id))
        });
        Ok(records)
    }
    fn effect_write(&self, mut patch: Patch, id: Option<&str>) -> Result<Record, String> {
        patch.validate()?;
        let records = self.effect_list()?;
        if records.iter().any(|r| {
            r.patch.kind == patch.kind
                && r.patch.name.to_lowercase() == patch.name.to_lowercase()
                && Some(r.id.as_str()) != id
        }) {
            return Err(format!(
                "An effect preset named “{}” already exists.",
                patch.name
            ));
        }
        let previous = id
            .map(|id| {
                records
                    .iter()
                    .find(|r| r.id == id)
                    .ok_or("This preset was removed. Use Save as new.")
            })
            .transpose()?;
        if previous.is_some_and(|r| r.patch.kind != patch.kind) {
            return Err("Choose a preset of the same effect type.".into());
        }
        let record = Record {
            id: previous.map_or_else(unique_id, |r| r.id.clone()),
            created_at: previous.map_or_else(timestamp, |r| r.created_at),
            modified_at: timestamp(),
            trashed_at: None,
            patch,
        };
        fs::create_dir_all(self.effect_root()?).map_err(|e| e.to_string())?;
        let temporary = self.effect_root()?.join(format!(".{}.tmp", unique_id()));
        let result = || -> Result<(), String> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|e| e.to_string())?;
            file.write_all(&encode(&record)?)
                .map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            fs::rename(&temporary, self.effect_path(&record.id)?).map_err(|e| e.to_string())
        };
        if let Err(e) = result() {
            let _ = fs::remove_file(temporary);
            return Err(e);
        }
        Ok(record)
    }
    pub(super) fn effect_presets_request(
        &mut self,
        path: &str,
        post: bool,
        data: &Value,
    ) -> Result<Value, String> {
        let id = || data["id"].as_str().ok_or("Choose an effect preset.");
        match (path, post) {
            ("/api/effect-presets", false) => Ok(json!(self.effect_list()?)),
            ("/api/effect-presets/save", true) => {
                let patch =
                    serde_json::from_value(data["preset"].clone()).map_err(|e| e.to_string())?;
                Ok(json!(self.effect_write(patch, data["id"].as_str())?))
            }
            ("/api/effect-presets/delete", true) => {
                let id = id()?;
                let trash = self.effect_root()?.join("Trash");
                fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
                fs::rename(self.effect_path(id)?, trash.join(format!("{id}.sadfx")))
                    .map_err(|e| e.to_string())?;
                Ok(json!({"ok":true}))
            }
            ("/api/effect-presets/restore", true) => {
                let id = id()?;
                let path = self.effect_path(id)?;
                if path.exists() {
                    return Err("This effect preset is already in the library.".into());
                }
                let record = decode(&read_file(
                    &self
                        .effect_root()?
                        .join("Trash")
                        .join(format!("{id}.sadfx")),
                )?)?;
                if self.effect_list()?.iter().any(|r| {
                    r.patch.kind == record.patch.kind
                        && r.patch.name.to_lowercase() == record.patch.name.to_lowercase()
                }) {
                    return Err("A preset with this name already exists. Rename it before undoing the deletion.".into());
                }
                fs::rename(
                    self.effect_root()?
                        .join("Trash")
                        .join(format!("{id}.sadfx")),
                    path,
                )
                .map_err(|e| e.to_string())?;
                Ok(json!({"ok":true}))
            }
            ("/api/effect-presets/import", true) => {
                let bytes = BASE64
                    .decode(
                        data["base64"]
                            .as_str()
                            .ok_or("Choose an effect preset file.")?,
                    )
                    .map_err(|e| e.to_string())?;
                let mut record = decode(&bytes)?;
                if let Some(kind) = data.get("kind") {
                    let kind: SaturatorType =
                        serde_json::from_value(kind.clone()).map_err(|e| e.to_string())?;
                    if kind != record.patch.kind {
                        return Err("Choose a preset of the same effect type.".into());
                    }
                }
                let existing = self.effect_list()?;
                let base = record.patch.name.clone();
                let mut suffix = 2;
                while existing.iter().any(|r| {
                    r.patch.kind == record.patch.kind
                        && r.patch.name.to_lowercase() == record.patch.name.to_lowercase()
                }) {
                    record.patch.name =
                        format!("{} ({suffix})", base.chars().take(70).collect::<String>());
                    suffix += 1;
                }
                Ok(json!(self.effect_write(record.patch, None)?))
            }
            ("/api/effect-presets/export", true) => {
                let bytes = read_file(&self.effect_path(id()?)?)?;
                decode(&bytes)?;
                Ok(
                    json!({"binary":BASE64.encode(bytes),"mime":"application/x-sad-pet-effect-preset"}),
                )
            }
            _ => Err("Unknown effect preset command.".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::preset_library::Library;
    use super::*;
    fn controller(root: &std::path::Path) -> Controller {
        let mut c = Controller::new(false);
        c.library = Library::at(root.to_path_buf());
        c
    }
    fn patch() -> Value {
        let settings = ColorSettings {
            lcr_sync: false,
            lcr_smooth: false,
            lcr_left_delay: 0.42,
            lcr_feedback: 0.71,
            ..Default::default()
        };
        json!({"format":"sad-pet-effect-v1","name":"Эхо / тихое","kind":"lcr_delay","mix":0.83,"settings":settings})
    }
    #[test]
    fn effect_library_is_shared_portable_and_does_not_touch_the_scene() {
        let root = std::env::temp_dir().join(format!("sad-fx-{}", unique_id()));
        let mut first = controller(&root);
        let before = serde_json::to_value(&first.preset).unwrap();
        let saved = first.request(&json!({"path":"/api/effect-presets/save","method":"POST","data":{"preset":patch()}})).unwrap();
        let id = saved["id"].as_str().unwrap();
        let mut second = controller(&root);
        let listed = second
            .effect_presets_request("/api/effect-presets", false, &Value::Null)
            .unwrap();
        assert_eq!(listed[0]["id"], id);
        assert_eq!(listed[0]["settings"]["lcr_sync"], false);
        assert_eq!(listed[0]["settings"]["lcr_smooth"], false);
        assert_eq!(listed[0]["mix"], saved["mix"]);
        let exported = first
            .effect_presets_request("/api/effect-presets/export", true, &json!({"id":id}))
            .unwrap();
        let bytes = BASE64.decode(exported["binary"].as_str().unwrap()).unwrap();
        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded.patch.name, "Эхо / тихое");
        assert_eq!(decoded.patch.settings.lcr_feedback, 0.71);
        let imported = second
            .effect_presets_request(
                "/api/effect-presets/import",
                true,
                &json!({"base64":exported["binary"]}),
            )
            .unwrap();
        assert_ne!(imported["id"], saved["id"]);
        assert_eq!(imported["name"], "Эхо / тихое (2)");
        assert!(
            second
                .effect_presets_request(
                    "/api/effect-presets/save",
                    true,
                    &json!({"preset":patch()})
                )
                .is_err()
        );
        let mut edited = patch();
        edited["mix"] = json!(0.2);
        let updated = second
            .effect_presets_request(
                "/api/effect-presets/save",
                true,
                &json!({"preset":edited,"id":id}),
            )
            .unwrap();
        assert_eq!(updated["id"], saved["id"]);
        assert_eq!(updated["created_at"], saved["created_at"]);
        assert_eq!(serde_json::to_value(&first.preset).unwrap(), before);
        second
            .effect_presets_request("/api/effect-presets/delete", true, &json!({"id":id}))
            .unwrap();
        assert_eq!(first.effect_list().unwrap().len(), 1);
        second
            .effect_presets_request("/api/effect-presets/restore", true, &json!({"id":id}))
            .unwrap();
        assert_eq!(first.effect_list().unwrap().len(), 2);
        assert!(
            second
                .effect_presets_request(
                    "/api/effect-presets/export",
                    true,
                    &json!({"id":"../file"})
                )
                .is_err()
        );
        assert!(decode(&bytes[..bytes.len() - 4]).is_err());
        assert!(decode(&vec![0; MAX_FILE + 1]).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}

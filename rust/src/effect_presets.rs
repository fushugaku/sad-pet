//! Original factory effect banks, shared by desktop, plug-ins and WASM.
use crate::{
    color::{ColorSettings, ColorSlot},
    preset::SaturatorType,
};
use serde::{Deserialize, Serialize};

const ECHO_BANK: &[u8] = include_bytes!("../../vendor/superecho/default-backup.elb");
const VERB_BANK: &str = include_str!("../../vendor/alphaverb/presets.json");

#[derive(Deserialize)]
struct SourcePreset {
    name: String,
    values: [f32; 20],
}

#[derive(Serialize)]
pub struct FactoryPreset {
    pub id: String,
    pub format: &'static str,
    pub factory: bool,
    pub name: String,
    pub kind: SaturatorType,
    pub mix: f32,
    pub settings: ColorSettings,
}

fn convert(source: SourcePreset, kind: SaturatorType, index: usize) -> FactoryPreset {
    let mut slot = ColorSlot {
        kind,
        ..Default::default()
    };
    // The control registry starts with the original DSP parameter order.
    // Mix lives outside the DSP in SAD PET; factory banks do not store freeze.
    let (count, key) = if kind == SaturatorType::SuperEcho {
        slot.mix = source.values[18];
        slot.settings.super_echo_freeze = 0.0;
        slot.settings.super_echo_sync = 0.0;
        (18, "super_echo")
    } else {
        slot.mix = 0.5; // Dayverb's factory loader retains its default dry/wet.
        slot.settings.alpha_verb_freeze = 0.0;
        (20, "alpha_verb")
    };
    for (&parameter, &value) in slot
        .control_parameters()
        .iter()
        .zip(&source.values[..count])
    {
        *slot.parameter_mut(parameter) = value;
    }
    FactoryPreset {
        id: format!("factory:{key}:{index}"),
        format: "sad-pet-effect-v1",
        factory: true,
        name: source.name,
        kind,
        mix: slot.mix,
        settings: slot.settings,
    }
}

pub fn factory() -> Vec<FactoryPreset> {
    // Pinned original ELB v1: 16-byte header, then 51 rows of 16-byte names
    // and 17 little-endian 12-bit control values. Apply the same pre-Dub
    // upgrades as KekLocator's PresetBank::from_elb / original firmware.
    let echo = ECHO_BANK[16..]
        .chunks_exact(50)
        .enumerate()
        .map(|(index, row)| {
            let end = row[..16].iter().position(|&b| b == 0).unwrap_or(16);
            let mut values = [0.0; 20];
            for (value, bytes) in values.iter_mut().zip(row[16..].chunks_exact(2)) {
                *value = u16::from_le_bytes([bytes[0], bytes[1]]) as f32 / 4095.0;
            }
            values[15] = 0.0;
            values[16] = 2047.0 / 4095.0;
            values[18] = 0.5;
            convert(
                SourcePreset {
                    name: String::from_utf8_lossy(&row[..end]).trim().to_owned(),
                    values,
                },
                SaturatorType::SuperEcho,
                index,
            )
        });
    let verb = serde_json::from_str::<Vec<SourcePreset>>(VERB_BANK)
        .expect("Pinned Dayverb factory presets")
        .into_iter()
        .enumerate()
        .map(|(index, preset)| convert(preset, SaturatorType::AlphaVerb, index));
    echo.chain(verb).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_banks_preserve_every_original_dsp_value() {
        assert_eq!(
            &ECHO_BANK[..12],
            &[239, 191, 19, 26, 1, 0, 0, 0, 51, 0, 0, 0]
        );
        assert_eq!(ECHO_BANK.len(), 16 + 51 * 50);
        let bank = factory();
        assert_eq!(bank.len(), 96);
        assert_eq!(bank[18].name, "GREAT EYE");
        assert_eq!(bank[50].name, "AMBIENTISER");
        assert_eq!(bank[51].name, "INFINITEVOID");
        assert_eq!(bank[95].name, "LEGEND OF HS");
        let source: Vec<SourcePreset> = serde_json::from_str(VERB_BANK).unwrap();
        for (index, preset) in bank.iter().enumerate() {
            let mut slot = ColorSlot {
                kind: preset.kind,
                mix: preset.mix,
                settings: preset.settings,
                ..Default::default()
            };
            slot.sanitize();
            assert_eq!(slot.settings, preset.settings);
            assert_eq!(preset.mix, 0.5);
            if preset.kind == SaturatorType::SuperEcho {
                let values = slot.super_echo_values();
                let row = &ECHO_BANK[16 + index * 50..16 + (index + 1) * 50];
                for (i, value) in values.iter().enumerate().take(15) {
                    let offset = 16 + i * 2;
                    assert_eq!(
                        *value,
                        u16::from_le_bytes([row[offset], row[offset + 1]]) as f32 / 4095.0
                    );
                }
                assert_eq!(&values[15..], &[0.0, 2047.0 / 4095.0, 0.0, 1.0, 0.0]);
                assert_eq!(slot.settings.super_echo_sync, 0.0);
            } else {
                let values = slot.alpha_verb_values();
                assert_eq!(&values[..20], &source[index - 51].values);
                assert_eq!(&values[20..], &[1.0, 0.0]);
            }
        }
    }
}

use sad_pet::{parameters, preset::Preset};
use sha2::{Digest, Sha256};

const RELEASED_PARAMETER_COUNT: usize = 5_802;

#[test]
fn host_parameter_catalog_matches_the_captured_serialized_contract() {
    let specs = parameters::specs();
    assert!(specs.len() >= RELEASED_PARAMETER_COUNT);
    let json = serde_json::to_vec(&specs[..RELEASED_PARAMETER_COUNT]).unwrap();

    assert_eq!(json.len(), 553_906);
    assert_eq!(
        format!("{:x}", Sha256::digest(&json)),
        "c510db118e688622e48562300c6ef696315d4fb05d9aefaff19d64457fe252b1"
    );
}

#[test]
fn scalar_specs_keep_clamping_rounding_and_non_numeric_rejection() {
    let mut preset = Preset::default();
    let changes = parameters::parse(&serde_json::json!({
        "position": 1.5,
        "resynth_mix": -0.5,
        "sample_root_note": 60.6,
    }))
    .unwrap();

    assert!(parameters::apply(&mut preset, &changes));
    assert_eq!(preset.params.position, 1.0);
    assert_eq!(preset.resynth.mix, 0.0);
    assert_eq!(preset.params.sample_root_note, 61.0);
    assert!(parameters::parse(&serde_json::json!({"position": null})).is_err());
}

#[test]
fn waveshaper_modes_keep_their_saved_names_and_host_indices() {
    use sad_pet::{color::WaveshaperMode, grain_groups::Waveshaper};

    // Captured names and order from the released DSP-owned enum. Moving this
    // setting into the model must preserve saved FX and per-grain selections.
    let names = [
        "drive",
        "hard_clip",
        "decimator",
        "oct_saw",
        "multi_triangle",
        "multi_sine",
        "sub_saw",
        "sub_square",
        "sub_triangle",
        "sub_sine",
        "pickup",
        "level_boost",
    ];
    assert_eq!(WaveshaperMode::ALL.len(), names.len());
    for (index, (mode, name)) in WaveshaperMode::ALL.into_iter().zip(names).enumerate() {
        let wire = serde_json::json!(name);
        assert_eq!(serde_json::to_value(mode).unwrap(), wire);
        assert_eq!(
            serde_json::from_value::<WaveshaperMode>(wire.clone()).unwrap(),
            mode
        );

        let grain_mode = Waveshaper::Type(mode);
        assert_eq!(grain_mode.index(), index + 1);
        assert_eq!(Waveshaper::from_index(index + 1), grain_mode);
        assert_eq!(serde_json::to_value(grain_mode).unwrap(), wire);
        assert_eq!(
            serde_json::from_value::<Waveshaper>(wire).unwrap(),
            grain_mode
        );
    }
    assert_eq!(serde_json::to_value(Waveshaper::Off).unwrap(), "off");
    assert_eq!(Waveshaper::from_index(0), Waveshaper::Off);
    assert_eq!(WaveshaperMode::default(), WaveshaperMode::Drive);
}

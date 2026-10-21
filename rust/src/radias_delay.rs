//! RADIAS SYS 2.00 control tables, copied from radias-emulator.
//! Source: ports/octatrack-radias/oracle/reference/radias-synth-infrastructure/
//! src/effect_audio_data.rs (SYS SHA-256
//! ad975a5d42d33c500ec8665104cfe5df184574ac354d1ee09472db15c3036967).
//! Audio topology is adapted from its native reconstruction; original FXD03
//! sample arithmetic is not established by that project.

/// The native RADIAS reconstruction uses 131072 frames at 48 kHz.
pub const MAX_SECONDS: f32 = 131070.0 / 48000.0;

pub fn index(value: f32, count: usize) -> usize {
    (value.clamp(0.0, 1.0) * (count - 1) as f32).round() as usize
}

pub fn seconds(sync: bool, ratio: f32, time: f32, note: f32, bpm: f32, mod_delay: bool) -> f32 {
    // L/C/R ratio is continuous even with musical tap divisions selected.
    // Mod Delay retains the original stepped mode when Smooth is off.
    let ratios = if sync { &SYNC_RATIO } else { &FREE_RATIO };
    let ratio = if mod_delay {
        ratios[index(ratio, 128)] as f32
    } else {
        interpolate(ratios, ratio)
    };
    let seconds = if sync {
        60.0 / bpm.max(1.0) * ratio / 1000.0 * NOTES[index(note, 14)] as f32 / 192.0
    } else {
        let milliseconds = if mod_delay { &MOD_MS } else { &LCR_MS };
        ratio * milliseconds[index(time, 128)] as f32 / 1_000_000.0
    };
    seconds.min(MAX_SECONDS)
}

fn interpolate(table: &[u16], value: f32) -> f32 {
    let position = value.clamp(0.0, 1.0) * (table.len() - 1) as f32;
    let left = position as usize;
    let right = (left + 1).min(table.len() - 1);
    let fraction = position - left as f32;
    table[left] as f32 + (table[right] as f32 - table[left] as f32) * fraction
}

/// Modulators use the knob's continuous physical range. Stepped table positions
/// are retained for saved patches, but must not create dead zones for an LFO.
pub fn modulate_lcr_ratio(sync: bool, base: f32, amount: f32) -> f32 {
    if amount == 0.0 {
        return base;
    }
    let table = if sync { &SYNC_RATIO } else { &FREE_RATIO };
    let first = table[0] as f32;
    let last = table[table.len() - 1] as f32;
    let target = (interpolate(table, base) + amount * (last - first)).clamp(first, last);
    if target <= first {
        return 0.0;
    }
    if target >= last {
        return 1.0;
    }
    let right = table.partition_point(|&value| (value as f32) < target);
    let left = right - 1;
    let fraction = (target - table[left] as f32) / (table[right] - table[left]) as f32;
    (left as f32 + fraction) / (table.len() - 1) as f32
}

/// Keep the musical divisions as anchors, allowing all times between them.
pub fn smooth_seconds(
    sync: bool,
    ratio: f32,
    time: f32,
    note: f32,
    bpm: f32,
    mod_delay: bool,
) -> f32 {
    let seconds = if sync {
        60.0 / bpm.max(1.0) * interpolate(&SYNC_RATIO, ratio) / 1000.0 * interpolate(&NOTES, note)
            / 192.0
    } else {
        let milliseconds = if mod_delay { &MOD_MS } else { &LCR_MS };
        interpolate(&FREE_RATIO, ratio) * interpolate(milliseconds, time) / 1_000_000.0
    };
    seconds.min(MAX_SECONDS)
}

pub fn lfo_hz(value: f32) -> f32 {
    LFO_FREQUENCY[index(value, 128)] as f32 * (1000.0 / 4_294_967_296.0)
}

pub fn display_tables() -> serde_json::Value {
    serde_json::json!({
        "free_ratio": FREE_RATIO.to_vec(), "sync_ratio": SYNC_RATIO.to_vec(),
        "notes": NOTES, "lcr_ms": LCR_MS.to_vec(), "mod_ms": MOD_MS.to_vec(),
        "lfo_hz": LFO_FREQUENCY.map(|v| v as f64 * 1000.0 / 4_294_967_296.0).to_vec(),
        "max_seconds": MAX_SECONDS,
    })
}

pub const FREE_RATIO: [u16; 128] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 110, 125, 130,
    140, 150, 167, 170, 180, 190, 200, 210, 220, 230, 250, 280, 300, 320, 333, 360, 380, 400, 420,
    440, 460, 480, 500, 530, 560, 590, 620, 650, 666, 720, 750, 780, 810, 850, 890, 920, 950, 980,
    1000, 1000, 1000, 1000, 1000, 1010, 1020, 1030, 1040, 1050, 1060, 1070, 1080, 1090, 1100, 1120,
    1140, 1160, 1180, 1200, 1220, 1230, 1250, 1280, 1300, 1333, 1340, 1360, 1380, 1400, 1430, 1460,
    1500, 1520, 1550, 1580, 1610, 1640, 1670, 1700, 1730, 1760, 1790, 1820, 1850, 1880, 1910, 1940,
    1970, 2000, 2050, 2100, 2150, 2200, 2250, 2300, 2400, 2500, 2600, 2700, 2800, 3000, 3200, 3400,
    3600, 3800, 4000,
];

pub const SYNC_RATIO: [u16; 128] = [
    125, 125, 125, 125, 125, 125, 125, 125, 167, 167, 167, 167, 167, 167, 167, 167, 250, 250, 250,
    250, 250, 250, 250, 250, 333, 333, 333, 333, 333, 333, 333, 333, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 666, 666, 666, 666, 666, 666, 666, 666, 666, 750, 750, 750, 750, 750, 750, 750,
    750, 750, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1250, 1250, 1250, 1250, 1250,
    1250, 1250, 1250, 1250, 1330, 1330, 1330, 1330, 1330, 1330, 1330, 1330, 1330, 1500, 1500, 1500,
    1500, 1500, 1500, 1500, 1500, 1500, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 2500,
    2500, 2500, 2500, 2500, 2500, 2500, 2500, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 4000,
    4000, 4000, 4000, 4000, 4000, 4000, 4000,
];

pub const NOTES: [u16; 14] = [
    12, 24, 32, 48, 64, 96, 128, 144, 192, 256, 288, 384, 576, 768,
];

pub const LCR_MS: [u16; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34,
    36, 38, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115, 120, 125, 130, 135,
    140, 145, 150, 155, 160, 165, 170, 175, 180, 185, 190, 200, 210, 220, 230, 240, 250, 260, 270,
    280, 290, 300, 310, 320, 330, 340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460,
    470, 480, 490, 500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650,
    660, 670, 680, 690, 700, 710, 720, 730, 740, 750, 760, 780, 800, 820, 840, 860, 880, 900, 920,
    940, 960, 980, 1000,
];

pub const MOD_MS: [u16; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34,
    36, 38, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115, 120, 125, 130, 135,
    140, 145, 150, 155, 160, 165, 170, 175, 180, 185, 190, 195, 200, 210, 220, 230, 240, 250, 260,
    270, 280, 290, 300, 310, 320, 330, 340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450,
    460, 470, 480, 490, 500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640,
    650, 660, 670, 680, 690, 700, 710, 720, 730, 740, 750, 760, 780, 800, 820, 840, 860, 880, 900,
    920, 940, 960, 980,
];

pub const LFO_FREQUENCY: [u32; 128] = [
    42949, 85899, 128849, 171798, 214748, 257698, 300647, 343597, 386547, 429496, 472446, 515396,
    558345, 601295, 644245, 687194, 730144, 773094, 816043, 858993, 901943, 944892, 987842,
    1030792, 1073741, 1245540, 1417339, 1803886, 2147483, 2491081, 2877628, 3221225, 3564822,
    3951369, 4294967, 4831838, 5368709, 5905580, 6442450, 6979321, 7516192, 8053063, 8589934,
    9126805, 9663676, 10200547, 10737418, 11274289, 11811160, 12348030, 12884901, 13421772,
    13958643, 14495514, 15032385, 15569256, 16106127, 16642998, 17179869, 17716740, 18253611,
    18790481, 19327352, 19864223, 20401094, 20937965, 21474836, 22548578, 23622320, 24696061,
    25769803, 26843545, 27917287, 28991029, 30064771, 31138512, 32212254, 33285996, 34359738,
    35433480, 36507222, 37580963, 38654705, 39728447, 40802189, 41875931, 42949672, 45097156,
    47244640, 49392123, 51539607, 53687091, 55834574, 57982058, 60129542, 62277025, 64424509,
    68719476, 73014444, 77309411, 81604378, 85899345, 92341796, 98784247, 105226698, 111669149,
    118111600, 124554051, 133143986, 141733920, 150323855, 158913789, 167503724, 176093659,
    188978561, 201863462, 214748364, 227633266, 244813135, 261993005, 279172874, 300647710,
    322122547, 343597383, 365072220, 386547056, 408021893, 429496729,
];

//! RackComp (RACKCOMP.ZDL): its callback is byte-identical to M Comp's, so it
//! runs the shared [`super::peak_comp`] kernel with RackComp's own decoded
//! editors and ROM image (0x80000d14).
//!
//! - THRSH and Ratio index one 51 x 11 ROM table (`Fx_DYN_RackComp_thr_tbl`):
//!   column 0 is the threshold c8 (-30 dBFS at 0 to -56 dBFS at 50, higher =
//!   more compression) and column `Ratio + 1` the makeup gain c11 (+0.7 to
//!   +24 dB) that keeps the loudness roughly constant. Both glide over 16 ms.
//! - Ratio → c9 from `ratio_tbl` (2, 3, 4, 5, 7, 9, 12, 18, 32, 60 : 1).
//! - ATTCK → c6 from `attack_tbl`: 1.3 ms (1) to 18 ms (10).
//! - Level → c2 = `v / 100`.
//!
//! ROM: release c7 = 0.99993/sample (320 ms), identity detector (c14 = 1),
//! 0.7 ms gain smoother. Output is mono like the original.
use super::common;
use super::peak_comp::{PeakComp, Rom};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_rack_comp",
    name: "RackComp",
    category: Category::Dynamics,
    zdl: "RACKCOMP.ZDL",
    description: "Rack-style peak compressor with automatic makeup",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 50,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Ratio",
            max: 9,
            default: 5,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "ATTCK",
            max: 9,
            default: 6,
            display: Display::offset(1.0),
        },
    ],
};

/// THRSH → [c8, c11 for Ratio 0..9] (Fx_DYN_RackComp_thr_tbl) (RACKCOMP 0x80000000).
const THRESHOLD: [[u32; 11]; 51] = [
    [
        0x3d0186e3, 0x3f8b9192, 0x3fad158c, 0x3fbf07f2, 0x3fc5b8bb, 0x3fdf5bd5, 0x3fe85d10,
        0x3ff56ead, 0x40098130, 0x4021bc2c, 0x40328e61,
    ],
    [
        0x3cf40004, 0x3f8e11dc, 0x3fb075f7, 0x3fc30846, 0x3fca2920, 0x3fe4fca4, 0x3feecdf2,
        0x3ffd0fa6, 0x400e91d1, 0x4029251c, 0x403c7fa2,
    ],
    [
        0x3ce5d235, 0x3f908217, 0x3fb3d662, 0x3fc708ee, 0x3fcea9e7, 0x3feaad2e, 0x3ff55e9e,
        0x4002685e, 0x4013b281, 0x4030be0e, 0x4046d8ed,
    ],
    [
        0x3cd87754, 0x3f930260, 0x3fb736ce, 0x3fcb1950, 0x3fd32a5a, 0x3ff06e1a, 0x3ffbdf8f,
        0x400650c6, 0x4018e315, 0x40388701, 0x4051923a,
    ],
    [
        0x3ccbe31f, 0x3f9592b8, 0x3fbaa748, 0x3fcf2a06, 0x3fd7aace, 0x3ff62eb2, 0x40014025,
        0x400a3958, 0x401e2bbf, 0x40407818, 0x405cb392,
    ],
    [
        0x3cc00a0b, 0x3f982310, 0x3fbe27d0, 0x3fd34a77, 0x3fdc3ba3, 0x3ffbef9e, 0x40049082,
        0x400e31cf, 0x40237c70, 0x40489100, 0x40683516,
    ],
    [
        0x3cb4e137, 0x3f9ac376, 0x3fc1a858, 0x3fd77af6, 0x3fe0dc33, 0x4000e022, 0x4007e910,
        0x4012324d, 0x4028dd2f, 0x4050da12, 0x40741672,
    ],
    [
        0x3caa5e67, 0x3f9d6388, 0x3fc5288d, 0x3fdbab75, 0x3fe57cc4, 0x4003d07d, 0x400b4174,
        0x401632ca, 0x402e45cc, 0x40594b1f, 0x40803001,
    ],
    [
        0x3ca077f3, 0x3fa013fd, 0x3fc8c932, 0x3fdfdbf5, 0x3fea1d54, 0x4006c0d7, 0x400ea1e0,
        0x401a3b4f, 0x4033c67e, 0x4061dc48, 0x408684cb,
    ],
    [
        0x3c9724c7, 0x3fa2c472, 0x3fcc5975, 0x3fe42c91, 0x3feede01, 0x4009b939, 0x40120a52,
        0x401e4bdc, 0x40394730, 0x406a9542, 0x408d09ab,
    ],
    [
        0x3c8e5c53, 0x3fa584a1, 0x3fd00a28, 0x3fe87d2c, 0x3ff38e4c, 0x400cb9a1, 0x40157aa2,
        0x40226445, 0x403edfce, 0x40736e83, 0x4093be77,
    ],
    [
        0x3c861688, 0x3fa85532, 0x3fd3ba88, 0x3feccd75, 0x3ff85f07, 0x400fba0a, 0x4018eb1c,
        0x402684e0, 0x40447896, 0x407c678c, 0x409a9f56,
    ],
    [
        0x3c7c979b, 0x3fab2570, 0x3fd77af6, 0x3ff12e1f, 0x3ffd2fc2, 0x4012c250, 0x401c639d,
        0x402aa551, 0x404a2142, 0x4082c059, 0x40a1ac32,
    ],
    [
        0x3c6de9fa, 0x3fadf5ae, 0x3fdb3b64, 0x3ff59ed8, 0x4001081c, 0x4015d2c8, 0x401fdbf5,
        0x402ecdc8, 0x404fd1f6, 0x408758e2, 0x40a8e925,
    ],
    [
        0x3c6016b4, 0x3fb0e608, 0x3fdf1bef, 0x3ffa1f4b, 0x40037881, 0x4018e315, 0x4023645a,
        0x4032fe72, 0x405592b8, 0x408c057d, 0x40b04e12,
    ],
    [
        0x3c53111b, 0x3fb3d662, 0x3fe2ec6c, 0x3ffeafcd, 0x4005f8cb, 0x401bfb94, 0x4026ecea,
        0x403736f8, 0x405b5b57, 0x4090ba1f, 0x40b7daf9,
    ],
    [
        0x3c46cd38, 0x3fb6c6bd, 0x3fe6dd05, 0x4001a027, 0x40087914, 0x401f1bef, 0x402a7d56,
        0x403b775c, 0x40612c28, 0x40957ea6, 0x40bf8fef,
    ],
    [
        0x3c3b3fca, 0x3fb9c726, 0x3feadd59, 0x4003f06f, 0x400b0165, 0x40224452, 0x402e0dc3,
        0x403fbff0, 0x406704d5, 0x409a4f4c, 0x40c768f1,
    ],
    [
        0x3c305e39, 0x3fbcd79d, 0x3feede01, 0x400648bf, 0x400d89b5, 0x402574bc, 0x4031ae3e,
        0x40440885, 0x406ce58a, 0x409f27e5, 0x40cf69ec,
    ],
    [
        0x3c261e87, 0x3fbff823, 0x3ff2ee63, 0x4008a10e, 0x40102214, 0x4028a527, 0x40354eba,
        0x404858f7, 0x4072ce46, 0x40a40c89, 0x40d78ef3,
    ],
    [
        0x3c1c774b, 0x3fc31855, 0x3ff70ed4, 0x400b096c, 0x4012ba49, 0x402be575, 0x4038f712,
        0x404cb9a1, 0x4078c711, 0x40a8fd22, 0x40dfd402,
    ],
    [
        0x3c135faa, 0x3fc648e9, 0x3ffb2f45, 0x400d79a7, 0x401562b7, 0x402f25ee, 0x403cafa3,
        0x40511a22, 0x407ec7e3, 0x40adf9c6, 0x40e83904,
    ],
    [
        0x3c0acf4b, 0x3fc98937, 0x3fff6fd2, 0x400fea0c, 0x40180afa, 0x4032764b, 0x4040680a,
        0x405582aa, 0x40826445, 0x40b2fa59, 0x40f0be0e,
    ],
    [
        0x3c02be53, 0x3fccc986, 0x4001e037, 0x4012624e, 0x401abb45, 0x4035c6a8, 0x40442878,
        0x4059fb40, 0x408570a4, 0x40b806f7, 0x40f95f31,
    ],
    [
        0x3bf64ab3, 0x3fd019e3, 0x40040885, 0x4014ea9e, 0x401d73c1, 0x40392713, 0x4047f8f4,
        0x405e73d6, 0x40887d18, 0x40bd1f9f, 0x41010e22,
    ],
    [
        0x3be7facc, 0x3fd37a4e, 0x400638da, 0x401772ef, 0x4020341a, 0x403c877f, 0x404bc971,
        0x4062fc50, 0x408b8d7a, 0x40c23c4b, 0x410578ab,
    ],
    [
        0x3bda7fcf, 0x3fd6eac8, 0x40087914, 0x401a0b4e, 0x4022fc50, 0x403ff7f9, 0x404fa9fc,
        0x406784f5, 0x408ea5ce, 0x40c760e9, 0x4109f141,
    ],
    [
        0x3bcdcd5b, 0x3fda6b51, 0x400ab94e, 0x401ca383, 0x4025d4bf, 0x4043707a, 0x40538a5d,
        0x406c1d7e, 0x4091c23b, 0x40cc9192, 0x410e73cb,
    ],
    [
        0x3bc1d7c9, 0x3fddebd9, 0x400d018e, 0x401f4bf1, 0x4028ad04, 0x4046f0d8, 0x40577af6,
        0x4070be0e, 0x4094e69b, 0x40d1ca43, 0x41130260,
    ],
    [
        0x3bb69420, 0x3fe18c2a, 0x400f51d7, 0x4021f434, 0x402b9581, 0x404a793e, 0x405b736d,
        0x40756ead, 0x40980efe, 0x40d706e2, 0x41179cf5,
    ],
    [
        0x3babf80b, 0x3fe52ccf, 0x4011b22d, 0x4024ac86, 0x402e7dd4, 0x404e11b2, 0x405f7bf2,
        0x407a1f4b, 0x409b3b64, 0x40dc4f8b, 0x411c3f87,
    ],
    [
        0x3ba1f9c9, 0x3fe8dd2f, 0x40141284, 0x40276cdf, 0x40317636, 0x4051b22d, 0x40638c7e,
        0x407edfce, 0x409e6fd2, 0x40e1a03c, 0x4120ee1a,
    ],
    [
        0x3b989031, 0x3fec8d8f, 0x401682bf, 0x402a353f, 0x4034769f, 0x40555aaf, 0x4067a4e8,
        0x4081d845, 0x40a1ac32, 0x40e6f8df, 0x4125a4b3,
    ],
    [
        0x3b8fb29f, 0x3ff05e0b, 0x4018f323, 0x402d05a7, 0x403786ec, 0x40591316, 0x406bcd8a,
        0x40844491, 0x40a4ec96, 0x40ec5586, 0x412a6349,
    ],
    [
        0x3b8758f0, 0x3ff43e96, 0x401b736d, 0x402fe5f3, 0x403a9763, 0x405cd3ae, 0x406ffe09,
        0x4086b8d0, 0x40a83501, 0x40f1c23b, 0x412f2be9,
    ],
    [
        0x3b7ef6f0, 0x3ff81f21, 0x401dfbbd, 0x4032ce46, 0x403db7bf, 0x40609c24, 0x4074368f,
        0x40893127, 0x40ab8573, 0x40f732df, 0x4133fc85,
    ],
    [
        0x3b702607, 0x3ffc1f75, 0x40208c15, 0x4035becb, 0x4040e022, 0x4064747e, 0x40788701,
        0x408bb170, 0x40aed9d3, 0x40fcaf8e, 0x4138d51d,
    ],
    [
        0x3b623181, 0x4000100e, 0x40232474, 0x4038bf0a, 0x40441893, 0x40685d10, 0x407cdfa4,
        0x408e35bd, 0x40b23650, 0x41011a22, 0x413db3bc,
    ],
    [
        0x3b550c99, 0x40022047, 0x4025ccb8, 0x403bbf72, 0x404758e2, 0x406c4d7f, 0x4080a012,
        0x4090c212, 0x40b59eae, 0x4103e080, 0x41429c58,
    ],
    [
        0x3b48ab37, 0x4004307f, 0x402874fb, 0x403ed7c7, 0x404aa969, 0x40704dfd, 0x4082d85a,
        0x4093566d, 0x40b90b24, 0x4106acda, 0x41478cf2,
    ],
    [
        0x3b3d0207, 0x400650c6, 0x402b2d77, 0x4041f045, 0x404df9c6, 0x40745682, 0x408518a8,
        0x4095f2bb, 0x40bc8390, 0x41097f2e, 0x414c8390,
    ],
    [
        0x3b320649, 0x4008710d, 0x402df5ae, 0x404518a8, 0x40516239, 0x4078771d, 0x408760e9,
        0x4098970f, 0x40c00000, 0x410c5586, 0x4151842e,
    ],
    [
        0x3b27adf3, 0x400aa162, 0x4030be0e, 0x4048511a, 0x4054d289, 0x407c9f95, 0x4089ad2e,
        0x409b436c, 0x40c3887b, 0x410f31e4, 0x41568ad0,
    ],
    [
        0x3b1def7f, 0x400cd994, 0x4033967d, 0x404b9168, 0x40584b0a, 0x4080680a, 0x408c057d,
        0x409df7ba, 0x40c718e7, 0x41121446, 0x415b9b71,
    ],
    [
        0x3b14c204, 0x400f11f1, 0x403676c9, 0x404ed9e8, 0x405bd36f, 0x40828c54, 0x408e61d0,
        0x40a0b40f, 0x40cab55f, 0x4114fca4, 0x4160b218,
    ],
    [
        0x3b0c1d0d, 0x40115a32, 0x40396723, 0x4052324d, 0x405f6be3, 0x4084b48d, 0x4090ca19,
        0x40a3786c, 0x40ce59c9, 0x4117eafd, 0x4165d0bb,
    ],
    [
        0x3b03f8af, 0x4013aa7a, 0x403c5f85, 0x40559abf, 0x40630c5f, 0x4086e8e6, 0x40933665,
        0x40a644d0, 0x40d20a3d, 0x411adf5b, 0x416af95d,
    ],
    [
        0x3af89ae4, 0x401602c9, 0x403f67f5, 0x40590b0f, 0x4066c4c6, 0x4089212d, 0x4095aab9,
        0x40a91926, 0x40d5c2b9, 0x411ddbc0, 0x41702a06,
    ],
    [
        0x3aea2897, 0x40186afd, 0x40427842, 0x405c8b97, 0x406a7d56, 0x408b6167, 0x40982b02,
        0x40abf987, 0x40d9872b, 0x4120de20, 0x417564ae,
    ],
    [
        0x3adc8d2c, 0x401ad35b, 0x404590c1, 0x40601c04, 0x406e4dd3, 0x408da9bd, 0x409ab353,
        0x40aee1da, 0x40dd57a8, 0x4123e67b, 0x417aa752,
    ],
    [
        0x3acfbc2f, 0x401d4b9d, 0x4048b924, 0x4063b478, 0x40722657, 0x408ffe09, 0x409d43aa,
        0x40b1d639, 0x40e1342f, 0x4126f8df, 0x417ff400,
    ],
];
/// ATTCK → c6 (Fx_DYN_RackComp_attack_tbl) (RACKCOMP 0x80000d68).
const ATTACK: [u32; 10] = [
    0x3c8e95c5, 0x3c555555, 0x3c316a3b, 0x3c0e95c5, 0x3bd55555, 0x3b8d7f21, 0x3b555555, 0x3b0b51da,
    0x3ae26502, 0x3aa57132,
];
/// Ratio → c9 (Fx_DYN_RackComp_ratio_tbl) (RACKCOMP 0x80000d90).
const RATIO: [u32; 10] = [
    0x3f000000, 0x3eaaaaab, 0x3e800000, 0x3e4ccccd, 0x3e124925, 0x3de38e39, 0x3daaaaab, 0x3d638e39,
    0x3d000000, 0x3c888889,
];
/// ROM c7 (release).
const RELEASE: u32 = 0x3f7ffb52;
const ROM: Rom = Rom {
    detector: [0x3f800000, 0x00000000, 0x00000000],
    smoothing: [0x3d09c63e, 0x3f77639c],
};

struct RackComp {
    core: PeakComp,
    cache: [usize; 3],
}

impl Effect for RackComp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let threshold = common::index(values[0], 50);
        let ratio = common::index(values[1], 9);
        let attack = common::index(values[3], 9);
        if [threshold, ratio] != [self.cache[0], self.cache[1]] {
            // ratio_edit (c9, then c11) also runs first inside thr_edit (c8).
            let row = THRESHOLD[threshold];
            self.core.set_ratio(f32::from_bits(RATIO[ratio]));
            self.core.set_makeup(f32::from_bits(row[ratio + 1]));
            self.core.set_threshold(f32::from_bits(row[0]));
        }
        if attack != self.cache[2] {
            self.core.set_attack(f32::from_bits(ATTACK[attack]));
        }
        self.core.set_level(values[2]);
        self.cache = [threshold, ratio, attack];
        self.core.prime();
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut core = PeakComp::new(&ROM, sample_rate);
    core.set_release(f32::from_bits(RELEASE));
    Box::new(RackComp {
        core,
        cache: [usize::MAX; 3],
    })
}

#[cfg(test)]
mod tests {
    use super::super::peak_comp::tests::steady_peak;
    use super::*;

    fn db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    #[test]
    fn threshold_table_spans_minus_30_to_minus_56_db() {
        assert!((db(f32::from_bits(THRESHOLD[0][0])) + 30.0).abs() < 0.01);
        assert!((db(f32::from_bits(THRESHOLD[50][0])) + 56.0).abs() < 0.01);
        assert!((1.0 / f32::from_bits(RATIO[9]) - 60.0).abs() < 1e-3);
    }

    #[test]
    fn compresses_with_the_selected_ratio_and_makeup() {
        let mut effect = create(44_100.0);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[0] = 0.0; // -30 dBFS
        values[1] = 2.0; // 4:1
        values[2] = 100.0;
        values[3] = 0.0;
        effect.set(&values, 120.0);
        let quiet = steady_peak(effect.as_mut(), 0.005, 44_100.0);
        let makeup = f32::from_bits(THRESHOLD[0][3]);
        assert!((quiet / 0.005 - makeup).abs() < 0.03 * makeup, "{quiet}");
        // Above the threshold the gain falls with level (the static curve
        // `(c8 + (p - c8) / ratio) / p` on the detector peak).
        let a = steady_peak(effect.as_mut(), 0.05, 44_100.0) / 0.05;
        let b = steady_peak(effect.as_mut(), 0.8, 44_100.0) / 0.8;
        assert!(b < 0.5 * a && b > 0.2 * a, "{a} {b}");
    }
}

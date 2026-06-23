use nvfp4_core::e2m1::decode_element;
use nvfp4_core::e4m3::{self, decode_e4m3};

const CASES: &[(u8, f32)] = &[
    (0x2F, 0.46875),       // normal:    (1 + 7/8) · 2^(5-7) = 1.875 · 0.25
    (0x03, 0.005_859_375), // subnormal: (3/8) · 2^-6 = 3/512
    (0x7C, 384.0),         // 接近 max:  (1 + 4/8) · 2^(15-7) = 1.5 · 256
    (0xB0, -0.5),          // 负 normal: -(1 + 0) · 2^(6-7)
];

#[test]
fn matches_e4m3_reference() {
    for &(raw, expected) in CASES {
        assert_eq!(
            decode_e4m3(raw).to_bits(),
            expected.to_bits(),
            "raw: {raw:#04x} mismatched the expected E4M3 value"
        );
    }
}

#[test]
fn end_to_end_block_parity() {
    let packed: [u8; 8] = [0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE];
    let micro_scale: u8 = 0x38; // E4M3 -> 1.0
    let tensor_scale: f32 = 2.0;
    let expected: [f32; 16] = [
        0.0, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, -0.0, -1.0, -2.0, -3.0, -4.0, -6.0, -8.0, -12.0,
    ];

    let mscale = e4m3::decode_e4m3(micro_scale);
    for j in 0..16usize {
        let byte = packed[j / 2];
        let nibble = if j & 1 == 0 {
            byte & 0x0F
        } else {
            (byte >> 4) & 0x0F
        };
        let value = decode_element(nibble) * mscale * tensor_scale;
        assert_eq!(
            value.to_bits(),
            expected[j].to_bits(),
            "element {j} quant error"
        );
    }
}

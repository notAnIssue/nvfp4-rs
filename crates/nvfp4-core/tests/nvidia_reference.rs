use nvfp4_core::e4m3::decode_e4m3;

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

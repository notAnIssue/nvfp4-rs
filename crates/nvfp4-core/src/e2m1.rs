pub const E2M1_LUT: [f32; 16] = [
    0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0, //  sign = 0  (codes 0x0..=0x7)
    -0.0, -0.5, -1.0, -1.5, -2.0, -3.0, -4.0, -6.0, //  sign = 1  (codes 0x8..=0xF)
];

#[must_use]
pub const fn decode_element(nibble: u8) -> f32 {
    E2M1_LUT[(nibble & 0x0F_u8) as usize]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn all_sixteen_codes_match_reference() {
        let expected: [f32; 16] = [
            0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0, -0.0, -0.5, -1.0, -1.5, -2.0, -3.0, -4.0, -6.0,
        ];

        for code in 0u8..16 {
            assert_eq!(
                expected[code as usize].to_bits(),
                decode_element(code).to_bits(),
                "code {code:#06b} decode error."
            );
        }
    }

    #[test]
    fn positive_and_negtive_zero_are_distinct() {
        assert_eq!(decode_element(0x00).to_bits(), 0.0_f32.to_bits());
        assert_eq!(decode_element(0x08).to_bits(), (-0.0_f32).to_bits());
        assert_ne!(
            decode_element(0x00).to_bits(),
            decode_element(0x08).to_bits()
        );
    }

    #[test]
    fn high_nibble_is_masked_off() {
        assert_eq!(
            decode_element(0xF7).to_bits(),
            decode_element(0x07).to_bits()
        );
        assert_eq!(decode_element(0xF7).to_bits(), 6.0_f32.to_bits());
    }
}

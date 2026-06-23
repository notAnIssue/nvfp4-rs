#[must_use]
pub fn decode_e4m3(nibble: u8) -> f32 {
    let bias = 7;
    let sign: f32 = if nibble & 0x80 != 0 {
        -1.0_f32
    } else {
        1.0_f32
    };
    let exp: u8 = (nibble >> 3) & 0x0F;
    let mant: u8 = nibble & 0x07;

    if exp == 0x0F && mant == 0x07 {
        return f32::NAN;
    }

    let mant_frac = f32::from(mant) / 8_f32;
    if exp == 0 {
        // subnormal
        sign * mant_frac * 2_f32.powi(1 - bias)
    } else {
        sign * (1.0 + mant_frac) * 2_f32.powi(i32::from(exp) - bias)
    }
}

#[cfg(test)]
mod tests {
    use crate::e4m3::decode_e4m3;

    #[test]
    fn zeros_are_signed_and_distinct() {
        assert_eq!(decode_e4m3(0x00).to_bits(), 0.0_f32.to_bits());
        assert_eq!(decode_e4m3(0x80).to_bits(), (-0.0_f32).to_bits());
    }

    #[test]
    fn subnormal() {
        assert_eq!(decode_e4m3(0x01).to_bits(), (1.0_f32 / 512.0).to_bits());
        assert_eq!(decode_e4m3(0x07).to_bits(), (7.0_f32 / 512.0).to_bits());
    }

    #[test]
    fn normal() {
        assert_eq!(decode_e4m3(0x08).to_bits(), 0.015_625_f32.to_bits());
        assert_eq!(decode_e4m3(0x38).to_bits(), 1.0_f32.to_bits());
        assert_eq!(decode_e4m3(0x7E).to_bits(), 448.0_f32.to_bits());
        assert_eq!(decode_e4m3(0xFE).to_bits(), (-448.0_f32).to_bits());
    }

    #[test]
    fn nan_encodings() {
        assert!(decode_e4m3(0x7F).is_nan());
        assert!(decode_e4m3(0xFF).is_nan());
    }
}

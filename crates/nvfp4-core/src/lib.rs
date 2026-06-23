// nvfp4 dequant and w4a16 rust logic.
pub mod e2m1;
pub mod e4m3;
pub mod error;
pub mod tensor;

use crate::e2m1::E2M1_LUT;
pub use error::Error;
pub use tensor::QuantTensor;
pub trait QuantFormat {
    const BLOCK_SIZE: usize;
    const HAS_TENSOR_SCALE: bool;

    type MicroScaleRaw: Copy;

    fn decode_micro_scale(raw: Self::MicroScaleRaw) -> f32;

    #[inline]
    #[must_use]
    fn decode_element(nibble: u8) -> f32 {
        E2M1_LUT[(nibble & 0x0F_u8) as usize]
    }
}

pub struct Nvfp4;

impl QuantFormat for Nvfp4 {
    const BLOCK_SIZE: usize = 16;
    const HAS_TENSOR_SCALE: bool = true;
    type MicroScaleRaw = u8;

    fn decode_micro_scale(raw: Self::MicroScaleRaw) -> f32 {
        e4m3::decode_e4m3(raw)
    }
}

const _: () = assert!(Nvfp4::BLOCK_SIZE == 16);

#[must_use]
pub fn sum_block_indices<F: QuantFormat>() -> usize {
    (0..F::BLOCK_SIZE).sum()
}
#[doc(hidden)]
#[must_use]
pub fn sum_block_indices_nvfp4() -> usize {
    sum_block_indices::<Nvfp4>()
}

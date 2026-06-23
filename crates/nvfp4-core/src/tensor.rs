use std::marker::PhantomData;

use crate::{Error, QuantForm};

pub struct QuantTensor<F: QuantForm> {
    pub rows: usize,
    pub cols: usize,
    pub packed: Vec<u8>,
    pub micro_scales: Vec<u8>,
    pub tensor_scale: f32,
    _marker: PhantomData<F>,
}

impl<F: QuantForm> QuantTensor<F> {
    /// new instance of `QuantTensor`
    ///
    /// # Errors
    ///
    /// `Error`: the passed parameter mismatch the preposed shape.
    pub fn new(
        rows: usize,
        cols: usize,
        packed: Vec<u8>,
        micro_scales: Vec<u8>,
        tensor_scale: f32,
    ) -> Result<Self, Error> {
        if !cols.is_multiple_of(F::BLOCK_SIZE) {
            return Err(Error::ShapeMismatch(format!(
                "cols ({cols}) must be a multiple of BLOCK_SIZE ({})",
                F::BLOCK_SIZE
            )));
        }
        let want_packed = rows * cols / 2;
        if packed.len() != want_packed {
            return Err(Error::ShapeMismatch(format!(
                "packed.len() is {}, expected rows*cols/2 = {want_packed}",
                packed.len()
            )));
        }
        let want_scale = rows * cols / F::BLOCK_SIZE;
        if micro_scales.len() != want_scale {
            return Err(Error::ShapeMismatch(format!(
                "micro_scales.len() is {}, expected rows*cols/BLOCK_SIZE = {want_scale}",
                micro_scales.len()
            )));
        }

        Ok(Self {
            rows,
            cols,
            packed,
            micro_scales,
            tensor_scale,
            _marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Nvfp4;

    fn valid_parts() -> (usize, usize, Vec<u8>, Vec<u8>, f32) {
        (2, 16, vec![0u8; 16], vec![0u8; 2], 1.0)
    }

    #[test]
    fn valid_construction_succeeds() {
        let (r, c, p, s, ts) = valid_parts();
        assert!(QuantTensor::<Nvfp4>::new(r, c, p, s, ts).is_ok());
    }

    #[test]
    fn cols_not_multiple_of_block_size_errors() {
        let r = QuantTensor::<Nvfp4>::new(1, 17, vec![0u8; 8], vec![0u8; 1], 1.0);
        assert!(matches!(r, Err(Error::ShapeMismatch(_))));
    }

    #[test]
    fn wrong_packed_len_errors() {
        let (r, c, _, s, ts) = valid_parts();
        let bad = QuantTensor::<Nvfp4>::new(r, c, vec![0u8; 15], s, ts);
        assert!(matches!(bad, Err(Error::ShapeMismatch(_))));
    }

    #[test]
    fn wrong_scale_len_errors() {
        let (r, c, p, _, ts) = valid_parts();
        let bad = QuantTensor::<Nvfp4>::new(r, c, p, vec![0u8; 1], ts);
        assert!(matches!(bad, Err(Error::ShapeMismatch(_))));
    }
}

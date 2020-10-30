mod structures;

pub use structures::*;

use super::traits::CType;
use lzham_sys::lzham_decompress_state_ptr;
use std::{
    io::{BufRead, Write},
    os::raw::c_uint,
};

/// A high level decompressor.
#[derive(Clone, Debug)]
pub struct Decompressor(lzham_decompress_state_ptr);

impl Decompressor {
    /// Creates a new [`Decompressor`] from [`options`].
    ///
    /// [`options`]: DecompressionOptions
    pub fn from_options(options: DecompressionOptions) -> Self {
        Self(unsafe { lzham_sys::lzham_decompress_init(&options.clone().to_c_type()) })
    }

    /// Reinitializes the decompressor.
    pub fn reinit(&self, options: DecompressionOptions) -> Self {
        Self(unsafe { lzham_sys::lzham_decompress_reinit(self.0, &options.to_c_type()) })
    }

    /// Deinitializes the decompressor.
    ///
    /// It cannot be used to decompress further.
    pub fn deinit(&self) -> u32 {
        unsafe { lzham_sys::lzham_decompress_deinit(self.0) as u32 }
    }

    /// Decompresses input data into the output buffer with already specified [`options`].
    ///
    /// [`options`]: DecompressionOptions
    pub fn decompress<R: BufRead, W: Write>(
        &self,
        input: &mut R,
        output: &mut W,
        uncompressed_size: usize,
    ) -> DecompressionStatus {
        let mut input_buf = Vec::new();
        if input.read_to_end(&mut input_buf).is_err() {
            return DecompressionStatus::Failed;
        }

        let mut output_buffer: Vec<u8> = Vec::with_capacity(uncompressed_size);
        let uncompressed_size = uncompressed_size as u64;

        let mut in_buf_ofs = 0;
        let mut out_buf_ofs = 0;
        let mut dst_bytes_left = uncompressed_size;

        let mut status;

        loop {
            let mut num_in_bytes = input_buf.len() as u64;
            let mut out_buf_len = uncompressed_size - out_buf_ofs;

            let status_int = unsafe {
                lzham_sys::lzham_decompress(
                    self.0,
                    input_buf.as_ptr().add(in_buf_ofs as usize),
                    &mut num_in_bytes,
                    output_buffer.as_mut_ptr().add(out_buf_ofs as usize),
                    &mut out_buf_len,
                    1 as c_uint
                )
            };

            in_buf_ofs += num_in_bytes;
            out_buf_ofs += out_buf_len;

            if out_buf_len > dst_bytes_left {
                return DecompressionStatus::Failed;
            }

            dst_bytes_left -= out_buf_len;

            status = DecompressionStatus::from_c_type(status_int);

            if status.is_success_or_first_failure() {
                break;
            }
        }

        unsafe { output_buffer.set_len(uncompressed_size as usize); }

        if output.write(&output_buffer).is_err() {
            return DecompressionStatus::Failed;
        }

        status
    }
}

mod structures;

pub use structures::*;

use super::traits::CType;
use lzham_sys::lzham_compress_state_ptr;
use std::{
    io::{BufRead, Write},
    os::raw::c_uint,
};

/// A high level compressor.
#[derive(Clone, Debug)]
pub struct Compressor(lzham_compress_state_ptr);

impl Compressor {
    /// Creates a new [`Compressor`] from [`options`].
    ///
    /// [`options`]: CompressionOptions
    pub fn from_options(options: CompressionOptions) -> Self {
        Self(unsafe { lzham_sys::lzham_compress_init(&options.to_c_type()) })
    }

    /// Reinitializes the compressor.
    pub fn reinit(&self) -> Self {
        Self(unsafe { lzham_sys::lzham_compress_reinit(self.0) })
    }

    /// Deinitializes the compressor.
    ///
    /// It cannot be used to compress further.
    pub fn deinit(&self) -> u32 {
        unsafe { lzham_sys::lzham_compress_deinit(self.0) }
    }

    /// Compresses input data into the output buffer with already specified [`options`].
    ///
    /// [`options`]: CompressionOptions
    pub fn compress<R: BufRead, W: Write>(
        &self,
        input: &mut R,
        output: &mut W,
    ) -> CompressionStatus {
        let mut input_buf = Vec::new();
        if input.read_to_end(&mut input_buf).is_err() {
            return CompressionStatus::Failed;
        }

        let mut output_buffer: Vec<u8> = Vec::with_capacity(2 << 12);

        let mut in_buf_ofs = 0;
        let mut out_buf_ofs = 0;

        let mut status;

        let mut num_in_bytes;
        let mut out_buf_len;

        loop {
            num_in_bytes = input_buf.len() as lzham_sys::size_t - in_buf_ofs;
            out_buf_len = output_buffer.capacity() as lzham_sys::size_t - out_buf_ofs;

            let status_int = unsafe {
                lzham_sys::lzham_compress(
                    self.0,
                    input_buf.as_ptr().add(in_buf_ofs as usize),
                    &mut num_in_bytes,
                    output_buffer.as_mut_ptr().add(out_buf_ofs as usize),
                    &mut out_buf_len,
                    1 as c_uint,
                )
            };

            unsafe {
                output_buffer.set_len(out_buf_len as usize);
            }

            in_buf_ofs += num_in_bytes;
            out_buf_ofs += out_buf_len;

            status = CompressionStatus::from_c_type(status_int);

            if status.is_success_or_first_failure() {
                break;
            } else if let CompressionStatus::HasMoreOutput = status {
                let size = output_buffer.len();

                output_buffer.resize(size + (size >> 3) + 6, 0);
            }
        }

        // Write contents of `out_buf` to user's `output` buffer
        if output.write(&output_buffer).is_err() {
            return CompressionStatus::Failed;
        }

        status
    }
}

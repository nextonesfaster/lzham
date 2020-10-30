//! Decompression related items.

#[doc(inline)]
pub use super::low::decompression::*;

use std::io::{BufRead, Write};

/// Decompresses input data into the output buffer with default [`options`].
///
/// [`options`]: DecompressionOptions
pub fn decompress<R: BufRead, W: Write>(
    input: &mut R,
    output: &mut W,
    uncompressed_size: usize,
) -> DecompressionStatus {
    let decompressor = Decompressor::from_options(DecompressionOptions::default());

    decompressor.decompress(input, output, uncompressed_size)
}

/// Decompresses input data into the output buffer with provided [`options`].
///
/// [`options`]: DecompressionOptions
pub fn decompress_with_options<R: BufRead, W: Write>(
    input: &mut R,
    output: &mut W,
    uncompressed_size: usize,
    options: DecompressionOptions,
) -> DecompressionStatus {
    let decompressor = Decompressor::from_options(options);

    decompressor.decompress(input, output, uncompressed_size)
}

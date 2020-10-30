//! Compression related items.

#[doc(inline)]
pub use super::low::compression::*;

use std::io::{BufRead, Write};

/// Compresses input data into the output buffer with default [`options`].
///
/// [`options`]: CompressionOptions
pub fn compress<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> CompressionStatus {
    let compressor = Compressor::from_options(CompressionOptions::default());

    compressor.compress(input, output)
}

/// Compresses input data into the output buffer with provided [`options`].
///
/// [`options`]: CompressionOptions
pub fn compress_with_options<R: BufRead, W: Write>(
    input: &mut R,
    output: &mut W,
    options: CompressionOptions,
) -> CompressionStatus {
    let compressor = Compressor::from_options(options);

    compressor.compress(input, output)
}

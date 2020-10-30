//! High-level Rust bindings over the [`lzham_codec`].
//!
//! `lzham` provides high level functions and structures to compress and decompress
//! data based on the LZHAM codec.
//!
//! The crate has not been tested enough yet and some functionality is still unimplemented.
//!
//! ## Examples
//!
//! ```
//! # use lzham::{compress, decompress};
//! let data = String::from("This is a test.");
//!
//! let mut comp = Vec::new();
//! let status = compress(&mut data.as_bytes(), &mut comp);
//!
//! assert!(status.is_success());
//!
//! let mut decomp = Vec::new();
//! let status = decompress(&mut comp.as_slice(), &mut decomp, data.len());
//!
//! assert!(status.is_success());
//! ```
//!
//! [`lzham_codec`]: https://github.com/richgel999/lzham_codec

pub mod compress;
pub mod decompress;
mod low;

#[doc(inline)]
pub use compress::{compress, compress_with_options, CompressionOptions};
#[doc(inline)]
pub use decompress::{decompress, decompress_with_options, DecompressionOptions};
#[doc(inline)]
pub use low::{TableUpdateInterval, TableUpdateRate};

mod test {
    #[test]
    fn test_compress_and_decompress() {
        use crate::{compress, decompress};

        let data = String::from(
            "This is a test.This is a test.This is a test.\
            1234567This is a test.This is a test.123456"
        );

        let mut comp = Vec::new();
        let status = compress(&mut data.as_bytes(), &mut comp);

        assert!(status.is_success());

        let mut decomp = Vec::new();
        let status = decompress(&mut comp.as_slice(), &mut decomp, data.len());

        assert!(status.is_success());
    }
}

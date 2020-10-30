# lzham

High-level Rust bindings for [lzham codec] built upon the lower-level [lzham-sys] crate.

You must have `cmake` and a C++ compiler to build this crate, as the [lzham-sys] crate builds [lzham] library and does not search for a prebuilt library.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
lzham = "0.1.0"
```

## Examples

```rust
use lzham::{compress, decompress};
let data = String::from("This is a test.");

let mut comp = Vec::new();
let status = compress(&mut data.as_bytes(), &mut comp);

assert!(status.is_success());

let mut decomp = Vec::new();
let status = decompress(&mut comp.as_slice(), &mut decomp, data.len());

assert!(status.is_success());
```

## Linking

`lzham` supports both static and dynamic linking. To link statically, you can either set `LIBLZHAM_STATIC` or `LZHAM_STATIC` environment variables to true, or use the `static` feature.

To link dynamically, use the `dynamic` feature.

If you don't set any environment variables or use any features, the build will be the expected default library linking method based on OS or target. For Windows, macOS and Linux with musl, it will be `static`. For Linux without musl, it will be `dynamic`.

Note that environment variables take precedence over features. In case of any ambiguity, it uses the default linking method.

## Features

The crate has the following two features:

- `static`: Links to the library statically
- `dynamic`: Links to the library dynamically

These set the appropriate [lzham-sys] features, which is responsible for building and linking the library.

## License

`lzham` is available under the MIT license. See [LICENSE](license) for more details.

[lzham codec]: https://github.com/richgel999/lzham_codec
[lzham]: https://github.com/richgel999/lzham_codec
[`bindgen`]: https://github.com/rust-lang/rust-bindgen
[lzham-sys]: https://github.com/AriusX7/lzham-sys

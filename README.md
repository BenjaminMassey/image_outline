![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/image_outline.svg)](https://crates.io/crates/image_outline)
[![api-docs](https://docs.rs/image_outline/badge.svg)](https://docs.rs/image_outline)
[![dependency-status](https://deps.rs/repo/github/BenjaminMassey/image_outline/status.svg)](https://deps.rs/repo/github/BenjaminMassey/image_outline)

# image_outline
Copyright &copy; 2025 Benjamin Massey (Version 0.1.0)

`image_outline`: a barebones method for adding outline pixels to an image.

## Example

```rust
let original = image::ImageReader::open("input.png")
    .expect("Image file open failure.")
    .decode()
    .expect("Image file decode failure.");
let outlined = image_outline::outline_rgba8(
    &original.to_rgba8(),
    None,
    (0, 0, 0),
    false,
    false,
);
outlined.save("output.png").expect("Image save failure.");
```


# License

This work is licensed under the "[MIT License](https://opensource.org/license/mit)".

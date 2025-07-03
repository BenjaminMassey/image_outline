![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/image_outline.svg)](https://crates.io/crates/image_outline)
[![api-docs](https://docs.rs/image_outline/badge.svg)](https://docs.rs/image_outline)
[![dependency-status](https://deps.rs/repo/github/BenjaminMassey/image_outline/status.svg)](https://deps.rs/repo/github/BenjaminMassey/image_outline)

# image_outline
Copyright &copy; 2025 Benjamin Massey (Version 0.1.2)

`image_outline`: a barebones method for adding outline pixels to an image.

## Example Code

```rust
let original = image::ImageReader::open("input.png")
    .expect("Image file open failure.")
    .decode()
    .expect("Image file decode failure.");
let outlined = image_outline::outline_rgba8(
    &original.to_rgba8(),
    None,
    (0, 0, 0),
    1,
); // outline transparent background image with one "wide" black pixels
outlined.save("output.png").expect("Image save failure.");
```

## Example Result

![input](docs/input.png)

transforms into

![output](docs/output.png)

## Approach

This crate is described as a "barebones" method because it does nearly nothing
to understand the actual structure of an image. Instead, it cycles through an
image pixel-by-pixel and checks if said pixel is a "foreground" pixel. If so, it
checks for surrounding "background" pixels (based on either transparency or
given color), and replaces these background pixels with the given outline color.

This is intended for rather simple scenarios where the type of image data is
rather well defined within a project. The project `text-to-image` in the
`examples` folder was the original intention of this project, and thus
demonstrates this the best. We can safely assume that the generated white-text
image only contains joined white pixels and transparent pixels, as defined by
how `text-to-png` functions (in consideration of our usage). More complex
outlining scenarios will need more complex approaches.

## License

This work is licensed under the "[MIT License](https://opensource.org/license/mit)".

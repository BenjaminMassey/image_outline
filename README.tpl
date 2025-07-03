![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/{{crate}}.svg)](https://crates.io/crates/{{crate}})
[![api-docs](https://docs.rs/{{crate}}/badge.svg)](https://docs.rs/{{crate}})
[![dependency-status](https://deps.rs/repo/github/BenjaminMassey/{{crate}}/status.svg)](https://deps.rs/repo/github/BenjaminMassey/{{crate}})

# {{crate}}
Copyright &copy; 2025 Benjamin Massey (Version {{version}})

{{readme}}

## Example Result

![input](docs/input.png)

transforms into

![output](docs/output.png)

## Approach

This crate is described as a "barebones" method because it does nearly nothing
to understand the actual structure of an image. Instead, it cycles through an
image pixel-by-pixel, checks if said pixel is surrounded by a given "background
pixel" (either transparency or given color), and replaces said pixel with a given
outline color, if so.

This is intended for rather simple scenarios where the type of image data is
rather well defined within a project. The project `text-to-image` in the
`examples` folder was the original intention of this project, and thus
demonstrates this the best. We can safely assume that the generated white-text
image only contains joined white pixels and transparent pixels, as defined by
how `text-to-png` functions (in consideration of our usage). More complex
outlining scenarios will need more complex approaches.

## License

This work is licensed under the "[MIT License](https://opensource.org/license/mit)".
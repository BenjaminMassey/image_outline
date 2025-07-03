#![doc(html_root_url = "https://docs.rs/image_outline/0.1.2")]

//! `image_outline`: a barebones method for adding outline pixels to an image.
//! 
//! # Example Code
//! 
//! ```
//! let original = image::ImageReader::open("input.png")
//!     .expect("Image file open failure.")
//!     .decode()
//!     .expect("Image file decode failure.");
//! let outlined = image_outline::outline_rgba8(
//!     &original.to_rgba8(),
//!     None,
//!     (0, 0, 0),
//!     1,
//! ); // outline transparent background image with one "wide" black pixels
//! outlined.save("output.png").expect("Image save failure.");
//! ```

pub use image;
use std::collections::HashSet;

/// **Outline a given image.**
///
/// This function takes an RGBA image in the form of the `Image` crate's
/// `ImageBuffer` (common pseudonym is `RgbaImage`), and returns a new version
/// of the image that has had the outline styling applied to it.
/// 
/// Argument `img` is the image that wants to have the outlining applied to it.
/// Consider utilizing `image_outline`'s reference to the `image` crate for your
/// image loading, for the sake of versioning: `image_outline::image`.
/// 
/// Argument `bg_color` is for the parsing of what is considered the background of
/// the image for the sake of outline addition. If `None`, it will assume to check
/// for the alpha of a pixel being zero. Otherwise, it will check the given
/// argument as an (R, G, B) color value.
/// 
/// Argument `outline_color` is what color should be inserted for any pixel
/// that the function determines to be an appropriate "outline" pixel. This
/// is in (R, G, B) format.
/// 
/// Argument `weight` determines how many pixels "wide" the outline should be.
/// Under the hood, this is calling the `outline_rgba8_single(..)` internal function
/// `weight` number of times.
pub fn outline_rgba8(
    img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    bg_color: Option<(u8, u8, u8)>,
    outline_color: (u8, u8, u8),
    weight: u32,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut the_image = img.to_owned();
    for _ in 0..weight {
        the_image = outline_rgba8_single(&the_image, bg_color, outline_color);
    }
    the_image
}

fn outline_rgba8_single(
    img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    bg_color: Option<(u8, u8, u8)>,
    outline_color: (u8, u8, u8),
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut the_image = img.to_owned();
    let (width, height) = the_image.dimensions();
    let mut outlined: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if outlined.contains(&(x as i32, y as i32)) {
                continue;
            }
            let pixel = the_image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            let is_fg = if let Some((bg_r, bg_g, bg_b)) = bg_color {
                r != bg_r || g != bg_g || b != bg_b
            } else {
                a > 0
            };
            if is_fg {
                let mut bg_pixels: Vec<(i32, i32)> = vec![];
                for x_mod in -1..=1 {
                    for y_mod in -1..=1 {
                        let target_loc = (x as i32 + x_mod, y as i32 + y_mod);
                        if target_loc.0 < 0 || target_loc.0 >= width as i32 || target_loc.1 < 0 || target_loc.1 >= height as i32 {
                            return outline_rgba8_single(
                                &add_pixel_padding(img),
                                bg_color,
                                outline_color,
                            );
                        };
                        let target_pixel = the_image.get_pixel(target_loc.0 as u32, target_loc.1 as u32);
                        let [target_r, target_g, target_b, target_a] = target_pixel.0;
                        let is_bg = if let Some((bg_r, bg_g, bg_b)) = bg_color {
                            target_r == bg_r && target_g == bg_g && target_b == bg_b
                        } else {
                            target_a == 0
                        };
                        if is_bg {
                            bg_pixels.push(target_loc);
                        }
                    }
                }
                for bg_pixel in &bg_pixels {
                    if !outlined.contains(bg_pixel) {
                        the_image.put_pixel(
                            bg_pixel.0 as u32,
                            bg_pixel.1 as u32,
                            image::Rgba([outline_color.0, outline_color.1, outline_color.2, 255])
                        );
                        outlined.insert(*bg_pixel);
                    }
                }
            }
        }
    }
    the_image
}

fn add_pixel_padding(img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut the_image = image::RgbaImage::new(width + 2, height + 2);
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            the_image.put_pixel(x + 1, y + 1, *pixel);
        }
    }
    the_image
}
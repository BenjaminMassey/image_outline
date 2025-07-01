#![doc(html_root_url = "https://docs.rs/image_outline/0.1.0")]

//! `image_outline`: a barebones method for adding outline pixels to an image.
//! 
//! # Example
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
//!     false,
//!     false,
//! );
//! outlined.save("output.png").expect("Image save failure.");
//! ```
//!

pub use image;

/// **Outline a given image.**
///
/// This function takes an RGBA image in the form of the `Image` crate's
/// `ImageBuffer` (common pseudonym is `RgbaImage`), and returns a new version
/// of the image that has had the outline styling applied to it.
/// 
/// Argument `bg_color` is for the parsing of what is considered the background of
/// the image for the sake of outline addition. If `None`, it will assume to check
/// for the alpha of a pixel being zero. Otherwise, it will check the given
/// argument as an (R, G, B) color value: note that this is greedy.
/// 
/// Argument `outline_color` is what color should be inserted for any pixel
/// that the function determines to be an appropriate "outline" pixel. This
/// is in (R, G, B) format.
/// 
/// Argument `pad_image` adds an initial call to the internal function `add_padding(..)`,
/// which will add fully transparent pixels around every edge of the image. This
/// is necessary for the adding of outline in cases where you want outline elements
/// at the edges of the image, since it is looking for adjacent "background" pixels.
/// 
/// Argument `extra_wide` will make an extra thick outline: through the simple process
/// of surrounding all normal outline pixels with more pixels of color `outline_color`.
pub fn outline_rgba8(
    img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    bg_color: Option<(u8, u8, u8)>,
    outline_color: (u8, u8, u8),
    pad_image: bool,
    extra_wide: bool,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut the_image = if pad_image {
        add_padding(img)
    } else {
        img.to_owned()
    };
    let surroundings: Vec<(i32, i32)> = vec![
        (-1, 0), (-1, -1), (-1, 1),
        (0, 0), (0, -1), (0, 1),
        (1, 0), (1, -1), (1, 1)
    ];
    let mut hits: Vec<(u32, u32)> = vec![];
    let (width, height) = the_image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = the_image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            let hit = if let Some((r0, g0, b0)) = bg_color {
                r != r0 || g != g0 || b != b0
            } else {
                a > 0
            };
            if hit {
                let mut border = false;
                for s in &surroundings {
                    let target = (x as i32 + s.0, y as i32 + s.1);
                    if target.0 < 0 || target.0 >= width as i32 || target.1 < 0 || target.1 >= height as i32 {
                        continue;
                    }
                    let test = the_image.get_pixel(target.0 as u32, target.1 as u32);
                    let [r0, g0, b0, a0] = test.0;
                    let hit = if let Some((r1, g1, b1)) = bg_color {
                        r0 == r1 && g0 == g1 && b0 == b1
                    } else {
                        a0 == 0
                    };
                    if hit {
                        border = true;
                        break;
                    }
                }
                if border {
                    the_image.put_pixel(x, y, image::Rgba([outline_color.0, outline_color.1, outline_color.2, 255]));
                    hits.push((x, y));
                }
            }
        }
    }
    if extra_wide {
        for (x, y) in hits {
            for s in &surroundings {
                let target = (x as i32 + s.0, y as i32 + s.1);
                if target.0 < 0 || target.0 >= width as i32 || target.1 < 0 || target.1 >= height as i32 {
                    continue;
                }
                the_image.put_pixel(target.0 as u32, target.1 as u32, image::Rgba([outline_color.0, outline_color.1, outline_color.2, 255]));
            }
        }
    }
    the_image
}

fn add_padding(
    img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut new_img = image::RgbaImage::new(width + 2, height + 2);
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            new_img.put_pixel(x + 1, y + 1, *pixel);
        }
    }
    new_img
}
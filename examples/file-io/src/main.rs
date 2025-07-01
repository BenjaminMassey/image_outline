use image_outline::image;

fn main() {
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
}
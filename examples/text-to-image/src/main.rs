use image_outline::image;

fn main() {
    let text_image = text_to_image("This is a test.");
    let result = image_outline::outline_rgba8(
        &text_image,
        None,
        (0, 0, 0),
        true,
        true,
    );
    let _ = result.save("output.png");
}

fn text_to_image(text: &str) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let renderer = text_to_png::TextRenderer::default();
    let text_png = renderer.render_text_to_png_data(text, 128, 0xFFFFFF)
        .expect("Encountered an error rendering with text-to-png.");
    image::load_from_memory_with_format(&text_png.data, image::ImageFormat::Png)
        .expect("Encountered an error reading PNG data into Image.")
        .to_rgba8()
}
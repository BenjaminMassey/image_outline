use image_outline::image;

fn main() {
    // https://huggingface.co/briaai/RMBG-1.4/blob/main/onnx/model.onnx
    let remover = rmbg::Rmbg::new("model.onnx").unwrap();
    let original = image::ImageReader::open("input.png")
        .expect("Image file open failure.")
        .decode()
        .expect("Image file decode failure.");
    let without_bg = remover.remove_background(&original).unwrap();
    let outlined = image_outline::outline_rgba8(
        &without_bg.to_rgba8(),
        None,
        (255, 0, 0),
        5,
    );
    outlined.save("output.png").expect("Image save failure.");
}
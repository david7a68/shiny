pub mod png;

pub fn write_png<C: self::png::PngColor>(
    pixels: image::pixelbuffer::PixelBuffer<C>,
    filename: &str,
) {
    use self::png::encode_png;
    use std::fs::File;

    let mut file = File::create(format!("sample_{}.png", filename)).unwrap();
    encode_png(pixels, &mut file);
}

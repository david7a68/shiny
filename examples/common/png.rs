use std::io::Write;

use shiny::{
    color::Space as ColorSpace,
    image::{Image, PixelFormat},
    pixel_buffer::PixelBuffer,
};

/// Encodes a [`PixelBuffer`] and write it into a [`Write`]r.
pub fn encode_png(pix: PixelBuffer, out: &mut impl Write) {
    let mut encoder = png::Encoder::new(out, pix.width(), pix.height());

    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_srgb(png::SrgbRenderingIntent::AbsoluteColorimetric);
    encoder.set_compression(png::Compression::Default);
    encoder.set_filter(png::FilterType::default());

    let mut writer = encoder.write_header().unwrap();
    let pix = pix.convert(PixelFormat::Rgba8, ColorSpace::Srgb);

    writer.write_image_data(pix.bytes()).unwrap();
}

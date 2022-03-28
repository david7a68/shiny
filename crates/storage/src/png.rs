use std::io::Write;

use image::{
    color::{Color, Rgb, Rgba, Srgb8, Srgba8},
    pixelbuffer::PixelBuffer,
};

/// Encodes a [`PixelBuffer`] and write it into a [`Write`]r.
pub fn export_png<C: PngColor>(pix: PixelBuffer<C>, out: &mut impl Write) {
    let mut encoder = png::Encoder::new(out, pix.width(), pix.height());

    C::encode_color_information(&mut encoder);
    encoder.set_compression(png::Compression::Default);
    encoder.set_filter(png::FilterType::default());

    let mut writer = encoder.write_header().unwrap();

    // allocates, unfortunately
    let bytes = pix.pixels().iter().fold(vec![], |acc, p| p.encode(acc));

    writer.write_image_data(&bytes).unwrap();
}

/// Trait that describes how a color is to be encoded in a PNG image.
pub trait PngColor: Color {
    /// Write out the information needed to interpret a color (e.g. Srgb8,
    /// LinearSrgb16, etc.).
    fn encode_color_information<W: Write>(encoder: &mut png::Encoder<W>);

    /// Encodes a single color satisfying the format described in
    /// `encode_color_information()`.
    fn encode<'a, W: Write>(&self, writer: W) -> W;
}

impl PngColor for Srgb8 {
    fn encode_color_information<W: Write>(encoder: &mut png::Encoder<W>) {
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_srgb(png::SrgbRenderingIntent::AbsoluteColorimetric);
    }

    fn encode<'a, W: Write>(&self, mut writer: W) -> W {
        let slice = unsafe {
            std::slice::from_raw_parts(
                &self.color as *const Rgb<u8> as *const u8,
                std::mem::size_of::<Rgb<u8>>(),
            )
        };
        writer.write_all(slice).unwrap();
        writer
    }
}

impl PngColor for Srgba8 {
    fn encode_color_information<W: Write>(encoder: &mut png::Encoder<W>) {
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_srgb(png::SrgbRenderingIntent::AbsoluteColorimetric);
    }

    fn encode<'a, W: Write>(&self, mut writer: W) -> W {
        let slice = unsafe {
            std::slice::from_raw_parts(
                &self.color as *const Rgba<u8, u8> as *const u8,
                std::mem::size_of::<Rgba<u8, u8>>(),
            )
        };
        writer.write_all(slice).unwrap();
        writer
    }
}

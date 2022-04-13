use crate::{
    color::{Color, Space as ColorSpace},
    math::ops::Normalize,
    pixel_buffer::PixelBuffer,
};

#[derive(Debug)]
pub enum Error {
    /// The requested bit depth is not sufficient to represent the entirety of
    /// the requested color space.
    InsufficientBitDepth {
        requested_format: PixelFormat,
        requested_color_space: ColorSpace,
    },
}

/// Describes the way that pixel data is stored within a [`PixelBuffer`].
/// Incongruities between the pixel format and color space will produce an
/// error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    /// 4-component RGBA with 8-bit unsigned normalized integer components. i.e.
    /// We represent each channel by mapping (0, 1) to (0, 255).
    Rgba8,

    /// 4-component RGBA with 10-bit unsigned normalized integer components, and
    /// 2-bit alpha.
    Rgb10a2,
}

impl PixelFormat {
    /// The number of bytes needed to store a pixel of this format.
    #[must_use]
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::Rgba8 => 4,
            PixelFormat::Rgb10a2 => 4,
        }
    }

    /// The number of bits used to represent the range of each channel. This
    /// must be at least the same as the number of bits per channel required to
    /// store colors of the associated color space.
    ///
    /// e.g. An image in linear sRGB must have at least 10 bits per channel.
    #[must_use]
    pub fn bits_per_channel(&self) -> usize {
        match self {
            PixelFormat::Rgba8 => 8,
            PixelFormat::Rgb10a2 => 10,
        }
    }

    /// Reads the color out of the byte stream at the given location.
    #[must_use]
    pub fn read_color(self, bytes: &[u8]) -> Color {
        match self {
            PixelFormat::Rgba8 => {
                let r = bytes[0];
                let g = bytes[1];
                let b = bytes[2];
                let a = bytes[3];
                Color::unknown(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
            }
            PixelFormat::Rgb10a2 => {
                let v = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
                let r = ((v >> 22) & 0x3ff) as f32 / 1023.0;
                let g = ((v >> 12) & 0x3ff) as f32 / 1023.0;
                let b = ((v >> 2) & 0x3ff) as f32 / 1023.0;
                let a = (v & 0x3) as f32 / 3.0;
                Color::unknown(r, g, b, a)
            }
        }
    }

    /// Writes the color to the byte stream at the given location stored in this
    /// format.
    pub fn write_color(self, color: Color, dest: &mut [u8]) {
        match self {
            PixelFormat::Rgba8 => {
                dest[0] = (color.r.clamp(0.0, 1.0) * u8::MAX as f32) as u8;
                dest[1] = (color.g.clamp(0.0, 1.0) * u8::MAX as f32) as u8;
                dest[2] = (color.b.clamp(0.0, 1.0) * u8::MAX as f32) as u8;
                dest[3] = (color.a.clamp(0.0, 1.0) * u8::MAX as f32) as u8;
            }
            PixelFormat::Rgb10a2 => {
                let r = (color.r.clamp(0.0, 1.0) * 1023.0) as u32;
                let g = (color.g.clamp(0.0, 1.0) * 1023.0) as u32;
                let b = (color.b.clamp(0.0, 1.0) * 1023.0) as u32;
                let a = (color.a.clamp(0.0, 1.0) * 3.0) as u32;
                let v = (r << 22) | (g << 12) | (b << 2) | a;
                dest[0..4].copy_from_slice(&v.to_le_bytes());
            }
        }
    }
}

/// Represents a handle to an image with a given color space and pixel format.
///
/// e.g.:
/// ```rust
/// # use shiny::pixel_buffer::PixelBuffer;
/// # use shiny::color::Srgb8;
/// type SrgbPixelBuffer = PixelBuffer<Srgb8>;
/// ```
pub trait Image {
    /// The width of the image.
    #[must_use]
    fn width(&self) -> u32;

    /// The height of the image.
    #[must_use]
    fn height(&self) -> u32;

    #[must_use]
    fn color_space(&self) -> ColorSpace;

    #[must_use]
    fn pixel_format(&self) -> PixelFormat;

    /// Retrieves a copy-on-write handle to the image's pixels.
    #[must_use]
    fn get_pixels(&self) -> PixelBuffer;
}

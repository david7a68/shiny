use crate::{color::Color, pixelbuffer::PixelBuffer};

/// Represents a handle to an image with a given color space and pixel format.
///
/// e.g.:
/// ```rust
/// # use image::image::CpuImage;
/// # use image::color::Srgb8;
/// type SrgbCpuImage = CpuImage<Srgb8>;
/// ```
pub trait Image<C: Color> {
    /// The width of the image.
    fn width(&self) -> u32;

    /// The height of the image.
    fn height(&self) -> u32;

    /// Retrieves a copy-on-write handle to the image's pixels.
    fn get_pixels(&self) -> PixelBuffer<C>;
}

/// A handle to an image kept in main memory. The image can be modified with
/// copy-on-write semantics.
#[derive(Clone)]
pub struct CpuImage<C: Color> {
    pixels: PixelBuffer<C>,
}

impl<C: Color> CpuImage<C> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: PixelBuffer::new(width, height),
        }
    }

    pub fn get(&mut self, x: u32, y: u32) -> C {
        self.pixels.get(x, y)
    }

    pub fn set(&mut self, x: u32, y: u32, color: C) {
        self.pixels.set(x, y, color);
    }
}

impl<C: Color> Image<C> for CpuImage<C> {
    fn width(&self) -> u32 {
        self.pixels.width()
    }

    fn height(&self) -> u32 {
        self.pixels.height()
    }

    fn get_pixels(&self) -> PixelBuffer<C> {
        self.pixels.clone()
    }
}

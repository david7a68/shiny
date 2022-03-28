use crate::{
    color::{Color, ColorFormat},
    pixelbuffer::PixelBuffer,
};

pub trait Image<C: Color> {
    /// The format that individual pixels are stored with.
    fn format(&self) -> ColorFormat;

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

    // Clears the image to a single color. Placeholder until a proper renderer
    // can be set up.
    pub fn clear(&mut self, color: C) {
        let pixels = self.pixels.pixels_mut();
        for p in pixels {
            *p = color;
        }
    }
}

impl<C: Color> Image<C> for CpuImage<C> {
    fn format(&self) -> ColorFormat {
        C::FORMAT
    }

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

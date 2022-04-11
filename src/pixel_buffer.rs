use std::rc::Rc;

use crate::{color::Color, image::Image};

/// A copy-on-write buffer of pixels.
#[derive(Clone)]
pub struct PixelBuffer<C: Color> {
    raw: Rc<RawPixelBuffer<C>>,
}

impl<C: Color> PixelBuffer<C> {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            raw: Rc::new(RawPixelBuffer::new(width, height)),
        }
    }

    /// Retrieves the color of a single pixel.
    #[must_use]
    pub fn get(&self, x: u32, y: u32) -> C {
        self.raw.get(x, y)
    }

    /// Sets the color of a single pixel, copying the buffer if other owning
    /// references exist.
    pub fn set(&mut self, x: u32, y: u32, color: C) {
        if (x < self.width()) & (y < self.height()) {
            Rc::make_mut(&mut self.raw).set(x, y, color);
        }
    }

    /// Retrieves an entire row of pixels.
    #[must_use]
    pub fn row(&self, y: u32) -> &[C] {
        self.raw.row(y)
    }

    /// Retrieves an entire row of mutable pixels, copying the buffer if other
    /// owning references exist.
    #[must_use]
    pub fn row_mut(&mut self, y: u32) -> &mut [C] {
        Rc::make_mut(&mut self.raw).row_mut(y)
    }

    /// Retrieves the entire buffer's contents.
    #[must_use]
    pub fn pixels(&self) -> &[C] {
        self.raw.pixels()
    }

    /// Retrieves the entire buffer's contents, coping the buffer if other
    /// owning references exist.
    #[must_use]
    pub fn pixels_mut(&mut self) -> &mut [C] {
        Rc::make_mut(&mut self.raw).pixels_mut()
    }

    pub fn clear(&mut self, color: C) {
        for px in self.pixels_mut() {
            *px = color;
        }
    }
}

impl<C: Color> Image<C> for PixelBuffer<C> {
    fn width(&self) -> u32 {
        self.raw.width()
    }

    fn height(&self) -> u32 {
        self.raw.height()
    }

    fn get_pixels(&self) -> PixelBuffer<C> {
        self.clone()
    }
}

/// A fixed-size buffer of pixels.
#[derive(Clone)]
struct RawPixelBuffer<C: Color> {
    width: u32,
    pixels: Box<[C]>,
}

impl<C: Color> RawPixelBuffer<C> {
    /// Creates a new buffer of the given size with uninitialized content.
    pub fn new(width: u32, height: u32) -> Self {
        let num_pixels = usize::try_from(width * height).unwrap();
        let pixels = vec![C::BLACK; num_pixels].into_boxed_slice();

        Self { width, pixels }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        u32::try_from(self.pixels.len() / usize::try_from(self.width).unwrap()).unwrap()
    }

    pub fn get(&self, x: u32, y: u32) -> C {
        let offset = usize::try_from(self.width * y).unwrap();
        self.pixels[offset + usize::try_from(x).unwrap()]
    }

    pub fn set(&mut self, x: u32, y: u32, color: C) {
        let offset = usize::try_from(self.width * y).unwrap();
        self.pixels[offset + usize::try_from(x).unwrap()] = color;
    }

    pub fn row(&self, y: u32) -> &[C] {
        let offset = usize::try_from(self.width * y).unwrap();
        &self.pixels[offset..offset + usize::try_from(self.width).unwrap()]
    }

    pub fn row_mut(&mut self, y: u32) -> &mut [C] {
        let offset = usize::try_from(self.width * y).unwrap();
        &mut self.pixels[offset..offset + usize::try_from(self.width).unwrap()]
    }

    pub fn pixels(&self) -> &[C] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [C] {
        &mut self.pixels
    }
}

use std::rc::Rc;

use crate::{
    color::{Color, Space as ColorSpace},
    image::{Error as ImageError, Image, PixelFormat},
};

/// A copy-on-write buffer of pixels.
#[derive(Clone)]
pub struct PixelBuffer {
    raw: Rc<RawPixelBuffer>,
}

impl PixelBuffer {
    /// Creates a new pixel buffer with the given dimensions and pixel format
    /// and color space. It is initialized to all black (and transparent, if
    /// there is an alpha component).
    ///
    /// # Errors
    ///
    /// Returns an error if the requested bit depth is insufficient to represent
    /// the entirety of the requested color space.
    pub fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
    ) -> Result<Self, ImageError> {
        if format.bits_per_channel() > color_space.bits_per_channel() {
            Err(ImageError::InsufficientBitDepth {
                requested_format: format,
                requested_color_space: color_space,
            })
        } else {
            Ok(Self {
                raw: Rc::new(RawPixelBuffer::new(width, height, format, color_space)),
            })
        }
    }

    pub fn iter(&self) -> PixelBufferIter {
        PixelBufferIter {
            buffer: self.raw.clone(),
            offset: 0,
        }
    }

    /// Retrieves the color of a single pixel.
    #[must_use]
    pub fn get(&self, x: u32, y: u32) -> Color {
        self.raw.get(x, y)
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        self.raw.bytes()
    }

    /// Sets the color of a single pixel, copying the buffer if other owning
    /// references exist.
    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        if (x < self.width()) & (y < self.height()) {
            Rc::make_mut(&mut self.raw).set(x, y, color);
        }
    }

    pub fn clear(&mut self, color: Color) {
        Rc::make_mut(&mut self.raw).clear(color);
    }

    /// Converts an image in one format and color space to another. This is a
    /// no-op if the format and color space are the same.
    pub fn convert(&self, format: PixelFormat, color_space: ColorSpace) -> Self {
        if self.color_space() == color_space && self.pixel_format() == format {
            self.clone()
        } else {
            Self {
                raw: Rc::new(self.raw.convert(format, color_space)),
            }
        }
    }
}

impl Image for PixelBuffer {
    fn width(&self) -> u32 {
        self.raw.width()
    }

    fn height(&self) -> u32 {
        self.raw.height()
    }

    fn color_space(&self) -> ColorSpace {
        self.raw.color_space
    }

    fn pixel_format(&self) -> PixelFormat {
        self.raw.format
    }

    fn get_pixels(&self) -> PixelBuffer {
        self.clone()
    }
}

pub struct PixelBufferIter {
    buffer: Rc<RawPixelBuffer>,
    offset: usize,
}

impl Iterator for PixelBufferIter {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.buffer.bytes.len() {
            let color = self
                .buffer
                .format
                .read_color(&self.buffer.bytes[self.offset..]);
            self.offset += self.buffer.format.bytes_per_pixel();
            Some(color)
        } else {
            None
        }
    }
}

/// A fixed-size buffer of pixels.
#[derive(Clone)]
struct RawPixelBuffer {
    /// The number of bytes needed to store a row.
    row_stride: usize,
    /// The byte format used to store the pixels.
    format: PixelFormat,
    /// The color space that the pixel colors refer to.
    color_space: ColorSpace,
    /// The bytes of the buffer.
    bytes: Box<[u8]>,
}

impl RawPixelBuffer {
    /// Creates a new buffer of the given size with uninitialized content.
    pub fn new(width: u32, height: u32, format: PixelFormat, color_space: ColorSpace) -> Self {
        let num_bytes = usize::try_from(width * height).unwrap() * format.bytes_per_pixel();
        let bytes = vec![0; num_bytes].into_boxed_slice();

        Self {
            row_stride: usize::try_from(width).unwrap() * format.bytes_per_pixel(),
            format,
            color_space,
            bytes,
        }
    }

    pub fn width(&self) -> u32 {
        u32::try_from(self.row_stride / self.format.bytes_per_pixel()).unwrap()
    }

    pub fn height(&self) -> u32 {
        u32::try_from(self.bytes.len() / self.row_stride).unwrap()
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        self.format
            .read_color(&self.bytes[self.offset_of(x, y)..])
            .in_color_space(self.color_space)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        let offset = self.offset_of(x, y);
        self.format.write_color(
            color.in_color_space(self.color_space),
            &mut self.bytes[offset..],
        );
    }

    pub fn clear(&mut self, color: Color) {
        for i in (0..self.bytes.len()).step_by(self.format.bytes_per_pixel()) {
            self.format
                .write_color(color.in_color_space(self.color_space), &mut self.bytes[i..]);
        }
    }

    pub fn convert(&self, format: PixelFormat, color_space: ColorSpace) -> Self {
        let mut new_buffer = Self::new(self.width(), self.height(), format, color_space);

        let mut new_buffer_offset = format.bytes_per_pixel();
        let mut old_buffer_offset = self.format.bytes_per_pixel();

        while new_buffer_offset < new_buffer.bytes.len() {
            format.write_color(
                self.format
                    .read_color(&self.bytes[old_buffer_offset..])
                    .in_color_space(color_space),
                &mut new_buffer.bytes[new_buffer_offset..],
            );

            new_buffer_offset += format.bytes_per_pixel();
            old_buffer_offset += self.format.bytes_per_pixel();
        }

        new_buffer
    }

    fn offset_of(&self, x: u32, y: u32) -> usize {
        self.row_stride * usize::try_from(y).unwrap()
            + self.format.bytes_per_pixel() * usize::try_from(x).unwrap()
    }
}

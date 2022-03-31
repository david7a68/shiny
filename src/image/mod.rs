pub mod cpu_image;
pub mod pixel_buffer;

use crate::color::Color;
use pixel_buffer::PixelBuffer;

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

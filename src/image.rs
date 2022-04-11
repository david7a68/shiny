use crate::color::Color;
use crate::pixel_buffer::PixelBuffer;

/// Represents a handle to an image with a given color space and pixel format.
///
/// e.g.:
/// ```rust
/// # use shiny::pixel_buffer::PixelBuffer;
/// # use shiny::color::Srgb8;
/// type SrgbPixelBuffer = PixelBuffer<Srgb8>;
/// ```
pub trait Image<C: Color> {
    /// The width of the image.
    #[must_use]
    fn width(&self) -> u32;

    /// The height of the image.
    #[must_use]
    fn height(&self) -> u32;

    /// Retrieves a copy-on-write handle to the image's pixels.
    #[must_use]
    fn get_pixels(&self) -> PixelBuffer<C>;
}

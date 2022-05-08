use crate::{
    color::Color,
    paint::{Paint, PaintConfig},
    pixel_buffer::PixelBuffer,
    shapes::path::{Builder as PathBuilder, Path},
};

pub struct CanvasOptions {
    /// Set to enable randomization of the color used for every draw command,
    /// overriding the paint passed to the canvas.
    pub debug_randomize_color: bool,
}

/// A 2D drawing context.
///
/// Drawing commands directed to a canvas are rendered out by the backend. All
/// canvases with the same backend share the same resources, so cached paints
/// and paths can be shared between them. In fact, canvases can be used as
/// inputs to other canvases as image components.
pub trait Canvas: CanvasOps {
    /// Retrieves the canvas' contents in pixels in a copy-on-write buffer. This
    /// function will block until all pending drawing commands for this canvas
    /// are complete.
    ///
    /// If the canvas is drawn to at a later time, the returned buffer will not
    /// update, and another will have to be retrieved from the canvas.
    fn get_pixels(&self) -> PixelBuffer;
}

/// Operations for rendering shapes to a render target, and querying its
/// properties.
pub trait CanvasOps {
    /// The width of the drawable area.
    fn width(&self) -> u32;

    /// The height of the drawable area.
    fn height(&self) -> u32;

    /// Clears the area's contents to the given color.
    fn clear(&mut self, color: Color);

    /// Creates a new, immutable paint object, and returns a reference to it.
    /// This allows the backend to cache the paint object in an
    /// implementation-specific way.
    fn create_paint(&mut self, config: PaintConfig) -> Paint;

    /// Marks a paint object for deletion. It will be deleted once all pending
    /// uses of the paint are complete. It is an error to to clone a paint after
    /// it has been marked for deletion.
    fn destroy_paint(&mut self, paint: Paint);

    /// Retrieves the paint config information used to create the cached paint
    /// object.
    fn paint_config(&self, paint: Paint) -> PaintConfig;

    /// Creates a new path builder.
    fn begin_path(&mut self) -> PathBuilder;

    /// Submits the given path to the canvas for rendering. Rendering occurs
    /// with the painter's algorithm (back-to-front), so paths drawn first will
    /// be hidden by paths drawn over them.
    ///
    /// The actual drawing may be deferred for an indeterminate time, but will
    /// be completed by the time a `get_pixels()` call or backend-equivalent
    /// returns.
    fn fill_path(&mut self, path: &Path, paint: Paint);

    /// Submits the given path to the canvas for rendering. Rendering occurs
    /// with the painter's algorithm (back-to-front), so paths drawn first will
    /// be hidden by paths drawn over them.
    fn stroke_path(&mut self, path: &Path, paint: Paint);
}

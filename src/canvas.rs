use crate::{
    color::Color,
    paint::{Paint, PaintConfig},
    pixel_buffer::PixelBuffer,
    shapes::{
        path::{Builder as PathBuilder, Path},
        rect::Rect,
    },
};

/// A 2D drawing context.
///
/// Drawing commands directed to a canvas are rendered out by the backend. All
/// canvases with the same backend share the same resources, so cached paints
/// and paths can be shared between them. In fact, canvases can be used as
/// inputs to other canvases as image components.
pub trait Canvas<C: Color>: CanvasOps<C> {
    /// Retrieves the canvas' contents in pixels in a copy-on-write buffer. This
    /// function will block until all pending drawing commands for this canvas
    /// are complete.
    /// 
    /// If the canvas is drawn to at a later time, the returned buffer will not
    /// update, and another will have to be retrieved from the canvas.
    fn get_pixels(&self) -> PixelBuffer<C>;
}

/// A rectangular subset of a canvas.
///
/// The coordinate system is reset, such that the top-left corner of the clipped
/// canvas is (0, 0), instead of the actual canvas' origin.
pub trait ClippedCanvas<C: Color>: CanvasOps<C> {
    /// Returns the offset of the clip region relative to the canvas' origin.
    fn clip_offset(&self) -> (u32, u32);

    /// The width of the entire canvas (not the clip region).
    fn actual_width(&self) -> u32;

    /// The height of the entire canvas (not the clip region).
    fn actual_height(&self) -> u32;
}

/// Operations for rendering shapes to a render target, and querying its
/// properties.
pub trait CanvasOps<C: Color> {
    /// The width of the drawable area.
    fn width(&self) -> u32;

    /// The height of the drawable area.
    fn height(&self) -> u32;

    /// Clears the area's contents to the given color.
    fn clear(&mut self, color: C);

    /// Creates a sub-area to draw to. Coordinates within the clipped area are
    /// relative to the clip's origin, with corresponding width and height.
    /// 
    /// If the clip region is larger than the drawable area, or extends outside
    /// of the drawable area, the region outside of the drawable area will also
    /// be clipped.
    fn clip(&mut self, rect: Rect) -> &mut dyn ClippedCanvas<C>;

    /// Creates a new, immutable paint object, and returns a reference to it.
    /// This allows the backend to cache the paint object in an
    /// implementation-specific way.
    fn create_paint(&mut self, config: PaintConfig<C>) -> Paint<C>;

    /// Marks a paint object for deletion. It will be deleted once all pending
    /// uses of the paint are complete. It is an error to to clone a paint after
    /// it has been marked for deletion.
    fn destroy_paint(&mut self, paint: Paint<C>);

    /// Retrieves the paint config information used to create the cached paint
    /// object.
    fn paint_config(&self, paint: Paint<C>) -> PaintConfig<C>;

    /// Creates a new path builder.
    fn begin_path(&mut self) -> PathBuilder;

    /// Submits the given path to the canvas for rendering. Rendering occurs
    /// with the painter's algorithm (back-to-front), so paths drawn first will
    /// be hidden by paths drawn over them.
    ///
    /// The actual drawing may be deferred for an indeterminate time, but will
    /// be completed by the time a `get_pixels()` call or backend-equivalent
    /// returns.
    fn fill_path(&mut self, path: &Path, paint: Paint<C>);

    /// Submits the given path to the canvas for rendering. Rendering occurs
    /// with the painter's algorithm (back-to-front), so paths drawn first will
    /// be hidden by paths drawn over them.
    fn stroke_path(&mut self, path: &Path, paint: Paint<C>);
}

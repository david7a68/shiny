use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    canvas::{Canvas, CanvasOps, ClippedCanvas},
    color::{Color, Space as ColorSpace},
    hash::{hash_of, PassThroughHasher},
    image::{Error as ImageError, Image, PixelFormat},
    paint::{Paint, PaintConfig},
    pixel_buffer::PixelBuffer,
    shapes::{
        path::{Builder as PathBuilder, Path},
        rect::Rect,
    },
};

use super::BackendState;

struct DrawCommand {
    paint: Paint,
    path: Path,
}

struct CanvasState {
    paints: HashMap<u64, PaintConfig, PassThroughHasher>,
    // Todo: Computing cpatches for paths is an expensive operation, so we want
    // to cache them in `CachedPath`s or something of the like. We also want to
    // process them in a multithreaded way.
    commands: Vec<DrawCommand>,
}

pub struct SoftwareCanvas {
    shared_state: Rc<BackendState>,
    internal_state: Rc<RefCell<CanvasState>>,
    pixels: PixelBuffer,
}

impl SoftwareCanvas {
    pub(super) fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
        shared_state: Rc<BackendState>,
    ) -> Result<Self, ImageError> {
        Ok(SoftwareCanvas {
            shared_state,
            internal_state: Rc::new(RefCell::new(CanvasState {
                paints: HashMap::with_hasher(PassThroughHasher::default()),
                commands: Vec::new(),
            })),
            pixels: PixelBuffer::new(width, height, format, color_space)?,
        })
    }
}

impl Canvas for SoftwareCanvas {
    fn get_pixels(&self) -> PixelBuffer {
        self.pixels.clone()
    }
}

impl CanvasOps for SoftwareCanvas {
    type Clipped = SoftwareClippedCanvas;

    fn width(&self) -> u32 {
        self.pixels.width()
    }

    fn height(&self) -> u32 {
        self.pixels.height()
    }

    fn clear(&mut self, color: Color) {
        self.pixels.clear(color);
    }

    fn clip(&mut self, rect: Rect) -> Self::Clipped {
        todo!()
    }

    fn create_paint(&mut self, config: PaintConfig) -> Paint {
        let hash = hash_of(&config);
        self.internal_state.borrow_mut().paints.insert(hash, config);
        Paint::new(hash)
    }

    fn destroy_paint(&mut self, paint: Paint) {
        todo!()
    }

    fn paint_config(&self, paint: Paint) -> PaintConfig {
        todo!()
    }

    fn begin_path(&mut self) -> PathBuilder {
        todo!()
    }

    fn fill_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }

    fn stroke_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }
}

pub struct SoftwareClippedCanvas {
    shared_state: Rc<BackendState>,
    internal_state: Rc<RefCell<CanvasState>>,
    clip_rect: Rect,
}

impl ClippedCanvas for SoftwareClippedCanvas {
    fn clip_offset(&self) -> (f32, f32) {
        (self.clip_rect.left(), self.clip_rect.top())
    }
}

impl CanvasOps for SoftwareClippedCanvas {
    type Clipped = Self;

    fn width(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn clear(&mut self, color: Color) {
        todo!()
    }

    fn clip(&mut self, rect: Rect) -> Self::Clipped {
        todo!()
    }

    fn create_paint(&mut self, config: PaintConfig) -> Paint {
        todo!()
    }

    fn destroy_paint(&mut self, paint: Paint) {
        todo!()
    }

    fn paint_config(&self, paint: Paint) -> PaintConfig {
        todo!()
    }

    fn begin_path(&mut self) -> PathBuilder {
        todo!()
    }

    fn fill_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }

    fn stroke_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }
}

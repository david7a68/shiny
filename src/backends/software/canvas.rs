use std::{collections::HashMap, rc::Rc};

use crate::{
    canvas::{Canvas, CanvasOps, ClippedCanvas},
    color::{Color, Space as ColorSpace},
    hash::{hash_of, PassThroughHasher},
    image::{Error as ImageError, Image, PixelFormat},
    paint::{Paint, PaintConfig},
    pixel_buffer::PixelBuffer,
    shapes::rect::Rect,
};

use super::Shared;

pub struct SoftwareCanvas {
    shared_state: Rc<Shared>,
    paints: HashMap<u64, PaintConfig, PassThroughHasher>,
    pixels: PixelBuffer,
}

impl SoftwareCanvas {
    pub(super) fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
        shared_state: Rc<Shared>,
    ) -> Result<Self, ImageError> {
        Ok(SoftwareCanvas {
            shared_state,
            paints: HashMap::with_hasher(PassThroughHasher::default()),
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
    fn width(&self) -> u32 {
        self.pixels.width()
    }

    fn height(&self) -> u32 {
        self.pixels.height()
    }

    fn clear(&mut self, color: Color) {
        self.pixels.clear(color);
    }

    fn clip(&mut self, rect: Rect) -> &mut dyn ClippedCanvas {
        todo!()
    }

    fn create_paint(&mut self, config: PaintConfig) -> Paint {
        let hash = hash_of(&config);
        self.paints.insert(hash, config);
        Paint::new(hash)
    }

    fn destroy_paint(&mut self, paint: Paint) {
        todo!()
    }

    fn paint_config(&self, paint: Paint) -> PaintConfig {
        todo!()
    }

    fn begin_path(&mut self) -> crate::shapes::path::Builder {
        todo!()
    }

    fn fill_path(&mut self, path: &crate::shapes::path::Path, paint: Paint) {
        todo!()
    }

    fn stroke_path(&mut self, path: &crate::shapes::path::Path, paint: Paint) {
        todo!()
    }
}

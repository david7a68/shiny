use std::{collections::HashMap, rc::Rc};

use crate::{
    canvas::{Canvas, CanvasOps, ClippedCanvas},
    color::Color,
    hash::{hash_of, PassThroughHasher},
    image::Image,
    paint::{PaintConfig, Paint},
    pixel_buffer::PixelBuffer,
    shapes::rect::Rect,
};

use super::Shared;

pub struct SoftwareCanvas<C: Color> {
    shared_state: Rc<Shared>,
    paints: HashMap<u64, PaintConfig<C>, PassThroughHasher>,
    pixels: PixelBuffer<C>,
}

impl<C: Color> SoftwareCanvas<C> {
    pub(super) fn new(width: u32, height: u32, shared_state: Rc<Shared>) -> Self {
        SoftwareCanvas {
            shared_state,
            paints: HashMap::with_hasher(PassThroughHasher::default()),
            pixels: PixelBuffer::new(width, height),
        }
    }
}

impl<C: Color> Canvas<C> for SoftwareCanvas<C> {
    fn get_pixels(&self) -> PixelBuffer<C> {
        self.pixels.clone()
    }
}

impl<C: Color> CanvasOps<C> for SoftwareCanvas<C> {
    fn width(&self) -> u32 {
        self.pixels.width()
    }

    fn height(&self) -> u32 {
        self.pixels.height()
    }

    fn clear(&mut self, color: C) {
        self.pixels.clear(color);
    }

    fn clip(&mut self, rect: Rect) -> &mut dyn ClippedCanvas<C> {
        todo!()
    }

    fn create_paint(&mut self, config: PaintConfig<C>) -> Paint<C> {
        let hash = hash_of(&config);
        self.paints.insert(hash, config);
        Paint::new(hash)
    }

    fn destroy_paint(&mut self, paint: Paint<C>) {
        todo!()
    }

    fn paint_config(&self, paint: Paint<C>) -> PaintConfig<C> {
        todo!()
    }

    fn begin_path(&mut self) -> crate::shapes::path::Builder {
        todo!()
    }

    fn fill_path(&mut self, path: &crate::shapes::path::Path, paint: Paint<C>) {
        todo!()
    }

    fn stroke_path(&mut self, path: &crate::shapes::path::Path, paint: Paint<C>) {
        todo!()
    }
}

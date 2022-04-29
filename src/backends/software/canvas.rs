use std::{cell::RefCell, rc::Rc};

use crate::{
    canvas::{Canvas, CanvasOps},
    color::{Color, Space as ColorSpace},
    hash::hash_of,
    image::{Error as ImageError, Image, PixelFormat},
    math::vector2::Vec2,
    paint::{Paint, PaintConfig},
    pixel_buffer::PixelBuffer,
    shapes::{
        bezier::Bezier,
        path::{Builder as PathBuilder, Path},
    },
};

use super::BackendState;

pub struct SoftwareCanvas {
    shared_state: Rc<RefCell<BackendState>>,
    pixels: PixelBuffer,
}

impl SoftwareCanvas {
    pub(super) fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
        shared_state: Rc<RefCell<BackendState>>,
    ) -> Result<Self, ImageError> {
        Ok(SoftwareCanvas {
            shared_state,
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

    fn create_paint(&mut self, config: PaintConfig) -> Paint {
        let hash = hash_of(&config);
        self.shared_state.borrow_mut().paints.insert(hash, config);
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
        for segment in path.iter() {
            for curve in segment {
                let mut t = 0.0;
                let delta = 0.0001;
                loop {
                    if t >= 1.0 {
                        break;
                    }

                    let p = curve.at(t) + Vec2::new(100.0, 100.0);
                    self.pixels.set(
                        p.x.round() as u32,
                        p.y.round() as u32,
                        self.shared_state
                            .borrow()
                            .paints
                            .get(&paint.handle)
                            .map_or(Color::DEFAULT, |p| p.fill_color),
                    );
                    t += delta;
                }
            }
        }
    }

    fn stroke_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }
}

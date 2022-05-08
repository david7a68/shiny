use std::{cell::RefCell, rc::Rc};

use crate::{
    backends::common::cpatch::{flatten, ChangeList, CurveBvh},
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
        rect::Rect,
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
        let mut path = path.clone();

        let prect = Rect::new(0.0, self.width() as f32, 0.0, self.height() as f32);

        if true {
            let mut change_buffer = ChangeList::default();
            let mut bvh_builder = CurveBvh::storage();

            // let mul = normalize(&mut path);
            println!("Num points before flattening: {}", path.x.len());
            let bvh = flatten(&mut path, &mut change_buffer, &mut bvh_builder);
            println!("BVH computed with {} nodes", bvh.nodes.len());
            println!("Num points after flattening: {}", path.x.len());

            // println!("\t nodes: {:?}", &bvh.nodes);

            for node in bvh.nodes.iter() {
                // let bbox = Rect::new(
                //     mul.left + (node.bbox.left * mul.width()),
                //     mul.left + (node.bbox.right * mul.width()),
                //     mul.top + (node.bbox.top * mul.height()),
                //     mul.top + (node.bbox.bottom * mul.height()),
                // );

                let bounds = (node.bbox + Vec2::new(400.0, 100.0)) & prect;
                if bounds.width() > 0.0 {
                    for x in bounds.left.round() as u32..bounds.right.round() as u32 {
                        self.pixels.set(x, bounds.top.round() as u32, Color::GREEN);
                        self.pixels
                            .set(x, bounds.bottom.round() as u32, Color::GREEN);
                    }
                    for y in bounds.top.round() as u32..bounds.bottom.round() as u32 {
                        self.pixels.set(bounds.left.round() as u32, y, Color::GREEN);
                        self.pixels
                            .set(bounds.right.round() as u32, y, Color::GREEN);
                    }
                }
            }
        }

        for segment in path.iter() {
            for curve in segment {
                let mut t = 0.0;
                let delta = 0.001;
                loop {
                    if t >= 1.0 {
                        break;
                    }

                    let p = curve.at(t) + Vec2::new(400.0, 100.0);
                    if p.x > 0.0 && p.y > 0.0 {
                        self.pixels.set(
                            p.x.round() as u32,
                            p.y.round() as u32,
                            self.shared_state
                                .borrow()
                                .paints
                                .get(&paint.handle)
                                .map_or(Color::DEFAULT, |p| p.fill_color),
                        );
                    }
                    t += delta;
                }

                if false {
                    // draw bounding boxes
                    let bounds = (curve.coarse_bounds() + Vec2::new(400.0, 100.0)) & prect;
                    if bounds.width() > 0.0 {
                        for x in bounds.left.round() as u32..bounds.right.round() as u32 {
                            self.pixels
                                .set(x, bounds.top.round() as u32, Color::BRIGHT_PINK);
                            self.pixels
                                .set(x, bounds.bottom.round() as u32, Color::BRIGHT_PINK);
                        }
                        for y in bounds.top.round() as u32..bounds.bottom.round() as u32 {
                            self.pixels
                                .set(bounds.left.round() as u32, y, Color::BRIGHT_PINK);
                            self.pixels
                                .set(bounds.right.round() as u32, y, Color::BRIGHT_PINK);
                        }
                    }
                }
            }
        }
    }

    fn stroke_path(&mut self, path: &Path, paint: Paint) {
        todo!()
    }
}

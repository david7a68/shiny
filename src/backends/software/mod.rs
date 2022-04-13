//! The software renderer backend.
//!
//! Rendering is done entirely on the CPU.

use std::rc::Rc;

use crate::{
    canvas::Canvas,
    color::Space as ColorSpace,
    image::{Error as ImageError, PixelFormat},
};

use self::canvas::SoftwareCanvas;

pub mod canvas;

pub struct Software {
    shared: Rc<Shared>,
}

impl Software {
    pub fn new() -> Self {
        Software {
            shared: Rc::new(Shared {}),
        }
    }

    pub fn new_canvas(
        &mut self,
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
    ) -> Result<impl Canvas, ImageError> {
        SoftwareCanvas::new(width, height, format, color_space, self.shared.clone())
    }
}

pub(super) struct Shared {}

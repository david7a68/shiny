//! The software renderer backend.
//!
//! Rendering is done entirely on the CPU.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    canvas::Canvas,
    color::Space as ColorSpace,
    hash::PassThroughHasher,
    image::{Error as ImageError, PixelFormat},
    paint::PaintConfig,
};

use self::canvas::SoftwareCanvas;

pub mod canvas;

pub struct Software {
    shared: Rc<RefCell<BackendState>>,
}

impl Software {
    pub fn new() -> Self {
        Software {
            shared: Rc::new(RefCell::new(BackendState {
                paints: HashMap::with_hasher(PassThroughHasher::default()),
            })),
        }
    }

    pub fn new_canvas(
        &self,
        width: u32,
        height: u32,
        format: PixelFormat,
        color_space: ColorSpace,
    ) -> Result<impl Canvas, ImageError> {
        SoftwareCanvas::new(width, height, format, color_space, self.shared.clone())
    }
}

pub(super) struct BackendState {
    paints: HashMap<u64, PaintConfig, PassThroughHasher>,
}

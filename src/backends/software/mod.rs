//! The software renderer backend.
//!
//! Rendering is done entirely on the CPU.

use std::rc::Rc;

use crate::{canvas::Canvas, color::Color};

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

    pub fn new_canvas<C: Color>(&mut self, width: u32, height: u32) -> impl Canvas<C> {
        SoftwareCanvas::new(width, height, self.shared.clone())
    }
}

pub(super) struct Shared {}

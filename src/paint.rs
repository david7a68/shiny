use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Paint {
    pub handle: u64,
}

impl Paint {
    pub(crate) fn new(handle: u64) -> Self {
        Paint { handle }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct PaintConfig {
    pub fill_color: Color,
    pub stroke_color: Color,
}

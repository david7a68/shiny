use crate::color::Color;

pub struct Paint {
    pub handle: u64,
}

impl Paint {
    pub(crate) fn new(handle: u64) -> Self {
        Paint { handle }
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PaintConfig {
    pub fill_color: Color,
    pub stroke_color: Color,
}

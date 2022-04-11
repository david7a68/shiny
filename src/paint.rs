use std::marker::PhantomData;

use crate::color::Color;

pub struct Paint<C: Color> {
    pub handle: u64,
    _phantom: PhantomData<C>,
}

#[derive(Clone, Debug)]
pub struct PaintConfig<C: Color> {
    pub fill_color: C,
    pub stroke_color: C,
}

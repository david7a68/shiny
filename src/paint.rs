use std::marker::PhantomData;

use crate::color::Color;

pub struct Paint<C: Color> {
    pub handle: u64,
    _phantom: PhantomData<C>,
}

impl <C: Color> Paint<C> {
    pub (crate) fn new(handle: u64) -> Self {
        Paint {
            handle,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PaintConfig<C: Color> {
    pub fill_color: C,
    pub stroke_color: C,
}

use crate::shapes::{path::Path, rect::Rect};
use std::marker::PhantomData;

/// Used for graph flattening (intersection) and fill scoring (ray cast).
pub struct CurveBvh<'a> {
    phantom: PhantomData<&'a f32>,
}

impl<'a> CurveBvh<'a> {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn find_intersecting_with(bounds: Rect, buffer: &mut Vec<usize>) {}
}

#[derive(Default)]
pub struct Builder {}

impl Builder {
    pub fn build(&mut self, path: &Path) -> CurveBvh {
        todo!()
    }
}

struct Node {}

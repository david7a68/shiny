use std::fmt::Debug;

#[cfg(target_arch = "x86_64")]
use super::x86::rect::Rect as RectImpl;

#[repr(transparent)]
pub struct Rect(RectImpl);

impl Rect {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self(RectImpl::new(left, right, top, bottom))
    }

    pub fn intersects_with(&self, rhs: &Rect) -> bool {
        self.0.intersects(&rhs.0)
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.0)
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("left", &self.0.left())
            .field("right", &self.0.right())
            .field("top", &self.0.top())
            .field("bottom", &self.0.bottom())
            .finish()
    }
}

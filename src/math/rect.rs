use std::fmt::Debug;

use super::simd::Float4;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Rect(Float4);

impl Rect {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self(Float4::new(left, right, top, bottom))
    }

    pub fn left(&self) -> f32 {
        self.0.a()
    }

    pub fn right(&self) -> f32 {
        self.0.b()
    }

    pub fn top(&self) -> f32 {
        self.0.c()
    }

    pub fn bottom(&self) -> f32 {
        self.0.d()
    }

    pub fn intersects_with(&self, rhs: &Rect) -> bool {
        let a = Float4::new(self.left(), self.top(), rhs.left(), rhs.top());
        let b = Float4::new(rhs.right(), rhs.bottom(), self.right(), self.bottom());
        a.less_or_equal(&b) == (true, true, true, true)
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("left", &self.left())
            .field("right", &self.right())
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let r = Rect::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(r.left(), 1.0);
        assert_eq!(r.right(), 2.0);
        assert_eq!(r.top(), 3.0);
        assert_eq!(r.bottom(), 4.0);
    }

    #[test]
    fn intersects() {
        {
            // miss on x-axis
            let left = Rect::new(10.0, 20.0, 10.0, 20.0);
            let right = Rect::new(30.0, 40.0, 10.0, 20.0);

            assert!(!left.intersects_with(&right));
            assert!(!right.intersects_with(&left));
        }
        {
            // miss on y axis
            let top = Rect::new(10.0, 20.0, 10.0, 20.0);
            let bottom = Rect::new(10.0, 20.0, 30.0, 40.0);

            assert!(!top.intersects_with(&bottom));
            assert!(!bottom.intersects_with(&top));
        }
        {
            // one in the other
            let outer = Rect::new(10.0, 20.0, 10.0, 20.0);
            let inner = Rect::new(12.0, 18.0, 12.0, 20.0);

            assert!(outer.intersects_with(&inner));
            assert!(inner.intersects_with(&outer));
        }
        {
            // one is the other
            let a = Rect::new(10.0, 20.0, 10.0, 20.0);
            let b = a;

            assert!(a.intersects_with(&b));
            assert!(b.intersects_with(&a));
        }
        {
            // normal intersection
            let a = Rect::new(10.0, 20.0, 10.0, 20.0);
            let b = Rect::new(15.0, 25.0, 15.0, 25.0);

            assert!(a.intersects_with(&b));
            assert!(b.intersects_with(&a));
        }
        {
            // line intersection
            let horizontal = Rect::new(10.0, 20.0, 10.0, 10.0);
            let vertical = Rect::new(15.0, 15.0, 10.0, 20.0);

            assert!(horizontal.intersects_with(&vertical));
            assert!(vertical.intersects_with(&horizontal));
        }
    }
}

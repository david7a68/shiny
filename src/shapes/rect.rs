use std::{
    fmt::Debug,
    ops::{Add, AddAssign, BitOr, BitAnd},
};

use crate::math::{
    cmp::{max, min},
    simd::Float4,
    vector2::Vec2,
};

use super::point::Point;

#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    #[inline]
    #[must_use]
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Finds the smallest rectangle that contains all the points in the given
    /// slice. Returns an empty rectangle if the slice is empty.
    #[must_use]
    pub fn enclosing(points: &[Point]) -> Self {
        let mut left = 0.0;
        let mut right = 0.0;
        let mut top = 0.0;
        let mut bottom = 0.0;

        for p in points {
            left = min!(left, p.x);
            right = max!(right, p.x);
            top = min!(top, p.y);
            bottom = max!(bottom, p.y);
        }

        Self::new(left, right, top, bottom)
    }

    #[must_use]
    pub fn extent(&self) -> Vec2 {
        Vec2::new(self.right - self.left, self.top - self.bottom)
    }

    #[must_use]
    pub fn origin(&self) -> Point {
        Point::new(self.left, self.bottom)
    }

    #[must_use]
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    #[must_use]
    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }

    #[must_use]
    pub fn centroid(&self) -> Point {
        Point::new(
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }

    #[must_use]
    pub fn intersects_with(&self, rhs: &Rect) -> bool {
        let a = Float4::new(self.left, self.top, rhs.left, rhs.top);
        let b = Float4::new(rhs.right, rhs.bottom, self.right, self.bottom);
        a.less_or_equal(b) == (true, true, true, true)
    }
}

impl Default for Rect {
    #[must_use]
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Add for Rect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            left: min!(self.left, rhs.left),
            right: max!(self.right, rhs.right),
            top: min!(self.top, rhs.top),
            bottom: max!(self.bottom, rhs.bottom),
        }
    }
}

impl Add<Vec2> for Rect {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            left: self.left + rhs.x(),
            right: self.right + rhs.x(),
            top: self.top + rhs.y(),
            bottom: self.bottom + rhs.y(),
        }
    }
}

impl AddAssign for Rect {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl BitAnd for Rect {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if !self.intersects_with(&rhs) {
            Self::default()
        } else {
            Self {
                left: max!(self.left, rhs.left),
                right: min!(self.right, rhs.right),
                top: max!(self.top, rhs.top),
                bottom: min!(self.bottom, rhs.bottom),
            }
        }
    }   
}

impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("top", &self.top)
            .field("bottom", &self.bottom)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let r = Rect::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(r.left, 1.0);
        assert_eq!(r.right, 2.0);
        assert_eq!(r.top, 3.0);
        assert_eq!(r.bottom, 4.0);
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

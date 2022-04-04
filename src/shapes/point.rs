use std::ops::Sub;

use crate::math::{cmp::ApproxEq, interp::Interpolate, vector::Vec2};

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn vec(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Sub<Point> for Point {
    type Output = Vec2;
    fn sub(self, rhs: Point) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<Vec2> for Point {
    fn from(v: Vec2) -> Self {
        Self::new(v.x(), v.y())
    }
}

impl Interpolate for Point {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        Self {
            x: self.x.lerp(t, &rhs.x),
            y: self.y.lerp(t, &rhs.y),
        }
    }
}

impl ApproxEq for Point {
    fn approx_eq(&self, other: Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y)
    }

    fn approx_eq_within(&self, other: Self, epsilon: f32) -> bool {
        self.x.approx_eq_within(other.x, epsilon) && self.y.approx_eq_within(other.y, epsilon)
    }
}

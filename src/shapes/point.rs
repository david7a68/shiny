use std::{
    hash::Hash,
    ops::{Add, Sub},
};

use crate::math::{
    cmp::{ApproxEq, F32_APPROX_EQUAL_THRESHOLD},
    ops::Interpolate,
    vector2::Vec2,
};

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default)]
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

impl Add<Vec2> for Point {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

impl Add<Point> for Vec2 {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x() + rhs.x, self.y() + rhs.y)
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
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_within(other, F32_APPROX_EQUAL_THRESHOLD)
    }

    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        self.x.approx_eq_within(&other.x, epsilon) && self.y.approx_eq_within(&other.y, epsilon)
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.x.to_bits());
        state.write_u32(self.y.to_bits());
    }
}

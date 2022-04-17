use std::ops::{Add, Div, Mul, Sub};

use crate::shapes::point::Point;

use super::cmp::ApproxEq;

/// A vector in 2D space.
#[derive(Clone, Copy, Default, PartialEq)]
#[allow(non_camel_case_types)]
pub struct Vec2(f32, f32);

impl Vec2 {
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    #[inline]
    #[must_use]
    pub fn x(self) -> f32 {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn y(self) -> f32 {
        self.1
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn length2(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        x + y
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Point> for Vec2 {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.0 + rhs.x, self.1 + rhs.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ApproxEq<&Self> for Vec2 {
    #[inline]
    #[must_use]
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(other.0) && self.1.approx_eq(other.1)
    }

    #[inline]
    #[must_use]
    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_within(other.0, epsilon) && self.1.approx_eq_within(other.1, epsilon)
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert!((a - b).approx_eq(&Vec2::new(-2.0, -2.0)));
    }

    #[test]
    fn multiplication() {
        {
            let a = Vec2::new(1.0, 2.0);
            assert!((a * 2.0).approx_eq(&Vec2::new(2.0, 4.0)));
        }
    }
}

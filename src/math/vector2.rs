use std::ops::{Add, Div, Mul, Neg, Sub};

use super::cmp::{ApproxEq, F32_APPROX_EQUAL_THRESHOLD};

/// A vector in 2D space.
#[derive(Clone, Copy, Default, PartialEq)]
#[allow(non_camel_case_types)]
pub struct Vec2(f32, f32);

impl Vec2 {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    #[must_use]
    pub fn x(self) -> f32 {
        self.0
    }

    #[must_use]
    pub fn y(self) -> f32 {
        self.1
    }

    #[must_use]
    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }

    #[must_use]
    pub fn length2(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        x + y
    }
}

// Unary Ops

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

// Binary Ops: Vec2 Vec2

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ApproxEq<Self> for Vec2 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_within(other, F32_APPROX_EQUAL_THRESHOLD)
    }

    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_within(&other.0, epsilon) && self.1.approx_eq_within(&other.1, epsilon)
    }
}

// Binary Ops: Vec2 f32

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2(self.0 + rhs, self.1 + rhs)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2(self.0 - rhs, self.1 - rhs)
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

// Binary Ops: f32 Vec2

impl Add<Vec2> for f32 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        rhs.add(self)
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self - rhs.0, self - rhs.1)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2(self / rhs.x(), self / rhs.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        // unop
        assert!(a.neg().approx_eq(&Vec2::new(-1.0, -2.0)));
        assert_eq!(format!("{:?}", a), "Vec2 { x: 1.0, y: 2.0 }");

        // binop: vector vector
        assert!(a.approx_eq(&a));
        assert!(!a.approx_eq(&b));
        assert!((a + b).approx_eq(&Vec2::new(4.0, 6.0)));
        assert!((a - b).approx_eq(&Vec2::new(-2.0, -2.0)));

        // binop: vector scalar
        assert!((a + 1.0).approx_eq(&Vec2::new(2.0, 3.0)));
        assert!((a - 1.0).approx_eq(&Vec2::new(0.0, 1.0)));
        assert!((a * 2.0).approx_eq(&Vec2::new(2.0, 4.0)));
        assert!((a / 2.0).approx_eq(&Vec2::new(0.5, 1.0)));

        // binop: scalar vector
        assert!((1.0 + a).approx_eq(&Vec2::new(2.0, 3.0)));
        assert!((1.0 - a).approx_eq(&Vec2::new(0.0, -1.0)));
        assert!((2.0 * a).approx_eq(&Vec2::new(2.0, 4.0)));
        assert!((2.0 / a).approx_eq(&Vec2::new(2.0, 1.0)));
    }
}

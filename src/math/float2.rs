use std::ops::{Add, Div, Mul, Sub};

/// A vector in 2D space.
#[derive(Clone, Copy, Default, PartialEq)]
#[allow(non_camel_case_types)]
pub struct Float2(pub f32, pub f32);

impl Float2 {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<f32> for Float2 {
    type Output = Float2;

    fn mul(self, rhs: f32) -> Self::Output {
        Float2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for Float2 {
    type Output = Float2;
    fn div(self, rhs: f32) -> Self::Output {
        Float2(self.0 / rhs, self.1 / rhs)
    }
}

impl Add<Float2> for Float2 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Self::Output {
        Float2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Float2> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Self::Output {
        Float2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::fmt::Debug for Float2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

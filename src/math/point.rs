use std::ops::Sub;

use super::float2::Float2;

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn vec(self) -> Float2 {
        Float2(self.0, self.1)
    }
}

impl Sub<Point> for Point {
    type Output = Float2;
    fn sub(self, rhs: Point) -> Self::Output {
        Float2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<Float2> for Point {
    fn from(v: Float2) -> Self {
        Self(v.0, v.1)
    }
}

use std::ops::Sub;

use super::vec2::Vec2;

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

    pub fn vec(self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
}

impl Sub<Point> for Point {
    type Output = Vec2;
    fn sub(self, rhs: Point) -> Self::Output {
        Vec2::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<Vec2> for Point {
    fn from(v: Vec2) -> Self {
        Self(v.x(), v.y())
    }
}

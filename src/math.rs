//! Math operations and geometric types such as [`Point`] and [`Vec2`].

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }
}

/// A vector in 2D space.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec2(pub f32, pub f32);

impl Vec2 {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.0)
            .field("y", &self.1)
            .finish()
    }
}

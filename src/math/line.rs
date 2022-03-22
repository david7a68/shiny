use super::{x86::vector4::Vector4, Point, Float4, constants::APPROX_EQUAL_THRESHOLD};

/// A line, held in normalized standard form.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Line {
    parts: Vector4,
}

impl Line {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self {
            parts: Vector4::from_tuple(a, b, c, 0.0),
        }
    }

    pub fn from_vector(vector: Float4) -> Self {
        Self {
            parts: vector.0
        }
    }

    pub fn a(&self) -> f32 {
        self.parts.extract().0
    }

    pub fn b(&self) -> f32 {
        self.parts.extract().1
    }

    pub fn c(&self) -> f32 {
        self.parts.extract().2
    }

    /// Calculates the position of a point at the given x-coordinate.
    pub fn at_x(&self, x: f32) -> Option<Point> {
        if !self.b().is_nan() {
            let y = (self.a() * x + self.c()) / self.b();
            Some(Point(x, y))
        } else {
            None
        }
    }

    /// Calculates the distance of the point from the line.
    pub fn distance(&self, point: Point) -> f32 {
        self.a() * point.x() + self.b() * point.y() + self.c()
    }

    pub fn approx_equal(&self, rhs: &Self) -> bool {
        let diff = self.parts.sub(rhs.parts).abs();
        let limit = Vector4::splat(APPROX_EQUAL_THRESHOLD);
        diff.less(&limit) == 0b111
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(other.parts) == 0b111
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line")
            .field("a", &self.a())
            .field("b", &self.b())
            .field("c", &self.c())
            .finish()
    }
}

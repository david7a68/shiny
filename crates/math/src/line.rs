use std::ops::Neg;

use super::{
    constants::APPROX_EQUAL_THRESHOLD, float4::Float4, point::Point, x86::vector4::Vector4,
};

/// A line, held in normalized standard form.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Line {
    parts: Vector4,
}

impl Line {
    /// Creates a new line from standard from and stores it in normalized
    /// representation.
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        // normalize
        let v = Vector4::from_tuple(a, b, c, 0.0);
        let div = (a * a + b * b).sqrt();

        Self {
            parts: v.div(Vector4::splat(div)),
        }
    }

    pub fn with_c(other: Line, c: f32) -> Self {
        let (a, b, ..) = other.parts.extract();
        Self {
            parts: Vector4::from_tuple(a, b, c, 0.0),
        }
    }

    pub fn between(p1: Point, p2: Point) -> Self {
        if p1.x() == p2.x() {
            Line::new(p1.x(), 0.0, 0.0)
        } else {
            let delta = p2 - p1;
            let slope = delta.y() / delta.x();
            let offset = p1.y() - slope * p1.x();
            Self::new(slope, -1.0, offset)
        }
    }

    /// Creates a new line in standard form from a normalized vector.
    pub unsafe fn from_normalized_vector(vector: Float4) -> Self {
        Self { parts: vector.0 }
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
        if self.b() != 0.0 {
            let y = (self.a() * x + self.c()) / -self.b();
            Some(Point(x, y))
        } else {
            None
        }
    }

    pub fn x_intercept(&self) -> f32 {
        self.a() / self.c()
    }

    /// Calculates the distance from `point` to the nearest point on the line.
    pub fn distance_to(&self, point: Point) -> f32 {
        (self.a() * point.x() + self.b() * point.y() + self.c()).abs()
    }

    pub fn approx_equal(&self, rhs: &Self) -> bool {
        let diff = self.parts.sub(rhs.parts).abs();
        let limit = Vector4::splat(APPROX_EQUAL_THRESHOLD);
        diff.less(&limit) == (true, true, true, true)
    }

    /// Calculates a line parallel to the current line that passes through
    /// `point`.
    pub fn parallel_through(&self, point: Point) -> Self {
        let c = -(self.a() * point.x()) - self.b() * point.y();
        Line {
            parts: Vector4::from_tuple(self.a(), self.b(), c, 0.0),
        }
    }
}

impl Neg for Line {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (a, b, c, _) = self.parts.extract();
        Line {
            parts: Vector4::from_tuple(-a, -b, c, 0.0),
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(other.parts) == (true, true, true, false)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn between_points() {
        {
            // regular line
            let line = Line::between(Point(2.0, 2.0), Point(6.0, 4.0));
            assert!(line.approx_equal(&Line::new(0.4472136, -0.8944272, 0.8944272)));
            assert_eq!(line.at_x(2.0).unwrap().y(), 2.0);
            assert_eq!(line.at_x(6.0).unwrap().y(), 4.0);
        }
        {
            // vertical line
            let line = Line::between(Point(5.0, 1.0), Point(5.0, 12.0));
            assert_eq!(line.at_x(1.0), None);
        }
        {
            // horizontal line
            let line = Line::between(Point(2.0, 2.0), Point(6.0, 2.0));
            assert_eq!(line.at_x(2.0).unwrap().y(), 2.0);
            assert_eq!(line.at_x(6.0).unwrap().y(), 2.0);
            assert_eq!(line.at_x(100.0).unwrap().y(), 2.0);
        }
    }

    #[test]
    fn distance() {
        let line = Line::between(Point(2.0, 2.0), Point(6.0, 4.0));
        assert_eq!(line.distance_to(Point(2.0, 2.0)), 0.0);
        assert_eq!(line.distance_to(Point(2.0, 3.0)), 0.89442706);
    }
}

use std::ops::Neg;

use super::point::Point;
use crate::math::cmp::ApproxEq;

/// A line, held in normalized standard form.
#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Line {
    /// Creates a new line from standard from and stores it in normalized
    /// representation.
    #[must_use]
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        // normalize
        let div = (a * a + b * b).sqrt();

        Self {
            a: a / div,
            b: b / div,
            c: c / div,
        }
    }

    #[must_use]
    pub fn with_c(other: Line, c: f32) -> Self {
        Self {
            a: other.a,
            b: other.b,
            c,
        }
    }

    #[must_use]
    pub fn between(p1: Point, p2: Point) -> Self {
        if p1.x.approx_eq(p2.x) {
            Line::new(p1.x, 0.0, 0.0)
        } else {
            let delta = p2 - p1;
            let slope = delta.y() / delta.x();
            let offset = -(slope * p1.x) + p1.y;
            Self::new(slope, -1.0, offset)
        }
    }

    /// Calculates the position of a point at the given x-coordinate.
    #[must_use]
    pub fn y_at(&self, x: f32) -> Option<Point> {
        if self.b == 0.0 {
            None
        } else {
            let y = -(self.a * x + self.c) / self.b;
            Some(Point::new(x, y))
        }
    }

    #[must_use]
    pub fn x_intercept(&self) -> f32 {
        -self.c / self.a
    }

    /// Calculates the signed distance from `point` to the nearest point on the
    /// line, where a negative result is on the same side as the origin and a
    /// positive result is on the opposite side from the origin.
    #[must_use]
    pub fn signed_distance_to(&self, point: Point) -> f32 {
        self.a * point.x + self.b * point.y + self.c
    }

    #[must_use]
    pub fn approx_eq(&self, rhs: &Self) -> bool {
        self.a.approx_eq(rhs.a) & self.b.approx_eq(rhs.b) & self.c.approx_eq(rhs.c)
    }

    /// Calculates a line parallel to the current line that passes through
    /// `point`.
    #[must_use]
    pub fn parallel_through(&self, point: Point) -> Self {
        let c = -(self.a * point.x) - (self.b * point.y);
        Self {
            a: self.a,
            b: self.b,
            c,
        }
    }
}

impl Neg for Line {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn between_points() {
        {
            // line with positive slope
            let line = Line::between(Point::new(2.0, 2.0), Point::new(6.0, 4.0));
            assert!(line.approx_eq(&Line::new(0.4472136, -0.8944272, 0.8944272)));
            assert!(2.0.approx_eq(line.y_at(2.0).unwrap().y));
            assert!(2.5.approx_eq(line.y_at(3.0).unwrap().y));
            assert!(3.0.approx_eq(line.y_at(4.0).unwrap().y));
            assert!(3.5.approx_eq(line.y_at(5.0).unwrap().y));
            assert!(4.0.approx_eq(line.y_at(6.0).unwrap().y));
        }
        {
            // line with negative slope
            let line = Line::between(Point::new(2.0, 4.0), Point::new(6.0, 2.0));
            assert!(4.0.approx_eq(line.y_at(2.0).unwrap().y));
            assert!(3.5.approx_eq(line.y_at(3.0).unwrap().y));
            assert!(3.0.approx_eq(line.y_at(4.0).unwrap().y));
            assert!(2.5.approx_eq(line.y_at(5.0).unwrap().y));
            assert!(2.0.approx_eq(line.y_at(6.0).unwrap().y));
        }
        {
            // vertical line
            let line = Line::between(Point::new(5.0, 1.0), Point::new(5.0, 12.0));
            assert_eq!(line.y_at(1.0), None);
        }
        {
            // horizontal line
            let line = Line::between(Point::new(2.0, 2.0), Point::new(6.0, 2.0));
            assert_eq!(line.y_at(2.0).unwrap().y, 2.0);
            assert_eq!(line.y_at(6.0).unwrap().y, 2.0);
            assert_eq!(line.y_at(100.0).unwrap().y, 2.0);
        }
    }

    #[test]
    fn distance() {
        let line = Line::between(Point::new(2.0, 2.0), Point::new(6.0, 4.0));
        assert_eq!(line.signed_distance_to(Point::new(2.0, 2.0)), 0.0);
        // point on same side as the origin
        assert!(0.89442706.approx_eq(line.signed_distance_to(Point::new(2.0, 1.0))));
        // point on opposite side of the origin
        assert!((-0.89442706).approx_eq(line.signed_distance_to(Point::new(2.0, 3.0))));
    }
}

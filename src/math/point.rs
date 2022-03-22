use std::ops::Sub;

use super::{float2::Float2, line::Line};

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
        Float2::new(self.0, self.1)
    }

    /// Identifies the line self->rhs and returns it in normalized standard form
    /// `0=Ax+Bx+C` where √(A² + B²) = 1.
    /// 
    /// Standard form was chosen because it can represent vertical lines.
    pub fn line_to(self, rhs: Self) -> Line {
        if self.x() == rhs.x() {
            Line::new(self.x(), 0.0, 0.0)
        }
        else {
            let delta = rhs - self;
            let slope = delta.y() / delta.x();
            let offset = self.y() - slope * self.x();
    
            let scale = Float2::new(slope, 1.0).length();
    
            let a = slope / scale;
            let b = 1.0 / scale;
            let c = offset / scale;
    
            Line::new(a, b, c)
        }
    }
}

impl Sub<Point> for Point {
    type Output = Float2;
    fn sub(self, rhs: Point) -> Self::Output {
        Float2::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<Float2> for Point {
    fn from(v: Float2) -> Self {
        Self(v.x(), v.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to() {
        let float_eq_threshold = 1e-6;
        {
            let line = Point(2.0, 2.0).line_to(Point(6.0, 4.0));
            assert_eq!(line, Line::new(0.4472136, 0.8944272, 0.8944272));

            println!("{:?} at x=2.0: {:?}", line, line.at_x(2.0));
            assert!((2.0 - line.at_x(2.0).unwrap().y()) < float_eq_threshold);
            assert!((4.0 - line.at_x(6.0).unwrap().y()) < float_eq_threshold);
        }
        {
            let line = Point(5.0, 1.0).line_to(Point(5.0, 12.0));
            assert_eq!(line.at_x(1.0), None);
        }
    }
}

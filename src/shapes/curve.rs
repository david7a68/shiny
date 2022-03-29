use crate::{
    math::{float4::Float4, float4x4::Float4x4, line::Line, point::Point},
    utils::cmp::{max, min},
};

pub struct Bezier<'a> {
    points: &'a [Point; 4],
}

impl<'a> Bezier<'a> {
    pub fn new(points: &'a [Point; 4]) -> Self {
        Self { points }
    }

    pub fn at(&self, t: f32) -> Point {
        // let t_inv = 1.0 - t;
        // ((t_inv.powf(3.0) * self.p0.vec()) + (3.0 * t_inv.powf(2.0) * t * self.p1.vec()) + (3.0 * t_inv * t.powf(2.0) * self.p2.vec()) + (t.powf(3.0) * self.p3.vec())).into()
        let t = Float4::new(1.0, t, t.powf(2.0), t.powf(3.0));
        let m = Float4x4::new(
            Float4::new(1.0, 0.0, 0.0, 0.0),
            Float4::new(-3.0, 3.0, 0.0, 0.0),
            Float4::new(3.0, -6.0, 3.0, 0.0),
            Float4::new(-1.0, 3.0, -3.0, 1.0),
        );

        let px = Float4::new(
            self.points[0].x(),
            self.points[1].x(),
            self.points[2].x(),
            self.points[3].x(),
        );
        let py = Float4::new(
            self.points[0].y(),
            self.points[1].y(),
            self.points[2].y(),
            self.points[3].y(),
        );

        let tm = t * m;
        let tmx = tm.mul_elements(&px);
        let tmy = tm.mul_elements(&py);

        let (x, y) = Float4::hsum2(tmx, tmy);
        Point(x, y)
    }
}

/// Implements the Bezier clipping algorithm to find intersections between two
/// curves.
pub mod clipping {
    use super::*;

    /// Clips `a` against `b`, producing t-bounds where `a` lies within `b`'s fat
    /// line.
    pub fn clip(curve: &Bezier, against: &Bezier) -> (f32, f32) {
        let (min_line, max_line) = {
            let thin = Line::between(against.points[0], against.points[3]);
            let line1 = thin.parallel_through(against.points[1]);
            let line2 = thin.parallel_through(against.points[2]);
            let min_c = min!(thin.c(), line1.c(), line2.c());
            let max_c = max!(thin.c(), line1.c(), line2.c());
            (-Line::with_c(thin, min_c), Line::with_c(thin, max_c))
        };

        let min_clip = clip_line(curve, &min_line);
        let max_clip = clip_line(curve, &max_line);

        (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
    }

    /// Clips `curve` against `line`, returning a t-bound that is guaranteed to
    /// be 'above' the line (distance is positive).
    /// 
    /// This algorithm does not attempt to calculate the precise point of
    /// intersection, but only a close-enough approximation.
    pub fn clip_line(curve: &Bezier, line: &Line) -> (f32, f32) {
        let e0 = Point(0.0 / 3.0, line.distance_to(curve.points[0]));
        let e1 = Point(1.0 / 3.0, line.distance_to(curve.points[1]));
        let e2 = Point(2.0 / 3.0, line.distance_to(curve.points[2]));
        let e3 = Point(3.0 / 3.0, line.distance_to(curve.points[3]));

        // Test the left of the curve (low-t)
        let low = if e0.y() < 0.0 {
            let x1 = Line::between(e0, e1).x_intercept();
            let x2 = Line::between(e0, e2).x_intercept();
            let x3 = Line::between(e0, e3).x_intercept();
            // Smallest value in the range (0, 1)
            let mut min = 1.0;
            if x1 > 0.0 && x1 < min {
                min = x1;
            }
            if x2 > 0.0 && x2 < min {
                min = x2;
            }
            if x3 > 0.0 && x3 < min {
                min = x3
            }
            min
        } else {
            0.0
        };

        // Test the right of the curve (high-t)
        let high = if e3.y() < 0.0 {
            let x1 = Line::between(e3, e0).x_intercept();
            let x2 = Line::between(e3, e1).x_intercept();
            let x3 = Line::between(e3, e2).x_intercept();
            // Largest value in the range (0, 1)
            let mut max = 0.0;
            if x1 < 1.0 && x1 > max {
                max = x1;
            }
            if x2 < 1.0 && x2 > max {
                max = x2;
            }
            if x3 < 1.0 && x3 > max {
                max = x3
            }
            max
        } else {
            1.0
        };

        (low, high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_bezier() {
        let points_1 = [
            Point(24.0, 21.0),
            Point(189.0, 40.0),
            Point(159.0, 137.0),
            Point(101.0, 261.0),
        ];
        let points_2 = [
            Point(18.0, 122.0),
            Point(15.0, 178.0),
            Point(247.0, 173.0),
            Point(251.0, 242.0),
        ];

        let curve_1 = Bezier { points: &points_1 };
        let curve_2 = Bezier { points: &points_2 };

        let curve1_limits = clipping::clip(&curve_1, &curve_2);
        assert_eq!(curve1_limits, (0.18543269, 0.91614604));
    }
}

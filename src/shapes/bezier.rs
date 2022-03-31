// use super::interp::Interpolate;
// use super::simd::Float4;
// use super::{line::Line, point::Point, rect::Rect};
// use super::{mat4x4::Mat4x4, vec4::Vec4};
use super::{line::Line, point::Point, rect::Rect};
use crate::{
    math::{interp::Interpolate, mat4x4::Mat4x4, simd::Float4, vec4::Vec4},
    utils::cmp::{max, min},
};

pub trait Bezier: Sized {
    type Owning;

    fn at(&self, t: f32) -> Point;

    fn coarse_bounds(&self) -> Rect;

    fn split(&self, t: f32) -> (Self::Owning, Self::Owning);

    fn split2(&self, t1: f32, t3: f32) -> (Self::Owning, Self::Owning, Self::Owning);
}

/// A cubic bezier curve.
pub struct CubicBezier {
    pub points: [Point; 4],
}

impl CubicBezier {
    pub fn as_ref(&self) -> CubicBezierSlice {
        CubicBezierSlice::new(&self.points)
    }
}

impl Bezier for CubicBezier {
    type Owning = Self;

    fn at(&self, t: f32) -> Point {
        evaluate(&self.points, t)
    }

    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(&self.points)
    }

    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        let (left, right) = split(&self.points, t);
        (
            Self::Owning { points: left },
            Self::Owning { points: right },
        )
    }

    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        let (left, center, right) = split2(&self.points, t1, t2);
        (
            Self::Owning { points: left },
            Self::Owning { points: center },
            Self::Owning { points: right },
        )
    }
}

/// A cubic bezier curve as a reference to a slice of 4 points. Useful for e.g.
/// composites of several curves, where the first and last point can be shared
/// with the curves before and after, respectively. This can significantly
/// reduce the number of points that need to be stored.
pub struct CubicBezierSlice<'a> {
    pub points: &'a [Point; 4],
}

impl<'a> CubicBezierSlice<'a> {
    pub fn new(points: &'a [Point; 4]) -> Self {
        Self { points }
    }
}

impl<'a> Bezier for CubicBezierSlice<'a> {
    type Owning = CubicBezier;

    fn at(&self, t: f32) -> Point {
        evaluate(self.points, t)
    }

    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(self.points)
    }

    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        let (left, right) = split(self.points, t);
        (
            Self::Owning { points: left },
            Self::Owning { points: right },
        )
    }

    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        let (left, center, right) = split2(self.points, t1, t2);
        (
            Self::Owning { points: left },
            Self::Owning { points: center },
            Self::Owning { points: right },
        )
    }
}

fn evaluate(bezier: &[Point; 4], t: f32) -> Point {
    // let t_inv = 1.0 - t;
    // ((t_inv.powf(3.0) * self.p0.vec()) + (3.0 * t_inv.powf(2.0) * t * self.p1.vec()) + (3.0 * t_inv * t.powf(2.0) * self.p2.vec()) + (t.powf(3.0) * self.p3.vec())).into()
    let t = Vec4::new(1.0, t, t.powf(2.0), t.powf(3.0));
    #[rustfmt::skip]
    let m = Mat4x4::new(
        1.0, 0.0, 0.0, 0.0,
        -3.0, 3.0, 0.0, 0.0,
        3.0, -6.0, 3.0, 0.0,
        -1.0, 3.0, -3.0, 1.0,
    );

    let px = Float4::new(bezier[0].x(), bezier[1].x(), bezier[2].x(), bezier[3].x());
    let py = Float4::new(bezier[0].y(), bezier[1].y(), bezier[2].y(), bezier[3].y());

    let tm = t * m;
    let tmx = tm.0 * px;
    let tmy = tm.0 * py;

    let (x, y) = Float4::horizontal_sum2(tmx, tmy);
    Point(x, y)
}

fn coarse_bounds(bezier: &[Point; 4]) -> Rect {
    // let x_min = self.p0.x().min(self.p1.x()).min(self.p2.x().min(self.p2.x()));
    // let x_max = self.p0.x().max(self.p1.x()).max(self.p2.x().max(self.p2.x()));
    // let y_min = self.p0.y().min(self.p1.y()).min(self.p2.y().min(self.p2.y()));
    // let y_max = self.p0.y().max(self.p1.y()).max(self.p2.y().max(self.p2.y()));

    // BoundingBox {
    //     left: x_min,
    //     right: x_max,
    //     top: y_min,
    //     bottom: y_max
    // }

    let a = Float4::new(bezier[0].x(), bezier[0].y(), bezier[1].x(), bezier[1].y());
    let b = Float4::new(bezier[2].x(), bezier[2].y(), bezier[3].x(), bezier[3].y());

    let min1 = a.min(&b);
    let min2 = min1.cdab();
    let min3 = min1.min(&min2);

    let max1 = a.max(&b);
    let max2 = max1.cdab();
    let max3 = max1.max(&max2);

    Rect::new(min3.a(), max3.a(), min3.b(), max3.b())
}

fn split(bezier: &[Point; 4], t: f32) -> ([Point; 4], [Point; 4]) {
    let mid_01 = bezier[0].lerp(t, &bezier[1]);
    let mid_12 = bezier[1].lerp(t, &bezier[2]);
    let mid_23 = bezier[2].lerp(t, &bezier[3]);

    let mid_01_12 = mid_01.lerp(t, &mid_12);
    let mid_12_23 = mid_12.lerp(t, &mid_23);

    let midpoint = mid_01_12.lerp(t, &mid_12_23);

    let a = [bezier[0], mid_01, mid_01_12, midpoint];
    let b = [midpoint, mid_12_23, mid_23, bezier[3]];

    (a, b)
}

fn split2(bezier: &[Point; 4], t1: f32, t2: f32) -> ([Point; 4], [Point; 4], [Point; 4]) {
    let (left, rest) = split(bezier, t1);
    let (mid, right) = split(&rest, (t2 - t1) / (1.0 - t1));
    (left, mid, right)
}

/// Clips `a` against `b`, producing t-bounds where `a` lies within `b`'s fat
/// line.
pub fn clip(curve: &[Point; 4], against: &[Point; 4]) -> (f32, f32) {
    let (min_line, max_line) = {
        let thin = Line::between(against[0], against[3]);
        let line1 = thin.parallel_through(against[1]);
        let line2 = thin.parallel_through(against[2]);
        let min_c = min!(thin.c, line1.c, line2.c);
        let max_c = max!(thin.c, line1.c, line2.c);
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
pub fn clip_line(curve: &[Point; 4], line: &Line) -> (f32, f32) {
    let e0 = Point(0.0, line.signed_distance_to(curve[0]));
    let e1 = Point(1.0 / 3.0, line.signed_distance_to(curve[1]));
    let e2 = Point(2.0 / 3.0, line.signed_distance_to(curve[2]));
    let e3 = Point(1.0, line.signed_distance_to(curve[3]));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        let bezier = CubicBezier {
            points: [
                Point(10.0, 5.0),
                Point(3.0, 11.0),
                Point(12.0, 20.0),
                Point(6.0, 15.0),
            ],
        };

        assert_eq!(bezier.at(0.0), Point(10.0, 5.0));
        assert_eq!(bezier.at(0.5), Point(7.625, 14.125));
        assert_eq!(bezier.at(1.0), Point(6.0, 15.0));
    }

    #[test]
    fn coarse_bounds() {
        let bezier = CubicBezier {
            points: [
                Point(10.0, 5.0),
                Point(3.0, 11.0),
                Point(12.0, 20.0),
                Point(6.0, 15.0),
            ],
        };

        let bounds = bezier.coarse_bounds();

        assert_eq!(bounds, Rect::new(3.0, 12.0, 5.0, 20.0));
    }

    #[test]
    fn clip_bezier() {
        let curve1 = CubicBezier {
            points: [
                Point(24.0, 21.0),
                Point(189.0, 40.0),
                Point(159.0, 137.0),
                Point(101.0, 261.0),
            ],
        };
        let curve2 = CubicBezier {
            points: [
                Point(18.0, 122.0),
                Point(15.0, 178.0),
                Point(247.0, 173.0),
                Point(251.0, 242.0),
            ],
        };

        let curve1_limits = clip(&curve1.points, &curve2.points);
        assert_eq!(curve1_limits, (0.18543269, 0.91614604));
    }
}

// /// Removes intersections between two curves, replacing each with
// /// non-intersecting spans. The resulting arrays of curves are in-order from t=0
// /// to t=1.
// ///
// /// This process makes use of the bezier clipping algorithm to identify
// /// curve-curve intersections.
// fn flatten<'a, 'b>(
//     lhs: &'a CubicBezier,
//     rhs: &'b CubicBezier,
// ) -> (
//     ArrayVec<CubicBezierSlice<'a>, 10>,
//     ArrayVec<CubicBezierSlice<'b>, 10>,
// ) {
//     let mut a = lhs.get(..);
//     let mut b = rhs.get(..);

//     let mut lhs_intersections = ArrayVec::<f32, 10>::new();
//     let mut rhs_intersections = ArrayVec::<f32, 10>::new();
//     let mut iterations = 0;
//     loop {
//         let a_ok = (a.end - a.start) < 0.0001;
//         let b_ok = (b.end - b.start) < 0.0001;
//         if a_ok & b_ok {
//             if (iterations & 1) == 0 {
//                 lhs_intersections.push(a.end);
//                 rhs_intersections.push(b.end);
//             } else {
//                 lhs_intersections.push(b.end);
//                 rhs_intersections.push(a.end);
//             }
//             break;
//         }

//         let (l, r) = clip(&a, &b);

//         // if l.diff or r.diff shrank by less than 20%, split the longest one in
//         // half and try again.

//         a = r;
//         b = l;
//         iterations += 1;
//     }

//     lhs_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     rhs_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

//     /*
//     The Bezier Clipping Method:

//     1. To clip P(t) against Q(u)...
//         a. Identify the fat line L that bounds Q(u).
//             i. Optionally, identify the fat line K perpendicular to Q(u) and
//                select the narrower of the two.
//         b. Identify the intervals of P(t) that fall outside L.
//         c. Extract the sub-curve of P(t) that lies inside L using the
//            de Casteljau method.
//         d. Return the result as P₂(t) aka P(t₁..t₂)
//     2. Repeat (1), clipping Q(u) against P₂(t) to produce Q₂(u).
//     3. Repeat (1) again clipping P₂(t) against, Q₂(u) to produce P₃(t).
//     4. Repeat until t₁ ≈ t₂ and u₁ ≈ u₂, within some error margin.
//     5. Finish with t and u as the identified interpolation factor.

//     On Identifying Multiple Intersections:

//     1. Heuristic: If a Bezier clip fails to reduce the parameter range of either
//        curve by at least 20%, subdivide the curve with the largest remaining
//        interval in half and test each segment separately.
//        a. Apply recursively until all intersections (max 9) have been found.
//        b. Sort the resulting interpolation factors 0-1.
//     */
//     let lhs_result = ArrayVec::new();
//     let rhs_result = ArrayVec::new();

//     // create spans 0..1 interrupted by intersection points

//     (lhs_result, rhs_result)
// }

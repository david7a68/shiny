use super::{line::Line, point::Point, rect::Rect};
use crate::{
    math::{
        cmp::ApproxEq,
        interp::Interpolate,
        matrix::{Mat1x4, Mat4x2, Mat4x4},
        simd::Float4,
    },
    utils::{
        arrayvec::ArrayVec,
        cmp::{max, min},
    },
};

pub trait Bezier: Sized {
    type Owning;

    #[must_use]
    fn at(&self, t: f32) -> Point;

    #[must_use]
    fn coarse_bounds(&self) -> Rect;

    #[must_use]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning);

    #[must_use]
    fn split2(&self, t1: f32, t3: f32) -> (Self::Owning, Self::Owning, Self::Owning);

    #[must_use]
    fn intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9>;
}

/// A cubic bezier curve.
pub struct Cubic {
    pub points: [Point; 4],
}

impl Cubic {
    #[must_use]
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        Self {
            points: [p1, p2, p3, p4],
        }
    }

    #[must_use]
    pub fn borrow(&self) -> CubicSlice {
        CubicSlice::new(&self.points)
    }
}

impl Bezier for Cubic {
    type Owning = Self;

    #[inline]
    fn at(&self, t: f32) -> Point {
        evaluate(&self.points, t)
    }

    #[inline]
    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(&self.points)
    }

    #[inline]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        let (left, right) = split(&self.points, t);
        (
            Self::Owning { points: left },
            Self::Owning { points: right },
        )
    }

    #[inline]
    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        let (left, center, right) = split2(&self.points, t1, t2);
        (
            Self::Owning { points: left },
            Self::Owning { points: center },
            Self::Owning { points: right },
        )
    }

    #[inline]
    fn intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        find_intersections(&self.points, &other.points)
    }
}

/// A cubic bezier curve as a reference to a slice of 4 points. Useful for e.g.
/// composites of several curves, where the first and last point can be shared
/// with the curves before and after, respectively. This can significantly
/// reduce the number of points that need to be stored.
#[derive(Debug)]
pub struct CubicSlice<'a> {
    pub points: &'a [Point; 4],
}

impl<'a> CubicSlice<'a> {
    #[must_use]
    pub fn new(points: &'a [Point; 4]) -> Self {
        Self { points }
    }
}

impl<'a> Bezier for CubicSlice<'a> {
    type Owning = Cubic;

    #[inline]
    fn at(&self, t: f32) -> Point {
        evaluate(self.points, t)
    }

    #[inline]
    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(self.points)
    }

    #[inline]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        let (left, right) = split(self.points, t);
        (
            Self::Owning { points: left },
            Self::Owning { points: right },
        )
    }

    #[inline]
    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        let (left, center, right) = split2(self.points, t1, t2);
        (
            Self::Owning { points: left },
            Self::Owning { points: center },
            Self::Owning { points: right },
        )
    }

    #[inline]
    fn intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        find_intersections(self.points, other.points)
    }
}

fn evaluate(bezier: &[Point; 4], t: f32) -> Point {
    // let t_inv = 1.0 - t;
    // ((t_inv.powf(3.0) * self.p0.vec()) + (3.0 * t_inv.powf(2.0) * t * self.p1.vec()) + (3.0 * t_inv * t.powf(2.0) * self.p2.vec()) + (t.powf(3.0) * self.p3.vec())).into()
    let t = Mat1x4::new(1.0, t, t.powf(2.0), t.powf(3.0));
    #[rustfmt::skip]
    let m = Mat4x4::new(
        1.0, 0.0, 0.0, 0.0,
        -3.0, 3.0, 0.0, 0.0,
        3.0, -6.0, 3.0, 0.0,
        -1.0, 3.0, -3.0, 1.0,
    );

    #[rustfmt::skip]
    let p = Mat4x2::new(
        bezier[0].x, bezier[0].y,
        bezier[1].x, bezier[1].y,
        bezier[2].x, bezier[2].y,
        bezier[3].x, bezier[3].y,
    );

    let tmp = t * m * p;
    Point::new(tmp.x(), tmp.y())
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

    let a = Float4::new(bezier[0].x, bezier[0].y, bezier[1].x, bezier[1].y);
    let b = Float4::new(bezier[2].x, bezier[2].y, bezier[3].x, bezier[3].y);

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

/// Calculates the t-value for every intersection between the two curves `a` and
/// `b`.
#[must_use]
pub fn find_intersections(a: &[Point; 4], b: &[Point; 4]) -> ArrayVec<(f32, f32), 9> {
    find_intersections_in_range(CurvePart::new(a, 0.0, 1.0), CurvePart::new(b, 0.0, 1.0))
}

#[derive(Debug, Clone, Copy)]
struct CurvePart<'a> {
    points: &'a [Point; 4],
    start: f32,
    end: f32,
}

impl<'a> CurvePart<'a> {
    fn new(points: &'a [Point; 4], start: f32, end: f32) -> Self {
        Self { points, start, end }
    }

    fn length(&self) -> f32 {
        self.end - self.start
    }

    fn get(&self) -> [Point; 4] {
        split2(self.points, self.start as f32, self.end as f32).1
    }

    fn split(&self, at: f32) -> (Self, Self) {
        let at = self.start + at * (self.end - self.start);
        (
            Self::new(self.points, self.start, at),
            Self::new(self.points, at, self.end),
        )
    }

    fn map_to_parent(&self, t: f32) -> f32 {
        debug_assert!((0.0..=1.0).contains(&t));
        self.start + (t * (self.end - self.start))
    }

    fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.start)
            && (0.0..=1.0).contains(&self.end)
            && (self.start <= self.end)
    }
}

/// Finds the intersections between two curves within the specified ranges.
fn find_intersections_in_range(mut a: CurvePart, mut b: CurvePart) -> ArrayVec<(f32, f32), 9> {
    let mut intersections = ArrayVec::new();
    let mut num_iterations = 0;

    #[derive(Debug)]
    enum Result {
        Split,
        NoSplit,
        NoIntersection,
    }

    let calc = |curve: &mut CurvePart, against: &mut CurvePart| {
        let initial_length = curve.length();

        let (start, end) = clip(&curve.get(), &against.get());
        (curve.start, curve.end) = (curve.map_to_parent(start), curve.map_to_parent(end));

        if curve.end < curve.start {
            Result::NoIntersection
        } else if curve.length() > initial_length * 0.8 {
            Result::Split
        } else {
            Result::NoSplit
        }
    };

    loop {
        debug_assert!(a.is_valid());
        debug_assert!(b.is_valid());

        assert!(
            num_iterations < 15,
            "Hit max iterations, degenerate case? a={:?}, b={:?}",
            a,
            b
        );

        // Alternate between a and b
        let needs_split = if (num_iterations & 1) == 0 {
            calc(&mut a, &mut b)
        } else {
            calc(&mut b, &mut a)
        };

        if a.start.approx_eq(a.end) & b.start.approx_eq(b.end) {
            intersections.push((a.start as f32, b.start as f32));
            break;
        }

        match needs_split {
            Result::Split => {
                if a.length() > b.length() {
                    let (left, right) = a.split(0.5);
                    intersections.extend(&find_intersections_in_range(left, b));
                    intersections.extend(&find_intersections_in_range(right, b));
                } else {
                    let (left, right) = b.split(0.5);
                    intersections.extend(&find_intersections_in_range(a, left));
                    intersections.extend(&find_intersections_in_range(a, right));
                }
                break;
            }
            Result::NoSplit => {
                // no-op, continue
            }
            Result::NoIntersection => {
                break;
            },
        }

        num_iterations += 1;
    }

    intersections
}

/// Clips `a` against `b`, producing t-bounds where `a` lies within `b`'s fat
/// line.
fn clip(curve: &[Point; 4], against: &[Point; 4]) -> (f32, f32) {
    let (min_line, max_line) = {
        let (low, high) = fat_line(against);
        (-low, high)
    };

    let min_clip = clip_line(curve, &min_line);
    let max_clip = clip_line(curve, &max_line);
    (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
}

/// Calculates the two lines that bound the curve. This is currently done using
/// only the control points. A more refined method using inflection points may
/// or may not improve performance (extra work per curve for possibly fewer
/// clipping operations).
fn fat_line(curve: &[Point; 4]) -> (Line, Line) {
    let thin = Line::between(curve[0], curve[3]);
    let line1 = thin.parallel_through(curve[1]);
    let line2 = thin.parallel_through(curve[2]);
    let min_c = min!(thin.c, line1.c, line2.c);
    let max_c = max!(thin.c, line1.c, line2.c);
    (Line::with_c(thin, min_c), Line::with_c(thin, max_c))
}

/// Clips `curve` against `line`, returning a t-bound that is guaranteed to
/// be 'above' the line (distance is positive).
///
/// This algorithm does not attempt to calculate the precise point of
/// intersection, but only a close-enough approximation.
fn clip_line(curve: &[Point; 4], line: &Line) -> (f32, f32) {
    let e0 = Point::new(0.0, line.signed_distance_to(curve[0]));
    let e1 = Point::new(1.0 / 3.0, line.signed_distance_to(curve[1]));
    let e2 = Point::new(2.0 / 3.0, line.signed_distance_to(curve[2]));
    let e3 = Point::new(1.0, line.signed_distance_to(curve[3]));

    // Test the left of the curve (low-t)
    let low = if e0.y < 0.0 {
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
            min = x3;
        }
        min
    } else {
        0.0
    };

    // Test the right of the curve (high-t)
    let high = if e3.y < 0.0 {
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
            max = x3;
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
        let bezier = Cubic {
            points: [
                Point::new(10.0, 5.0),
                Point::new(3.0, 11.0),
                Point::new(12.0, 20.0),
                Point::new(6.0, 15.0),
            ],
        };

        assert_eq!(bezier.at(0.0), Point::new(10.0, 5.0));
        assert_eq!(bezier.at(0.5), Point::new(7.625, 14.125));
        assert_eq!(bezier.at(1.0), Point::new(6.0, 15.0));
    }

    #[test]
    fn coarse_bounds() {
        let bezier = Cubic {
            points: [
                Point::new(10.0, 5.0),
                Point::new(3.0, 11.0),
                Point::new(12.0, 20.0),
                Point::new(6.0, 15.0),
            ],
        };

        let bounds = bezier.coarse_bounds();

        assert_eq!(bounds, Rect::new(3.0, 12.0, 5.0, 20.0));
    }

    #[test]
    fn split() {
        let bezier = Cubic {
            points: [
                Point::new(10.0, 5.0),
                Point::new(3.0, 11.0),
                Point::new(12.0, 20.0),
                Point::new(6.0, 15.0),
            ],
        };

        let (left, right) = bezier.split(0.5);

        for t in 0..50 {
            let t = t as f32 / 50.0;
            let left = left.at(t);
            let original = bezier.at(t / 2.0);
            assert!(
                left.approx_eq(original),
                "left: {:?}, original: {:?}",
                left,
                original
            );
        }

        for t in 0..50 {
            let t = t as f32 / 50.0;
            let right = right.at(t);
            let original = bezier.at(0.5 + t / 2.0);
            assert!(
                right.approx_eq(original),
                "right: {:?}, original: {:?}",
                right,
                original
            );
        }
    }

    #[test]
    fn one_intersection() {
        let curve1 = [
            Point { x: 24.0, y: 21.0 },
            Point { x: 189.0, y: 40.0 },
            Point { x: 159.0, y: 137.0 },
            Point { x: 101.0, y: 261.0 },
        ];

        let curve2 = [
            Point { x: 18.0, y: 122.0 },
            Point { x: 15.0, y: 178.0 },
            Point { x: 247.0, y: 173.0 },
            Point { x: 251.0, y: 242.0 },
        ];

        let t = find_intersections(&curve1, &curve2);

        assert_eq!(t.len(), 1);

        assert!(t[0].0.approx_eq(0.76273954));
        assert!(t[0].1.approx_eq(0.50988877));
    }

    #[test]
    fn two_intersections() {
        let curve1 = [
            Point::new(204.0, 41.0),
            Point::new(45.0, 235.0),
            Point::new(220.0, 235.0),
            Point::new(226.0, 146.0),
        ];

        let curve2 = [
            Point::new(100.0, 98.0),
            Point::new(164.0, 45.0),
            Point::new(187.0, 98.0),
            Point::new(119.0, 247.0),
        ];

        let t = find_intersections(&curve1, &curve2);
        assert_eq!(t.len(), 2);

        for &(left, right) in &t {
            assert!(super::evaluate(&curve1, left)
                .approx_eq_within(super::evaluate(&curve2, right), 0.001));
        }
    }

    #[test]
    fn three_intersections() {
        let curve1 = [
            Point::new(18.0, 122.0),
            Point::new(15.0, 178.0),
            Point::new(247.0, 173.0),
            Point::new(251.0, 242.0),
        ];

        let curve2 = [
            Point::new(20.0, 213.0),
            Point::new(189.0, 40.0),
            Point::new(85.0, 283.0),
            Point::new(271.0, 217.0),
        ];

        let t = find_intersections(&curve1, &curve2);
        assert_eq!(t.len(), 3);

        for &(left, right) in &t {
            assert!(super::evaluate(&curve1, left)
                .approx_eq_within(super::evaluate(&curve2, right), 0.001));
        }
    }

    #[test]
    fn four_intersections() {
        let curve1 = [
            Point::new(236.0, 200.0),
            Point::new(52.0, 76.0),
            Point::new(157.0, 233.0),
            Point::new(264.0, 160.0)
        ];

        let curve2 = [
            Point::new(57.0, 172.0),
            Point::new(202.0, 255.0),
            Point::new(236.0, 0.0),
            Point::new(112.0, 229.0)
        ];

        let t = find_intersections(&curve1, &curve2);
        assert_eq!(t.len(), 4);

        for &(left, right) in &t {
            assert!(super::evaluate(&curve1, left)
                .approx_eq_within(super::evaluate(&curve2, right), 0.001));
        }
    }

    #[test]
    fn clip() {
        let curve1 = Cubic {
            points: [
                Point::new(24.0, 21.0),
                Point::new(189.0, 40.0),
                Point::new(159.0, 137.0),
                Point::new(101.0, 261.0),
            ],
        };
        let curve2 = Cubic {
            points: [
                Point::new(18.0, 122.0),
                Point::new(15.0, 178.0),
                Point::new(247.0, 173.0),
                Point::new(251.0, 242.0),
            ],
        };

        let curve1_limits = super::clip(&curve1.points, &curve2.points);
        assert_eq!(curve1_limits, (0.18543269, 0.91614604));
    }

    #[test]
    fn fat_line() {
        let curve = Cubic {
            points: [
                Point::new(18.0, 122.0),
                Point::new(15.0, 178.0),
                Point::new(247.0, 173.0),
                Point::new(251.0, 242.0),
            ],
        };

        let thin = Line::between(curve.points[0], curve.points[3]);
        let (low, high) = super::fat_line(&curve.points);

        assert!(low.c.approx_eq(40.70803));
        assert!(high.c.approx_eq(151.37787));

        assert!(thin.a.approx_eq(low.a));
        assert!(thin.b.approx_eq(low.b));

        assert!(low.signed_distance_to(curve.points[2]).approx_eq(0.0));
        assert!(high.signed_distance_to(curve.points[1]).approx_eq(0.0));
    }

    #[test]
    fn clip_line() {
        let line = Line::new(0.0, 0.0, 1.0);
        let curve = Cubic {
            points: [
                Point::new(24.0, 21.0),
                Point::new(189.0, 40.0),
                Point::new(159.0, 137.0),
                Point::new(101.0, 261.0),
            ],
        };

        let clip = super::clip_line(&curve.points, &line);
        assert_eq!(clip, (0.0, 1.0));
    }
}

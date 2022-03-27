use std::ops::RangeBounds;

use super::{float4::Float4, float4x4::Float4x4};

use utils::arrayvec::ArrayVec;
use utils::{max, min};

use super::{line::Line, point::Point, rect::Rect};

pub struct QuadraticBezier {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl QuadraticBezier {
    pub fn elevate(&mut self) -> CubicBezier {
        let part1 = self.p1.vec() * (1.0 / 3.0);
        let part2 = self.p2.vec() * (2.0 / 3.0);
        let part3 = self.p3.vec() * (1.0 / 3.0);

        CubicBezier {
            p0: self.p1,
            p1: (part1 + part2).into(),
            p2: (part2 + part3).into(),
            p3: self.p3,
        }
    }
}

pub struct CubicBezier {
    p0: Point,
    p1: Point,
    p2: Point,
    p3: Point,
}

impl CubicBezier {
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

        let px = Float4::new(self.p0.x(), self.p1.x(), self.p2.x(), self.p3.x());
        let py = Float4::new(self.p0.y(), self.p1.y(), self.p2.y(), self.p3.y());

        let tm = t * m;
        let tmx = tm.mul_elements(&px);
        let tmy = tm.mul_elements(&py);

        let (x, y) = Float4::hsum2(tmx, tmy);
        Point(x, y)
    }

    pub fn coarse_bounds(&self) -> Rect {
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

        let a = Float4::new(self.p0.x(), self.p0.y(), self.p1.x(), self.p1.y());
        let b = Float4::new(self.p2.x(), self.p2.y(), self.p3.x(), self.p3.y());

        let min1 = a.min(&b);
        let min2 = min1.zwxy();
        let min3 = min1.min(&min2);

        let max1 = a.max(&b);
        let max2 = max1.zwxy();
        let max3 = max1.max(&max2);

        Rect::new(min3.x(), max3.x(), min3.y(), max3.y())
    }

    pub fn intersects_with(&self, rhs: &CubicBezier) -> ArrayVec<f32, 9> {
        let (line1, line2) = line_to_2(self.p0, self.p3, rhs.p0, rhs.p3);

        // if the end points of self lie within the fat line of rhs or
        // if the end points of rhs lie within the fat line of self
        // the two lines contain multiple intersections, and one of the curves must be split at the midpoint.

        let d1 = line1.distance_to(self.p1);
        let d2 = line1.distance_to(self.p2);

        let half_width = if d1 * d2 > 0.0 { 3.0 / 4.0 } else { 4.0 / 9.0 };

        let d_min = d1.min(d2.min(0.0)) * half_width;
        let d_max = d1.max(d2.max(0.0)) * half_width;

        // let t_min = t where line1.distance_to(self.at(t)) < d_min, note that it may never be < d_min
        // let t_max = t where line1.distance_to(self.at(t)) > d_max, note that it may never be > d_max

        // isolate rhs from t_min..t_max
        // swap rhs and lhs, and repeat

        todo!()
    }

    fn get<R>(&self, range: R) -> CubicBezierSlice
    where
        R: RangeBounds<f32>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s,
            std::ops::Bound::Unbounded => 0.0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(e) => *e,
            std::ops::Bound::Excluded(e) => *e,
            std::ops::Bound::Unbounded => 1.0,
        };

        CubicBezierSlice {
            bezier: &self,
            start,
            end,
        }
    }
}

#[derive(Clone, Copy)]
struct CubicBezierSlice<'a> {
    bezier: &'a CubicBezier,
    start: f32,
    end: f32,
}

impl<'a> CubicBezierSlice<'a> {
    fn p0(&self) -> Point {
        self.bezier.p0
    }

    fn p1(&self) -> Point {
        self.bezier.p1
    }

    fn p2(&self) -> Point {
        self.bezier.p2
    }

    fn p3(&self) -> Point {
        self.bezier.p3
    }

    fn get<R>(&self, range: R) -> CubicBezierSlice<'a>
    where
        R: RangeBounds<f32>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s,
            std::ops::Bound::Unbounded => 0.0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(e) => *e,
            std::ops::Bound::Excluded(e) => *e,
            std::ops::Bound::Unbounded => 1.0,
        };

        let span = self.end - self.start;
        CubicBezierSlice {
            bezier: self.bezier,
            start: self.start + (span * start),
            end: self.end + (span * end),
        }
    }
}

/// Removes intersections between two curves, replacing each with
/// non-intersecting spans. The resulting arrays of curves are in-order from t=0
/// to t=1.
///
/// This process makes use of the bezier clipping algorithm to identify
/// curve-curve intersections.
fn flatten<'a, 'b>(
    lhs: &'a CubicBezier,
    rhs: &'b CubicBezier,
) -> (
    ArrayVec<CubicBezierSlice<'a>, 10>,
    ArrayVec<CubicBezierSlice<'b>, 10>,
) {
    let mut a = lhs.get(..);
    let mut b = rhs.get(..);

    let mut lhs_intersections = ArrayVec::<f32, 10>::new();
    let mut rhs_intersections = ArrayVec::<f32, 10>::new();
    let mut iterations = 0;
    loop {
        let a_ok = (a.end - a.start) < 0.0001;
        let b_ok = (b.end - b.start) < 0.0001;
        if a_ok & b_ok {
            if (iterations & 1) == 0 {
                lhs_intersections.push(a.end);
                rhs_intersections.push(b.end);
            } else {
                lhs_intersections.push(b.end);
                rhs_intersections.push(a.end);
            }
            break;
        }

        let (l, r) = clip(&a, &b);

        // if l.diff or r.diff shrank by less than 20%, split the longest one in
        // half and try again.

        a = r;
        b = l;
        iterations += 1;
    }

    lhs_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rhs_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

    /*
    The Bezier Clipping Method:

    1. To clip P(t) against Q(u)...
        a. Identify the fat line L that bounds Q(u).
            i. Optionally, identify the fat line K perpendicular to Q(u) and
               select the narrower of the two.
        b. Identify the intervals of P(t) that fall outside L.
        c. Extract the sub-curve of P(t) that lies inside L using the
           de Casteljau method.
        d. Return the result as P₂(t) aka P(t₁..t₂)
    2. Repeat (1), clipping Q(u) against P₂(t) to produce Q₂(u).
    3. Repeat (1) again clipping P₂(t) against, Q₂(u) to produce P₃(t).
    4. Repeat until t₁ ≈ t₂ and u₁ ≈ u₂, within some error margin.
    5. Finish with t and u as the identified interpolation factor.

    On Identifying Multiple Intersections:

    1. Heuristic: If a Bezier clip fails to reduce the parameter range of either
       curve by at least 20%, subdivide the curve with the largest remaining
       interval in half and test each segment separately.
       a. Apply recursively until all intersections (max 9) have been found.
       b. Sort the resulting interpolation factors 0-1.
    */

    let lhs_result = ArrayVec::new();
    let rhs_result = ArrayVec::new();

    // create spans 0..1 interrupted by intersection points

    (lhs_result, rhs_result)
}

fn clip<'a, 'b>(
    a: &CubicBezierSlice<'a>,
    b: &CubicBezierSlice<'b>,
) -> (CubicBezierSlice<'a>, CubicBezierSlice<'b>) {
    // 1. Identify the fat line that bounds a.
    let (l_min, l_max) = {
        let line = Line::between(a.p0(), a.p3());
        let l1 = line.parallel_through(a.p1());
        let l2 = line.parallel_through(a.p3());

        let min_c = min!(line.c(), l1.c(), l2.c());
        let max_c = max!(line.c(), l1.c(), l2.c());

        // Negate min to keep the curve in the positive half-space.
        (-Line::with_c(line, min_c), Line::with_c(line, max_c))
    };

    // Extract the curve that lies within the fat line
    let clipped_min = clip_against_line(l_min, &b);
    let clipped_max = clip_against_line(l_max, &b);

    // use the smallest possible fit
    let low = min!(clipped_min.0, clipped_max.0);
    let high = min!(clipped_min.1, clipped_max.1);

    (*a, b.get(low..high))
}

/// Clips a curve against a line, returning the span within which the curve is
/// above the line.
fn clip_against_line(line: Line, curve: &CubicBezierSlice) -> (f32, f32) {
    let e0 = Point(0.0, line.distance_to(curve.p0()));
    let e1 = Point(1.0, line.distance_to(curve.p1()));
    let e2 = Point(2.0, line.distance_to(curve.p2()));
    let e3 = Point(3.0, line.distance_to(curve.p3()));

    if {
        // if all 4 points are below 0, clip the entire thing.
        let v = Float4::new(e0.y(), e1.y(), e2.y(), e3.y());
        (v.lt_elements(&Float4::splat(0.0))) == (true, true, true, true)
    } {
        (0.0, 0.0)
    } else {
        // If e0 lies below the line, clip the left side of the curve.
        let low = if e0.y() < 0.0 {
            let l1 = Line::between(e0, e1);
            let l2 = Line::between(e0, e2);
            let l3 = Line::between(e0, e3);
            let x1 = l1.x_intercept();
            let x2 = l2.x_intercept();
            let x3 = l3.x_intercept();
            min!(x1, x2, x3)
        } else {
            0.0
        };

        // If e3 lies below the line, clip the right side of the curve.
        let high = if e3.y() < 0.0 {
            let l1 = Line::between(e3, e2);
            let l2 = Line::between(e3, e1);
            let l3 = Line::between(e3, e0);
            let x1 = l1.x_intercept();
            let x2 = l2.x_intercept();
            let x3 = l3.x_intercept();
            max!(x1, x2, x3)
        } else {
            1.0
        };

        (low, high)
    }
}

/// Calculates the lines a->b and c->d simultaneously.
fn line_to_2(p1: Point, p2: Point, p3: Point, p4: Point) -> (Line, Line) {
    #[cold]
    fn _straight_line(p1: Point, p2: Point, p3: Point, p4: Point) -> (Line, Line) {
        (Line::between(p1, p2), Line::between(p3, p4))
    }

    let a = Float4::new(p2.x(), p2.y(), p4.x(), p4.y());
    let b = Float4::new(p1.x(), p1.y(), p3.x(), p3.y());

    // if neight a->b or c->d are straight lines
    if a.eq_elements(&b) == (false, true, false, true) {
        let delta = { a - b };
        let slopes = delta.div_elements(&delta.yxwz());

        let (_, offset1, _, offset2) = {
            let f = Float4::new(0.0, p1.x(), 0.0, p3.x());
            let g = slopes.mul_elements(&f);
            let h = Float4::new(0.0, p1.y(), 0.0, p3.y());
            h - g
        }
        .unpack();

        let (_, scale1, _, scale2) = {
            let j = slopes.mul_elements(&slopes);
            let k = Float4::new(0.0, 1.0, 0.0, 1.0);
            (j + k).sqrt_elements().unpack()
        };

        let (_, slope1, _, slope2) = slopes.unpack();

        let line1 = Float4::new(slope1, 1.0, offset1, 0.0).div_elements(&Float4::splat(scale1));
        let line2 = Float4::new(slope2, 1.0, offset2, 0.0).div_elements(&Float4::splat(scale2));

        unsafe {
            (
                Line::from_normalized_vector(line1),
                Line::from_normalized_vector(line2),
            )
        }
    } else {
        _straight_line(p1, p2, p3, p4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        let (a, b) = line_to_2(
            Point(2.0, 2.0),
            Point(6.0, 4.0),
            Point(5.0, 1.0),
            Point(5.0, 12.0),
        );
        let (c, d) = (
            Line::between(Point(2.0, 2.0), Point(6.0, 4.0)),
            Line::between(Point(5.0, 1.0), Point(5.0, 12.0)),
        );
        assert!(a.approx_equal(&c));
        assert!(b.approx_equal(&d));
    }

    #[test]
    fn cubic_at() {
        let bezier = CubicBezier {
            p0: Point(10.0, 5.0),
            p1: Point(3.0, 11.0),
            p2: Point(12.0, 20.0),
            p3: Point(6.0, 15.0),
        };

        assert_eq!(bezier.at(0.0), Point(10.0, 5.0));
        assert_eq!(bezier.at(0.5), Point(7.625, 14.125));
        assert_eq!(bezier.at(1.0), Point(6.0, 15.0));
    }

    #[test]
    fn coarse_bounds() {
        let bezier = CubicBezier {
            p0: Point(10.0, 5.0),
            p1: Point(3.0, 11.0),
            p2: Point(12.0, 20.0),
            p3: Point(6.0, 15.0),
        };

        let bounds = bezier.coarse_bounds();

        assert_eq!(bounds, Rect::new(3.0, 12.0, 5.0, 20.0));
    }

    #[test]
    fn intersects_with() {
        {
            let b1 = CubicBezier {
                p0: Point(50.0, 35.0),
                p1: Point(45.0, 235.0),
                p2: Point(220.0, 235.0),
                p3: Point(220.0, 135.0),
            };

            let b2 = CubicBezier {
                p0: Point(113.0, 112.0),
                p1: Point(120.0, 20.0),
                p2: Point(220.0, 95.0),
                p3: Point(140.0, 240.0),
            };

            let v = b1.intersects_with(&b2);
            assert_eq!(v.len(), 1);
            // assert!((v[0] - ))
        }
    }
}

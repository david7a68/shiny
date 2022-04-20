use super::split2;
use crate::{
    math::cmp::ApproxEq,
    shapes::{line::Line, point::Point},
    utils::{
        arrayvec::ArrayVec,
        cmp::{max, min},
    },
};

/// Calculates the t-value for every intersection between the two curves `a` and
/// `b`.
#[must_use]
pub fn find(a: &[Point; 4], b: &[Point; 4]) -> ArrayVec<(f32, f32), 9> {
    let mut intersections = ArrayVec::new();
    find_intersections_in_range(
        CurvePart::new(a, 0.0, 1.0),
        CurvePart::new(b, 0.0, 1.0),
        &mut intersections,
    );
    intersections
}

/// Checks if the curve intersects with itself (forms a loop), and identifies
/// the t-values of the intersection if so.
#[must_use]
pub fn find_self(curve: &[Point; 4]) -> Option<(f32, f32)> {
    // Instructions from "Resolution Independent Curve Rendering using
    // Programmable Graphics Hardware by Charles Loop, Jim Blinn (2005)."

    // Algorithm:
    // 1. Classify the curve as a serpentine, cusp, or loop.
    //
    //   a. Convert the bezier control points into the power basis by the
    //   product `C = M_3*B` within a homogenous coordinate system.
    //
    //   b. Calculate the vector `d = [d0, d1, d2, d3]` according to the formula:
    //
    //            x3 y3 w3            x3 y3 w3
    //   d0 = det x2 y2 w2  d1 = -det x2 y2 w2
    //            x1 y1 w1            x0 y0 w0
    //
    //           x3 y3 w3             x2 y2 w2
    //  d2 = det x1 y1 w1   d3 = -det x1 y1 w1
    //           x0 y0 w0             x0 y0 w0
    //
    //   c. Calculate a discriminant according to the formula
    //   `4(p0*p2-p1^2)(p1*p3-p2^2)-(p1*p2-p0*p3)`.
    //
    //   d. If the discriminant is 0, the curve is a loop.
    //
    // 2. If the curve is a loop, calculate the real roots of the curve and
    //    return it.
    //
    // 3. Else return None.

    let a1 = {
        let x0 = curve[0].x;
        let y0 = curve[0].y;
        let x1 = curve[1].x;
        let y1 = curve[1].y;
        let x2 = curve[2].x;
        let y2 = curve[2].y;
        let x3 = curve[3].x;
        let y3 = curve[3].y;
    };

    let left = CurvePart::new(curve, 0.0, 0.5);
    let right = CurvePart::new(curve, 0.5, 1.0);

    let mut intersections = ArrayVec::new();
    find_intersections_in_range(left, right, &mut intersections);

    if intersections.is_empty() {
        None
    } else {
        debug_assert!(intersections.len() == 1);
        let (t1, t2) = intersections[0];
        Some((t1, t2))
    }
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

    fn map_to_original(&self, t: f32) -> f32 {
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
fn find_intersections_in_range(
    mut a: CurvePart,
    mut b: CurvePart,
    intersections: &mut ArrayVec<(f32, f32), 9>,
) {
    fn calc(curve: &mut CurvePart, against: &CurvePart) -> f32 {
        let initial_length = curve.length();

        let (start, end) = clip(&curve.get(), &against.get());
        (curve.start, curve.end) = (curve.map_to_original(start), curve.map_to_original(end));

        curve.length() / initial_length
    }

    let mut num_iterations = 0;
    loop {
        debug_assert!(a.is_valid());
        debug_assert!(b.is_valid());

        assert!(
            num_iterations < 15,
            "Hit max iterations, degenerate case? a={:?}, b={:?}",
            a,
            b
        );

        assert!(
            !intersections.is_full(),
            "Hit max intersections, degenerate case? a:{:?}, b:{:?}",
            a,
            b
        );

        // Alternate between a and b
        let proportion_remaining = if (num_iterations & 1) == 0 {
            calc(&mut a, &b)
        } else {
            calc(&mut b, &a)
        };

        if proportion_remaining < 0.0 {
            // There is no intersection in this region, so we can stop.
            break;
        } else if (a.length() + b.length()).approx_eq(&0.0) {
            // The combined curve errors are close enough to zero that we can
            // safely say we've found the intersection.

            // Ignore intersections that are too similar to the previously found
            // intersection. This often happens if the splitting the curve
            // produces two values that are both close enough to the actual
            // intersection to register. Ideally, there should be some way to
            // avoid doing this (maybe by looking ahead?), but this works for
            // now.
            if let Some((a_prev, _)) = intersections.last() {
                if !a_prev.approx_eq(&a.start) {
                    intersections.push((a.start, b.start));
                }
            } else {
                intersections.push((a.start, b.start));
            }
            break;
        } else if proportion_remaining > 0.8 {
            // The clip did not result in a significant reduction in the curve's
            // length, so split the longest curve in half and look for
            // intersections in each half.
            if a.length() > b.length() {
                let (left, right) = a.split(0.5);
                find_intersections_in_range(left, b, intersections);
                find_intersections_in_range(right, b, intersections);
            } else {
                let (left, right) = b.split(0.5);
                find_intersections_in_range(a, left, intersections);
                find_intersections_in_range(a, right, intersections);
            }
            break;
        }

        num_iterations += 1;
    }
}

/// Clips `a` against `b`, producing t-bounds where `a` lies within `b`'s fat
/// line.
fn clip(curve: &[Point; 4], against: &[Point; 4]) -> (f32, f32) {
    let parallel = {
        let (min_line, max_line) = {
            let (low, high) = fat_line_parallel(against);
            (-low, high)
        };

        let min_clip = clip_line(curve, &min_line);
        let max_clip = clip_line(curve, &max_line);
        (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
    };

    let perpendicular = {
        let (min_line, max_line) = {
            let (low, high) = fat_line_perpendicular(against);
            (-low, high)
        };

        let min_clip = clip_line(curve, &min_line);
        let max_clip = clip_line(curve, &max_line);
        (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
    };

    if (perpendicular.1 - perpendicular.0).abs() < (parallel.1 - parallel.0).abs() {
        perpendicular
    } else {
        parallel
    }
}

/// Calculates the two lines that bound the curve. This is currently done using
/// only the control points. A more refined method using inflection points may
/// or may not improve performance (extra work per curve for possibly fewer
/// clipping operations).
fn fat_line_parallel(curve: &[Point; 4]) -> (Line, Line) {
    let thin = Line::between(curve[0], curve[3]);
    let line1 = thin.parallel_through(curve[1]);
    let line2 = thin.parallel_through(curve[2]);

    let min_c = min!(thin.c, line1.c, line2.c);
    let max_c = max!(thin.c, line1.c, line2.c);
    (Line::with_c(thin, min_c), Line::with_c(thin, max_c))
}

fn fat_line_perpendicular(curve: &[Point; 4]) -> (Line, Line) {
    let thin = Line::between(curve[0], curve[3]);
    let line0 = thin.perpendicular_through(curve[0]);
    let line1 = thin.perpendicular_through(curve[1]);
    let line2 = thin.perpendicular_through(curve[2]);
    let line3 = thin.perpendicular_through(curve[3]);

    (
        Line::with_c(line0, min!(line0.c, line1.c, line2.c, line3.c)),
        Line::with_c(line0, max!(line0.c, line1.c, line2.c, line3.c)),
    )
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
        if x1 > 0.0 {
            min = min!(x1, min);
        }
        if x2 > 0.0 {
            min = min!(x2, min);
        }
        if x3 > 0.0 {
            min = min!(x3, min);
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
        if x1 < 1.0 {
            max = max!(x1, max);
        }
        if x2 < 1.0 {
            max = max!(x2, max);
        }
        if x3 < 1.0 {
            max = max!(x3, max);
        }
        max
    } else {
        1.0
    };

    (low, high)
}

#[cfg(test)]
mod test {
    use super::super::evaluate;
    use super::*;

    #[test]
    fn find_intersections() {
        struct Pair {
            curve1: [Point; 4],
            curve2: [Point; 4],
            num_intersections: usize,
        }

        let pairs = [
            Pair {
                curve1: [
                    Point { x: 24.0, y: 21.0 },
                    Point { x: 189.0, y: 40.0 },
                    Point { x: 159.0, y: 137.0 },
                    Point { x: 101.0, y: 261.0 },
                ],
                curve2: [
                    Point { x: 18.0, y: 122.0 },
                    Point { x: 15.0, y: 178.0 },
                    Point { x: 247.0, y: 173.0 },
                    Point { x: 251.0, y: 242.0 },
                ],
                num_intersections: 1,
            },
            Pair {
                curve1: [
                    Point::new(204.0, 41.0),
                    Point::new(45.0, 235.0),
                    Point::new(220.0, 235.0),
                    Point::new(226.0, 146.0),
                ],
                curve2: [
                    Point::new(100.0, 98.0),
                    Point::new(164.0, 45.0),
                    Point::new(187.0, 98.0),
                    Point::new(119.0, 247.0),
                ],
                num_intersections: 2,
            },
            Pair {
                curve1: [
                    Point::new(50.0, 35.0),
                    Point::new(45.0, 235.0),
                    Point::new(220.0, 235.0),
                    Point::new(220.0, 135.0),
                ],
                curve2: [
                    Point::new(110.0, 209.0),
                    Point::new(17.0, 56.0),
                    Point::new(56.0, 55.0),
                    Point::new(93.0, 158.0),
                ],
                num_intersections: 3,
            },
            Pair {
                curve1: [
                    Point::new(236.0, 200.0),
                    Point::new(52.0, 76.0),
                    Point::new(157.0, 233.0),
                    Point::new(264.0, 160.0),
                ],
                curve2: [
                    Point::new(57.0, 172.0),
                    Point::new(202.0, 255.0),
                    Point::new(236.0, 0.0),
                    Point::new(112.0, 229.0),
                ],
                num_intersections: 4,
            },
            Pair {
                curve1: [
                    Point::new(108.0, 219.0),
                    Point::new(143.0, 16.0),
                    Point::new(121.0, 255.0),
                    Point::new(143.0, 136.0),
                ],
                curve2: [
                    Point::new(62.0, 156.0),
                    Point::new(267.0, 192.0),
                    Point::new(14.0, 125.0),
                    Point::new(156.0, 153.0),
                ],
                num_intersections: 9,
            },
        ];

        for pair in pairs.iter() {
            let intersections = find(&pair.curve1, &pair.curve2);
            assert_eq!(intersections.len(), pair.num_intersections);

            for (a, b) in intersections.iter() {
                let point1 = evaluate(&pair.curve1, *a);
                let point2 = evaluate(&pair.curve2, *b);
                assert!(point1.approx_eq_within(&point2, 0.001));
            }
        }
    }

    #[test]
    fn clip() {
        let curve1 = [
            Point::new(24.0, 21.0),
            Point::new(189.0, 40.0),
            Point::new(159.0, 137.0),
            Point::new(101.0, 261.0),
        ];
        let curve2 = [
            Point::new(18.0, 122.0),
            Point::new(15.0, 178.0),
            Point::new(247.0, 173.0),
            Point::new(251.0, 242.0),
        ];

        let curve1_limits = super::clip(&curve1, &curve2);
        assert_eq!(curve1_limits, (0.18543269, 0.91614604));
    }

    #[test]
    fn fat_line() {
        let curve = [
            Point::new(18.0, 122.0),
            Point::new(15.0, 178.0),
            Point::new(247.0, 173.0),
            Point::new(251.0, 242.0),
        ];

        let thin = Line::between(curve[0], curve[3]);
        let (low, high) = super::fat_line_parallel(&curve);

        assert!(low.c.approx_eq(&40.70803));
        assert!(high.c.approx_eq(&151.37787));

        assert!(thin.a.approx_eq(&low.a));
        assert!(thin.b.approx_eq(&low.b));

        assert!(low.signed_distance_to(curve[2]).approx_eq(&0.0));
        assert!(high.signed_distance_to(curve[1]).approx_eq(&0.0));
    }

    #[test]
    fn clip_line() {
        let line = Line::new(0.0, 0.0, 1.0);
        let curve = [
            Point::new(24.0, 21.0),
            Point::new(189.0, 40.0),
            Point::new(159.0, 137.0),
            Point::new(101.0, 261.0),
        ];

        let clip = super::clip_line(&curve, &line);
        assert_eq!(clip, (0.0, 1.0));
    }
}

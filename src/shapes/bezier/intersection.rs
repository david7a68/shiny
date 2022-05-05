use super::{split2, Cubic, CubicSlice};
use crate::{
    math::{
        cmp::{max, min, ApproxEq},
        simd::Float4,
    },
    shapes::{bezier::Bezier, line::Line},
    utils::arrayvec::ArrayVec,
};

/// Calculates the t-value for every intersection between the two curves `a` and
/// `b`.
#[must_use]
pub fn find(a: CubicSlice, b: CubicSlice) -> ArrayVec<(f32, f32), 9> {
    let mut intersections = ArrayVec::new();
    find_intersections_in_range(
        CurvePart::new(a, 0.0, 1.0),
        CurvePart::new(b, 0.0, 1.0),
        &mut intersections,
    );
    intersections
}

#[derive(Debug, Clone, Copy)]
struct CurvePart<'a> {
    curve: CubicSlice<'a>,
    start: f32,
    end: f32,
}

impl<'a> CurvePart<'a> {
    fn new(curve: CubicSlice<'a>, start: f32, end: f32) -> Self {
        Self { curve, start, end }
    }

    fn length(&self) -> f32 {
        self.end - self.start
    }

    fn get(&self) -> Cubic {
        // split2(self.points, self.start as f32, self.end as f32).1
        // todo!()
        split2(self.curve, self.start, self.end).1
    }

    fn split(&self, at: f32) -> (Self, Self) {
        let at = self.start + at * (self.end - self.start);
        (
            Self::new(self.curve, self.start, at),
            Self::new(self.curve, at, self.end),
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

        let (start, end) = clip(curve.get().borrow(), against.get().borrow());
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
fn clip(curve: CubicSlice, against: CubicSlice) -> (f32, f32) {
    fn x_intercepts(
        p1_x: Float4,
        p1_y: Float4,
        distance_from_line: Float4,
    ) -> (f32, f32, f32, f32) {
        let (a, c) = {
            let m = {
                let p2_x = Float4::new(0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0);
                let p2_y = distance_from_line;
                let dx = p2_x - p1_x;
                let dy = p2_y - p1_y;
                dy / dx
            };
            let o = -(m * p1_x) + p1_y;
            (m, o)
        };
        let (a, c) = {
            let div = (a * a + Float4::splat(-1.0 * -1.0)).sqrt();
            (a / div, c / div)
        };
        (-c / a).unpack()
    }

    // Computes the approximate region of the curve that lies above `line`.
    fn clip_line(x: Float4, y: Float4, line: &Line) -> (f32, f32) {
        // let e0 = Point::new(0.0, line.signed_distance_to(curve[0]));
        // let e1 = Point::new(1.0 / 3.0, line.signed_distance_to(curve[1]));
        // let e2 = Point::new(2.0 / 3.0, line.signed_distance_to(curve[2]));
        // let e3 = Point::new(1.0, line.signed_distance_to(curve[3]));
        let distance_from_line = line.a * x + line.b * y + Float4::splat(line.c);
        let (e0_low, _, _, e3_low) = distance_from_line.less_than(Float4::splat(0.0));

        // Test the left of the curve (low-t)
        let low = if e0_low {
            // let x1 = Line::between(e0, e1).x_intercept();
            // let x2 = Line::between(e0, e2).x_intercept();
            // let x3 = Line::between(e0, e3).x_intercept();
            let (_, x1, x2, x3) = x_intercepts(
                Float4::splat(0.0),
                Float4::splat(distance_from_line.a()),
                distance_from_line,
            );

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
        let high = if e3_low {
            let (x1, x2, x3, _) = x_intercepts(
                Float4::splat(1.0),
                Float4::splat(distance_from_line.d()),
                distance_from_line,
            );

            // let x1 = Line::between(e3, e0).x_intercept();
            // let x2 = Line::between(e3, e1).x_intercept();
            // let x3 = Line::between(e3, e2).x_intercept();

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

    let thin = Line::between(against.p0(), against.p3());

    let parallel = {
        let (min_line, max_line) = {
            let line1 = thin.parallel_through(against.p1());
            let line2 = thin.parallel_through(against.p2());

            (
                -Line::with_c(thin, min!(thin.c, line1.c, line2.c)),
                Line::with_c(thin, max!(thin.c, line1.c, line2.c)),
            )
        };

        let min_clip = clip_line(curve.x.into(), curve.y.into(), &min_line);
        let max_clip = clip_line(curve.x.into(), curve.y.into(), &max_line);
        (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
    };

    let perpendicular = {
        let (min_line, max_line) = {
            // Computes the line perpendicular to the thin line for each of the
            // 4 control points simultaneously. This is equivalent to calling
            // `thin.perpendicular_through(against.p0())`, etc.
            let slope = 1.0 / (thin.a / thin.b);
            let offset = -(Float4::splat(slope) * against.x.into()) + against.y.into();
            let (c0, c1, c2, c3) = offset.unpack();

            (
                -Line::new(slope, -1.0, min!(c0, c1, c2, c3)),
                Line::new(slope, -1.0, max!(c0, c1, c2, c3)),
            )
        };

        let min_clip = clip_line(curve.x.into(), curve.y.into(), &min_line);
        let max_clip = clip_line(curve.x.into(), curve.y.into(), &max_line);
        (max!(min_clip.0, max_clip.0), min!(min_clip.1, max_clip.1))
    };

    if (perpendicular.1 - perpendicular.0).abs() < (parallel.1 - parallel.0).abs() {
        perpendicular
    } else {
        parallel
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_intersections() {
        #[derive(Debug)]
        struct Pair {
            curve1: Cubic,
            curve2: Cubic,
            num_intersections: usize,
        }

        let pairs = [
            Pair {
                curve1: Cubic {
                    x: [24.0, 189.0, 159.0, 101.0],
                    y: [21.0, 40.0, 137.0, 261.0],
                },
                curve2: Cubic {
                    x: [18.0, 15.0, 247.0, 251.0],
                    y: [122.0, 178.0, 173.0, 242.0],
                },
                num_intersections: 1,
            },
            Pair {
                curve1: Cubic {
                    x: [204.0, 45.0, 220.0, 226.0],
                    y: [41.0, 235.0, 235.0, 146.0],
                },
                curve2: Cubic {
                    x: [100.0, 164.0, 187.0, 119.0],
                    y: [98.0, 45.0, 98.0, 247.0],
                },
                num_intersections: 2,
            },
            Pair {
                curve1: Cubic {
                    x: [50.0, 45.0, 220.0, 220.0],
                    y: [35.0, 235.0, 235.0, 135.0],
                },
                curve2: Cubic {
                    x: [110.0, 17.0, 56.0, 93.0],
                    y: [209.0, 56.0, 55.0, 158.0],
                },
                num_intersections: 3,
            },
            Pair {
                curve1: Cubic {
                    x: [236.0, 52.0, 157.0, 264.0],
                    y: [200.0, 76.0, 233.0, 160.0],
                },
                curve2: Cubic {
                    x: [57.0, 202.0, 236.0, 112.0],
                    y: [172.0, 255.0, 0.0, 229.0],
                },
                num_intersections: 4,
            },
            Pair {
                curve1: Cubic {
                    x: [108.0, 143.0, 121.0, 143.0],
                    y: [219.0, 16.0, 255.0, 136.0],
                },
                curve2: Cubic {
                    x: [62.0, 267.0, 14.0, 156.0],
                    y: [156.0, 192.0, 125.0, 153.0],
                },
                num_intersections: 9,
            },
        ];

        for pair in pairs.iter() {
            let intersections = find(pair.curve1.borrow(), pair.curve2.borrow());
            assert_eq!(intersections.len(), pair.num_intersections);

            for (a, b) in intersections.iter() {
                let point1 = pair.curve1.at(*a);
                let point2 = pair.curve2.at(*b);
                assert!(point1.approx_eq_within(&point2, 0.001));
            }
        }
    }

    #[test]
    fn clip() {
        let curve1 = Cubic {
            x: [24.0, 189.0, 159.0, 101.0],
            y: [21.0, 40.0, 137.0, 261.0],
        };
        let curve2 = Cubic {
            x: [18.0, 15.0, 247.0, 251.0],
            y: [122.0, 178.0, 173.0, 242.0],
        };

        let curve1_limits = super::clip(curve1.borrow(), curve2.borrow());
        assert_eq!(curve1_limits, (0.18543269, 0.91614604));
    }
}

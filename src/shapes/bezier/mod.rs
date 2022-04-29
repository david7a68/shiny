use super::{point::Point, rect::Rect};
use crate::{
    math::{
        matrix4::{Mat1x4, Mat4x2, Mat4x4},
        ops::Interpolate,
        simd::Float4,
    },
    utils::arrayvec::ArrayVec,
};

mod intersection;

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
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9>;

    #[must_use]
    fn find_self_intersection(&self) -> Option<(f32, f32)>;
}

/// A cubic bezier curve.
#[derive(Clone, Copy, Debug, Default)]
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
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        intersection::find(&self.points, &other.points)
    }

    #[inline]
    fn find_self_intersection(&self) -> Option<(f32, f32)> {
        intersection::find_self(&self.points)
    }
}

/// A cubic bezier curve as a reference to a slice of 4 points. Useful for e.g.
/// composites of several curves, where the first and last point can be shared
/// with the curves before and after, respectively. This can significantly
/// reduce the number of points that need to be stored.
#[derive(Clone, Copy, Debug)]
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
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        intersection::find(self.points, other.points)
    }

    #[inline]
    fn find_self_intersection(&self) -> Option<(f32, f32)> {
        intersection::find_self(self.points)
    }
}

fn evaluate(bezier: &[Point; 4], t: f32) -> Point {
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
    let a = Float4::new(bezier[0].x, bezier[0].y, bezier[1].x, bezier[1].y);
    let b = Float4::new(bezier[2].x, bezier[2].y, bezier[3].x, bezier[3].y);

    let min1 = a.min(b);
    let min2 = min1.swap_high_low();
    let min3 = min1.min(min2);

    let max1 = a.max(b);
    let max2 = max1.swap_high_low();
    let max3 = max1.max(max2);

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

#[cfg(test)]
mod tests {
    use crate::math::cmp::ApproxEq;

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

        assert!(bezier.at(0.0).approx_eq(&Point::new(10.0, 5.0)));
        assert!(bezier.at(0.5).approx_eq(&Point::new(7.625, 14.125)));
        assert!(bezier.at(1.0).approx_eq(&Point::new(6.0, 15.0)));
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
                left.approx_eq(&original),
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
                right.approx_eq(&original),
                "right: {:?}, original: {:?}",
                right,
                original
            );
        }
    }
}

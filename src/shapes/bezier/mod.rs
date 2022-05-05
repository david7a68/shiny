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
    fn p0(&self) -> Point;

    #[must_use]
    fn p1(&self) -> Point;

    #[must_use]
    fn p2(&self) -> Point;

    #[must_use]
    fn p3(&self) -> Point;

    #[must_use]
    fn coarse_bounds(&self) -> Rect;

    #[must_use]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning);

    #[must_use]
    fn split2(&self, t1: f32, t3: f32) -> (Self::Owning, Self::Owning, Self::Owning);

    #[must_use]
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9>;
}

/// A cubic bezier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic {
    pub x: [f32; 4],
    pub y: [f32; 4],
}

impl Cubic {
    #[must_use]
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        Self {
            x: [p1.x, p2.x, p3.x, p4.x],
            y: [p1.y, p2.y, p3.y, p4.y],
        }
    }

    #[must_use]
    pub fn borrow(&self) -> CubicSlice {
        CubicSlice::new(&self.x, &self.y)
    }
}

impl Bezier for Cubic {
    type Owning = Self;

    #[inline]
    fn at(&self, t: f32) -> Point {
        evaluate(self.borrow(), t)
    }

    #[inline]
    fn p0(&self) -> Point {
        Point::new(self.x[0], self.y[0])
    }

    #[inline]
    fn p1(&self) -> Point {
        Point::new(self.x[1], self.y[1])
    }

    #[inline]
    fn p2(&self) -> Point {
        Point::new(self.x[2], self.y[2])
    }

    #[inline]
    fn p3(&self) -> Point {
        Point::new(self.x[3], self.y[3])
    }

    #[inline]
    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(self.borrow())
    }

    #[inline]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        split(self.borrow(), t)
    }

    #[inline]
    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        split2(self.borrow(), t1, t2)
    }

    #[inline]
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        intersection::find(self.borrow(), other.borrow())
    }
}

/// A cubic bezier curve as a reference to a slice of 4 points. Useful for e.g.
/// composites of several curves, where the first and last point can be shared
/// with the curves before and after, respectively. This can significantly
/// reduce the number of points that need to be stored.
#[derive(Clone, Copy, Debug)]
pub struct CubicSlice<'a> {
    pub x: &'a [f32; 4],
    pub y: &'a [f32; 4],
}

impl<'a> CubicSlice<'a> {
    #[must_use]
    pub fn new(x: &'a [f32; 4], y: &'a [f32; 4]) -> Self {
        Self { x, y }
    }
}

impl<'a> Bezier for CubicSlice<'a> {
    type Owning = Cubic;

    #[inline]
    fn at(&self, t: f32) -> Point {
        evaluate(*self, t)
    }

    #[inline]
    fn p0(&self) -> Point {
        Point::new(self.x[0], self.y[0])
    }

    #[inline]
    fn p1(&self) -> Point {
        Point::new(self.x[1], self.y[1])
    }

    #[inline]
    fn p2(&self) -> Point {
        Point::new(self.x[2], self.y[2])
    }

    #[inline]
    fn p3(&self) -> Point {
        Point::new(self.x[3], self.y[3])
    }

    #[inline]
    fn coarse_bounds(&self) -> Rect {
        coarse_bounds(*self)
    }

    #[inline]
    fn split(&self, t: f32) -> (Self::Owning, Self::Owning) {
        split(*self, t)
    }

    #[inline]
    fn split2(&self, t1: f32, t2: f32) -> (Self::Owning, Self::Owning, Self::Owning) {
        split2(*self, t1, t2)
    }

    #[inline]
    fn find_intersections(&self, other: &Self) -> ArrayVec<(f32, f32), 9> {
        intersection::find(*self, *other)
    }
}

fn evaluate(curve: CubicSlice, t: f32) -> Point {
    let t = Mat1x4::new(1.0, t, t.powf(2.0), t.powf(3.0));
    #[rustfmt::skip]
    let m = Mat4x4::new(
        1.0, 0.0, 0.0, 0.0,
        -3.0, 3.0, 0.0, 0.0,
        3.0, -6.0, 3.0, 0.0,
        -1.0, 3.0, -3.0, 1.0,
    );

    let p = Mat4x2::from_columns(Float4::from_array(curve.x), Float4::from_array(curve.y));

    let tmp = t * m * p;
    Point::new(tmp.x(), tmp.y())
}

fn coarse_bounds(curve: CubicSlice) -> Rect {
    let (min, max) = Float4::horizontal_min_max4(
        curve.x.into(),
        curve.y.into(),
        Float4::splat(0.0),
        Float4::splat(0.0),
    );
    Rect::new(min.a(), max.a(), min.b(), max.b())
}

fn split(curve: CubicSlice, t: f32) -> (Cubic, Cubic) {
    let mid_01_and_12 = {
        let a = Float4::new(curve.x[0], curve.y[0], curve.x[1], curve.y[1]);
        let b = Float4::new(curve.x[1], curve.y[1], curve.x[2], curve.y[2]);
        a.lerp(t, &b)
    };
    let mid_23_and_zero = {
        let a = Float4::new(curve.x[2], curve.y[2], 0.0, 0.0);
        let b = Float4::new(curve.x[3], curve.y[3], 0.0, 0.0);
        a.lerp(t, &b)
    };
    // let mid_01 = bezier[0].lerp(t, &bezier[1]);
    // let mid_12 = bezier[1].lerp(t, &bezier[2]);
    // let mid_23 = bezier[2].lerp(t, &bezier[3]);

    let mid_12_23 = mid_01_and_12.combine_high_low(mid_23_and_zero);
    let mid_01_12_and_12_23 = mid_01_and_12.lerp(t, &mid_12_23);
    // let mid_01_12 = mid_01.lerp(t, &mid_12);
    // let mid_12_23 = mid_12.lerp(t, &mid_23);

    let midpoint_low = mid_01_12_and_12_23.lerp(t, &mid_01_12_and_12_23.swap_high_low());
    // let midpoint = mid_01_12.lerp(t, &mid_12_23);

    let (mid_01_x, mid_01_y, ..) = mid_01_and_12.unpack();
    let (midpoint_x, midpoint_y, ..) = midpoint_low.unpack();
    let (mid_01_12_x, mid_01_12_y, mid_12_23_x, mid_12_23_y) = mid_01_12_and_12_23.unpack();
    let (mid_23_x, mid_23_y, ..) = mid_23_and_zero.unpack();

    // let a = [bezier[0], mid_01, mid_01_12, midpoint];
    // let b = [midpoint, mid_12_23, mid_23, bezier[3]];

    // (a, b)
    (
        Cubic {
            x: [curve.x[0], mid_01_x, mid_01_12_x, midpoint_x],
            y: [curve.y[0], mid_01_y, mid_01_12_y, midpoint_y],
        },
        Cubic {
            x: [midpoint_x, mid_12_23_x, mid_23_x, curve.x[3]],
            y: [midpoint_y, mid_12_23_y, mid_23_y, curve.y[3]],
        },
    )
}

fn split2(
    curve: CubicSlice,
    t1: f32,
    t2: f32,
) -> (Cubic, Cubic, Cubic) {
    let (left, rest) = split(curve, t1);
    let (mid, right) = split(rest.borrow(), (t2 - t1) / (1.0 - t1));
    (left, mid, right)
}

#[cfg(test)]
mod tests {
    use crate::math::cmp::ApproxEq;

    use super::*;

    #[test]
    fn evaluate() {
        let bezier = Cubic {
            x: [10.0, 3.0, 12.0, 6.0],
            y: [5.0, 11.0, 20.0, 15.0],
        };

        assert!(bezier.at(0.0).approx_eq(&Point::new(10.0, 5.0)));
        assert!(bezier.at(0.5).approx_eq(&Point::new(7.625, 14.125)));
        assert!(bezier.at(1.0).approx_eq(&Point::new(6.0, 15.0)));
    }

    #[test]
    fn coarse_bounds() {
        let bezier = Cubic {
            x: [10.0, 3.0, 12.0, 6.0],
            y: [5.0, 11.0, 20.0, 15.0],
        };

        let bounds = bezier.coarse_bounds();
        assert_eq!(bounds, Rect::new(3.0, 12.0, 5.0, 20.0));
    }

    #[test]
    fn split() {
        let bezier = Cubic {
            x: [10.0, 3.0, 12.0, 6.0],
            y: [5.0, 11.0, 20.0, 15.0],
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

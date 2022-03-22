use crate::{
    arrayvec::ArrayVec,
    math::{Float4, Float4x4},
};

use super::{line::Line, Point, Rect};

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

        todo!()
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
    if (a.eq_elements(&b) & 0b0101) == 0 {
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

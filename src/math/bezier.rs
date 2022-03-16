use crate::math::{Float4, Float4x4};

use super::{Point, Rect};

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
            Float4::new(-1.0, 3.0, -3.0, 1.0)
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

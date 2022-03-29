use math::{float4::Float4, float4x4::Float4x4, point::Point};

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

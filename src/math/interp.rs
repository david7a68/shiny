use super::point::Point;

pub trait Interpolate {
    fn lerp(&self, t: f32, rhs: &Self) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        (1.0 - t) * self + t * rhs
    }
}

impl Interpolate for Point {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        Self(self.0.lerp(t, &rhs.0), self.1.lerp(t, &rhs.1))
    }
}

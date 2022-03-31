pub trait Interpolate {
    fn lerp(&self, t: f32, rhs: &Self) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        (1.0 - t) * self + t * rhs
    }
}

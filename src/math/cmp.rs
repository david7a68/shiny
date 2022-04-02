pub const APPROX_EQUAL_THRESHOLD: f32 = 1e-6;

pub trait ApproxEq<Rhs = Self> {

    #[must_use]
    fn approx_eq(&self, other: Rhs) -> bool;
}

impl ApproxEq<f32> for f32 {
    fn approx_eq(&self, other: f32) -> bool {
        (other - *self).abs() < APPROX_EQUAL_THRESHOLD
    }
}

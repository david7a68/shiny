pub const F32_APPROX_EQUAL_THRESHOLD: f32 = 1e-5;

pub trait ApproxEq<Rhs = Self> {
    #[must_use]
    fn approx_eq(&self, other: Rhs) -> bool;

    #[must_use]
    fn approx_eq_within(&self, other: Rhs, epsilon: f32) -> bool;
}

impl ApproxEq<f32> for f32 {
    fn approx_eq(&self, other: f32) -> bool {
        (other - *self).abs() <= F32_APPROX_EQUAL_THRESHOLD
    }

    fn approx_eq_within(&self, other: f32, epsilon: f32) -> bool {
        (other - *self).abs() <= epsilon
    }
}

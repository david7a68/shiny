pub const F32_APPROX_EQUAL_THRESHOLD: f32 = 1e-5;
pub const F64_APPROX_EQUAL_THRESHOLD: f64 = 1e-5;

pub trait ApproxEq<Rhs = Self> {
    #[must_use]
    fn approx_eq(&self, other: Rhs) -> bool;
}

impl ApproxEq<f32> for f32 {
    fn approx_eq(&self, other: f32) -> bool {
        (other - *self).abs() <= F32_APPROX_EQUAL_THRESHOLD
    }
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(&self, other: f64) -> bool {
        (other - *self).abs() <= F64_APPROX_EQUAL_THRESHOLD
    }
}

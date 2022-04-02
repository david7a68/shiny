/// Trait for implementing interpolation beween two values according to a scale
/// factor.
pub trait Interpolate {
    /// Implements linear interpolation.
    ///
    /// ```rust
    /// let a = 1.0;
    /// let b = 2.0;
    /// assert_eq(a.lerp(b, 0.5), 1.5);
    /// ```
    #[must_use]
    fn lerp(&self, t: f32, rhs: &Self) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        (1.0 - t) * self + t * rhs
    }
}

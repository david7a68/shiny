/// Trait for implementing interpolation beween two values according to a scale
/// factor.
pub trait Interpolate {
    /// Implements linear interpolation.
    ///
    /// Formula: `(1.0 - t) * self + (t * self)`
    ///
    /// ```rust
    /// # use shiny::math::ops::Interpolate;
    /// let a = 1.0;
    /// let b = 2.0;
    /// assert_eq!(a.lerp(0.5, &b), 1.5);
    /// ```
    #[must_use]
    fn lerp(&self, t: f32, rhs: &Self) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        (1.0 - t) * self + t * rhs
    }
}

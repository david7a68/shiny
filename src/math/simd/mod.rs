use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "x86.rs"]
mod arch;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Float4(arch::Float4);

impl Float4 {
    /// Creates a new 4-float vector.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self(arch::pack(a, b, c, d))
    }

    /// Repeats the value `v` in every element of the vector.
    pub fn splat(v: f32) -> Self {
        Self(arch::splat(v))
    }

    /// Computes the horizontal sum of 2 4-float vectors at the same time.
    pub fn horizontal_sum2(a: Float4, b: Float4) -> (f32, f32) {
        arch::horizontal_sum2(a.0, b.0)
    }

    pub fn a(&self) -> f32 {
        self.unpack().0
    }

    pub fn b(&self) -> f32 {
        self.unpack().1
    }

    pub fn c(&self) -> f32 {
        self.unpack().2
    }

    pub fn d(&self) -> f32 {
        self.unpack().3
    }

    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        arch::unpack(self.0)
    }

    /// Computes the absolute value of each element in the vector.This is
    /// semantically equivalent to performing each operation separately, but may
    /// make use of SIMD instructions to improve performance.
    pub fn abs(&self) -> Self {
        Self(arch::abs(self.0))
    }

    /// Computes the square root of each element in the vector.This is
    /// semantically equivalent to performing each operation separately, but may
    /// make use of SIMD instructions to improve performance.
    pub fn sqrt(&self) -> Self {
        Self(arch::sqrt(self.0))
    }

    /// Returns the elements of the vector in reverse order.
    ///
    /// ```rust
    /// let v = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(v.reverse().unpack(), (4.0, 3.0, 2.0, 1.0));
    /// ```
    pub fn reverse(&self) -> Self {
        Self(arch::swizzle_reverse(self.0))
    }

    /// Returns the elements of the vector such that:
    ///
    /// ```rust
    /// let v = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(v.cdab().unpack(), (3.0, 4.0, 1.0, 2.0));
    /// ```
    pub fn cdab(&self) -> Self {
        Self(arch::swizzle_cdab(self.0))
    }

    /// Computes the smaller value for each pair of elements in the two vectors.
    /// This is semantically equivalent to performing each operation separately,
    /// but may make use of SIMD instructions to improve performance.
    ///
    /// ```rust
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.min(b), Float4::new(1.0, 2.0, 2.0, 0.0));
    /// ```
    pub fn min(&self, rhs: &Self) -> Self {
        Self(arch::min(self.0, rhs.0))
    }

    /// Computes the larger value for each pair of elements in the two vectors.
    /// This is semantically equivalent to performing each operation separately,
    /// but may make use of SIMD instructions to improve performance.
    ///
    /// ```rust
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.max(b), Float4::new(4.0, 3.0, 3.0, 0.0));
    /// ```
    pub fn max(&self, rhs: &Self) -> Self {
        Self(arch::max(self.0, rhs.0))
    }

    /// Performs a less-than comparison for each pair of elements in the two
    /// vectors. This is semantically equivalent to performing each operation
    /// separately, but may make use of SIMD instructions to improve
    /// performance.
    ///
    /// ```rust
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.less_than(b), (true, true, false, false));
    /// ```
    pub fn less_than(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        arch::less(self.0, rhs.0)
    }

    /// Performs a less-than-or-equal comparison for each pair of elements in
    /// the two vectors. This is semantically equivalent to performing each
    /// operation separately, but may make use of SIMD instructions to improve
    /// performance.
    ///
    /// ```rust
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.less_than(b), (true, true, false, true));
    /// ```
    pub fn less_or_equal(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        arch::less_or_equal(self.0, rhs.0)
    }

    pub fn eq(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        arch::equal(self.0, rhs.0)
    }
}

impl Neg for Float4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(arch::neg(self.0))
    }
}

impl Add for Float4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(arch::add(self.0, rhs.0))
    }
}

impl Sub for Float4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(arch::sub(self.0, rhs.0))
    }
}

impl Mul for Float4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(arch::mul(self.0, rhs.0))
    }
}

impl Mul<Float4> for f32 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4::splat(self) * rhs
    }
}

impl Div for Float4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(arch::div(self.0, rhs.0))
    }
}

impl PartialEq for Float4 {
    fn eq(&self, other: &Self) -> bool {
        arch::equal(self.0, other.0) == (true, true, true, true)
    }
}

impl Debug for Float4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b, c, d) = self.unpack();
        f.debug_tuple("Float4")
            .field(&a)
            .field(&b)
            .field(&c)
            .field(&d)
            .finish()
    }
}

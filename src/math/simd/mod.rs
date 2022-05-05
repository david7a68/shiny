#![allow(clippy::inline_always)]

use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::{cmp::{ApproxEq, F32_APPROX_EQUAL_THRESHOLD}, ops::Interpolate};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "x86.rs"]
mod arch;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Float4(arch::Float4);

impl Float4 {
    /// Creates a new 4-float vector.
    #[inline]
    #[must_use]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self(arch::pack(a, b, c, d))
    }

    /// Loads a 4-element array into a vector.
    #[inline]
    #[must_use]
    pub fn from_array(arr: &[f32; 4]) -> Self {
        Self(arch::pack_array(arr))
    }

    /// Repeats the value `v` in every element of the vector.
    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Self(arch::splat(v))
    }

    /// Computes the horizontal sum of 2 4-float vectors at the same time.
    #[inline]
    #[must_use]
    pub fn horizontal_sum2(a: Float4, b: Float4) -> (f32, f32) {
        arch::horizontal_sum2(a.0, b.0)
    }

    #[inline]
    #[must_use]
    pub fn horizontal_sum4(a: Self, b: Self, c: Self, d: Self) -> Self {
        Self(arch::horizontal_sum4(a.0, b.0, c.0, d.0))
    }

    #[inline]
    #[must_use]
    pub fn horizontal_min4(a: Self, b: Self, c: Self, d: Self) -> Self {
        Self(arch::horizontal_min4(a.0, b.0, c.0, d.0))
    }

    #[inline]
    #[must_use]
    pub fn horizontal_max4(a: Self, b: Self, c: Self, d: Self) -> Self {
        Self(arch::horizontal_max4(a.0, b.0, c.0, d.0))
    }

    #[inline]
    #[must_use]
    pub fn horizontal_min_max4(a: Self, b: Self, c: Self, d: Self) -> (Self, Self) {
        let (min, max) = arch::horizontal_min_max4(a.0, b.0, c.0, d.0);
        (Self(min), Self(max))
    }

    /// Transposes a 4x4 matrix.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let r0 = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// let r1 = Float4::new(5.0, 6.0, 7.0, 8.0);
    /// let r2 = Float4::new(9.0, 10.0, 11.0, 12.0);
    /// let r3 = Float4::new(13.0, 14.0, 15.0, 16.0);
    ///
    /// let (c0, c1, c2, c3) = Float4::transpose4x4(r0, r1, r2, r3);
    /// assert_eq!(c0, Float4::new(1.0, 5.0, 9.0, 13.0));
    /// assert_eq!(c1, Float4::new(2.0, 6.0, 10.0, 14.0));
    /// assert_eq!(c2, Float4::new(3.0, 7.0, 11.0, 15.0));
    /// assert_eq!(c3, Float4::new(4.0, 8.0, 12.0, 16.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn transpose4x4(a: Self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        let (a, b, c, d) = arch::transpose(a.0, b.0, c.0, d.0);
        (Self(a), Self(b), Self(c), Self(d))
    }

    /// Computes 4 dot products simultaneously.
    #[inline]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn dot4(
        lhs1: Self,
        rhs1: Self,
        lhs2: Self,
        rhs2: Self,
        lhs3: Self,
        rhs3: Self,
        lhs4: Self,
        rhs4: Self,
    ) -> Self {
        Self(arch::dot4(
            lhs1.0, rhs1.0, lhs2.0, rhs2.0, lhs3.0, rhs3.0, lhs4.0, rhs4.0,
        ))
    }

    /// The first element in the vector.
    #[inline]
    #[must_use]
    pub fn a(&self) -> f32 {
        self.unpack().0
    }

    /// The second element in the vector.
    #[inline]
    #[must_use]
    pub fn b(&self) -> f32 {
        self.unpack().1
    }

    /// The third element in the vector.
    #[inline]
    #[must_use]
    pub fn c(&self) -> f32 {
        self.unpack().2
    }

    /// The fourth element in the vector.
    #[inline]
    #[must_use]
    pub fn d(&self) -> f32 {
        self.unpack().3
    }

    /// Unpacks the vector into a tuple.
    #[inline]
    #[must_use]
    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        arch::unpack(self.0)
    }

    /// Computes the absolute value of each element in the vector.This is
    /// semantically equivalent to performing each operation separately, but may
    /// make use of SIMD instructions to improve performance.
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        Self(arch::abs(self.0))
    }

    /// Computes the square root of each element in the vector.This is
    /// semantically equivalent to performing each operation separately, but may
    /// make use of SIMD instructions to improve performance.
    #[inline]
    #[must_use]
    pub fn sqrt(&self) -> Self {
        Self(arch::sqrt(self.0))
    }

    /// Returns the elements of the vector in reverse order.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let v = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(v.reverse().unpack(), (4.0, 3.0, 2.0, 1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(&self) -> Self {
        Self(arch::swizzle_reverse(self.0))
    }

    /// Swaps the high and low halves of the vector.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let v = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(v.swap_high_low().unpack(), (3.0, 4.0, 1.0, 2.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn swap_high_low(&self) -> Self {
        Self(arch::swap_high_low(self.0))
    }

    /// Combines two vectors by placing the high half of `self` in the low half
    /// of the result, and the low half of `rhs` in the high half of the result.
    /// 
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let v1 = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Float4::new(5.0, 6.0, 7.0, 8.0);
    /// assert_eq!(v1.combine_high_low(v2).unpack(), (3.0, 4.0, 5.0, 6.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn combine_high_low(&self, rhs: Self) -> Self {
        Self(arch::combine_high_low(self.0, rhs.0))
    }

    /// Computes the dot product of the vector with another.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// let b = Float4::new(5.0, 6.0, 7.0, 8.0);
    /// assert_eq!(a.dot(b), 70.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn dot(&self, rhs: Self) -> f32 {
        arch::dot(self.0, rhs.0)
    }

    /// Computes the cross product of two vectors.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 4.0);
    /// let b = Float4::new(1.0, 3.0, 3.0, 150.0);
    ///
    /// // Note how the 4th element is always 0.
    /// assert_eq!(a.cross(b).unpack(), (-3.0, 0.0, 1.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn cross(&self, rhs: Self) -> Self {
        Self(arch::cross(self.0, rhs.0))
    }

    /// Computes the smaller value for each pair of elements in the two vectors.
    /// This is semantically equivalent to performing each operation separately,
    /// but may make use of SIMD instructions to improve performance.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.min(b), Float4::new(1.0, 2.0, 2.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn min(&self, rhs: Self) -> Self {
        Self(arch::min(self.0, rhs.0))
    }

    /// Computes the larger value for each pair of elements in the two vectors.
    /// This is semantically equivalent to performing each operation separately,
    /// but may make use of SIMD instructions to improve performance.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.max(b), Float4::new(4.0, 3.0, 3.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn max(&self, rhs: Self) -> Self {
        Self(arch::max(self.0, rhs.0))
    }

    /// Performs a less-than comparison for each pair of elements in the two
    /// vectors. This is semantically equivalent to performing each operation
    /// separately, but may make use of SIMD instructions to improve
    /// performance.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.less_than(b), (true, true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn less_than(&self, rhs: Self) -> (bool, bool, bool, bool) {
        arch::less(self.0, rhs.0)
    }

    /// Performs a less-than-or-equal comparison for each pair of elements in
    /// the two vectors. This is semantically equivalent to performing each
    /// operation separately, but may make use of SIMD instructions to improve
    /// performance.
    ///
    /// ```rust
    /// # use shiny::math::simd::Float4;
    /// let a = Float4::new(1.0, 2.0, 3.0, 0.0);
    /// let b = Float4::new(4.0, 3.0, 2.0, 0.0);
    /// assert_eq!(a.less_than(b), (true, true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn less_or_equal(&self, rhs: Self) -> (bool, bool, bool, bool) {
        arch::less_or_equal(self.0, rhs.0)
    }

    /// Rotates the vector elements by the given amount. This is semantically equivalent to the following:
    ///
    /// ```rust
    /// fn rotate_right(v: (f32, f32, f32, f32), amount: usize) -> (f32, f32, f32, f32) {
    ///     match amount & 0b11 {
    ///         0 => v,
    ///         1 => (v.3, v.0, v.1, v.2),
    ///         2 => (v.2, v.3, v.0, v.1),
    ///         3 => (v.1, v.2, v.3, v.0),
    ///         _ => unreachable!(),
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn rotate_right(&self, amount: usize) -> Self {
        match amount & 0b11 {
            0 => *self,
            1 => Self(arch::rotate_right_1(self.0)),
            2 => Self(arch::rotate_right_2(self.0)),
            3 => Self(arch::rotate_right_3(self.0)),
            _ => unreachable!(),
        }
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

impl ApproxEq for Float4 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_within(other, F32_APPROX_EQUAL_THRESHOLD)
    }

    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        let diff = (*self - *other).abs();
        let (a, b, c, d) = diff.less_or_equal(Float4::splat(epsilon));
        a & b & c & d
    }
}

impl Interpolate for Float4 {
    fn lerp(&self, t: f32, rhs: &Self) -> Self {
        ((1.0 - t) * *self) + (t * *rhs)
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

impl From<[f32; 4]> for Float4 {
    fn from(arr: [f32; 4]) -> Self {
        Self(arch::pack_array(&arr))
    }
}

impl From<&[f32; 4]> for Float4 {
    fn from(arr: &[f32; 4]) -> Self {
        Self(arch::pack_array(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float4() {
        let a = Float4::new(1.0, 2.0, 3.0, 4.0);
        let b = Float4::new(5.0, 6.0, 7.0, 8.0);

        // Unary ops
        assert_eq!(a.a(), 1.0);
        assert_eq!(a.b(), 2.0);
        assert_eq!(a.c(), 3.0);
        assert_eq!(a.d(), 4.0);
        assert_eq!(a.unpack(), (1.0, 2.0, 3.0, 4.0));
        assert_eq!(a.reverse().unpack(), (4.0, 3.0, 2.0, 1.0));
        assert_eq!(a.reverse().reverse(), a);
        assert!((-a).approx_eq(&Float4::new(-1.0, -2.0, -3.0, -4.0)));
        assert!((-a).abs().approx_eq(&a));
        assert!(a.sqrt().approx_eq(&Float4::new(
            1.0f32.sqrt(),
            2.0f32.sqrt(),
            3.0f32.sqrt(),
            4.0f32.sqrt()
        )));
        assert_eq!(a.swap_high_low(), Float4::new(3.0, 4.0, 1.0, 2.0));
        assert_eq!(a.swap_high_low().swap_high_low(), a);

        {
            let (x, y, z, w) = Float4::transpose4x4(a, a, a, a);
            assert_eq!(x, Float4::splat(1.0));
            assert_eq!(y, Float4::splat(2.0));
            assert_eq!(z, Float4::splat(3.0));
            assert_eq!(w, Float4::splat(4.0));
        }

        assert_eq!(a.rotate_right(0), a);
        assert_eq!(a.rotate_right(1), Float4::new(4.0, 1.0, 2.0, 3.0));
        assert_eq!(a.rotate_right(2), Float4::new(3.0, 4.0, 1.0, 2.0));
        assert_eq!(a.rotate_right(3), Float4::new(2.0, 3.0, 4.0, 1.0));

        // Binary Ops: Float4 Float4
        assert!(a.approx_eq(&a));
        assert!(!a.approx_eq(&b));
        assert!(a
            .dot(b)
            .approx_eq(&(1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0)));
        {
            let x = Float4::new(1.0, 2.0, 3.0, 7.0);
            let y = Float4::new(1.0, 3.0, 2.0, 10.0);
            // Note how the fourth element is always 0.
            assert!(x.cross(y).approx_eq(&Float4::new(-5.0, 1.0, 1.0, 0.0)));
        }
        assert_eq!(a.min(b), a);
        assert_eq!(b.min(a), a);
        assert_eq!(a.max(b), b);
        assert_eq!(b.max(a), b);
        assert_eq!(a.less_than(a), (false, false, false, false));
        assert_eq!(a.less_than(b), (true, true, true, true));
        assert_eq!(a.less_or_equal(b), (true, true, true, true));
        assert_eq!(a.less_or_equal(a), (true, true, true, true));

        {
            // Determinant of 3x3 matrix using the scalar triple product.
            let x = Float4::new(3.0, 2.0, 1.0, 0.0);
            let y = Float4::new(1.0, 1.0, 5.0, 0.0);
            let z = Float4::new(9.0, 10.0, 11.0, 0.0);
            assert!(x.dot(y.cross(z)).approx_eq(&-48.0));
        }

        // Wide Ops
        assert!(Float4::horizontal_sum2(a, b).approx_eq(&(10.0, 26.0)));
        assert!(Float4::horizontal_sum4(a, b, a, b).approx_eq(&Float4::new(10.0, 26.0, 10.0, 26.0)));
        assert!(Float4::dot4(a, a, a, a, a, a, a, a).approx_eq(&Float4::new(
            a.dot(a),
            a.dot(a),
            a.dot(a),
            a.dot(a)
        )));
    }
}

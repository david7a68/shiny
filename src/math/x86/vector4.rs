use std::arch::x86_64::{
    __m128, _mm_add_ps, _mm_andnot_ps, _mm_castsi128_ps, _mm_cmp_ps, _mm_div_ps, _mm_max_ps,
    _mm_min_ps, _mm_movemask_ps, _mm_mul_ps, _mm_set1_epi32, _mm_set1_ps, _mm_set_ps,
    _mm_shuffle_ps, _mm_sqrt_ps, _mm_sub_ps, _CMP_EQ_OQ, _CMP_LT_OQ,
};

use super::utils::_MM_SHUFFLE;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vector4(__m128);

impl Vector4 {
    #[inline(always)]
    pub fn from_tuple(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(unsafe { _mm_set_ps(w, z, y, x) })
    }

    #[inline(always)]
    pub fn splat(v: f32) -> Self {
        Self(unsafe { _mm_set1_ps(v) })
    }

    #[inline(always)]
    pub fn hsum2(a: Vector4, b: Vector4) -> (f32, f32) {
        unsafe {
            let x = _mm_shuffle_ps(a.0, b.0, _MM_SHUFFLE(3, 2, 3, 2));
            let y = _mm_shuffle_ps(a.0, b.0, _MM_SHUFFLE(1, 0, 1, 0));
            let z = _mm_add_ps(x, y);
            let w = _mm_shuffle_ps(z, z, _MM_SHUFFLE(2, 3, 0, 1));
            let i = _mm_add_ps(z, w);
            let r: (f32, f32, f32, f32) = std::mem::transmute(i);
            (r.0, r.2)
        }
    }

    #[inline(always)]
    pub fn extract(&self) -> (f32, f32, f32, f32) {
        unsafe { std::mem::transmute(*self) }
    }

    #[inline(always)]
    pub fn zwxy(&self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(1, 0, 3, 2)) })
    }

    #[inline(always)]
    pub fn yxwz(&self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(2, 3, 0, 1)) })
    }

    #[inline(always)]
    pub fn add(&self, b: Self) -> Self {
        Self(unsafe { _mm_add_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn sub(&self, b: Self) -> Self {
        Self(unsafe { _mm_sub_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn mul(&self, b: Self) -> Self {
        Self(unsafe { _mm_mul_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn div(&self, b: Self) -> Self {
        Self(unsafe { _mm_div_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn sqrt(&self) -> Self {
        Self(unsafe { _mm_sqrt_ps(self.0) })
    }

    #[inline(always)]
    pub fn max(&self, b: Self) -> Self {
        Self(unsafe { _mm_max_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn min(&self, b: Self) -> Self {
        Self(unsafe { _mm_min_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        unsafe {
            let mask = _mm_castsi128_ps(_mm_set1_epi32(1 << 31));
            Self(_mm_andnot_ps(mask, self.0))
        }
    }

    #[inline(always)]
    /// Sets each of the first 4 bits to true if equal. 1st bit for element 1
    /// (usually x), 2nd bit for element 2, etc.
    pub fn eq_mask(&self, b: Self) -> i32 {
        unsafe { _mm_movemask_ps(_mm_cmp_ps(self.0, b.0, _CMP_EQ_OQ)) }
    }

    #[inline(always)]
    pub fn eq(&self, b: Self) -> (bool, bool, bool, bool) {
        let mask = unsafe { _mm_movemask_ps(_mm_cmp_ps(self.0, b.0, _CMP_EQ_OQ)) };
        ((mask & 0b1) != 0, (mask & 0b10) != 0, (mask & 0b100) != 0, (mask & 0b1000) != 0)
    }

    #[inline(always)]
    /// Sets each of the first 4 bits to true if equal. 1st bit for element 1
    /// (usually x), 2nd bit for element 2, etc.
    pub fn less(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        let mask = unsafe { _mm_movemask_ps(_mm_cmp_ps(self.0, rhs.0, _CMP_LT_OQ)) };
        ((mask & 0b1) != 0, (mask & 0b10) != 0, (mask & 0b100) != 0, (mask & 0b1000) != 0)
    }
}

impl std::fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self.extract().0)
            .field("y", &self.extract().1)
            .field("z", &self.extract().2)
            .field("w", &self.extract().3)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsum2() {
        let a = Vector4::from_tuple(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::from_tuple(5.0, 6.0, 7.0, 8.0);

        let (c, d) = Vector4::hsum2(a, b);

        // Close enough for something simple like this.
        assert!((c - 10.0).abs() < 0.0001);
        assert!((d - 26.0).abs() < 0.0001);
    }

    #[test]
    fn swizzle() {
        let a = (1.0, 2.0, 3.0, 4.0);
        let b = Vector4::from_tuple(a.0, a.1, a.2, a.3).yxwz().extract();

        println!("{:?}", b);

        assert_eq!(b.0, a.1);
        assert_eq!(b.1, a.0);
        assert_eq!(b.2, a.3);
        assert_eq!(b.3, a.2);
    }

    #[test]
    fn abs() {
        let a = Vector4::from_tuple(1.0, -1.0, f32::NAN, f32::NEG_INFINITY);
        let b = a.abs();

        let (x, y, z, w) = b.extract();

        assert_eq!(x, 1.0);
        assert_eq!(y, 1.0);
        assert!(z.is_nan());
        assert_eq!(w, f32::INFINITY);
    }

    #[test]
    fn eq() {
        let a = Vector4::from_tuple(1.0, 2.0, 3.0, 4.0);
        assert!(a.eq_mask(a) == 0b1111);
    }

    #[test]
    fn less() {
        let a = Vector4::from_tuple(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::from_tuple(1.0, 1.0, 4.0, 2.0);

        assert_eq!(a.less(&a), (false, false, false, false));
        assert_eq!(b.less(&b), (false, false, false, false));
        assert_eq!(b.less(&a), (false, true, false, true));
    }
}

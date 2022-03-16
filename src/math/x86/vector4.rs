use std::arch::x86_64::{
    __m128, _mm_add_ps, _mm_cmp_ps, _mm_div_ps, _mm_max_ps, _mm_min_ps, _mm_movemask_ps,
    _mm_mul_ps, _mm_set_ps, _mm_shuffle_ps, _mm_sub_ps,
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
    pub fn max(&self, b: Self) -> Self {
        Self(unsafe { _mm_max_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn min(&self, b: Self) -> Self {
        Self(unsafe { _mm_min_ps(self.0, b.0) })
    }

    #[inline(always)]
    pub fn eq(&self, b: Self) -> bool {
        unsafe { _mm_movemask_ps(_mm_cmp_ps(self.0, b.0, 0)) == 0b1111 }
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
}

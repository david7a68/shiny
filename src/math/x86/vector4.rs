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

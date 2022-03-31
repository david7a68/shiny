use std::arch::x86_64::{
    __m128, _mm_add_ps, _mm_andnot_ps, _mm_castsi128_ps, _mm_cmpeq_ps, _mm_cmple_ps, _mm_cmplt_ps,
    _mm_div_ps, _mm_max_ps, _mm_min_ps, _mm_movemask_ps, _mm_mul_ps, _mm_set1_epi32, _mm_set1_ps,
    _mm_set_ps, _mm_shuffle_ps, _mm_sqrt_ps, _mm_sub_ps,
};

pub type Float4 = __m128;

#[inline(always)]
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(x: i32, y: i32, z: i32, w: i32) -> i32 {
    (x << 6) | (y << 4) | (z << 2) | w
}

pub fn pack(a: f32, b: f32, c: f32, d: f32) -> Float4 {
    unsafe { _mm_set_ps(d, c, b, a) }
}

pub fn splat(v: f32) -> Float4 {
    unsafe { _mm_set1_ps(v) }
}

pub fn unpack(v: Float4) -> (f32, f32, f32, f32) {
    unsafe { std::mem::transmute(v) }
}

pub fn abs(v: Float4) -> Float4 {
    unsafe {
        let mask = _mm_castsi128_ps(_mm_set1_epi32(1 << 31));
        _mm_andnot_ps(mask, v)
    }
}

pub fn neg(v: Float4) -> Float4 {
    unsafe { _mm_sub_ps(_mm_set1_ps(0.0), v) }
}

pub fn sqrt(v: Float4) -> Float4 {
    unsafe { _mm_sqrt_ps(v) }
}

pub fn swizzle_reverse(v: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(v, v, _MM_SHUFFLE(0, 1, 2, 3)) }
}

pub fn swizzle_cdab(v: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(v, v, _MM_SHUFFLE(1, 0, 3, 2)) }
}

pub fn add(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_add_ps(lhs, rhs) }
}

pub fn sub(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_sub_ps(lhs, rhs) }
}

pub fn mul(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_mul_ps(lhs, rhs) }
}

pub fn div(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_div_ps(lhs, rhs) }
}

pub fn min(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_min_ps(lhs, rhs) }
}

pub fn max(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_max_ps(lhs, rhs) }
}

pub fn equal(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmpeq_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

pub fn less(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmplt_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

pub fn less_or_equal(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmple_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

/// Computes the horizontal sum of two 4-float vectors simultaneously in order
/// to improve register usage.
pub fn horizontal_sum2(a: Float4, b: Float4) -> (f32, f32) {
    unsafe {
        let x = _mm_shuffle_ps(a, b, _MM_SHUFFLE(3, 2, 3, 2));
        let y = _mm_shuffle_ps(a, b, _MM_SHUFFLE(1, 0, 1, 0));
        let z = _mm_add_ps(x, y);
        let w = _mm_shuffle_ps(z, z, _MM_SHUFFLE(2, 3, 0, 1));
        let r: (f32, f32, f32, f32) = unpack(_mm_add_ps(z, w));
        (r.0, r.2)
    }
}

fn bitmask_to_bool4(mask: i32) -> (bool, bool, bool, bool) {
    (
        (mask & 0b1) != 0,
        (mask & 0b10) != 0,
        (mask & 0b100) != 0,
        (mask & 0b1000) != 0,
    )
}

use std::arch::x86_64::{
    __m128, _mm_add_ps, _mm_andnot_ps, _mm_castsi128_ps, _mm_cmpeq_ps, _mm_cmple_ps, _mm_cmplt_ps,
    _mm_div_ps, _mm_loadu_ps, _mm_max_ps, _mm_min_ps, _mm_movemask_ps, _mm_mul_ps, _mm_set1_epi32,
    _mm_set1_ps, _mm_set_ps, _mm_shuffle_ps, _mm_sqrt_ps, _mm_sub_ps, _MM_TRANSPOSE4_PS,
};

pub type Float4 = __m128;

#[inline(always)]
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(x: i32, y: i32, z: i32, w: i32) -> i32 {
    (w << 6) | (z << 4) | (y << 2) | x
}

#[inline]
#[must_use]
pub fn pack(a: f32, b: f32, c: f32, d: f32) -> Float4 {
    unsafe { _mm_set_ps(d, c, b, a) }
}

#[inline]
#[must_use]
pub fn pack_array(arr: &[f32; 4]) -> Float4 {
    unsafe { _mm_loadu_ps(arr.as_ptr()) }
}

#[inline]
#[must_use]
pub fn splat(v: f32) -> Float4 {
    unsafe { _mm_set1_ps(v) }
}

#[inline]
#[must_use]
pub fn transpose(
    v1: Float4,
    v2: Float4,
    v3: Float4,
    v4: Float4,
) -> (Float4, Float4, Float4, Float4) {
    unsafe {
        let mut a = v1;
        let mut b = v2;
        let mut c = v3;
        let mut d = v4;
        _MM_TRANSPOSE4_PS(&mut a, &mut b, &mut c, &mut d);
        (a, b, c, d)
    }
}

#[inline]
#[must_use]
pub fn unpack(v: Float4) -> (f32, f32, f32, f32) {
    unsafe { std::mem::transmute(v) }
}

#[inline]
#[must_use]
pub fn abs(v: Float4) -> Float4 {
    unsafe {
        let mask = _mm_castsi128_ps(_mm_set1_epi32(1 << 31));
        _mm_andnot_ps(mask, v)
    }
}

#[inline]
#[must_use]
pub fn neg(v: Float4) -> Float4 {
    unsafe { _mm_sub_ps(_mm_set1_ps(0.0), v) }
}

#[inline]
#[must_use]
pub fn sqrt(v: Float4) -> Float4 {
    unsafe { _mm_sqrt_ps(v) }
}

#[inline]
#[must_use]
pub fn swizzle_reverse(v: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(v, v, _MM_SHUFFLE(3, 2, 1, 0)) }
}

#[inline]
#[must_use]
pub fn swap_high_low(v: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(v, v, _MM_SHUFFLE(2, 3, 0, 1)) }
}

#[inline]
#[must_use]
pub fn add(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_add_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn sub(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_sub_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn mul(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_mul_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn div(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_div_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn min(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_min_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn dot(lhs: Float4, rhs: Float4) -> f32 {
    // Profiling shows this to be faster than an attempt at hsum. Naturally, a
    // dot4 can take advantage of SIMD hsum and will probably be faster still.
    let tmp0 = mul(lhs, rhs);
    let (a, b, c, d) = unpack(tmp0);
    a + b + c + d
}

#[inline]
#[must_use]
pub fn cross(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe {
        let lhs_120 = _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(1, 2, 0, 3));
        let rhs_120 = _mm_shuffle_ps(rhs, rhs, _MM_SHUFFLE(1, 2, 0, 3));
        let minuend = mul(lhs, rhs_120);
        let subtrahend = mul(rhs, lhs_120);
        let unshuffled = sub(minuend, subtrahend);
        _mm_shuffle_ps(unshuffled, unshuffled, _MM_SHUFFLE(1, 2, 0, 3))
    }
}

#[inline]
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn dot4(
    l1: Float4,
    r1: Float4,
    l2: Float4,
    r2: Float4,
    l3: Float4,
    r3: Float4,
    l4: Float4,
    r4: Float4,
) -> Float4 {
    horizontal_sum4(mul(l1, r1), mul(l2, r2), mul(l3, r3), mul(l4, r4))
}

#[inline]
#[must_use]
pub fn max(lhs: Float4, rhs: Float4) -> Float4 {
    unsafe { _mm_max_ps(lhs, rhs) }
}

#[inline]
#[must_use]
pub fn equal(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmpeq_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

#[inline]
#[must_use]
pub fn less(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmplt_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

#[inline]
#[must_use]
pub fn less_or_equal(lhs: Float4, rhs: Float4) -> (bool, bool, bool, bool) {
    let mask = unsafe { _mm_movemask_ps(_mm_cmple_ps(lhs, rhs)) };
    bitmask_to_bool4(mask)
}

#[inline]
#[must_use]
pub fn rotate_right_1(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(3, 0, 1, 2)) }
}

#[inline]
#[must_use]
pub fn rotate_right_2(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(2, 3, 0, 1)) }
}

#[inline]
#[must_use]
pub fn rotate_right_3(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(1, 2, 3, 0)) }
}

/// Computes the horizontal sum of two 4-float vectors simultaneously in order
/// to improve register usage.
#[inline]
#[must_use]
pub fn horizontal_sum2(v1: Float4, v2: Float4) -> (f32, f32) {
    // initial: [a b c d][e f g h]
    unsafe {
        // [a b e f]
        let hi_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(2, 3, 2, 3));
        // [c d g h]
        let lo_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(0, 1, 0, 1));
        // [a+c b+d e+g f+h]
        let sum_hi_lo_12 = _mm_add_ps(hi_12, lo_12);
        // [b+d a+c f+h e+g]
        let rotated = _mm_shuffle_ps(sum_hi_lo_12, sum_hi_lo_12, _MM_SHUFFLE(1, 0, 3, 2));
        // [a+b+c+d a+b+c+d e+f+g+h e+f+g+h]
        let r: (f32, f32, f32, f32) = unpack(_mm_add_ps(sum_hi_lo_12, rotated));
        (r.0, r.2)
    }
}

#[inline]
#[must_use]
pub fn horizontal_sum4(v1: Float4, v2: Float4, v3: Float4, v4: Float4) -> Float4 {
    let (v1, v2, v3, v4) = transpose(v1, v2, v3, v4);
    add(add(v1, v2), add(v3, v4))
}

fn bitmask_to_bool4(mask: i32) -> (bool, bool, bool, bool) {
    (
        (mask & 0b1) != 0,
        (mask & 0b10) != 0,
        (mask & 0b100) != 0,
        (mask & 0b1000) != 0,
    )
}

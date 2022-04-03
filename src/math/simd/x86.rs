use std::arch::x86_64::{
    __m128, _mm_add_ps, _mm_andnot_ps, _mm_castsi128_ps, _mm_cmpeq_ps, _mm_cmple_ps, _mm_cmplt_ps,
    _mm_div_ps, _mm_loadu_ps, _mm_max_ps, _mm_min_ps, _mm_movemask_ps, _mm_mul_ps, _mm_set1_epi32,
    _mm_set1_ps, _mm_set_ps, _mm_shuffle_ps, _mm_sqrt_ps, _mm_sub_ps,
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

pub fn pack_array(arr: &[f32; 4]) -> Float4 {
    unsafe { _mm_loadu_ps(arr.as_ptr()) }
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

pub fn rotate_right_1(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(2, 1, 0, 3)) }
}

pub fn rotate_right_2(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(1, 0, 3, 2)) }
}

pub fn rotate_right_3(lhs: Float4) -> Float4 {
    unsafe { _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(0, 3, 2, 1)) }
}

/// Computes the horizontal sum of two 4-float vectors simultaneously in order
/// to improve register usage.
pub fn horizontal_sum2(v1: Float4, v2: Float4) -> (f32, f32) {
    // initial: [a b c d][e f g h]
    unsafe {
        // [a b e f]
        let hi_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(3, 2, 3, 2));
        // [c d g h]
        let lo_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(1, 0, 1, 0));
        // [a+c b+d e+g f+h]
        let sum_hi_lo_12 = _mm_add_ps(hi_12, lo_12);
        // [b+d a+c f+h e+g]
        let rotated = _mm_shuffle_ps(sum_hi_lo_12, sum_hi_lo_12, _MM_SHUFFLE(2, 3, 0, 1));
        // [a+b+c+d a+b+c+d e+f+g+h e+f+g+h]
        let r: (f32, f32, f32, f32) = unpack(_mm_add_ps(sum_hi_lo_12, rotated));
        (r.0, r.2)
    }
}

pub fn horizontal_sum4(v1: Float4, v2: Float4, v3: Float4, v4: Float4) -> Float4 {
    // initial: [a b c d][e f g h][i j k l][m n o p]
    unsafe {
        // [a b e f]
        let hi_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(3, 2, 3, 2));
        // [c d g h]
        let lo_12 = _mm_shuffle_ps(v1, v2, _MM_SHUFFLE(1, 0, 1, 0));
        // [a+c b+d e+g f+h]
        let sum_hi_lo_12 = _mm_add_ps(hi_12, lo_12);

        // [i j m n]
        let hi_34 = _mm_shuffle_ps(v3, v4, _MM_SHUFFLE(3, 2, 3, 2));
        // [k l o p]
        let lo_34 = _mm_shuffle_ps(v3, v4, _MM_SHUFFLE(1, 0, 1, 0));
        // [i+k j+l m+o n+p]
        let sum_hi_lo_34 = _mm_add_ps(hi_34, lo_34);

        // [a+c e+g i+k m+o]
        let hi_1234 = _mm_shuffle_ps(sum_hi_lo_12, sum_hi_lo_34, _MM_SHUFFLE(3, 1, 3, 1));
        // [b+d f+h j+l n+p]
        let lo_1234 = _mm_shuffle_ps(sum_hi_lo_12, sum_hi_lo_34, _MM_SHUFFLE(2, 0, 2, 0));
        // [a+b+c+d e+f+g+h i+j+k+l m+n+o+p]
        _mm_add_ps(hi_1234, lo_1234)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_sum4() {
        let a = pack(1.0, 2.0, 3.0, 4.0);
        let b = pack(5.0, 6.0, 7.0, 8.0);
        let c = pack(9.0, 10.0, 11.0, 12.0);
        let d = pack(13.0, 14.0, 15.0, 16.0);
        assert_eq!(
            unpack(super::horizontal_sum4(a, b, c, d)),
            (10.0, 26.0, 42.0, 58.0)
        );
    }

    #[test]
    fn rotate_right_1() {
        let a = pack(1.0, 2.0, 3.0, 4.0);
        assert_eq!(unpack(super::rotate_right_1(a)), (4.0, 1.0, 2.0, 3.0));
    }

    #[test]
    fn rotate_right_2() {
        let a = pack(1.0, 2.0, 3.0, 4.0);
        assert_eq!(unpack(super::rotate_right_2(a)), (3.0, 4.0, 1.0, 2.0));
    }

    #[test]
    fn rotate_right_3() {
        let a = pack(1.0, 2.0, 3.0, 4.0);
        assert_eq!(unpack(super::rotate_right_3(a)), (2.0, 3.0, 4.0, 1.0));
    }
}

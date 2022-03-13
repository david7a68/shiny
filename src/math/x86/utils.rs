#[inline(always)]
#[allow(non_snake_case)]
pub const fn _MM_SHUFFLE(x: i32, y: i32, z: i32, w: i32) -> i32 {
    (x << 6) | (y << 4) | (z << 2) | w
}

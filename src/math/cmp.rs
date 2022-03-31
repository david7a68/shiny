pub const APPROX_EQUAL_THRESHOLD: f32 = 1e-6;

pub fn approx_eq(a: f32, b: f32) -> bool {
    (b - a).abs() < APPROX_EQUAL_THRESHOLD
}

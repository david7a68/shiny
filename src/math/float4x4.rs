use std::ops::Mul;

#[cfg(target_arch = "x86_64")]
use super::x86::matrix4::Matrix4;
use super::Float4;

#[repr(transparent)]
pub struct Float4x4(Matrix4);

impl Float4x4 {
    pub fn new(r0: Float4, r1: Float4, r2: Float4, r3: Float4) -> Self {
        Self(Matrix4::new(r0.0, r1.0, r2.0, r3.0))
    }
}

impl Mul<Float4> for Float4x4 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4(self.0.mul(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_vec4() {
        let m = Float4x4::new(
            Float4::new(1.0, 2.0, 3.0, 4.0),
            Float4::new(5.0, 6.0, 7.0, 8.0),
            Float4::new(9.0, 10.0, 11.0, 12.0),
            Float4::new(13.0, 14.0, 15.0, 16.0),
        ) * Float4::new(17.0, 18.0, 19.0, 20.0);
        assert_eq!(m, (190.0, 486.0, 782.0, 1078.0));
    }
}

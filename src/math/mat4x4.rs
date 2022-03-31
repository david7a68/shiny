use std::ops::Mul;

use super::{simd::Float4, vec4::Vec4};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4x4(Float4, Float4, Float4, Float4);

impl Mat4x4 {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn new(v11: f32, v12: f32, v13: f32, v14: f32,
               v21: f32, v22: f32, v23: f32, v24: f32,
               v31: f32, v32: f32, v33: f32, v34: f32,
               v41: f32, v42: f32, v43: f32, v44: f32) -> Self {
        Self(
            Float4::new(v11, v12, v13, v14),
            Float4::new(v21, v22, v23, v24),
            Float4::new(v31, v32, v33, v34),
            Float4::new(v41, v42, v43, v44)
        )
    }

    #[inline(always)]
    pub fn r0(&self) -> &Float4 {
        &self.0
    }

    #[inline(always)]
    pub fn r1(&self) -> &Float4 {
        &self.1
    }

    #[inline(always)]
    pub fn r2(&self) -> &Float4 {
        &self.2
    }

    #[inline(always)]
    pub fn r3(&self) -> &Float4 {
        &self.3
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let r0 = (self.0 * rhs.0).unpack();
        let r1 = (self.1 * rhs.0).unpack();
        let r2 = (self.2 * rhs.0).unpack();
        let r3 = (self.3 * rhs.0).unpack();
        let c0 = Vec4::new(r0.0, r1.0, r2.0, r3.0);
        let c1 = Vec4::new(r0.1, r1.1, r2.1, r3.1);
        let c2 = Vec4::new(r0.2, r1.2, r2.2, r3.2);
        let c3 = Vec4::new(r0.3, r1.3, r2.3, r3.3);
        c0 + c1 + c2 + c3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_vec4() {
        #[rustfmt::skip]
        let m = Mat4x4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ) * Vec4::new(17.0, 18.0, 19.0, 20.0);
        assert_eq!(m.unpack(), (190.0, 486.0, 782.0, 1078.0));
    }
}

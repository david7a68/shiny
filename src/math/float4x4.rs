use std::ops::Mul;

use super::float4::Float4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Float4x4(Float4, Float4, Float4, Float4);

impl Float4x4 {
    #[inline(always)]
    pub fn new(r0: Float4, r1: Float4, r2: Float4, r3: Float4) -> Self {
        Self(r0, r1, r2, r3)
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

impl Mul<Float4> for Float4x4 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        let r0 = self.0.mul_elements(&rhs).unpack();
        let r1 = self.1.mul_elements(&rhs).unpack();
        let r2 = self.2.mul_elements(&rhs).unpack();
        let r3 = self.3.mul_elements(&rhs).unpack();
        let c0 = Float4::new(r0.0, r1.0, r2.0, r3.0);
        let c1 = Float4::new(r0.1, r1.1, r2.1, r3.1);
        let c2 = Float4::new(r0.2, r1.2, r2.2, r3.2);
        let c3 = Float4::new(r0.3, r1.3, r2.3, r3.3);
        c0 + c1 + c2 + c3
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

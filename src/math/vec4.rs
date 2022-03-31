use std::ops::{Add, Mul, Sub};

use super::mat4x4::Mat4x4;

#[cfg(target_arch = "x86_64")]
use super::x86::vector4::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec4(pub(super) Vector4);

impl Vec4 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::from_tuple(x, y, z, w))
    }

    #[inline(always)]
    pub fn splat(value: f32) -> Self {
        Self(Vector4::splat(value))
    }

    #[inline(always)]
    pub fn hsum2(a: Vec4, b: Vec4) -> (f32, f32) {
        Vector4::hsum2(a.0, b.0)
    }

    #[inline(always)]
    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        self.0.extract()
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.0.extract().0
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.0.extract().1
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.0.extract().2
    }

    #[inline(always)]
    pub fn w(&self) -> f32 {
        self.0.extract().3
    }

    #[inline(always)]
    pub fn zwxy(&self) -> Self {
        Self(self.0.zwxy())
    }

    #[inline(always)]
    pub fn yxwz(&self) -> Self {
        Self(self.0.yxwz())
    }

    #[inline(always)]
    pub fn max(&self, rhs: &Self) -> Self {
        Self(self.0.max(rhs.0))
    }

    #[inline(always)]
    pub fn min(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0))
    }

    #[inline(always)]
    pub fn mul_elements(&self, rhs: &Self) -> Self {
        Self(self.0.mul(rhs.0))
    }

    #[inline(always)]
    pub fn div_elements(&self, rhs: &Self) -> Self {
        Self(self.0.div(rhs.0))
    }

    #[inline(always)]
    pub fn sqrt_elements(&self) -> Self {
        Self(self.0.sqrt())
    }

    #[inline(always)]
    pub fn eq_elements(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        self.0.eq(rhs.0)
    }

    #[inline(always)]
    pub fn lt_elements(&self, rhs: &Self) -> (bool, bool, bool, bool) {
        self.0.less(&rhs.0)
    }
}

impl Add for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Sub for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs.mul_elements(&Vec4::splat(self))
    }
}

impl Mul<Mat4x4> for Vec4 {
    type Output = Vec4;

    #[inline(always)]
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let r0 = self.x() * *rhs.r0();
        let r1 = self.y() * *rhs.r1();
        let r2 = self.z() * *rhs.r2();
        let r3 = self.w() * *rhs.r3();
        r0 + r1 + r2 + r3
    }
}

impl PartialEq for Vec4 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_mask(other.0) == 0b1111
    }
}

impl PartialEq<(f32, f32, f32, f32)> for Vec4 {
    #[inline(always)]
    fn eq(&self, other: &(f32, f32, f32, f32)) -> bool {
        self.0
            .eq_mask(Vector4::from_tuple(other.0, other.1, other.2, other.3))
            == 0b1111
    }
}

impl std::fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Float4")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .field("w", &self.w())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v, (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn swizzle() {
        {
            // zwxy
            let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
            assert_eq!(v.zwxy(), Vec4::new(3.0, 4.0, 1.0, 2.0));
        }
    }

    #[test]
    fn float4_x_float4x4() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let m = Mat4x4::new(
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
            Vec4::new(17.0, 18.0, 19.0, 20.0),
        );

        assert_eq!(v * m, Vec4::new(130.0, 140.0, 150.0, 160.0));
    }
}

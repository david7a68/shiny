use super::vector4::Vector4;

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Matrix4 {
    pub r0: Vector4,
    pub r1: Vector4,
    pub r2: Vector4,
    pub r3: Vector4,
}

impl Matrix4 {
    pub fn new(r0: Vector4, r1: Vector4, r2: Vector4, r3: Vector4) -> Self {
        Self { r0, r1, r2, r3 }
    }

    #[inline]
    pub fn mul(self, rhs: Vector4) -> Vector4 {
        let r0: (f32, f32, f32, f32) = unsafe { std::mem::transmute(self.r0.mul(rhs)) };
        let r1: (f32, f32, f32, f32) = unsafe { std::mem::transmute(self.r1.mul(rhs)) };
        let r2: (f32, f32, f32, f32) = unsafe { std::mem::transmute(self.r2.mul(rhs)) };
        let r3: (f32, f32, f32, f32) = unsafe { std::mem::transmute(self.r3.mul(rhs)) };
        let c0 = Vector4::from_tuple(r0.0, r1.0, r2.0, r3.0);
        let c1 = Vector4::from_tuple(r0.1, r1.1, r2.1, r3.1);
        let c2 = Vector4::from_tuple(r0.2, r1.2, r2.2, r3.2);
        let c3 = Vector4::from_tuple(r0.3, r1.3, r2.3, r3.3);
        c0.add(c1).add(c2).add(c3)
    }
}

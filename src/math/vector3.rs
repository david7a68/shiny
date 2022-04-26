use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{
    cmp::{ApproxEq, F32_APPROX_EQUAL_THRESHOLD},
    simd::Float4,
};

/// A vector in 3D space.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    packed: Float4,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            packed: Float4::new(x, y, z, 0.0),
        }
    }

    pub fn dot4(a: (Self, Self), b: (Self, Self), c: (Self, Self), d: (Self, Self)) -> Float4 {
        Float4::dot4(
            a.0.packed, a.1.packed, b.0.packed, b.1.packed, c.0.packed, c.1.packed, d.0.packed,
            d.1.packed,
        )
    }

    pub fn x(self) -> f32 {
        self.packed.a()
    }

    pub fn y(self) -> f32 {
        self.packed.b()
    }

    pub fn z(self) -> f32 {
        self.packed.c()
    }

    pub fn xyz(self) -> (f32, f32, f32) {
        let (x, y, z, _) = self.packed.unpack();
        (x, y, z)
    }

    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }

    pub fn length2(self) -> f32 {
        self.dot(self)
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.packed.dot(rhs.packed)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            packed: self.packed.cross(rhs.packed),
        }
    }
}

// Unary Ops
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            packed: -self.packed,
        }
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z, _) = self.packed.unpack();
        f.debug_struct("Vec3")
            .field("x", &x)
            .field("y", &y)
            .field("z", &z)
            .finish()
    }
}

// Binary Ops: Vec3 Vec3

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            packed: self.packed + rhs.packed,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            packed: self.packed - rhs.packed,
        }
    }
}

impl ApproxEq for Vec3 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_within(other, F32_APPROX_EQUAL_THRESHOLD)
    }

    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        let diff = (self.packed - other.packed).abs();
        let (a, b, c, _) = diff.less_or_equal(Float4::splat(epsilon));
        a & b & c
    }
}

// Binary Ops: Vec3 f32

impl Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            packed: self.packed + Float4::splat(rhs),
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            packed: self.packed - Float4::splat(rhs),
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            packed: self.packed.div(Float4::splat(rhs)),
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            packed: self.packed.mul(Float4::splat(rhs)),
        }
    }
}

// Binary Ops: f32 Vec3

impl Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            packed: Float4::splat(self) + rhs.packed,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            packed: Float4::splat(self) - rhs.packed,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            packed: Float4::splat(self) / rhs.packed,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            packed: Float4::splat(self) * rhs.packed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        // unop
        assert!(a.neg().approx_eq(&Vec3::new(-1.0, -2.0, -3.0)));
        assert_eq!(format!("{:?}", a), "Vec3 { x: 1.0, y: 2.0, z: 3.0 }");

        // binop: vector vector
        assert!(a.approx_eq(&a));
        assert!(!a.approx_eq(&b));
        assert!((a + b).approx_eq(&Vec3::new(5.0, 7.0, 9.0)));
        assert!((a - b).approx_eq(&Vec3::new(-3.0, -3.0, -3.0)));
        assert!((a.dot(b)).approx_eq(&32.0));

        // binop: vector scalar
        assert!((a + 3.0).approx_eq(&Vec3::new(4.0, 5.0, 6.0)));
        assert!((a - 3.0).approx_eq(&Vec3::new(-2.0, -1.0, 0.0)));
        assert!((a * 3.0).approx_eq(&Vec3::new(3.0, 6.0, 9.0)));
        assert!((a / 3.0).approx_eq(&Vec3::new(0.333333, 0.666666, 1.0)));

        // binop: scalar vector
        assert!((3.0 + a).approx_eq(&Vec3::new(4.0, 5.0, 6.0)));
        assert!((3.0 - a).approx_eq(&Vec3::new(2.0, 1.0, 0.0)));
        assert!((3.0 * a).approx_eq(&Vec3::new(3.0, 6.0, 9.0)));
        assert!((3.0 / a).approx_eq(&Vec3::new(3.0, 1.5, 1.0)));
    }
}

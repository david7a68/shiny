use std::ops::Mul;

use super::{simd::Float4, vector2::Vec2};

/// A matrix with 1 row and 4 columns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat1x4 {
    pub r0: Float4,
}

impl Mat1x4 {
    #[inline]
    #[must_use]
    pub fn new(v11: f32, v12: f32, v13: f32, v14: f32) -> Self {
        Self {
            r0: Float4::new(v11, v12, v13, v14),
        }
    }
}

impl Mul<Mat4x2> for Mat1x4 {
    type Output = Vec2;

    fn mul(self, rhs: Mat4x2) -> Self::Output {
        let r0 = self.r0 * rhs.c0;
        let r1 = self.r0 * rhs.c1;
        let (a, b) = Float4::horizontal_sum2(r0, r1);
        Vec2::new(a, b)
    }
}

impl Mul<Mat4x4> for Mat1x4 {
    type Output = Self;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let (x, y, z, w) = self.r0.unpack();
        let r0 = x * rhs.r0;
        let r1 = y * rhs.r1;
        let r2 = z * rhs.r2;
        let r3 = w * rhs.r3;

        Self {
            r0: r0 + r1 + r2 + r3,
        }
    }
}

/// A matrix with 4 rows and 2 columns.
pub struct Mat4x2 {
    pub c0: Float4,
    pub c1: Float4,
}

impl Mat4x2 {
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn new(v11: f32, v12: f32,
               v21: f32, v22: f32,
               v31: f32, v32: f32,
               v41: f32, v42: f32) -> Self {
        Self {
            c0: Float4::new(v11, v21, v31, v41),
            c1: Float4::new(v12, v22, v32, v42)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4x4 {
    pub(super) r0: Float4,
    pub(super) r1: Float4,
    pub(super) r2: Float4,
    pub(super) r3: Float4,
}

impl Mat4x4 {
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn new(v11: f32, v12: f32, v13: f32, v14: f32,
               v21: f32, v22: f32, v23: f32, v24: f32,
               v31: f32, v32: f32, v33: f32, v34: f32,
               v41: f32, v42: f32, v43: f32, v44: f32) -> Self {
        Self {
            r0: Float4::new(v11, v12, v13, v14),
            r1: Float4::new(v21, v22, v23, v24),
            r2: Float4::new(v31, v32, v33, v34),
            r3: Float4::new(v41, v42, v43, v44)
        }
    }

    #[inline]
    #[must_use]
    pub fn r0(&self) -> (f32, f32, f32, f32) {
        self.r0.unpack()
    }

    #[inline]
    #[must_use]
    pub fn r1(&self) -> (f32, f32, f32, f32) {
        self.r1.unpack()
    }

    #[inline]
    #[must_use]
    pub fn r2(&self) -> (f32, f32, f32, f32) {
        self.r2.unpack()
    }

    #[inline]
    #[must_use]
    pub fn r3(&self) -> (f32, f32, f32, f32) {
        self.r3.unpack()
    }

    /// Flips the matrix across its diagonal.
    ///
    /// ```rust
    /// # use shiny::math::matrix4::Mat4x4;
    /// let m = Mat4x4::new(
    ///     1.0, 2.0, 3.0, 4.0,
    ///     5.0, 6.0, 7.0, 8.0,
    ///     9.0, 10.0, 11.0, 12.0,
    ///     13.0, 14.0, 15.0, 16.0,
    /// );
    /// let t = m.transpose();
    /// assert_eq!(t.r0(), (1.0, 5.0, 9.0, 13.0));
    /// assert_eq!(t.r1(), (2.0, 6.0, 10.0, 14.0));
    /// assert_eq!(t.r2(), (3.0, 7.0, 11.0, 15.0));
    /// assert_eq!(t.r3(), (4.0, 8.0, 12.0, 16.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn transpose(&self) -> Self {
        let (r0, r1, r2, r3) = Float4::transpose4x4(self.r0, self.r1, self.r2, self.r3);
        Self { r0, r1, r2, r3 }
    }
}

impl Mul<Mat4x2> for Mat4x4 {
    type Output = Mat4x2;

    fn mul(self, rhs: Mat4x2) -> Self::Output {
        let c0 = {
            let r0 = self.r0 * rhs.c0;
            let r1 = self.r1 * rhs.c0;
            let r2 = self.r2 * rhs.c0;
            let r3 = self.r3 * rhs.c0;
            Float4::horizontal_sum4(r0, r1, r2, r3)
        };

        let c1 = {
            let r0 = self.r0 * rhs.c1;
            let r1 = self.r1 * rhs.c1;
            let r2 = self.r2 * rhs.c1;
            let r3 = self.r3 * rhs.c1;
            Float4::horizontal_sum4(r0, r1, r2, r3)
        };

        Mat4x2 { c0, c1 }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector2::Vec2;

    use super::*;
    #[test]
    fn mul_1x4_2x4() {
        let a = Mat1x4::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat4x2::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let c = a * b;
        assert_eq!(c, Vec2::new(50.0, 60.0));
    }

    #[test]
    fn mul_1x4_4x4() {
        let a = Mat1x4::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let c = a * b;
        assert_eq!(c, Mat1x4::new(90.0, 100.0, 110.0, 120.0));
    }

    #[test]
    fn transpose_4x4() {
        let m = Mat4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let t = m.transpose();
        assert_eq!(t.r0(), (1.0, 5.0, 9.0, 13.0));
        assert_eq!(t.r1(), (2.0, 6.0, 10.0, 14.0));
        assert_eq!(t.r2(), (3.0, 7.0, 11.0, 15.0));
        assert_eq!(t.r3(), (4.0, 8.0, 12.0, 16.0));
    }
}

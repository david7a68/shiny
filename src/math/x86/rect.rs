use std::arch::x86_64::{__m128, _mm_cmp_ps, _mm_cmplt_ps, _mm_movemask_ps, _mm_set_ps};

/// An axis-aligned bounding box with SIMD-accelerated box-box intersection
/// testing.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Rect(__m128);

impl Rect {
    // Note: The order of the members in the __m128 is important to ensure
    // optimal layout for `intersects()`! Changing it could double the amount of
    // assembly that the compiler generates, so be careful and measure before
    // and after making changes!

    #[inline]
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self(unsafe { _mm_set_ps(right, bottom, left, top) })
    }

    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.extract().3
    }

    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.extract().1
    }

    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.extract().2
    }

    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.extract().0
    }

    /// Tests if this box intersects with another.
    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        unsafe {
            let a = _mm_set_ps(other.left(), other.top(), self.left(), self.top());
            let b = _mm_set_ps(self.right(), self.bottom(), other.right(), other.bottom());
            _mm_movemask_ps(_mm_cmplt_ps(a, b)) == 0
        }
    }

    #[inline(always)]
    pub fn eq(&self, b: Self) -> bool {
        unsafe { _mm_movemask_ps(_mm_cmp_ps(self.0, b.0, 0)) == 0b1111 }
    }

    #[inline(always)]
    fn extract(&self) -> (f32, f32, f32, f32) {
        unsafe { std::mem::transmute(*self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let r = Rect::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(r.extract(), (3.0, 1.0, 4.0, 2.0));
    }

    #[test]
    fn bounding_box() {
        {
            // miss on x-axis
            let left = Rect::new(10.0, 20.0, 10.0, 20.0);
            let right = Rect::new(30.0, 40.0, 10.0, 20.0);

            assert!(!left.intersects(&right));
            assert!(!right.intersects(&left));
        }
        {
            // miss on y axis
            let top = Rect::new(10.0, 20.0, 10.0, 20.0);
            let bottom = Rect::new(10.0, 20.0, 30.0, 40.0);

            assert!(!top.intersects(&bottom));
            assert!(!bottom.intersects(&top));
        }
        {
            // one in the other
            let outer = Rect::new(10.0, 20.0, 10.0, 20.0);
            let inner = Rect::new(12.0, 18.0, 12.0, 20.0);

            assert!(outer.intersects(&inner));
            assert!(inner.intersects(&outer));
        }
        {
            // one is the other
            let a = Rect::new(10.0, 20.0, 10.0, 20.0);
            let b = a;

            assert!(a.intersects(&b));
            assert!(b.intersects(&a));
        }
        {
            // normal intersection
            let a = Rect::new(10.0, 20.0, 10.0, 20.0);
            let b = Rect::new(15.0, 25.0, 15.0, 25.0);

            assert!(a.intersects(&b));
            assert!(b.intersects(&a));
        }
        {
            // line intersection
            let horizontal = Rect::new(10.0, 20.0, 10.0, 10.0);
            let vertical = Rect::new(15.0, 15.0, 10.0, 20.0);

            assert!(horizontal.intersects(&vertical));
            assert!(vertical.intersects(&horizontal));
        }
    }
}

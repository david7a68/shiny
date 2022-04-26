pub const F32_APPROX_EQUAL_THRESHOLD: f32 = 1e-5;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

pub(crate) use max;
pub(crate) use min;

pub trait ApproxEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    #[must_use]
    fn approx_eq(&self, other: &Rhs) -> bool;

    #[must_use]
    fn approx_eq_within(&self, other: &Rhs, epsilon: f32) -> bool;
}

impl ApproxEq<f32> for f32 {
    fn approx_eq(&self, other: &f32) -> bool {
        (other - *self).abs() <= F32_APPROX_EQUAL_THRESHOLD
    }

    fn approx_eq_within(&self, other: &f32, epsilon: f32) -> bool {
        (other - *self).abs() <= epsilon
    }
}

impl<A> ApproxEq for (A,)
where
    A: ApproxEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }

    fn approx_eq_within(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_within(&other.0, epsilon)
    }
}

impl<A, B> ApproxEq for (A, B)
where
    A: ApproxEq,
    B: ApproxEq,
{
    fn approx_eq(&self, other: &(A, B)) -> bool {
        self.0.approx_eq(&other.0) && self.1.approx_eq(&other.1)
    }

    fn approx_eq_within(&self, other: &(A, B), epsilon: f32) -> bool {
        self.0.approx_eq_within(&other.0, epsilon) && self.1.approx_eq_within(&other.1, epsilon)
    }
}

impl<A, B, C> ApproxEq for (A, B, C)
where
    A: ApproxEq,
    B: ApproxEq,
    C: ApproxEq + ?Sized,
{
    fn approx_eq(&self, other: &(A, B, C)) -> bool {
        self.0.approx_eq(&other.0) && self.1.approx_eq(&other.1) && self.2.approx_eq(&other.2)
    }

    fn approx_eq_within(&self, other: &(A, B, C), epsilon: f32) -> bool {
        self.0.approx_eq_within(&other.0, epsilon)
            && self.1.approx_eq_within(&other.1, epsilon)
            && self.2.approx_eq_within(&other.2, epsilon)
    }
}

impl<A, B, C, D> ApproxEq for (A, B, C, D)
where
    A: ApproxEq,
    B: ApproxEq,
    C: ApproxEq,
    D: ApproxEq,
{
    fn approx_eq(&self, other: &(A, B, C, D)) -> bool {
        self.0.approx_eq(&other.0)
            && self.1.approx_eq(&other.1)
            && self.2.approx_eq(&other.2)
            && self.3.approx_eq(&other.3)
    }

    fn approx_eq_within(&self, other: &(A, B, C, D), epsilon: f32) -> bool {
        self.0.approx_eq_within(&other.0, epsilon)
            && self.1.approx_eq_within(&other.1, epsilon)
            && self.2.approx_eq_within(&other.2, epsilon)
            && self.3.approx_eq_within(&other.3, epsilon)
    }
}

pub type Srgb8 = Standard<Rgb<u8>>;
pub type Srgba8 = Standard<Rgba<u8, u8>>;

pub trait Color: Copy {
    type Component;
    type Alpha;

    const BLACK: Self;
    const WHITE: Self;

    fn red(&self) -> Self::Component;

    fn green(&self) -> Self::Component;

    fn blue(&self) -> Self::Component;

    fn alpha(&self) -> Self::Alpha;
}

pub trait Raw: Color {}

pub trait Component: Copy {
    const MAX: Self;
    const ZERO: Self;
    const BIT_DEPTH: usize;
}

impl Component for u8 {
    const MAX: Self = u8::MAX;
    const ZERO: Self = 0;
    const BIT_DEPTH: usize = 8;
}

#[derive(Clone, Copy)]
pub struct Rgb<C: Component> {
    pub r: C,
    pub g: C,
    pub b: C,
}

impl<C: Component> Raw for Rgb<C> {}

impl<C: Component> Color for Rgb<C> {
    type Component = C;
    type Alpha = ();

    const BLACK: Self = Self {
        r: C::ZERO,
        g: C::ZERO,
        b: C::ZERO,
    };

    const WHITE: Self = Self {
        r: C::MAX,
        g: C::MAX,
        b: C::MAX,
    };

    fn red(&self) -> Self::Component {
        self.r
    }

    fn green(&self) -> Self::Component {
        self.g
    }

    fn blue(&self) -> Self::Component {
        self.b
    }

    fn alpha(&self) -> Self::Alpha {}
}

#[derive(Clone, Copy)]
pub struct Rgba<C: Component, A: Component> {
    pub r: C,
    pub g: C,
    pub b: C,
    pub a: A,
}

impl<C: Component, A: Component> Raw for Rgba<C, A> {}

impl<C: Component, A: Component> Color for Rgba<C, A> {
    type Component = C;
    type Alpha = A;

    const BLACK: Self = Self {
        r: C::ZERO,
        g: C::ZERO,
        b: C::ZERO,
        a: A::ZERO,
    };

    const WHITE: Self = Self {
        r: C::MAX,
        g: C::MAX,
        b: C::MAX,
        a: A::MAX,
    };

    fn red(&self) -> Self::Component {
        self.r
    }

    fn green(&self) -> Self::Component {
        self.g
    }

    fn blue(&self) -> Self::Component {
        self.b
    }

    fn alpha(&self) -> Self::Alpha {
        self.a
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Standard<C: Raw> {
    pub color: C,
}

impl<C: Raw> Color for Standard<C> {
    type Component = C::Component;
    type Alpha = C::Alpha;

    const BLACK: Self = Self { color: C::BLACK };

    const WHITE: Self = Self { color: C::WHITE };

    fn red(&self) -> Self::Component {
        self.color.red()
    }

    fn green(&self) -> Self::Component {
        self.color.green()
    }

    fn blue(&self) -> Self::Component {
        self.color.blue()
    }

    fn alpha(&self) -> Self::Alpha {
        self.color.alpha()
    }
}

pub trait Color: Copy {
    type Component;
    type Alpha;

    fn red(&self) -> Self::Component;

    fn green(&self) -> Self::Component;

    fn blue(&self) -> Self::Component;

    fn alpha(&self) -> Self::Alpha;
}

pub trait RawColor: Color {}

pub trait Component: Copy {
    const BIT_DEPTH: usize;
}

impl Component for u8 {
    const BIT_DEPTH: usize = 8;
}

#[derive(Clone, Copy)]
pub struct Rgb<C: Component> {
    pub r: C,
    pub g: C,
    pub b: C,
}

impl<C: Component> RawColor for Rgb<C> {}

impl<C: Component> Color for Rgb<C> {
    type Component = C;
    type Alpha = ();

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

impl<C: Component, A: Component> RawColor for Rgba<C, A> {}

impl<C: Component, A: Component> Color for Rgba<C, A> {
    type Component = C;
    type Alpha = A;

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
pub struct Standard<C: RawColor> {
    pub color: C,
}

impl<C: RawColor> Color for Standard<C> {
    type Component = C::Component;
    type Alpha = C::Alpha;

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

pub type Srgb8 = Standard<Rgb<u8>>;
pub type Srgba8 = Standard<Rgba<u8, u8>>;

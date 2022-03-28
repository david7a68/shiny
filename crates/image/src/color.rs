pub trait Color: Copy {
    type Component;
    type Alpha;

    fn red(&self) -> Self::Component;

    fn green(&self) -> Self::Component;

    fn blue(&self) -> Self::Component;

    fn alpha(&self) -> Self::Alpha;
}

pub trait RawColor: Color {}

#[derive(Clone, Copy)]
pub struct Rgb<Component: Copy> {
    pub r: Component,
    pub g: Component,
    pub b: Component,
}

impl<Component: Copy> RawColor for Rgb<Component> {}

impl<Component: Copy> Color for Rgb<Component> {
    type Component = Component;
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
pub struct Rgba<Component, Alpha>
where
    Component: Copy,
    Alpha: Copy,
{
    pub r: Component,
    pub g: Component,
    pub b: Component,
    pub a: Alpha,
}

impl<Component: Copy, Alpha: Copy> RawColor for Rgba<Component, Alpha> {}

impl<Component: Copy, Alpha: Copy> Color for Rgba<Component, Alpha> {
    type Component = Component;
    type Alpha = Alpha;

    fn red(&self) -> Component {
        self.r
    }

    fn green(&self) -> Component {
        self.g
    }

    fn blue(&self) -> Component {
        self.b
    }

    fn alpha(&self) -> Alpha {
        self.a
    }
}

#[derive(Clone, Copy)]
pub struct Standard<C: RawColor> {
    color: C,
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
pub type Srgba8 = Standard<Rgb<u8>>;

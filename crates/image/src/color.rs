pub enum ColorFormat {
    Rgba8,
}

pub trait Color: Copy {
    const FORMAT: ColorFormat;
    // fn format(&self) -> ColorFormat;

    fn red(&self) -> f32;

    fn green(&self) -> f32;

    fn blue(&self) -> f32;

    fn alpha(&self) -> f32;
}

#[derive(Clone, Copy)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color for Rgba8 {
    const FORMAT: ColorFormat = ColorFormat::Rgba8;

    fn red(&self) -> f32 {
        self.r as f32 / u8::MAX as f32
    }

    fn green(&self) -> f32 {
        self.g as f32 / u8::MAX as f32
    }

    fn blue(&self) -> f32 {
        self.b as f32 / u8::MAX as f32
    }

    fn alpha(&self) -> f32 {
        self.a as f32 / u8::MAX as f32
    }
}

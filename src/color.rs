//! Representations of color, color spaces, and their interactions.
//!
//! A color is represented by a tuple of 4 floating-point values between 0.0 and
//! 1.0, and represent the intensity of red, green, and blue relative to a color
//! space, as well as transparency (alpha). By default, that color space is the
//! sRGB color space, which is most commonly used color space today (2022).
//!
//! A color space defines the reddest red, greenest green, bluest blue, and
//! whitest white that can be represented, as well as the conversion between it
//! and the intermediate color space CIEL XYZ at the standard illuminant (white
//! point) D50. This color space was chosen because it represents all visually
//! perceptible colors and is commonly used for this purpose. The standard
//! illuminant was similarly chosen because it is what is defined in ICC color
//! profiles and is well published. The ready availability of documentation in
//! both cases was a significant factor.
//!

use std::{hash::Hash, ops::Add};

/// A 4-component color specifying red, green, blue, and transparency (alpha).
/// This type is used when specifying colors for drawing commands, and is
/// defined relative to the color space of the render target.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub space: Space,
}

impl Color {
    pub const RED: Self = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const GREEN: Self = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const BLUE: Self = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const BLACK: Self = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const WHITE: Self = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const BRIGHT_PINK: Self = Color {
        r: 1.0,
        g: 0.0,
        b: 127.0 / 255.0,
        a: 1.0,
        space: Space::Unknown,
    };

    pub const DEFAULT: Self = Self::BRIGHT_PINK;

    pub fn auto(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a,
            space: Space::Unknown,
        }
    }

    pub fn unknown(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a,
            space: Space::Unknown,
        }
    }

    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a,
            space: Space::Srgb,
        }
    }

    pub fn as_unknown(&self) -> Self {
        let mut c = *self;
        c.space = Space::Unknown;
        c
    }

    pub fn in_color_space(&self, target: Space) -> Self {
        match self.space {
            Space::Unknown => {
                let mut c = *self;
                c.space = target;
                c
            }
            Space::Srgb => match target {
                Space::Unknown => self.as_unknown(),
                Space::Srgb => *self,
                Space::LinearSrgb => {
                    // this is doable with SIMD... is it worth it?

                    let r = if self.r <= 0.04045 {
                        self.r / 12.92
                    } else {
                        ((self.r + 0.055) / 1.055).powf(2.4)
                    };

                    let g = if self.g <= 0.04045 {
                        self.g / 12.92
                    } else {
                        ((self.g + 0.055) / 1.055).powf(2.4)
                    };

                    let b = if self.b <= 0.04045 {
                        self.b / 12.92
                    } else {
                        ((self.b + 0.055) / 1.055).powf(2.4)
                    };

                    Color {
                        r,
                        g,
                        b,
                        a: self.a,
                        space: Space::LinearSrgb,
                    }
                }
                Space::Rec2020 => todo!(),
            },
            Space::LinearSrgb => match target {
                Space::Unknown => self.as_unknown(),
                Space::Srgb => {
                    let r = if self.r <= 0.0031308 {
                        self.r * 12.92
                    } else {
                        (1.055 * self.r.powf(1.0 / 2.4) - 0.055).max(0.0).min(1.0)
                    };

                    let g = if self.g <= 0.0031308 {
                        self.g * 12.92
                    } else {
                        (1.055 * self.g.powf(1.0 / 2.4) - 0.055).max(0.0).min(1.0)
                    };

                    let b = if self.b <= 0.0031308 {
                        self.b * 12.92
                    } else {
                        (1.055 * self.b.powf(1.0 / 2.4) - 0.055).max(0.0).min(1.0)
                    };

                    Color {
                        r,
                        g,
                        b,
                        a: self.a,
                        space: Space::Srgb,
                    }
                }
                Space::LinearSrgb => *self,
                Space::Rec2020 => todo!(),
            },
            Space::Rec2020 => match target {
                Space::Unknown => self.as_unknown(),
                Space::Srgb => todo!(),
                Space::LinearSrgb => todo!(),
                Space::Rec2020 => *self,
            },
        }
    }

    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * u8::MAX as f32).round() as u8,
            (self.g * u8::MAX as f32).round() as u8,
            (self.b * u8::MAX as f32).round() as u8,
            (self.a * u8::MAX as f32).round() as u8,
        ]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BRIGHT_PINK
    }
}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.a.to_bits().hash(state);
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
            space: self.space,
        }
    }
}

/// A color space describes the relationship between colors as represented by
/// [`Color`] and what is percieved by the human eye.
///
/// Each channel of color in RGB represents the proportion of red, green, and
/// blue, within the range described by the color space's primaries (reddest
/// red, greenest green, and bluest blue). Similarly, the white point `(1.0,
/// 1.0, 1.0)` determines just what 'white' means within the color space. A
/// color such as `(0.5, 1.0, 0.3)` may produce produce different colors
/// depending on the color space used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Unknown,
    /// The sRGB color space, which is the most commonly used color space today.
    /// This is a non-linear color space, meaning that operations such as blurs,
    /// shadows, and other manipulations **will not look right**. Instead,
    /// prefer to use LinearSrgb or another linear color space for those
    /// operations, and then convert to sRGB for output.
    Srgb,
    /// A linear sRGB color space. Pixels of this color space are typically
    /// represented with 10 bits per channel per pixel.
    LinearSrgb,
    /// A color space with expanded color gamut used for HDR displays. Pixels of
    /// this color space are best represented with at least 10 bits per channel.
    ///
    /// This color space is also known as the BT.2020 color space.
    Rec2020,
}

impl Space {
    pub fn is_linear(&self) -> bool {
        matches!(self, Space::LinearSrgb)
    }

    /// Queries the minimum number of bits per channel required to represent the
    /// color space.
    ///
    /// This does not include the alpha channel.
    pub fn bits_per_channel(&self) -> usize {
        match self {
            Space::Unknown => 0,
            Space::Srgb => 8,
            Space::LinearSrgb => 10,
            Space::Rec2020 => 10,
        }
    }
}

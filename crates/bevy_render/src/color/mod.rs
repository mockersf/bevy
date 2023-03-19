use bevy_reflect::{FromReflect, Reflect, ReflectDeserialize, ReflectSerialize};
use palette::{convert::FromColorUnclamped, rgb::Rgba, Hsla, Lcha, LinSrgba, Srgba};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use palette::*;

#[derive(Debug, Clone, Copy, PartialEq, Reflect, FromReflect)]
#[reflect(PartialEq)]
pub enum Color {
    /// sRGBA color
    Rgba(Srgba),
    /// RGBA color in the Linear sRGB colorspace (often colloquially referred to as "linear", "RGB", or "linear RGB").
    RgbaLinear(LinSrgba),
    /// HSL (hue, saturation, lightness) color with an alpha channel
    Hsla(Hsla),
    /// LCH(ab) (lightness, chroma, hue) color with an alpha channel
    Lcha(Lcha),
}

impl Color {
    /// <div style="background-color:rgb(94%, 97%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ALICE_BLUE: Color = Color::rgb(0.94, 0.97, 1.0);
    /// <div style="background-color:rgb(98%, 92%, 84%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ANTIQUE_WHITE: Color = Color::rgb(0.98, 0.92, 0.84);
    /// <div style="background-color:rgb(49%, 100%, 83%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const AQUAMARINE: Color = Color::rgb(0.49, 1.0, 0.83);
    /// <div style="background-color:rgb(94%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const AZURE: Color = Color::rgb(0.94, 1.0, 1.0);
    /// <div style="background-color:rgb(96%, 96%, 86%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BEIGE: Color = Color::rgb(0.96, 0.96, 0.86);
    /// <div style="background-color:rgb(100%, 89%, 77%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BISQUE: Color = Color::rgb(1.0, 0.89, 0.77);
    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(86%, 8%, 24%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const CRIMSON: Color = Color::rgb(0.86, 0.08, 0.24);
    /// <div style="background-color:rgb(0%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);
    /// <div style="background-color:rgb(25%, 25%, 25%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const DARK_GRAY: Color = Color::rgb(0.25, 0.25, 0.25);
    /// <div style="background-color:rgb(0%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const DARK_GREEN: Color = Color::rgb(0.0, 0.5, 0.0);
    /// <div style="background-color:rgb(100%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const FUCHSIA: Color = Color::rgb(1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 84%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GOLD: Color = Color::rgb(1.0, 0.84, 0.0);
    /// <div style="background-color:rgb(50%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    /// <div style="background-color:rgb(28%, 0%, 51%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const INDIGO: Color = Color::rgb(0.29, 0.0, 0.51);
    /// <div style="background-color:rgb(20%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const LIME_GREEN: Color = Color::rgb(0.2, 0.8, 0.2);
    /// <div style="background-color:rgb(50%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const MAROON: Color = Color::rgb(0.5, 0.0, 0.0);
    /// <div style="background-color:rgb(10%, 10%, 44%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const MIDNIGHT_BLUE: Color = Color::rgb(0.1, 0.1, 0.44);
    /// <div style="background-color:rgb(0%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const NAVY: Color = Color::rgb(0.0, 0.0, 0.5);
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    #[doc(alias = "transparent")]
    pub const NONE: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(50%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const OLIVE: Color = Color::rgb(0.5, 0.5, 0.0);
    /// <div style="background-color:rgb(100%, 65%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ORANGE: Color = Color::rgb(1.0, 0.65, 0.0);
    /// <div style="background-color:rgb(100%, 27%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ORANGE_RED: Color = Color::rgb(1.0, 0.27, 0.0);
    /// <div style="background-color:rgb(100%, 8%, 57%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PINK: Color = Color::rgb(1.0, 0.08, 0.58);
    /// <div style="background-color:rgb(50%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PURPLE: Color = Color::rgb(0.5, 0.0, 0.5);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    /// <div style="background-color:rgb(98%, 50%, 45%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SALMON: Color = Color::rgb(0.98, 0.5, 0.45);
    /// <div style="background-color:rgb(18%, 55%, 34%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SEA_GREEN: Color = Color::rgb(0.18, 0.55, 0.34);
    /// <div style="background-color:rgb(75%, 75%, 75%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SILVER: Color = Color::rgb(0.75, 0.75, 0.75);
    /// <div style="background-color:rgb(0%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TEAL: Color = Color::rgb(0.0, 0.5, 0.5);
    /// <div style="background-color:rgb(100%, 39%, 28%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TOMATO: Color = Color::rgb(1.0, 0.39, 0.28);
    /// <div style="background-color:rgb(25%, 88%, 82%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TURQUOISE: Color = Color::rgb(0.25, 0.88, 0.82);
    /// <div style="background-color:rgb(93%, 51%, 93%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const VIOLET: Color = Color::rgb(0.93, 0.51, 0.93);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    /// <div style="background-color:rgb(60%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YELLOW_GREEN: Color = Color::rgb(0.6, 0.8, 0.2);

    /// New `Color` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0.0, 1.0]
    /// * `g` - Green channel. [0.0, 1.0]
    /// * `b` - Blue channel. [0.0, 1.0]
    ///
    /// See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].
    ///
    pub const fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color::Rgba(Srgba::new(red, green, blue, 1.0))
    }

    /// New `Color` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0.0, 1.0]
    /// * `g` - Green channel. [0.0, 1.0]
    /// * `b` - Blue channel. [0.0, 1.0]
    /// * `a` - Alpha channel. [0.0, 1.0]
    ///
    /// See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].
    ///
    pub const fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color::Rgba(Srgba::new(red, green, blue, alpha))
    }

    /// New `Color` from linear RGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0.0, 1.0]
    /// * `g` - Green channel. [0.0, 1.0]
    /// * `b` - Blue channel. [0.0, 1.0]
    ///
    /// See also [`Color::rgb`], [`Color::rgba_linear`].
    ///
    pub const fn rgb_linear(red: f32, green: f32, blue: f32) -> Color {
        Color::RgbaLinear(LinSrgba::new(red, green, blue, 1.0))
    }

    /// New `Color` from linear RGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0.0, 1.0]
    /// * `g` - Green channel. [0.0, 1.0]
    /// * `b` - Blue channel. [0.0, 1.0]
    /// * `a` - Alpha channel. [0.0, 1.0]
    ///
    /// See also [`Color::rgba`], [`Color::rgb_linear`].
    ///
    pub const fn rgba_linear(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color::RgbaLinear(LinSrgba::new(red, green, blue, alpha))
    }
    /// New `Color` with HSL representation in sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue channel. [0.0, 360.0]
    /// * `saturation` - Saturation channel. [0.0, 1.0]
    /// * `lightness` - Lightness channel. [0.0, 1.0]
    ///
    /// See also [`Color::hsla`].
    ///
    pub const fn hsl(hue: f32, saturation: f32, lightness: f32) -> Color {
        Color::Hsla(Hsla::new_const(
            RgbHue::new(hue),
            saturation,
            lightness,
            1.0,
        ))
    }

    /// New `Color` with HSL representation in sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue channel. [0.0, 360.0]
    /// * `saturation` - Saturation channel. [0.0, 1.0]
    /// * `lightness` - Lightness channel. [0.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    ///
    /// See also [`Color::hsl`].
    ///
    pub const fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Color {
        Color::Hsla(Hsla::new_const(
            RgbHue::new(hue),
            saturation,
            lightness,
            alpha,
        ))
    }

    /// New `Color` with LCH representation in sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `lightness` - Lightness channel. [0.0, 1.5]
    /// * `chroma` - Chroma channel. [0.0, 1.5]
    /// * `hue` - Hue channel. [0.0, 360.0]
    ///
    /// See also [`Color::lcha`].
    pub const fn lch(lightness: f32, chroma: f32, hue: f32) -> Color {
        Color::Lcha(Lcha::new_const(lightness, chroma, LabHue::new(hue), 1.0))
    }

    /// New `Color` with LCH representation in sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `lightness` - Lightness channel. [0.0, 1.5]
    /// * `chroma` - Chroma channel. [0.0, 1.5]
    /// * `hue` - Hue channel. [0.0, 360.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    ///
    /// See also [`Color::lch`].
    pub const fn lcha(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Color {
        Color::Lcha(Lcha::new_const(lightness, chroma, LabHue::new(hue), alpha))
    }

    /// New `Color` from sRGB colorspace.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_render::color::Color;
    /// let color = Color::hex("FF00FF").unwrap(); // fuchsia
    /// let color = Color::hex("FF00FF7F").unwrap(); // partially transparent fuchsia
    ///
    /// // A standard hex color notation is also available
    /// assert_eq!(Color::hex("#FFFFFF").unwrap(), Color::rgb(1.0, 1.0, 1.0));
    /// ```
    ///
    pub fn hex<T: AsRef<str>>(hex: T) -> Result<Color, HexColorError> {
        let hex = hex.as_ref();
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match *hex.as_bytes() {
            // RGB
            [r, g, b] => {
                let [r, g, b, ..] = decode_hex([r, r, g, g, b, b])?;
                Ok(Color::rgb_u8(r, g, b))
            }
            // RGBA
            [r, g, b, a] => {
                let [r, g, b, a, ..] = decode_hex([r, r, g, g, b, b, a, a])?;
                Ok(Color::rgba_u8(r, g, b, a))
            }
            // RRGGBB
            [r1, r2, g1, g2, b1, b2] => {
                let [r, g, b, ..] = decode_hex([r1, r2, g1, g2, b1, b2])?;
                Ok(Color::rgb_u8(r, g, b))
            }
            // RRGGBBAA
            [r1, r2, g1, g2, b1, b2, a1, a2] => {
                let [r, g, b, a, ..] = decode_hex([r1, r2, g1, g2, b1, b2, a1, a2])?;
                Ok(Color::rgba_u8(r, g, b, a))
            }
            _ => Err(HexColorError::Length),
        }
    }

    /// New `Color` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    ///
    /// See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].
    ///
    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Color {
        Color::rgba_u8(r, g, b, u8::MAX)
    }

    // Float operations in const fn are not stable yet
    // see https://github.com/rust-lang/rust/issues/57241
    /// New `Color` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    /// * `a` - Alpha channel. [0, 255]
    ///
    /// See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].
    ///
    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::rgba(
            r as f32 / u8::MAX as f32,
            g as f32 / u8::MAX as f32,
            b as f32 / u8::MAX as f32,
            a as f32 / u8::MAX as f32,
        )
    }

    /// Get red in sRGB colorspace.
    pub fn r(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba(rgba) => rgba.red,
            _ => unreachable!(),
        }
    }

    /// Get green in sRGB colorspace.
    pub fn g(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba(rgba) => rgba.green,
            _ => unreachable!(),
        }
    }

    /// Get blue in sRGB colorspace.
    pub fn b(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba(rgba) => rgba.blue,
            _ => unreachable!(),
        }
    }

    /// Set red in sRGB colorspace.
    pub fn set_r(&mut self, r: f32) -> &mut Self {
        *self = self.as_rgba();
        match self {
            Color::Rgba(rgba) => rgba.red = r,
            _ => unreachable!(),
        }
        self
    }

    /// Returns this color with red set to a new value in sRGB colorspace.
    #[must_use]
    pub fn with_r(mut self, r: f32) -> Self {
        self.set_r(r);
        self
    }

    /// Set green in sRGB colorspace.
    pub fn set_g(&mut self, g: f32) -> &mut Self {
        *self = self.as_rgba();
        match self {
            Color::Rgba(rgba) => rgba.green = g,
            _ => unreachable!(),
        }
        self
    }

    /// Returns this color with green set to a new value in sRGB colorspace.
    #[must_use]
    pub fn with_g(mut self, g: f32) -> Self {
        self.set_g(g);
        self
    }

    /// Set blue in sRGB colorspace.
    pub fn set_b(&mut self, b: f32) -> &mut Self {
        *self = self.as_rgba();
        match self {
            Color::Rgba(rgba) => rgba.blue = b,
            _ => unreachable!(),
        }
        self
    }

    /// Returns this color with blue set to a new value in sRGB colorspace.
    #[must_use]
    pub fn with_b(mut self, b: f32) -> Self {
        self.set_b(b);
        self
    }

    /// Get alpha.
    #[inline(always)]
    pub fn a(&self) -> f32 {
        match self {
            Color::Rgba(c) => c.alpha,
            Color::RgbaLinear(c) => c.alpha,
            Color::Hsla(c) => c.alpha,
            Color::Lcha(c) => c.alpha,
        }
    }

    /// Set alpha.
    pub fn set_a(&mut self, a: f32) -> &mut Self {
        match self {
            Color::Rgba(c) => c.alpha = a,
            Color::RgbaLinear(c) => c.alpha = a,
            Color::Hsla(c) => c.alpha = a,
            Color::Lcha(c) => c.alpha = a,
        }
        self
    }

    /// Returns this color with a new alpha value.
    #[must_use]
    pub fn with_a(mut self, a: f32) -> Self {
        self.set_a(a);
        self
    }

    /// Converts a `Color` to variant `Color::Rgba`
    pub fn as_rgba(self: &Color) -> Color {
        match self {
            Color::Rgba(_) => *self,
            Color::RgbaLinear(linrgba) => Color::Rgba((*linrgba).into()),
            Color::Hsla(hsla) => Color::Rgba(Srgba::from_color_unclamped(*hsla)),
            Color::Lcha(lcha) => Color::Rgba(Srgba::from_color_unclamped(*lcha)),
        }
    }

    /// Converts a `Color` to variant `Color::RgbaLinear`
    pub fn as_rgba_linear(self: &Color) -> Color {
        match self {
            Color::Rgba(rgba) => Color::RgbaLinear((*rgba).into()),
            Color::RgbaLinear(_) => *self,
            Color::Hsla(hsla) => Color::RgbaLinear(Srgba::from_color_unclamped(*hsla).into()),
            Color::Lcha(lcha) => Color::RgbaLinear(Srgba::from_color_unclamped(*lcha).into()),
        }
    }

    /// Converts a `Color` to variant `Color::Hsla`
    pub fn as_hsla(self: &Color) -> Color {
        match self {
            Color::Rgba(rgba) => Color::Hsla(Hsla::from_color_unclamped(*rgba)),
            Color::RgbaLinear(rgbalinear) => {
                let rgba: Srgba = (*rgbalinear).into();
                Color::Hsla(Hsla::from_color_unclamped(rgba))
            }
            Color::Hsla(_) => *self,
            Color::Lcha(lcha) => Color::Hsla(Hsla::from_color_unclamped(*lcha)),
        }
    }

    /// Converts a `Color` to variant `Color::Lcha`
    pub fn as_lcha(self: &Color) -> Color {
        match self {
            Color::Rgba(rgba) => Color::Lcha(Lcha::from_color_unclamped(*rgba)),
            Color::RgbaLinear(rgbalinear) => {
                let rgba: Srgba = (*rgbalinear).into();
                Color::Lcha(Lcha::from_color_unclamped(rgba))
            }
            Color::Hsla(hsla) => Color::Lcha(Lcha::from_color_unclamped(*hsla)),
            Color::Lcha(_) => *self,
        }
    }

    /// Converts a `Color` to a `[f32; 4]` from sRGB colorspace
    pub fn as_rgba_f32(self: Color) -> [f32; 4] {
        let Color::Rgba(rgba) = self.as_rgba() else {unreachable!()};
        let (r, g, b, a) = rgba.into_components();
        [r, g, b, a]
    }

    /// Converts a `Color` to a `[f32; 4]` from linear RGB colorspace
    #[inline]
    pub fn as_linear_rgba_f32(self: Color) -> [f32; 4] {
        let Color::RgbaLinear(rgbaliner) = self.as_rgba_linear() else {unreachable!()};
        let (r, g, b, a) = rgbaliner.into_components();
        [r, g, b, a]
    }

    /// Converts a `Color` to a `[f32; 4]` from HSL colorspace
    pub fn as_hsla_f32(self: Color) -> [f32; 4] {
        let Color::Hsla(hsla) = self.as_hsla() else {unreachable!()};
        let (hue, saturation, lightness, a) = hsla.into_components();
        [hue.into_positive_degrees(), saturation, lightness, a]
    }

    /// Converts a `Color` to a `[f32; 4]` from LCH colorspace
    pub fn as_lcha_f32(self: Color) -> [f32; 4] {
        let Color::Lcha(lcha) = self.as_lcha() else {unreachable!()};
        let (lightness, colorfulness, hue, a) = lcha.into_components();
        [lightness, colorfulness, hue.into_positive_degrees(), a]
    }

    /// Converts `Color` to a `u32` from sRGB colorspace.
    ///
    /// Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
    /// `A` will be the most significant byte and `R` the least significant.
    pub fn as_rgba_u32(self: Color) -> u32 {
        let color = self.as_rgba_f32();
        u32::from_le_bytes([
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        ])
    }

    /// Converts Color to a u32 from linear RGB colorspace.
    ///
    /// Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
    /// `A` will be the most significant byte and `R` the least significant.
    pub fn as_linear_rgba_u32(self: Color) -> u32 {
        let color = self.as_linear_rgba_f32();
        u32::from_le_bytes([
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        ])
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        let Color::RgbaLinear(rgba_linear) = color.as_rgba_linear() else { unreachable!() };
        wgpu::Color {
            r: rgba_linear.red as f64,
            g: rgba_linear.green as f64,
            b: rgba_linear.blue as f64,
            a: rgba_linear.alpha as f64,
        }
    }
}

impl encase::ShaderType for Color {
    type ExtraMetadata = ();

    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = {
        let size =
            encase::private::SizeValue::from(<f32 as encase::private::ShaderSize>::SHADER_SIZE)
                .mul(4);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);

        encase::private::Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            extra: (),
        }
    };

    const UNIFORM_COMPAT_ASSERT: fn() = || {};
}

impl encase::private::WriteInto for Color {
    fn write_into<B: encase::private::BufferMut>(&self, writer: &mut encase::private::Writer<B>) {
        let linear = self.as_linear_rgba_f32();
        for el in &linear {
            encase::private::WriteInto::write_into(el, writer);
        }
    }
}

impl encase::private::ReadFrom for Color {
    fn read_from<B: encase::private::BufferRef>(
        &mut self,
        reader: &mut encase::private::Reader<B>,
    ) {
        let mut buffer = [0.0f32; 4];
        for el in &mut buffer {
            encase::private::ReadFrom::read_from(el, reader);
        }

        *self = Color::rgba_linear(buffer[0], buffer[1], buffer[2], buffer[3])
    }
}

impl encase::private::CreateFrom for Color {
    fn create_from<B>(reader: &mut encase::private::Reader<B>) -> Self
    where
        B: encase::private::BufferRef,
    {
        // These are intentionally not inlined in the constructor to make this
        // resilient to internal Color refactors / implicit type changes.
        let red: f32 = encase::private::CreateFrom::create_from(reader);
        let green: f32 = encase::private::CreateFrom::create_from(reader);
        let blue: f32 = encase::private::CreateFrom::create_from(reader);
        let alpha: f32 = encase::private::CreateFrom::create_from(reader);
        Color::rgba_linear(red, green, blue, alpha)
    }
}

impl encase::ShaderSize for Color {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HexColorError {
    #[error("Unexpected length of hex string")]
    Length,
    #[error("Invalid hex char")]
    Char(char),
}

/// Converts hex bytes to an array of RGB\[A\] components
///
/// # Example
/// For RGB: *b"ffffff" -> [255, 255, 255, ..]
/// For RGBA: *b"E2E2E2FF" -> [226, 226, 226, 255, ..]
const fn decode_hex<const N: usize>(mut bytes: [u8; N]) -> Result<[u8; N], HexColorError> {
    let mut i = 0;
    while i < bytes.len() {
        // Convert single hex digit to u8
        let val = match hex_value(bytes[i]) {
            Ok(val) => val,
            Err(byte) => return Err(HexColorError::Char(byte as char)),
        };
        bytes[i] = val;
        i += 1;
    }
    // Modify the original bytes to give an `N / 2` length result
    i = 0;
    while i < bytes.len() / 2 {
        // Convert pairs of u8 to R/G/B/A
        // e.g `ff` -> [102, 102] -> [15, 15] = 255
        bytes[i] = bytes[i * 2] * 16 + bytes[i * 2 + 1];
        i += 1;
    }
    Ok(bytes)
}

/// Parse a single hex digit (a-f/A-F/0-9) as a `u8`
const fn hex_value(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        // Wrong hex digit
        _ => Err(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color() {
        assert_eq!(Color::hex("FFF"), Ok(Color::WHITE));
        assert_eq!(Color::hex("FFFF"), Ok(Color::WHITE));
        assert_eq!(Color::hex("FFFFFF"), Ok(Color::WHITE));
        assert_eq!(Color::hex("FFFFFFFF"), Ok(Color::WHITE));
        assert_eq!(Color::hex("000"), Ok(Color::BLACK));
        assert_eq!(Color::hex("000F"), Ok(Color::BLACK));
        assert_eq!(Color::hex("000000"), Ok(Color::BLACK));
        assert_eq!(Color::hex("000000FF"), Ok(Color::BLACK));
        assert_eq!(Color::hex("03a9f4"), Ok(Color::rgb_u8(3, 169, 244)));
        assert_eq!(Color::hex("yy"), Err(HexColorError::Length));
        assert_eq!(Color::hex("yyy"), Err(HexColorError::Char('y')));
        assert_eq!(Color::hex("#f2a"), Ok(Color::rgb_u8(255, 34, 170)));
        assert_eq!(Color::hex("#e23030"), Ok(Color::rgb_u8(226, 48, 48)));
        assert_eq!(Color::hex("#ff"), Err(HexColorError::Length));
        assert_eq!(Color::hex("##fff"), Err(HexColorError::Char('#')));
    }

    // #[test]
    // fn conversions_vec4() {
    //     let starting_vec4 = Vec4::new(0.4, 0.5, 0.6, 1.0);
    //     let starting_color = Color::from(starting_vec4);

    //     assert_eq!(starting_vec4, Vec4::from(starting_color),);

    //     let transformation = Vec4::new(0.5, 0.5, 0.5, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::from(starting_vec4 * transformation),
    //     );
    // }

    // #[test]
    // fn mul_and_mulassign_f32() {
    //     let transformation = 0.5;
    //     let starting_color = Color::rgba(0.4, 0.5, 0.6, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::rgba(0.4 * 0.5, 0.5 * 0.5, 0.6 * 0.5, 1.0),
    //     );

    //     let mut mutated_color = starting_color;
    //     mutated_color *= transformation;

    //     assert_eq!(starting_color * transformation, mutated_color,);
    // }

    // #[test]
    // fn mul_and_mulassign_f32by3() {
    //     let transformation = [0.4, 0.5, 0.6];
    //     let starting_color = Color::rgba(0.4, 0.5, 0.6, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::rgba(0.4 * 0.4, 0.5 * 0.5, 0.6 * 0.6, 1.0),
    //     );

    //     let mut mutated_color = starting_color;
    //     mutated_color *= transformation;

    //     assert_eq!(starting_color * transformation, mutated_color,);
    // }

    // #[test]
    // fn mul_and_mulassign_f32by4() {
    //     let transformation = [0.4, 0.5, 0.6, 0.9];
    //     let starting_color = Color::rgba(0.4, 0.5, 0.6, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::rgba(0.4 * 0.4, 0.5 * 0.5, 0.6 * 0.6, 1.0 * 0.9),
    //     );

    //     let mut mutated_color = starting_color;
    //     mutated_color *= transformation;

    //     assert_eq!(starting_color * transformation, mutated_color,);
    // }

    // #[test]
    // fn mul_and_mulassign_vec3() {
    //     let transformation = Vec3::new(0.2, 0.3, 0.4);
    //     let starting_color = Color::rgba(0.4, 0.5, 0.6, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::rgba(0.4 * 0.2, 0.5 * 0.3, 0.6 * 0.4, 1.0),
    //     );

    //     let mut mutated_color = starting_color;
    //     mutated_color *= transformation;

    //     assert_eq!(starting_color * transformation, mutated_color,);
    // }

    // #[test]
    // fn mul_and_mulassign_vec4() {
    //     let transformation = Vec4::new(0.2, 0.3, 0.4, 0.5);
    //     let starting_color = Color::rgba(0.4, 0.5, 0.6, 1.0);

    //     assert_eq!(
    //         starting_color * transformation,
    //         Color::rgba(0.4 * 0.2, 0.5 * 0.3, 0.6 * 0.4, 1.0 * 0.5),
    //     );

    //     let mut mutated_color = starting_color;
    //     mutated_color *= transformation;

    //     assert_eq!(starting_color * transformation, mutated_color,);
    // }

    // regression test for https://github.com/bevyengine/bevy/pull/8040
    #[test]
    fn convert_to_rgba_linear() {
        let rgba = Color::rgba(0., 0., 0., 0.);
        let rgba_l = Color::rgba_linear(0., 0., 0., 0.);
        let hsla = Color::hsla(0., 0., 0., 0.);
        let lcha = Color::lcha(0., 0., 0., 0.);
        assert_eq!(rgba_l, rgba_l.as_rgba_linear());
        let Color::RgbaLinear { .. } = rgba.as_rgba_linear() else { panic!("from Rgba") };
        let Color::RgbaLinear { .. } = hsla.as_rgba_linear() else { panic!("from Hsla") };
        let Color::RgbaLinear { .. } = lcha.as_rgba_linear() else { panic!("from Lcha") };
    }
}

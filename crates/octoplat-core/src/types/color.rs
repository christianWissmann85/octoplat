use serde::{Deserialize, Serialize};

/// An RGBA color with components in the range 0.0 to 1.0.
/// Replaces macroquad::Color for use in core logic.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    // Common color constants
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const ORANGE: Self = Self::rgb(1.0, 0.5, 0.0);
    pub const PURPLE: Self = Self::rgb(0.5, 0.0, 0.5);
    pub const PINK: Self = Self::rgb(1.0, 0.75, 0.8);
    pub const BROWN: Self = Self::rgb(0.6, 0.3, 0.0);
    pub const GRAY: Self = Self::rgb(0.5, 0.5, 0.5);
    pub const DARK_GRAY: Self = Self::rgb(0.25, 0.25, 0.25);
    pub const LIGHT_GRAY: Self = Self::rgb(0.75, 0.75, 0.75);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    /// Creates a new color with the given RGBA components.
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new color with the given RGB components and full opacity.
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color with the given RGBA components.
    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a color from 8-bit RGBA components (0-255).
    #[inline]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Creates a color from a hex string (e.g., "#FF5500" or "FF5500").
    /// Supports 3, 4, 6, and 8 character hex strings (RGB, RGBA, RRGGBB, RRGGBBAA).
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // RGB shorthand
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                Some(Self::from_rgba8(r * 17, g * 17, b * 17, 255))
            }
            4 => {
                // RGBA shorthand
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
                Some(Self::from_rgba8(r * 17, g * 17, b * 17, a * 17))
            }
            6 => {
                // RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::from_rgba8(r, g, b, 255))
            }
            8 => {
                // RRGGBBAA
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::from_rgba8(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Converts the color to a hex string in RRGGBB format.
    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    /// Converts the color to a hex string in RRGGBBAA format.
    #[inline]
    pub fn to_hex_rgba(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8
        )
    }

    /// Returns a tuple of 8-bit RGBA components.
    #[inline]
    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    /// Returns the color with a new alpha value.
    #[inline]
    pub const fn with_alpha(&self, a: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }

    /// Linearly interpolates between two colors.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Lightens the color by the given factor (0.0 to 1.0).
    #[inline]
    pub fn lighten(&self, factor: f32) -> Self {
        Self {
            r: (self.r + (1.0 - self.r) * factor).min(1.0),
            g: (self.g + (1.0 - self.g) * factor).min(1.0),
            b: (self.b + (1.0 - self.b) * factor).min(1.0),
            a: self.a,
        }
    }

    /// Darkens the color by the given factor (0.0 to 1.0).
    #[inline]
    pub fn darken(&self, factor: f32) -> Self {
        Self {
            r: (self.r * (1.0 - factor)).max(0.0),
            g: (self.g * (1.0 - factor)).max(0.0),
            b: (self.b * (1.0 - factor)).max(0.0),
            a: self.a,
        }
    }

    /// Creates a grayscale version of the color.
    #[inline]
    pub fn grayscale(&self) -> Self {
        let gray = 0.299 * self.r + 0.587 * self.g + 0.114 * self.b;
        Self {
            r: gray,
            g: gray,
            b: gray,
            a: self.a,
        }
    }

    /// Creates a color from HSL values.
    /// h: hue (0.0 to 1.0), s: saturation (0.0 to 1.0), l: lightness (0.0 to 1.0)
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        if s == 0.0 {
            return Self::rgb(l, l, l);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 0.5 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            p
        }

        Self::rgb(
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    }

    /// Converts the color to HSL values.
    /// Returns (h, s, l) where all values are in the range 0.0 to 1.0.
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let l = (max + min) / 2.0;

        if max == min {
            return (0.0, 0.0, l);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if max == self.r {
            (self.g - self.b) / d + (if self.g < self.b { 6.0 } else { 0.0 })
        } else if max == self.g {
            (self.b - self.r) / d + 2.0
        } else {
            (self.r - self.g) / d + 4.0
        };

        (h / 6.0, s, l)
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    #[inline]
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<(f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self { r, g, b, a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        let c = Color::rgb(1.0, 0.5, 0.0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.5);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_from_rgba8() {
        let c = Color::from_rgba8(255, 128, 0, 255);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex() {
        let c = Color::from_hex("#FF8000").unwrap();
        assert_eq!(c.r, 1.0);
        assert!((c.g - 0.5).abs() < 0.01);
        assert_eq!(c.b, 0.0);

        let c2 = Color::from_hex("F80").unwrap();
        assert_eq!(c2.r, 1.0);
        assert!((c2.g - 0.533).abs() < 0.01);
        assert_eq!(c2.b, 0.0);
    }

    #[test]
    fn test_to_hex() {
        let c = Color::rgb(1.0, 0.5, 0.0);
        assert_eq!(c.to_hex(), "FF7F00");
    }

    #[test]
    fn test_lerp() {
        let a = Color::BLACK;
        let b = Color::WHITE;
        let c = a.lerp(&b, 0.5);
        assert!((c.r - 0.5).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert!((c.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_serde() {
        let c = Color::rgb(0.1, 0.2, 0.3);
        let json = serde_json::to_string(&c).unwrap();
        let c2: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(c, c2);
    }

    #[test]
    fn test_hsl_roundtrip() {
        let c = Color::rgb(0.8, 0.2, 0.5);
        let (h, s, l) = c.to_hsl();
        let c2 = Color::from_hsl(h, s, l);
        assert!((c.r - c2.r).abs() < 0.01);
        assert!((c.g - c2.g).abs() < 0.01);
        assert!((c.b - c2.b).abs() < 0.01);
    }
}

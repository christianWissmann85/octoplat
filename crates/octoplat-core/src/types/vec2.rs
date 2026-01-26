use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 2D vector with x and y components.
/// Replaces macroquad::Vec2 for use in core logic.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// All ones.
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// Unit vector pointing up (negative Y).
    pub const UP: Self = Self { x: 0.0, y: -1.0 };

    /// Unit vector pointing down (positive Y).
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };

    /// Unit vector pointing left (negative X).
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };

    /// Unit vector pointing right (positive X).
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    /// Creates a new Vec2 with the given x and y components.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a Vec2 with both components set to the same value.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Computes the squared length of the vector.
    /// Faster than `length()` when you only need to compare lengths.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns a normalized (unit length) version of the vector.
    /// Returns ZERO if the vector has zero length.
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Self::ZERO
        }
    }

    /// Returns a normalized vector, or None if the vector has zero length.
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        if len > 0.0 {
            Some(self / len)
        } else {
            None
        }
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the perpendicular vector (rotated 90 degrees counter-clockwise).
    #[inline]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Computes the distance to another point.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Computes the squared distance to another point.
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Linearly interpolates between two vectors.
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Returns a vector with the absolute values of each component.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector with each component clamped to the given range.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /// Returns the component-wise minimum of two vectors.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the component-wise maximum of two vectors.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Returns a vector with each component rounded to the nearest integer.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Returns a vector with each component floored.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Returns a vector with each component ceiled.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Returns the angle of the vector in radians.
    #[inline]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Creates a unit vector from an angle in radians.
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Rotates the vector by the given angle in radians.
    #[inline]
    pub fn rotate(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

/// Convenience function to create a Vec2.
#[inline]
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: vec.x * self,
            y: vec.y * self,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let v = Vec2::new(3.0, 4.0).normalize();
        assert!((v.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((a.dot(b) - 11.0).abs() < 1e-6);
    }

    #[test]
    fn test_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c.x, 4.0);
        assert_eq!(c.y, 6.0);
    }

    #[test]
    fn test_lerp() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 10.0);
        let c = a.lerp(b, 0.5);
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 5.0);
    }

    #[test]
    fn test_serde() {
        let v = Vec2::new(1.5, 2.5);
        let json = serde_json::to_string(&v).unwrap();
        let v2: Vec2 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, v2);
    }
}

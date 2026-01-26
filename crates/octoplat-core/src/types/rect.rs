use super::Vec2;
use serde::{Deserialize, Serialize};

/// An axis-aligned bounding box (AABB) rectangle.
/// Replaces macroquad::Rect for use in core logic.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// Creates a new rectangle with the given position and size.
    #[inline]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Creates a rectangle from a position Vec2 and size Vec2.
    #[inline]
    pub fn from_vecs(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }

    /// Returns the top-left corner of the rectangle.
    #[inline]
    pub fn point(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Returns the size of the rectangle as a Vec2.
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    /// Returns the center of the rectangle.
    #[inline]
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }

    /// Returns the left edge x coordinate.
    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Returns the right edge x coordinate.
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Returns the top edge y coordinate.
    #[inline]
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Returns the bottom edge y coordinate.
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    /// Checks if this rectangle overlaps with another.
    #[inline]
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    /// Checks if this rectangle contains a point.
    #[inline]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x < self.x + self.w
            && point.y >= self.y
            && point.y < self.y + self.h
    }

    /// Checks if this rectangle fully contains another rectangle.
    #[inline]
    pub fn contains_rect(&self, other: &Rect) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.x + other.w <= self.x + self.w
            && other.y + other.h <= self.y + self.h
    }

    /// Returns the intersection of two rectangles, or None if they don't overlap.
    #[inline]
    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.w).min(other.x + other.w);
        let bottom = (self.y + self.h).min(other.y + other.h);

        if right > x && bottom > y {
            Some(Rect::new(x, y, right - x, bottom - y))
        } else {
            None
        }
    }

    /// Returns the smallest rectangle that contains both rectangles.
    #[inline]
    pub fn combine(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.w).max(other.x + other.w);
        let bottom = (self.y + self.h).max(other.y + other.h);
        Rect::new(x, y, right - x, bottom - y)
    }

    /// Moves the rectangle by the given offset.
    #[inline]
    pub fn offset(&self, offset: Vec2) -> Rect {
        Rect::new(self.x + offset.x, self.y + offset.y, self.w, self.h)
    }

    /// Grows the rectangle by the given amount on all sides.
    #[inline]
    pub fn grow(&self, amount: f32) -> Rect {
        Rect::new(
            self.x - amount,
            self.y - amount,
            self.w + amount * 2.0,
            self.h + amount * 2.0,
        )
    }

    /// Shrinks the rectangle by the given amount on all sides.
    #[inline]
    pub fn shrink(&self, amount: f32) -> Rect {
        self.grow(-amount)
    }

    /// Returns the area of the rectangle.
    #[inline]
    pub fn area(&self) -> f32 {
        self.w * self.h
    }

    /// Scales the rectangle size by the given factor, keeping the same top-left position.
    #[inline]
    pub fn scale(&self, factor: f32) -> Rect {
        Rect::new(self.x, self.y, self.w * factor, self.h * factor)
    }

    /// Scales the rectangle size by the given factor, keeping the center in place.
    #[inline]
    pub fn scale_from_center(&self, factor: f32) -> Rect {
        let new_w = self.w * factor;
        let new_h = self.h * factor;
        Rect::new(
            self.x + (self.w - new_w) * 0.5,
            self.y + (self.h - new_h) * 0.5,
            new_w,
            new_h,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Rect::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(r.x, 10.0);
        assert_eq!(r.y, 20.0);
        assert_eq!(r.w, 30.0);
        assert_eq!(r.h, 40.0);
    }

    #[test]
    fn test_center() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        let c = r.center();
        assert_eq!(c.x, 50.0);
        assert_eq!(c.y, 50.0);
    }

    #[test]
    fn test_overlaps() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 10.0, 10.0);
        let c = Rect::new(20.0, 20.0, 10.0, 10.0);

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        assert!(!a.overlaps(&c));
        assert!(!c.overlaps(&a));
    }

    #[test]
    fn test_contains_point() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(Vec2::new(5.0, 5.0)));
        assert!(r.contains(Vec2::new(0.0, 0.0)));
        assert!(!r.contains(Vec2::new(10.0, 10.0)));
        assert!(!r.contains(Vec2::new(-1.0, 5.0)));
    }

    #[test]
    fn test_intersect() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 10.0, 10.0);

        let intersection = a.intersect(&b).unwrap();
        assert_eq!(intersection.x, 5.0);
        assert_eq!(intersection.y, 5.0);
        assert_eq!(intersection.w, 5.0);
        assert_eq!(intersection.h, 5.0);
    }

    #[test]
    fn test_serde() {
        let r = Rect::new(1.0, 2.0, 3.0, 4.0);
        let json = serde_json::to_string(&r).unwrap();
        let r2: Rect = serde_json::from_str(&json).unwrap();
        assert_eq!(r, r2);
    }
}

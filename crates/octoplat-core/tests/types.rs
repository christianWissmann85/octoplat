//! Integration tests for core types (Vec2, Rect, Color)

use octoplat_core::{vec2, Color, Rect, Vec2};
use serde_json;

// =============================================================================
// Vec2 Tests
// =============================================================================

#[test]
fn test_vec2_creation() {
    let v = Vec2::new(3.0, 4.0);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 4.0);
}

#[test]
fn test_vec2_macro() {
    let v = vec2(1.0, 2.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}

#[test]
fn test_vec2_zero() {
    let v = Vec2::ZERO;
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}

#[test]
fn test_vec2_one() {
    let v = Vec2::ONE;
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
}

#[test]
fn test_vec2_add() {
    let a = vec2(1.0, 2.0);
    let b = vec2(3.0, 4.0);
    let c = a + b;
    assert_eq!(c.x, 4.0);
    assert_eq!(c.y, 6.0);
}

#[test]
fn test_vec2_sub() {
    let a = vec2(5.0, 7.0);
    let b = vec2(2.0, 3.0);
    let c = a - b;
    assert_eq!(c.x, 3.0);
    assert_eq!(c.y, 4.0);
}

#[test]
fn test_vec2_mul_scalar() {
    let v = vec2(2.0, 3.0);
    let result = v * 2.0;
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 6.0);
}

#[test]
fn test_vec2_div_scalar() {
    let v = vec2(4.0, 6.0);
    let result = v / 2.0;
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 3.0);
}

#[test]
fn test_vec2_length() {
    let v = vec2(3.0, 4.0);
    assert!((v.length() - 5.0).abs() < 0.0001);
}

#[test]
fn test_vec2_length_squared() {
    let v = vec2(3.0, 4.0);
    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn test_vec2_normalize() {
    let v = vec2(3.0, 4.0);
    let n = v.normalize();
    assert!((n.length() - 1.0).abs() < 0.0001);
    assert!((n.x - 0.6).abs() < 0.0001);
    assert!((n.y - 0.8).abs() < 0.0001);
}

#[test]
fn test_vec2_normalize_zero() {
    let v = Vec2::ZERO;
    let n = v.normalize();
    // Normalizing zero vector should return zero
    assert_eq!(n.x, 0.0);
    assert_eq!(n.y, 0.0);
}

#[test]
fn test_vec2_dot() {
    let a = vec2(1.0, 2.0);
    let b = vec2(3.0, 4.0);
    assert_eq!(a.dot(b), 11.0);
}

#[test]
fn test_vec2_distance() {
    let a = vec2(0.0, 0.0);
    let b = vec2(3.0, 4.0);
    assert!((a.distance(b) - 5.0).abs() < 0.0001);
}

#[test]
fn test_vec2_lerp() {
    let a = vec2(0.0, 0.0);
    let b = vec2(10.0, 10.0);

    let mid = a.lerp(b, 0.5);
    assert_eq!(mid.x, 5.0);
    assert_eq!(mid.y, 5.0);

    let start = a.lerp(b, 0.0);
    assert_eq!(start.x, 0.0);

    let end = a.lerp(b, 1.0);
    assert_eq!(end.x, 10.0);
}

#[test]
fn test_vec2_neg() {
    let v = vec2(3.0, -4.0);
    let neg = -v;
    assert_eq!(neg.x, -3.0);
    assert_eq!(neg.y, 4.0);
}

#[test]
fn test_vec2_add_assign() {
    let mut v = vec2(1.0, 2.0);
    v += vec2(3.0, 4.0);
    assert_eq!(v.x, 4.0);
    assert_eq!(v.y, 6.0);
}

#[test]
fn test_vec2_sub_assign() {
    let mut v = vec2(5.0, 7.0);
    v -= vec2(2.0, 3.0);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 4.0);
}

#[test]
fn test_vec2_mul_assign() {
    let mut v = vec2(2.0, 3.0);
    v *= 2.0;
    assert_eq!(v.x, 4.0);
    assert_eq!(v.y, 6.0);
}

#[test]
fn test_vec2_serialization() {
    let v = vec2(3.5, 4.5);
    let json = serde_json::to_string(&v).expect("serialize");
    let v2: Vec2 = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(v.x, v2.x);
    assert_eq!(v.y, v2.y);
}

// =============================================================================
// Rect Tests
// =============================================================================

#[test]
fn test_rect_creation() {
    let r = Rect::new(10.0, 20.0, 100.0, 50.0);
    assert_eq!(r.x, 10.0);
    assert_eq!(r.y, 20.0);
    assert_eq!(r.w, 100.0);
    assert_eq!(r.h, 50.0);
}

#[test]
fn test_rect_contains_point() {
    let r = Rect::new(0.0, 0.0, 100.0, 100.0);
    assert!(r.contains(vec2(50.0, 50.0)));
    assert!(r.contains(vec2(0.0, 0.0)));
    assert!(!r.contains(vec2(-1.0, 50.0)));
    assert!(!r.contains(vec2(101.0, 50.0)));
}

#[test]
fn test_rect_overlaps() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(50.0, 50.0, 100.0, 100.0);
    assert!(a.overlaps(&b));

    let c = Rect::new(200.0, 200.0, 10.0, 10.0);
    assert!(!a.overlaps(&c));
}

#[test]
fn test_rect_intersect() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(50.0, 50.0, 100.0, 100.0);

    let intersection = a.intersect(&b);
    assert!(intersection.is_some());

    let i = intersection.unwrap();
    assert_eq!(i.x, 50.0);
    assert_eq!(i.y, 50.0);
    assert_eq!(i.w, 50.0);
    assert_eq!(i.h, 50.0);
}

#[test]
fn test_rect_no_intersection() {
    let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    let b = Rect::new(100.0, 100.0, 10.0, 10.0);

    assert!(a.intersect(&b).is_none());
}

#[test]
fn test_rect_center() {
    let r = Rect::new(0.0, 0.0, 100.0, 100.0);
    let center = r.center();
    assert_eq!(center.x, 50.0);
    assert_eq!(center.y, 50.0);
}

#[test]
fn test_rect_right_bottom() {
    let r = Rect::new(10.0, 20.0, 100.0, 50.0);
    assert_eq!(r.right(), 110.0);
    assert_eq!(r.bottom(), 70.0);
}

#[test]
fn test_rect_serialization() {
    let r = Rect::new(10.0, 20.0, 30.0, 40.0);
    let json = serde_json::to_string(&r).expect("serialize");
    let r2: Rect = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(r.x, r2.x);
    assert_eq!(r.y, r2.y);
    assert_eq!(r.w, r2.w);
    assert_eq!(r.h, r2.h);
}

// =============================================================================
// Color Tests
// =============================================================================

#[test]
fn test_color_creation() {
    let c = Color::new(1.0, 0.5, 0.25, 0.8);
    assert_eq!(c.r, 1.0);
    assert_eq!(c.g, 0.5);
    assert_eq!(c.b, 0.25);
    assert_eq!(c.a, 0.8);
}

#[test]
fn test_color_from_rgba8() {
    let c = Color::from_rgba8(255, 128, 64, 255);
    assert!((c.r - 1.0).abs() < 0.01);
    assert!((c.g - 0.5).abs() < 0.01);
    assert!((c.b - 0.25).abs() < 0.01);
    assert_eq!(c.a, 1.0);
}

#[test]
fn test_color_from_hex() {
    let c = Color::from_hex("#FF8040").expect("valid hex");
    assert!((c.r - 1.0).abs() < 0.01);
    assert!((c.g - 0.5).abs() < 0.01);
    assert!((c.b - 0.25).abs() < 0.01);
}

#[test]
fn test_color_from_hex_short() {
    let c = Color::from_hex("F80").expect("valid short hex");
    assert!((c.r - 1.0).abs() < 0.01);
    assert!((c.g - 0.53).abs() < 0.01);
    assert!((c.b - 0.0).abs() < 0.01);
}

#[test]
fn test_color_from_hex_invalid() {
    assert!(Color::from_hex("invalid").is_none());
    assert!(Color::from_hex("GGHHII").is_none());
}

#[test]
fn test_color_with_alpha() {
    let c = Color::new(1.0, 0.5, 0.25, 1.0);
    let c2 = c.with_alpha(0.5);
    assert_eq!(c2.r, 1.0);
    assert_eq!(c2.g, 0.5);
    assert_eq!(c2.b, 0.25);
    assert_eq!(c2.a, 0.5);
}

#[test]
fn test_color_lighten() {
    let c = Color::new(0.5, 0.5, 0.5, 1.0);
    let lighter = c.lighten(0.5);
    assert!(lighter.r > c.r);
    assert!(lighter.g > c.g);
    assert!(lighter.b > c.b);
}

#[test]
fn test_color_darken() {
    let c = Color::new(0.5, 0.5, 0.5, 1.0);
    let darker = c.darken(0.5);
    assert!(darker.r < c.r);
    assert!(darker.g < c.g);
    assert!(darker.b < c.b);
}

#[test]
fn test_color_serialization() {
    let c = Color::new(1.0, 0.5, 0.25, 0.8);
    let json = serde_json::to_string(&c).expect("serialize");
    let c2: Color = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(c.r, c2.r);
    assert_eq!(c.g, c2.g);
    assert_eq!(c.b, c2.b);
    assert_eq!(c.a, c2.a);
}

#[test]
fn test_color_constants() {
    // Just verify they exist and have expected values
    assert_eq!(Color::WHITE.r, 1.0);
    assert_eq!(Color::BLACK.r, 0.0);
    assert_eq!(Color::RED.r, 1.0);
    assert_eq!(Color::GREEN.g, 1.0);
    assert_eq!(Color::BLUE.b, 1.0);
}

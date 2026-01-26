//! Integration tests for collision utilities

use octoplat_game::{aabb_collision, CollisionResult, Hitbox};
use macroquad::prelude::*;

// =============================================================================
// Hitbox Tests
// =============================================================================

#[test]
fn test_hitbox_creation() {
    let hitbox = Hitbox::new(24.0, 30.0);
    assert_eq!(hitbox.width, 24.0);
    assert_eq!(hitbox.height, 30.0);
}

#[test]
fn test_hitbox_dimensions() {
    let hitbox = Hitbox::new(20.0, 40.0);
    // Width and height are directly accessible
    assert_eq!(hitbox.width, 20.0);
    assert_eq!(hitbox.height, 40.0);
}

#[test]
fn test_hitbox_to_rect() {
    let hitbox = Hitbox::new(24.0, 30.0);
    let pos = vec2(100.0, 100.0);
    let rect = hitbox.to_rect(pos);

    // Rect should be centered on position
    assert_eq!(rect.w, 24.0);
    assert_eq!(rect.h, 30.0);
    // X should be position - half width
    assert_eq!(rect.x, 100.0 - 12.0);
    assert_eq!(rect.y, 100.0 - 15.0);
}

// =============================================================================
// AABB Collision Tests
// =============================================================================

#[test]
fn test_aabb_collision_overlapping() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(50.0, 50.0, 100.0, 100.0);
    let result = aabb_collision(a, b);
    assert!(result.collided);
}

#[test]
fn test_aabb_collision_not_overlapping() {
    let a = Rect::new(0.0, 0.0, 50.0, 50.0);
    let b = Rect::new(100.0, 100.0, 50.0, 50.0);
    let result = aabb_collision(a, b);
    assert!(!result.collided);
}

#[test]
fn test_aabb_collision_touching() {
    let a = Rect::new(0.0, 0.0, 50.0, 50.0);
    let b = Rect::new(50.0, 0.0, 50.0, 50.0);
    let result = aabb_collision(a, b);
    // Touching (no overlap) should not collide
    assert!(!result.collided);
}

#[test]
fn test_aabb_collision_contained() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(25.0, 25.0, 50.0, 50.0);
    let result = aabb_collision(a, b);
    assert!(result.collided);
}

#[test]
fn test_aabb_collision_has_penetration() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(50.0, 50.0, 100.0, 100.0);
    let result = aabb_collision(a, b);
    assert!(result.collided);
    assert!(result.penetration > 0.0);
}

#[test]
fn test_aabb_collision_has_normal() {
    let a = Rect::new(0.0, 0.0, 100.0, 100.0);
    let b = Rect::new(50.0, 50.0, 100.0, 100.0);
    let result = aabb_collision(a, b);
    assert!(result.collided);
    // Normal should be non-zero when colliding
    assert!(result.normal.x != 0.0 || result.normal.y != 0.0);
}

// =============================================================================
// CollisionResult Tests
// =============================================================================

#[test]
fn test_collision_result_default() {
    let result = CollisionResult::default();
    assert!(!result.collided);
    assert_eq!(result.penetration, 0.0);
}

#[test]
fn test_collision_result_has_fields() {
    let result = CollisionResult {
        collided: true,
        normal: vec2(1.0, 0.0),
        penetration: 5.0,
    };
    assert!(result.collided);
    assert_eq!(result.normal.x, 1.0);
    assert_eq!(result.penetration, 5.0);
}

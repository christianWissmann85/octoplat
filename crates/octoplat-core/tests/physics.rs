//! Integration tests for physics and collision

use octoplat_core::physics::{aabb_collision, check_ground, check_wall, CollisionResult, Hitbox};
use octoplat_core::{vec2, Rect};

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
fn test_hitbox_to_rect() {
    let hitbox = Hitbox::new(24.0, 30.0);
    let pos = vec2(100.0, 100.0);
    let rect = hitbox.to_rect(pos);

    // Rect should be centered on position
    assert_eq!(rect.x, 100.0 - 12.0);
    assert_eq!(rect.y, 100.0 - 15.0);
    assert_eq!(rect.w, 24.0);
    assert_eq!(rect.h, 30.0);
}

#[test]
fn test_hitbox_dimensions_via_rect() {
    let hitbox = Hitbox::new(24.0, 30.0);
    let pos = vec2(50.0, 50.0);
    let rect = hitbox.to_rect(pos);

    // Verify the rect dimensions
    assert_eq!(rect.w, 24.0);
    assert_eq!(rect.h, 30.0);
    // Verify centering (half extents are width/2 and height/2)
    assert_eq!(rect.x, 50.0 - 12.0);
    assert_eq!(rect.y, 50.0 - 15.0);
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
// Ground Check Tests
// =============================================================================

#[test]
fn test_check_ground_on_platform() {
    let player_rect = Rect::new(50.0, 50.0, 24.0, 30.0);
    let tiles = vec![
        Rect::new(0.0, 80.0, 200.0, 32.0), // Platform below player
    ];

    let result = check_ground(player_rect, &tiles);
    assert!(result, "Player should be on ground");
}

#[test]
fn test_check_ground_in_air() {
    let player_rect = Rect::new(50.0, 50.0, 24.0, 30.0);
    let tiles = vec![
        Rect::new(0.0, 200.0, 200.0, 32.0), // Platform far below player
    ];

    let result = check_ground(player_rect, &tiles);
    assert!(!result, "Player should be in air");
}

#[test]
fn test_check_ground_no_tiles() {
    let player_rect = Rect::new(50.0, 50.0, 24.0, 30.0);
    let tiles: Vec<Rect> = vec![];

    let result = check_ground(player_rect, &tiles);
    assert!(!result, "Player should be in air with no tiles");
}

// =============================================================================
// Wall Check Tests
// =============================================================================

#[test]
fn test_check_wall_touching_left() {
    let player_rect = Rect::new(32.0, 50.0, 24.0, 30.0);
    let tiles = vec![
        Rect::new(0.0, 0.0, 32.0, 200.0), // Wall to the left
    ];

    let result = check_wall(player_rect, &tiles);
    assert_eq!(result, -1, "Should detect wall on left");
}

#[test]
fn test_check_wall_touching_right() {
    let player_rect = Rect::new(50.0, 50.0, 24.0, 30.0);
    let tiles = vec![
        Rect::new(74.0, 0.0, 32.0, 200.0), // Wall to the right
    ];

    let result = check_wall(player_rect, &tiles);
    assert_eq!(result, 1, "Should detect wall on right");
}

#[test]
fn test_check_wall_no_wall() {
    let player_rect = Rect::new(100.0, 50.0, 24.0, 30.0);
    let tiles = vec![
        Rect::new(0.0, 0.0, 32.0, 200.0), // Wall far to the left
    ];

    let result = check_wall(player_rect, &tiles);
    assert_eq!(result, 0, "Should not detect wall");
}

#[test]
fn test_check_wall_empty_tiles() {
    let player_rect = Rect::new(50.0, 50.0, 24.0, 30.0);
    let tiles: Vec<Rect> = vec![];

    let result = check_wall(player_rect, &tiles);
    assert_eq!(result, 0, "Should not detect wall with no tiles");
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

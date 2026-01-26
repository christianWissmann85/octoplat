//! Integration tests for platform mechanics

use octoplat_game::{CrumblingPlatform, CrumblingState, MovingPlatform};
use macroquad::prelude::*;

// =============================================================================
// MovingPlatform Tests
// =============================================================================

#[test]
fn test_moving_platform_creation() {
    let platform = MovingPlatform::new(
        vec2(100.0, 100.0),  // start
        vec2(200.0, 100.0),  // end
        vec2(32.0, 8.0),     // size
    );

    assert_eq!(platform.start, vec2(100.0, 100.0));
    assert_eq!(platform.end, vec2(200.0, 100.0));
    assert_eq!(platform.size, vec2(32.0, 8.0));
}

#[test]
fn test_moving_platform_initial_position() {
    let platform = MovingPlatform::new(
        vec2(100.0, 100.0),
        vec2(200.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Should start at the start position
    assert_eq!(platform.position, vec2(100.0, 100.0));
}

#[test]
fn test_moving_platform_collision_rect() {
    let platform = MovingPlatform::new(
        vec2(100.0, 100.0),
        vec2(200.0, 100.0),
        vec2(32.0, 8.0),
    );

    let rect = platform.collision_rect();
    // Rect should be centered on position
    assert_eq!(rect.w, 32.0);
    assert_eq!(rect.h, 8.0);
}

#[test]
fn test_moving_platform_velocity_starts_zero() {
    let platform = MovingPlatform::new(
        vec2(100.0, 100.0),
        vec2(200.0, 100.0),
        vec2(32.0, 8.0),
    );

    assert_eq!(platform.velocity, Vec2::ZERO);
}

#[test]
fn test_moving_platform_vertical() {
    let platform = MovingPlatform::new(
        vec2(100.0, 0.0),    // start at top
        vec2(100.0, 100.0),  // end at bottom
        vec2(32.0, 8.0),
    );

    // Should handle vertical movement
    assert_eq!(platform.start.y, 0.0);
    assert_eq!(platform.end.y, 100.0);
}

#[test]
fn test_moving_platform_diagonal() {
    let platform = MovingPlatform::new(
        vec2(0.0, 0.0),
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Should handle diagonal movement
    assert_ne!(platform.start.x, platform.end.x);
    assert_ne!(platform.start.y, platform.end.y);
}

// =============================================================================
// CrumblingPlatform Tests
// =============================================================================

#[test]
fn test_crumbling_platform_creation() {
    let platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),  // position
        vec2(32.0, 8.0),     // size
    );

    assert_eq!(platform.position, vec2(100.0, 100.0));
    assert_eq!(platform.size, vec2(32.0, 8.0));
}

#[test]
fn test_crumbling_platform_initial_state() {
    let platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    assert!(matches!(platform.state, CrumblingState::Stable));
}

#[test]
fn test_crumbling_platform_start_position() {
    let platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Start position should match initial position
    assert_eq!(platform.start_position, vec2(100.0, 100.0));
}

#[test]
fn test_crumbling_platform_timer_initial() {
    let platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    assert_eq!(platform.timer, 0.0);
}

// =============================================================================
// CrumblingState Tests
// =============================================================================

#[test]
fn test_crumbling_state_variants() {
    // All state variants exist
    let _stable = CrumblingState::Stable;
    let _shaking = CrumblingState::Shaking;
    let _falling = CrumblingState::Falling;
    let _respawning = CrumblingState::Respawning;
}

#[test]
fn test_crumbling_state_equality() {
    assert_eq!(CrumblingState::Stable, CrumblingState::Stable);
    assert_ne!(CrumblingState::Stable, CrumblingState::Shaking);
}

#[test]
fn test_crumbling_state_debug() {
    // CrumblingState should implement Debug
    let state = CrumblingState::Stable;
    let _ = format!("{:?}", state);
}

//! Integration tests for player movement and abilities

use macroquad::prelude::*;
use octoplat_game::{GameConfig, Hitbox, Player, PlayerState};

// =============================================================================
// Player Creation Tests
// =============================================================================

#[test]
fn test_player_creation() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.position, vec2(100.0, 100.0));
    assert_eq!(player.velocity, Vec2::ZERO);
}

#[test]
fn test_player_initial_state() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player starts in Falling state (gravity will be applied)
    assert_eq!(player.state, PlayerState::Falling);
}

#[test]
fn test_player_initial_facing() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player starts facing right
    assert!(player.facing_right);
}

#[test]
fn test_player_hitbox_matches_config() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.hitbox.width, config.player_hitbox.x);
    assert_eq!(player.hitbox.height, config.player_hitbox.y);
}

// =============================================================================
// PlayerState Tests
// =============================================================================

#[test]
fn test_player_state_variants() {
    // All state variants exist and are accessible
    let _idle = PlayerState::Idle;
    let _running = PlayerState::Running;
    let _jumping = PlayerState::Jumping;
    let _falling = PlayerState::Falling;
    let _wall_grip = PlayerState::WallGrip;
    let _jet_boosting = PlayerState::JetBoosting;
    let _swinging = PlayerState::Swinging;
}

#[test]
fn test_player_state_equality() {
    assert_eq!(PlayerState::Idle, PlayerState::Idle);
    assert_ne!(PlayerState::Idle, PlayerState::Running);
    assert_ne!(PlayerState::Jumping, PlayerState::Falling);
}

#[test]
fn test_player_state_default() {
    let state = PlayerState::default();
    assert_eq!(state, PlayerState::Idle);
}

#[test]
fn test_player_state_debug() {
    let state = PlayerState::Jumping;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Jumping"));
}

// =============================================================================
// Wall Mechanics Tests
// =============================================================================

#[test]
fn test_player_initial_wall_stamina() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Wall stamina starts at max
    assert_eq!(player.wall_stamina, config.wall_stamina_max);
}

#[test]
fn test_player_initial_wall_jumps() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Wall jumps start at max
    assert_eq!(player.wall_jumps_remaining, config.wall_jumps_max);
}

#[test]
fn test_player_wall_direction_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // No wall direction initially
    assert_eq!(player.wall_direction, 0);
}

#[test]
fn test_player_wall_jump_cooldown_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.wall_jump_cooldown, 0.0);
}

// =============================================================================
// Jet Boost Tests
// =============================================================================

#[test]
fn test_player_initial_jet_charges() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Jet charges start at max
    assert_eq!(player.jet_charges, config.jet_max_charges);
}

#[test]
fn test_player_jet_timer_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.jet_timer, 0.0);
}

#[test]
fn test_player_jet_direction_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.jet_direction, Vec2::ZERO);
}

// =============================================================================
// Ink Cloud Tests
// =============================================================================

#[test]
fn test_player_initial_ink_charges() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.ink_charges, config.ink_max_charges);
}

#[test]
fn test_player_ink_timer_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.ink_timer, 0.0);
    assert!(!player.is_inked);
}

// =============================================================================
// Grapple/Swing Tests
// =============================================================================

#[test]
fn test_player_grapple_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert!(player.grapple_point.is_none());
    assert_eq!(player.rope_length, 0.0);
    assert_eq!(player.swing_angular_velocity, 0.0);
}

// =============================================================================
// Timer Tests
// =============================================================================

#[test]
fn test_player_coyote_timer_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.coyote_timer, 0.0);
}

#[test]
fn test_player_landing_recovery_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.landing_recovery_timer, 0.0);
}

// =============================================================================
// Visual State Tests
// =============================================================================

#[test]
fn test_player_visual_scale_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.visual_scale_y, 1.0);
    assert_eq!(player.target_scale_y, 1.0);
}

#[test]
fn test_player_visual_rotation_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.visual_rotation, 0.0);
    assert_eq!(player.target_rotation, 0.0);
}

#[test]
fn test_player_breathing_phase_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.breathing_phase, 0.0);
}

#[test]
fn test_player_invincibility_initial() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert_eq!(player.hit_flash_timer, 0.0);
    assert_eq!(player.invincibility_timer, 0.0);
}

// =============================================================================
// Collision Rect Tests
// =============================================================================

#[test]
fn test_player_collision_rect() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);
    let rect = player.collision_rect();

    // Rect dimensions should match hitbox
    assert_eq!(rect.w, config.player_hitbox.x);
    assert_eq!(rect.h, config.player_hitbox.y);

    // Rect should be centered on position
    let expected_x = 100.0 - config.player_hitbox.x / 2.0;
    let expected_y = 100.0 - config.player_hitbox.y / 2.0;
    assert_eq!(rect.x, expected_x);
    assert_eq!(rect.y, expected_y);
}

#[test]
fn test_player_collision_rect_moves_with_position() {
    let config = GameConfig::default();
    let mut player = Player::new(vec2(100.0, 100.0), &config);

    // Move player
    player.position = vec2(200.0, 150.0);
    let rect = player.collision_rect();

    // Rect should follow
    let expected_x = 200.0 - config.player_hitbox.x / 2.0;
    let expected_y = 150.0 - config.player_hitbox.y / 2.0;
    assert_eq!(rect.x, expected_x);
    assert_eq!(rect.y, expected_y);
}

// =============================================================================
// Hitbox Tests (from player module)
// =============================================================================

#[test]
fn test_hitbox_to_rect_centered() {
    let hitbox = Hitbox::new(24.0, 30.0);
    let pos = vec2(100.0, 100.0);
    let rect = hitbox.to_rect(pos);

    // Center of rect should be at position
    let center_x = rect.x + rect.w / 2.0;
    let center_y = rect.y + rect.h / 2.0;
    assert_eq!(center_x, 100.0);
    assert_eq!(center_y, 100.0);
}

// =============================================================================
// GameConfig Player Settings Tests
// =============================================================================

#[test]
fn test_config_player_hitbox_positive() {
    let config = GameConfig::default();
    assert!(config.player_hitbox.x > 0.0);
    assert!(config.player_hitbox.y > 0.0);
}

#[test]
fn test_config_wall_stamina_positive() {
    let config = GameConfig::default();
    assert!(config.wall_stamina_max > 0.0);
}

#[test]
fn test_config_jet_charges_nonzero() {
    let config = GameConfig::default();
    assert!(config.jet_max_charges > 0);
}

#[test]
fn test_config_jump_velocity_negative() {
    let config = GameConfig::default();
    // Jump velocity should be negative (upward in screen coords)
    assert!(config.jump_velocity < 0.0);
}

#[test]
fn test_config_gravity_positive() {
    let config = GameConfig::default();
    // Gravity should be positive (pulls downward)
    assert!(config.gravity > 0.0);
}

#[test]
fn test_config_coyote_time_positive() {
    let config = GameConfig::default();
    assert!(config.coyote_time >= 0.0);
}

#[test]
fn test_config_validation_passes_for_default() {
    let config = GameConfig::default();
    assert!(config.validate().is_ok());
}

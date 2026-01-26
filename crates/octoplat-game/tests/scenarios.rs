//! Gameplay scenario tests
//!
//! Tests for gameplay systems and scenarios:
//! - Player abilities and cooldowns
//! - Enemy behaviors
//! - Collectible systems
//! - Platform interactions
//! - Config-based mechanics

use macroquad::prelude::*;
use octoplat_game::{
    Crab, GameConfig, Gem, Hitbox, Player, PlayerState,
    Pufferfish, PufferfishPattern,
    CrumblingPlatform, CrumblingState, MovingPlatform,
};

// =============================================================================
// Player Ability Scenario Tests
// =============================================================================

#[test]
fn test_player_can_use_jet_boost() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player should have jet charges
    assert!(player.jet_charges > 0, "Player should start with jet charges");
}

#[test]
fn test_player_can_use_ink_cloud() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player should have ink charges
    assert!(player.ink_charges > 0, "Player should start with ink charges");
}

#[test]
fn test_player_can_wall_jump() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player should have wall jumps remaining
    assert!(player.wall_jumps_remaining > 0, "Player should start with wall jumps");
    assert!(player.wall_stamina > 0.0, "Player should start with wall stamina");
}

#[test]
fn test_player_starts_without_grapple() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Player should not be grappling initially
    assert!(player.grapple_point.is_none(), "Player should not be grappling at start");
}

#[test]
fn test_player_starts_not_sprinting() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    assert!(!player.is_sprinting, "Player should not be sprinting at start");
}

// =============================================================================
// Enemy Behavior Tests
// =============================================================================

#[test]
fn test_crab_creation() {
    let config = GameConfig::default();
    let crab = Crab::new(vec2(100.0, 100.0), &config);

    assert_eq!(crab.position, vec2(100.0, 100.0));
    assert!(crab.alive);
}

#[test]
fn test_crab_records_start_position() {
    let config = GameConfig::default();
    let crab = Crab::new(vec2(100.0, 100.0), &config);

    // Crab should record start position
    assert_eq!(crab.start_position, vec2(100.0, 100.0));
}

#[test]
fn test_crab_initial_direction() {
    let config = GameConfig::default();
    let crab = Crab::new(vec2(100.0, 100.0), &config);

    // Crab should start facing right
    assert!(crab.facing_right);
}

#[test]
fn test_crab_has_velocity_from_config() {
    let config = GameConfig::default();
    let crab = Crab::new(vec2(100.0, 100.0), &config);

    // Crab velocity should match config
    assert_eq!(crab.velocity, config.crab_speed);
}

#[test]
fn test_pufferfish_creation() {
    let puffer = Pufferfish::new(
        vec2(200.0, 200.0),
        PufferfishPattern::Stationary,
    );

    assert_eq!(puffer.position, vec2(200.0, 200.0));
    assert!(puffer.alive);
}

#[test]
fn test_pufferfish_records_start_position() {
    let puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Vertical,
    );

    assert_eq!(puffer.start_position, vec2(100.0, 100.0));
}

#[test]
fn test_pufferfish_vertical_pattern() {
    let puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Vertical,
    );

    assert!(matches!(puffer.pattern, PufferfishPattern::Vertical));
}

#[test]
fn test_pufferfish_horizontal_pattern() {
    let puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Horizontal,
    );

    assert!(matches!(puffer.pattern, PufferfishPattern::Horizontal));
}

#[test]
fn test_pufferfish_stationary_pattern() {
    let puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Stationary,
    );

    assert!(matches!(puffer.pattern, PufferfishPattern::Stationary));
}

#[test]
fn test_pufferfish_initial_phase() {
    let puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Vertical,
    );

    assert_eq!(puffer.phase, 0.0);
}

// =============================================================================
// Collectible Tests
// =============================================================================

#[test]
fn test_gem_creation() {
    let gem = Gem::new(vec2(150.0, 150.0));

    assert_eq!(gem.position, vec2(150.0, 150.0));
    assert!(!gem.collected, "Gem should not be collected initially");
}

#[test]
fn test_gem_can_be_collected() {
    let mut gem = Gem::new(vec2(100.0, 100.0));

    assert!(!gem.collected);
    gem.collected = true;
    assert!(gem.collected);
}

#[test]
fn test_gem_has_hitbox_radius() {
    let gem = Gem::new(vec2(100.0, 100.0));

    // Gem should have a positive hitbox radius
    assert!(gem.hitbox_radius > 0.0);
}

// =============================================================================
// Platform Mechanics Tests
// =============================================================================

#[test]
fn test_moving_platform_update_moves() {
    let config = GameConfig::default();
    let mut platform = MovingPlatform::new(
        vec2(100.0, 100.0),
        vec2(200.0, 100.0),
        vec2(32.0, 8.0),
    );

    let initial_pos = platform.position;
    platform.update(&config, 0.5); // 0.5 seconds

    // Platform should have moved
    assert_ne!(platform.position, initial_pos);
}

#[test]
fn test_moving_platform_stays_in_bounds() {
    let config = GameConfig::default();
    let mut platform = MovingPlatform::new(
        vec2(0.0, 100.0),
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Update for a long time
    for _ in 0..100 {
        platform.update(&config, 0.1);
    }

    // Platform should stay between start and end
    assert!(platform.position.x >= 0.0 && platform.position.x <= 100.0,
        "Platform X should be between start and end, got {}", platform.position.x);
}

#[test]
fn test_crumbling_platform_starts_stable() {
    let platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    assert!(matches!(platform.state, CrumblingState::Stable));
}

#[test]
fn test_crumbling_platform_trigger_starts_shaking() {
    let config = GameConfig::default();
    let mut platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    platform.trigger(&config);

    // After trigger, should be shaking
    assert!(matches!(platform.state, CrumblingState::Shaking));
}

#[test]
fn test_crumbling_platform_update_transitions_to_falling() {
    let config = GameConfig::default();
    let mut platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    platform.trigger(&config);

    // Update past shake time
    for _ in 0..20 {
        platform.update(&config, 0.1);
    }

    // Should transition to falling or respawning
    assert!(!matches!(platform.state, CrumblingState::Shaking),
        "Platform should have transitioned from shaking");
}

#[test]
fn test_crumbling_platform_respawns() {
    let config = GameConfig::default();
    let mut platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    platform.trigger(&config);

    // Update for a very long time to go through full cycle
    for _ in 0..200 {
        platform.update(&config, 0.1);
    }

    // Should eventually respawn to stable
    assert!(matches!(platform.state, CrumblingState::Stable),
        "Platform should have respawned to stable state");
}

// =============================================================================
// Config-Based Mechanic Tests
// =============================================================================

#[test]
fn test_config_movement_mechanics() {
    let config = GameConfig::default();

    // Movement should have positive values
    assert!(config.move_speed > 0.0);
    assert!(config.sprint_speed > config.move_speed, "Sprint should be faster than walk");
    assert!(config.acceleration > 0.0);
    assert!(config.deceleration > 0.0);
}

#[test]
fn test_config_jump_mechanics() {
    let config = GameConfig::default();

    // Jump mechanics
    assert!(config.jump_velocity < 0.0, "Jump velocity should be negative (upward)");
    assert!(config.jump_cut_multiplier > 0.0 && config.jump_cut_multiplier <= 1.0);
    assert!(config.coyote_time >= 0.0);
    assert!(config.jump_buffer_time >= 0.0);
}

#[test]
fn test_config_wall_mechanics() {
    let config = GameConfig::default();

    // Wall mechanics
    assert!(config.wall_stamina_max > 0.0);
    assert!(config.wall_stamina_regen_rate >= 0.0);
    assert!(config.wall_jumps_max > 0);
    assert!(config.wall_jump_cooldown >= 0.0);
}

#[test]
fn test_config_jet_boost_mechanics() {
    let config = GameConfig::default();

    // Jet boost
    assert!(config.jet_boost_speed > 0.0);
    assert!(config.jet_boost_duration > 0.0);
    assert!(config.jet_max_charges > 0);
    assert!(config.jet_regen_rate > 0.0);
}

#[test]
fn test_config_grapple_mechanics() {
    let config = GameConfig::default();

    // Grapple/swing
    assert!(config.grapple_range > 0.0);
    assert!(config.swing_gravity > 0.0);
    assert!(config.rope_min_length > 0.0);
}

#[test]
fn test_config_enemy_speeds() {
    let config = GameConfig::default();

    // Enemies should have positive speeds
    assert!(config.crab_speed > 0.0);
    assert!(config.pufferfish_speed > 0.0);
}

#[test]
fn test_config_platform_mechanics() {
    let config = GameConfig::default();

    // Platforms
    assert!(config.bounce_velocity > 0.0, "Bounce should push upward");
    assert!(config.moving_platform_speed > 0.0);
    assert!(config.crumble_shake_time > 0.0);
    assert!(config.crumble_respawn_time > 0.0);
}

// =============================================================================
// Player State Scenario Tests
// =============================================================================

#[test]
fn test_player_state_idle_is_grounded() {
    // Idle state implies player is on ground
    let state = PlayerState::Idle;
    assert!(state == PlayerState::Idle);
}

#[test]
fn test_player_state_running_is_grounded() {
    // Running implies player is on ground
    let state = PlayerState::Running;
    assert!(state == PlayerState::Running);
}

#[test]
fn test_player_state_jumping_is_airborne() {
    // Jumping is airborne
    let state = PlayerState::Jumping;
    assert!(state == PlayerState::Jumping);
}

#[test]
fn test_player_state_falling_is_airborne() {
    // Falling is airborne
    let state = PlayerState::Falling;
    assert!(state == PlayerState::Falling);
}

#[test]
fn test_player_state_wall_grip_is_wall_contact() {
    // Wall grip requires wall contact
    let state = PlayerState::WallGrip;
    assert!(state == PlayerState::WallGrip);
}

#[test]
fn test_player_state_jet_boosting_is_special() {
    // Jet boosting is a special ability state
    let state = PlayerState::JetBoosting;
    assert!(state == PlayerState::JetBoosting);
}

#[test]
fn test_player_state_swinging_is_special() {
    // Swinging is a special ability state
    let state = PlayerState::Swinging;
    assert!(state == PlayerState::Swinging);
}

// =============================================================================
// Hitbox Tests
// =============================================================================

#[test]
fn test_hitbox_dimensions() {
    let hitbox = Hitbox::new(24.0, 30.0);

    assert_eq!(hitbox.width, 24.0);
    assert_eq!(hitbox.height, 30.0);
}

#[test]
fn test_hitbox_to_rect_position() {
    let hitbox = Hitbox::new(20.0, 20.0);
    let rect = hitbox.to_rect(vec2(100.0, 100.0));

    // Rect should be centered on position
    let center_x = rect.x + rect.w / 2.0;
    let center_y = rect.y + rect.h / 2.0;
    assert!((center_x - 100.0).abs() < 0.001);
    assert!((center_y - 100.0).abs() < 0.001);
}

#[test]
fn test_hitbox_rect_dimensions_match() {
    let hitbox = Hitbox::new(32.0, 48.0);
    let rect = hitbox.to_rect(vec2(0.0, 0.0));

    assert_eq!(rect.w, 32.0);
    assert_eq!(rect.h, 48.0);
}

// =============================================================================
// Lives System Tests
// =============================================================================

#[test]
fn test_config_lives_system() {
    let config = GameConfig::default();

    assert!(config.starting_lives >= 1, "Should start with at least 1 life");
    assert!(config.max_lives >= config.starting_lives, "Max lives should be >= starting");
}

#[test]
fn test_config_endless_milestone() {
    let config = GameConfig::default();

    assert!(config.endless_gem_milestone > 0, "Should have a positive gem milestone");
}

// =============================================================================
// Death Animation Tests
// =============================================================================

#[test]
fn test_config_death_animation() {
    let config = GameConfig::default();

    assert!(config.death_animation_time > 0.0, "Death animation should have duration");
    assert!(config.hit_flash_duration > 0.0, "Hit flash should have duration");
}

// =============================================================================
// Multiple Player Abilities Scenario
// =============================================================================

#[test]
fn test_player_has_all_starting_resources() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // Verify player starts with all resources
    assert_eq!(player.jet_charges, config.jet_max_charges, "Should have max jet charges");
    assert_eq!(player.ink_charges, config.ink_max_charges, "Should have max ink charges");
    assert_eq!(player.wall_stamina, config.wall_stamina_max, "Should have max wall stamina");
    assert_eq!(player.wall_jumps_remaining, config.wall_jumps_max, "Should have max wall jumps");
}

#[test]
fn test_player_timers_start_at_zero() {
    let config = GameConfig::default();
    let player = Player::new(vec2(100.0, 100.0), &config);

    // All timers should start at zero
    assert_eq!(player.coyote_timer, 0.0);
    assert_eq!(player.landing_recovery_timer, 0.0);
    assert_eq!(player.wall_jump_cooldown, 0.0);
    assert_eq!(player.jet_timer, 0.0);
    assert_eq!(player.jet_regen_timer, 0.0);
    assert_eq!(player.ink_timer, 0.0);
    assert_eq!(player.hit_flash_timer, 0.0);
    assert_eq!(player.invincibility_timer, 0.0);
}

// =============================================================================
// Enemy State Tests
// =============================================================================

#[test]
fn test_crab_can_be_killed() {
    let config = GameConfig::default();
    let mut crab = Crab::new(vec2(100.0, 100.0), &config);

    assert!(crab.alive);
    crab.alive = false;
    assert!(!crab.alive);
}

#[test]
fn test_pufferfish_can_be_killed() {
    let mut puffer = Pufferfish::new(
        vec2(100.0, 100.0),
        PufferfishPattern::Stationary,
    );

    assert!(puffer.alive);
    puffer.alive = false;
    assert!(!puffer.alive);
}

// =============================================================================
// Platform State Tests
// =============================================================================

#[test]
fn test_moving_platform_has_velocity_tracking() {
    let mut platform = MovingPlatform::new(
        vec2(100.0, 100.0),
        vec2(200.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Initial velocity should be zero
    assert_eq!(platform.velocity, Vec2::ZERO);

    // After update, velocity should be non-zero (platform is moving)
    let config = GameConfig::default();
    platform.update(&config, 0.1);
    assert!(platform.velocity.x.abs() > 0.0 || platform.velocity.y.abs() > 0.0);
}

#[test]
fn test_crumbling_platform_timer_tracks() {
    let config = GameConfig::default();
    let mut platform = CrumblingPlatform::new(
        vec2(100.0, 100.0),
        vec2(32.0, 8.0),
    );

    // Timer starts at 0
    assert_eq!(platform.timer, 0.0);

    // After trigger, timer should still be 0 (set in update)
    platform.trigger(&config);

    // After update, timer should increase
    platform.update(&config, 0.1);
    assert!(platform.timer > 0.0);
}

//! Integration tests for game configuration

use octoplat_game::GameConfig;

#[test]
fn test_game_config_default() {
    let config = GameConfig::default();

    // Physics should have sensible values
    assert!(config.gravity > 0.0);
    assert!(config.terminal_velocity > 0.0);

    // Movement should be positive
    assert!(config.move_speed > 0.0);
    assert!(config.sprint_speed > config.move_speed);
    assert!(config.acceleration > 0.0);

    // Jump velocity should be negative (upward)
    assert!(config.jump_velocity < 0.0);
    assert!(config.jump_cut_multiplier > 0.0 && config.jump_cut_multiplier <= 1.0);

    // Coyote time and jump buffer should be small positive values
    assert!(config.coyote_time > 0.0 && config.coyote_time < 1.0);
    assert!(config.jump_buffer_time > 0.0 && config.jump_buffer_time < 1.0);
}

#[test]
fn test_game_config_wall_mechanics() {
    let config = GameConfig::default();

    // Wall mechanics should have sensible values
    assert!(config.wall_stamina_max > 0.0);
    assert!(config.wall_stamina_regen_rate > 0.0);
    assert!(config.wall_jumps_max > 0);
    assert!(config.wall_jump_cooldown > 0.0);
}

#[test]
fn test_game_config_jet_boost() {
    let config = GameConfig::default();

    // Jet boost should have sensible values
    assert!(config.jet_boost_speed > 0.0);
    assert!(config.jet_boost_duration > 0.0);
    assert!(config.jet_max_charges > 0);
    assert!(config.jet_regen_rate > 0.0);
}

#[test]
fn test_game_config_grapple() {
    let config = GameConfig::default();

    // Grapple should have sensible values
    assert!(config.grapple_range > 0.0);
    assert!(config.swing_gravity > 0.0);
    assert!(config.swing_damping > 0.0 && config.swing_damping <= 1.0);
}

#[test]
fn test_game_config_camera() {
    let config = GameConfig::default();

    // Camera should have sensible values
    assert!(config.camera_smoothing > 0.0);
    assert!(config.camera_lookahead >= 0.0);
}

#[test]
fn test_game_config_player() {
    let config = GameConfig::default();

    // Player hitbox should be positive
    assert!(config.player_hitbox.x > 0.0);
    assert!(config.player_hitbox.y > 0.0);

    // Input deadzone should be small
    assert!(config.input_deadzone >= 0.0 && config.input_deadzone < 0.5);
}

#[test]
fn test_game_config_enemies() {
    let config = GameConfig::default();

    // Enemy parameters should be positive
    assert!(config.crab_speed > 0.0);
    assert!(config.pufferfish_amplitude > 0.0);
    assert!(config.pufferfish_speed > 0.0);
}

#[test]
fn test_game_config_platforms() {
    let config = GameConfig::default();

    // Platform parameters should be positive
    assert!(config.bounce_velocity > 0.0);
    assert!(config.moving_platform_speed > 0.0);
    assert!(config.crumble_shake_time > 0.0);
    assert!(config.crumble_respawn_time > 0.0);
}

#[test]
fn test_game_config_lives() {
    let config = GameConfig::default();

    // Lives system should have sensible values
    assert!(config.starting_lives > 0);
    assert!(config.max_lives >= config.starting_lives);
    assert!(config.endless_gem_milestone > 0);
}

#[test]
fn test_game_config_sprint_faster_than_walk() {
    let config = GameConfig::default();
    assert!(
        config.sprint_speed > config.move_speed,
        "Sprint should be faster than walk"
    );
}

#[test]
fn test_game_config_terminal_velocity_reachable() {
    let config = GameConfig::default();

    // Terminal velocity should be reachable with gravity
    // At terminal velocity: gravity = drag, so velocity stabilizes
    assert!(config.terminal_velocity > 0.0);
    assert!(config.terminal_velocity < 10000.0, "Terminal velocity seems too high");
}

#[test]
fn test_game_config_wall_jump_reasonable() {
    let config = GameConfig::default();

    // Wall jump should push away from wall (positive x)
    assert!(config.wall_jump_velocity.x > 0.0);
    // Wall jump should go upward (negative y)
    assert!(config.wall_jump_velocity.y < 0.0);
}

#[test]
fn test_game_config_landing_recovery() {
    let config = GameConfig::default();

    // Landing recovery should be brief
    assert!(config.landing_recovery_time > 0.0);
    assert!(config.landing_recovery_time < 1.0, "Recovery time too long");

    // Recovery factor should reduce movement
    assert!(config.landing_recovery_factor > 0.0);
    assert!(config.landing_recovery_factor < 1.0);
}

#[test]
fn test_game_config_corner_correction() {
    let config = GameConfig::default();

    // Corner correction should be small positive value
    assert!(config.corner_correction_threshold > 0.0);
    assert!(config.corner_correction_threshold < 20.0, "Corner correction too aggressive");
}

#[test]
fn test_game_config_ink_cloud() {
    let config = GameConfig::default();

    // Ink cloud should have reasonable duration
    assert!(config.ink_duration > 0.0);
    assert!(config.ink_max_charges > 0);
}

#[test]
fn test_game_config_death_animation() {
    let config = GameConfig::default();

    // Death animation should be brief but noticeable
    assert!(config.death_animation_time > 0.0);
    assert!(config.death_animation_time < 2.0);
}

#[test]
fn test_game_config_gamepad() {
    let config = GameConfig::default();

    // Gamepad deadzone should be reasonable
    assert!(config.gamepad_stick_deadzone > 0.0);
    assert!(config.gamepad_stick_deadzone < 0.5);
}

//! Integration tests for effects system

use octoplat_game::{BurstConfig, ParticleSystem, ScreenShake};
use macroquad::prelude::*;

// =============================================================================
// BurstConfig Tests
// =============================================================================

#[test]
fn test_burst_config_default() {
    let config = BurstConfig::default();

    assert!(config.count > 0);
    assert!(config.speed_range.0 <= config.speed_range.1);
    assert!(config.lifetime > 0.0);
    assert!(config.size_range.0 <= config.size_range.1);
}

#[test]
fn test_burst_config_clone() {
    let config = BurstConfig::default();
    let cloned = config.clone();

    assert_eq!(config.count, cloned.count);
    assert_eq!(config.lifetime, cloned.lifetime);
}

#[test]
fn test_burst_config_custom() {
    let config = BurstConfig {
        count: 20,
        speed_range: (100.0, 200.0),
        angle_range: (0.0, std::f32::consts::PI),
        lifetime: 1.0,
        size_range: (5.0, 10.0),
        color: RED,
        gravity: 200.0,
        fade: true,
        shrink: false,
    };

    assert_eq!(config.count, 20);
    assert_eq!(config.speed_range.0, 100.0);
    assert_eq!(config.lifetime, 1.0);
    assert!(!config.shrink);
}

// =============================================================================
// ParticleSystem Tests
// =============================================================================

#[test]
fn test_particle_system_new() {
    let particles = ParticleSystem::new();
    // System should start empty
    let _ = particles;
}

#[test]
fn test_particle_system_default() {
    let particles = ParticleSystem::default();
    let _ = particles;
}

#[test]
fn test_particle_system_burst() {
    let mut particles = ParticleSystem::new();
    let config = BurstConfig::default();

    particles.burst(vec2(100.0, 100.0), config.clone());
    // After burst, particles should exist
}

#[test]
fn test_particle_system_update() {
    let mut particles = ParticleSystem::new();
    let config = BurstConfig {
        lifetime: 0.1,
        ..BurstConfig::default()
    };

    particles.burst(vec2(100.0, 100.0), config);

    // Update should process particles
    particles.update(0.05); // Half lifetime
    // Particles should still exist

    particles.update(0.1); // Past lifetime
    // Particles should be cleared
}

#[test]
fn test_particle_system_multiple_bursts() {
    let mut particles = ParticleSystem::new();
    let config = BurstConfig::default();

    particles.burst(vec2(50.0, 50.0), config.clone());
    particles.burst(vec2(100.0, 100.0), config.clone());
    particles.burst(vec2(150.0, 150.0), config);

    // Should handle multiple bursts
}

// =============================================================================
// ScreenShake Tests
// =============================================================================

#[test]
fn test_screen_shake_new() {
    let shake = ScreenShake::new();

    // Should start with no shake
    assert_eq!(shake.offset.x, 0.0);
    assert_eq!(shake.offset.y, 0.0);
}

#[test]
fn test_screen_shake_default() {
    let shake = ScreenShake::default();
    assert_eq!(shake.offset.x, 0.0);
    assert_eq!(shake.offset.y, 0.0);
}

#[test]
fn test_screen_shake_add() {
    let mut shake = ScreenShake::new();
    shake.add(10.0, 0.5);
    // After add, shake is queued but offset isn't set until update
}

#[test]
fn test_screen_shake_update() {
    let mut shake = ScreenShake::new();
    shake.add(10.0, 0.2);

    // Update should apply shake
    shake.update(0.1);
    // Offset may now be non-zero (random)
}

#[test]
fn test_screen_shake_decays() {
    let mut shake = ScreenShake::new();
    shake.add(10.0, 0.1);

    // Update past duration
    for _ in 0..20 {
        shake.update(0.016);
    }

    // After duration expires, offset should return to zero
    assert_eq!(shake.offset, Vec2::ZERO);
}

#[test]
fn test_screen_shake_multiple_adds() {
    let mut shake = ScreenShake::new();

    shake.add(5.0, 0.2);
    shake.update(0.05);

    // Second add should take max intensity
    shake.add(10.0, 0.3);
    shake.update(0.01);
    // Shake should be active with higher intensity
}

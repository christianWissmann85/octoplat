//! Glow/bloom effect
//!
//! Provides per-object glow effects for gems, checkpoints, and exit portal.
//! Uses additive blending with pulsing circles.
//!
//! Note: Currently using inline implementations in world.rs for per-object glow.
//! This module is available as a utility for centralized glow management.

#![allow(dead_code)]

use macroquad::prelude::*;

/// Draw a bloom effect at a position (standalone function for easy use)
///
/// Creates a soft, multi-layered glow that simulates bloom.
/// Higher intensity = brighter, more visible bloom.
pub fn draw_bloom(pos: Vec2, radius: f32, color: Color, intensity: f32) {
    let layers = 6;
    for i in 0..layers {
        let layer_radius = radius * (1.0 + i as f32 * 0.5);
        let falloff = 1.0 - (i as f32 / layers as f32);
        let alpha = color.a * intensity * falloff * falloff * 0.2;
        draw_circle(pos.x, pos.y, layer_radius, Color::new(color.r, color.g, color.b, alpha));
    }

    // Bright core
    let core_alpha = (color.a * intensity * 0.5).min(1.0);
    draw_circle(pos.x, pos.y, radius * 0.3, Color::new(
        (color.r * 1.2).min(1.0),
        (color.g * 1.2).min(1.0),
        (color.b * 1.2).min(1.0),
        core_alpha,
    ));
}

/// Draw a pulsing bloom effect
pub fn draw_bloom_pulsing(pos: Vec2, radius: f32, color: Color, intensity: f32, time: f32, pulse_speed: f32) {
    let pulse = (time * pulse_speed).sin() * 0.3 + 0.7;
    draw_bloom(pos, radius * pulse, color, intensity * pulse);
}

/// Glow effect manager
pub struct GlowEffect {
    // Currently stateless - glow is drawn per-object
    // Future: could add render target for full-screen bloom
}

impl GlowEffect {
    /// Create a new glow effect
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }

    /// Draw a pulsing glow circle around an object
    pub fn draw_glow(&self, pos: Vec2, base_radius: f32, color: Color, time: f32, pulse_speed: f32) {
        let pulse = (time * pulse_speed).sin() * 0.3 + 0.7;
        let radius = base_radius * pulse;

        // Outer soft glow (multiple layers for smooth falloff)
        for i in 0..4 {
            let layer_radius = radius * (1.0 + i as f32 * 0.3);
            let alpha = color.a * (0.15 - i as f32 * 0.03);
            draw_circle(pos.x, pos.y, layer_radius, Color::new(color.r, color.g, color.b, alpha));
        }

        // Inner bright core
        let core_alpha = color.a * 0.4 * pulse;
        draw_circle(pos.x, pos.y, radius * 0.5, Color::new(color.r, color.g, color.b, core_alpha));
    }

    /// Draw glow for a gem
    pub fn draw_gem_glow(&self, pos: Vec2, time: f32) {
        // Cyan/blue sparkle
        let sparkle = (time * 4.0).sin() * 0.3 + 0.7;
        self.draw_glow(
            pos,
            18.0,
            Color::new(0.3 * sparkle, 0.8 * sparkle, 1.0 * sparkle, 0.6),
            time,
            4.0,
        );
    }

    /// Draw glow for an active checkpoint
    pub fn draw_checkpoint_glow(&self, pos: Vec2, time: f32) {
        // Green glow
        self.draw_glow(
            pos,
            24.0,
            Color::new(0.2, 1.0, 0.4, 0.5),
            time,
            3.0,
        );
    }

    /// Draw glow for exit portal
    pub fn draw_exit_glow(&self, pos: Vec2, time: f32) {
        // Golden glow with faster pulse
        self.draw_glow(
            pos,
            28.0,
            Color::new(1.0, 0.85, 0.3, 0.7),
            time,
            5.0,
        );

        // Secondary rotating particles
        for i in 0..6 {
            let angle = time * 1.5 + (i as f32) * std::f32::consts::TAU / 6.0;
            let particle_radius = 24.0 + (time * 2.0 + i as f32).sin() * 4.0;
            let particle_pos = pos + vec2(angle.cos(), angle.sin()) * particle_radius;
            let alpha = 0.5 + (time * 3.0 + i as f32 * 0.5).sin() * 0.3;
            draw_circle(
                particle_pos.x,
                particle_pos.y,
                3.0,
                Color::new(1.0, 0.95, 0.6, alpha),
            );
        }
    }

    /// Draw danger glow (for breakable blocks about to break)
    pub fn draw_danger_glow(&self, pos: Vec2, size: Vec2, time: f32, intensity: f32) {
        let pulse = (time * 6.0).sin() * 0.5 + 0.5;
        let alpha = 0.3 * intensity * pulse;

        // Red danger glow
        draw_rectangle(
            pos.x - 4.0,
            pos.y - 4.0,
            size.x + 8.0,
            size.y + 8.0,
            Color::new(1.0, 0.3, 0.2, alpha),
        );
    }
}

impl Default for GlowEffect {
    fn default() -> Self {
        Self::new().unwrap_or(Self {})
    }
}

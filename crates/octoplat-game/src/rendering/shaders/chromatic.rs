//! Chromatic aberration effect
//!
//! RGB channel offset that radiates from hit position and decays over time.

use macroquad::prelude::*;

/// State for chromatic aberration effect
pub struct ChromaticAberration {
    /// Current intensity (0.0-1.0, decays over time)
    pub intensity: f32,
    /// Center position in normalized screen coordinates (0-1)
    pub center: Vec2,
    /// Decay rate per second
    decay_rate: f32,
}

impl ChromaticAberration {
    /// Create a new chromatic aberration state
    pub fn new() -> Self {
        Self {
            intensity: 0.0,
            center: vec2(0.5, 0.5),
            decay_rate: 4.0, // Fully decays in ~0.25 seconds
        }
    }

    /// Trigger chromatic aberration effect
    pub fn trigger(&mut self, screen_pos: Vec2, intensity: f32) {
        // Convert to normalized screen coordinates
        self.center = vec2(
            screen_pos.x / screen_width(),
            screen_pos.y / screen_height(),
        );
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    /// Update the effect (decay intensity)
    pub fn update(&mut self, dt: f32) {
        if self.intensity > 0.0 {
            self.intensity = (self.intensity - self.decay_rate * dt).max(0.0);
        }
    }

    /// Check if the effect is currently active
    pub fn is_active(&self) -> bool {
        self.intensity > 0.01
    }

    /// Apply chromatic aberration effect
    /// Note: This is a simplified version that draws colored overlays
    /// A full implementation would use a post-process shader with render targets
    pub fn apply(&self) {
        if !self.is_active() {
            return;
        }

        // Draw subtle colored vignette radiating from center
        let sw = screen_width();
        let sh = screen_height();
        let center_screen = vec2(self.center.x * sw, self.center.y * sh);

        // Draw colored edge overlays to simulate RGB split
        // Note: A full shader implementation would offset RGB channels, but
        // this simplified version uses colored overlays for the same effect
        let edge_alpha = self.intensity * 0.3;

        // Left edge (red tint)
        draw_rectangle(
            0.0,
            0.0,
            sw * 0.1,
            sh,
            Color::new(1.0, 0.0, 0.0, edge_alpha * (1.0 - self.center.x)),
        );

        // Right edge (blue tint)
        draw_rectangle(
            sw * 0.9,
            0.0,
            sw * 0.1,
            sh,
            Color::new(0.0, 0.0, 1.0, edge_alpha * self.center.x),
        );

        // Top edge
        draw_rectangle(
            0.0,
            0.0,
            sw,
            sh * 0.1,
            Color::new(1.0, 0.0, 0.5, edge_alpha * (1.0 - self.center.y)),
        );

        // Bottom edge
        draw_rectangle(
            0.0,
            sh * 0.9,
            sw,
            sh * 0.1,
            Color::new(0.0, 0.5, 1.0, edge_alpha * self.center.y),
        );

        // Central flash
        let flash_alpha = self.intensity * 0.15;
        let flash_radius = 100.0 + self.intensity * 50.0;
        draw_circle(
            center_screen.x,
            center_screen.y,
            flash_radius,
            Color::new(1.0, 1.0, 1.0, flash_alpha),
        );
    }
}

impl Default for ChromaticAberration {
    fn default() -> Self {
        Self::new()
    }
}

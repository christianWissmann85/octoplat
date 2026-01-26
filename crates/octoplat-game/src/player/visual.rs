//! Player visual effects
//!
//! Handles squash/stretch animations, breathing idle animation, hit flash,
//! and invincibility frames.

use crate::config::GameConfig;
use super::Player;
use super::PlayerState;

impl Player {
    /// Update visual effect timers with smooth interpolation
    pub fn update_visual_effects(&mut self, dt: f32) {
        use crate::rendering::easing;

        // Update target scale based on state
        match self.state {
            PlayerState::Jumping => {
                self.target_scale_y = 1.15;
            }
            PlayerState::Falling => {
                self.target_scale_y = 1.05;
            }
            PlayerState::JetBoosting if self.is_jet_downward() => {
                self.target_scale_y = 1.2;
                self.target_rotation = std::f32::consts::PI * 0.1;
            }
            _ => {
                self.target_scale_y = 1.0;
                self.target_rotation = 0.0;
            }
        }

        // Smooth interpolation for scale
        self.visual_scale_y = easing::smooth_towards(
            self.visual_scale_y,
            self.target_scale_y,
            12.0,
            dt,
        );

        // Smooth interpolation for rotation
        self.visual_rotation = easing::smooth_towards(
            self.visual_rotation,
            self.target_rotation,
            10.0,
            dt,
        );

        // Update breathing phase for idle animation
        self.breathing_phase += dt * 2.0;
        if self.breathing_phase > std::f32::consts::TAU {
            self.breathing_phase -= std::f32::consts::TAU;
        }

        // Decay hit flash
        if self.hit_flash_timer > 0.0 {
            self.hit_flash_timer = (self.hit_flash_timer - dt).max(0.0);
        }

        // Decay invincibility
        if self.invincibility_timer > 0.0 {
            self.invincibility_timer = (self.invincibility_timer - dt).max(0.0);
        }
    }

    /// Trigger stretch effect (for jump start)
    pub fn trigger_stretch(&mut self) {
        self.visual_scale_y = 1.25;
        self.target_scale_y = 1.15;
    }

    /// Trigger squash effect (for landing)
    pub fn trigger_squash(&mut self) {
        self.visual_scale_y = 0.7;
        self.target_scale_y = 0.85;
    }

    /// Trigger hard landing recovery
    pub fn trigger_landing_recovery(&mut self, duration: f32) {
        self.landing_recovery_timer = duration;
    }

    /// Get landing recovery factor (1.0 = normal, lower = reduced movement)
    pub fn landing_recovery_factor_with_config(&self, config: &GameConfig) -> f32 {
        if self.landing_recovery_timer > 0.0 {
            config.landing_recovery_factor
        } else {
            1.0
        }
    }

    /// Get landing recovery factor using default config value
    /// Prefer landing_recovery_factor_with_config when config is available
    pub fn landing_recovery_factor(&self) -> f32 {
        if self.landing_recovery_timer > 0.0 {
            0.3 // Fallback default; use landing_recovery_factor_with_config for configurable value
        } else {
            1.0
        }
    }

    /// Trigger hit flash effect
    pub fn trigger_hit_flash(&mut self, config: &GameConfig) {
        self.hit_flash_timer = config.hit_flash_duration;
    }

    /// Start invincibility frames
    pub fn start_invincibility(&mut self, duration: f32) {
        self.invincibility_timer = duration;
    }

    /// Check if player is invincible (i-frames or jet boosting)
    pub fn is_invincible(&self) -> bool {
        self.invincibility_timer > 0.0 || self.state == PlayerState::JetBoosting
    }
}

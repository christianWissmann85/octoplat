//! Player visual effects
//!
//! Handles squash/stretch animations, breathing idle animation, hit flash,
//! invincibility frames, and anticipation animations.

use crate::config::GameConfig;
use super::Player;
use super::PlayerState;
use super::AnticipationType;

/// Duration of anticipation animation in seconds (very short to not delay gameplay feel)
const ANTICIPATION_DURATION: f32 = 0.06;

impl Player {
    /// Update visual effect timers with smooth interpolation
    pub fn update_visual_effects(&mut self, dt: f32) {
        use crate::rendering::easing;

        // Update anticipation timer
        if self.anticipation_timer > 0.0 {
            self.anticipation_timer -= dt;
            if self.anticipation_timer <= 0.0 {
                self.anticipation_timer = 0.0;
                self.anticipation_type = AnticipationType::None;
            }
        }

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

        // Handle landing overshoot animation
        if self.landing_overshoot_timer > 0.0 {
            self.landing_overshoot_timer -= dt;

            // Calculate animation progress (0 = start, 1 = end)
            let progress = 1.0 - (self.landing_overshoot_timer / self.landing_overshoot_duration).max(0.0);

            // Use elastic easing for bouncy overshoot effect
            // This will squash down, then overshoot past 1.0, then settle
            let elastic = easing::ease_out_elastic(progress);

            // Interpolate from squash (0.7) to normal (1.0) with overshoot
            let squash_amount = 0.7;
            self.visual_scale_y = squash_amount + (1.0 - squash_amount) * elastic;

            // Reset when done
            if self.landing_overshoot_timer <= 0.0 {
                self.landing_overshoot_timer = 0.0;
                self.landing_overshoot_intensity = 0.0;
            }
        } else {
            // Normal smooth interpolation for scale (when not in overshoot)
            self.visual_scale_y = easing::smooth_towards(
                self.visual_scale_y,
                self.target_scale_y,
                12.0,
                dt,
            );
        }

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

    /// Trigger squash effect with overshoot (for landing)
    ///
    /// The intensity parameter (0.0-1.0+) affects how dramatic the bounce-back is.
    /// Based on fall velocity - harder landings have more bounce.
    pub fn trigger_squash(&mut self, intensity: f32) {
        // Clamp intensity to reasonable range
        let intensity = intensity.clamp(0.0, 1.5);

        // Initial squash based on intensity (harder landing = more squash)
        let squash_amount = 0.7 - intensity * 0.1; // 0.7 to 0.55 based on intensity
        self.visual_scale_y = squash_amount.max(0.55);

        // Start overshoot animation
        // Duration scales with intensity - harder landings take longer to settle
        self.landing_overshoot_duration = 0.25 + intensity * 0.15; // 0.25s to 0.4s
        self.landing_overshoot_timer = self.landing_overshoot_duration;
        self.landing_overshoot_intensity = intensity;
    }

    /// Trigger squash effect without overshoot (legacy, for simple uses)
    pub fn trigger_squash_simple(&mut self) {
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

    // ========================================================================
    // HP System
    // ========================================================================

    /// Take damage from a hazard or enemy
    ///
    /// Returns true if the player died (HP reached 0)
    pub fn take_damage(&mut self, amount: u8, config: &GameConfig) -> bool {
        if amount == 0 || self.current_hp == 0 {
            return self.current_hp == 0;
        }

        // Apply damage
        self.current_hp = self.current_hp.saturating_sub(amount);

        // Trigger visual feedback
        self.trigger_hit_flash(config);

        // Start invincibility frames (using config duration)
        self.start_invincibility(config.invincibility_duration);

        // Return true if player died
        self.current_hp == 0
    }

    /// Heal HP by the given amount (capped at max_hp)
    pub fn heal(&mut self, amount: u8) {
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
    }

    /// Reset HP to maximum (called at level start)
    pub fn reset_hp(&mut self, config: &GameConfig) {
        self.max_hp = config.player_max_hp;
        self.current_hp = self.max_hp;
    }

    /// Get HP as a fraction (0.0 to 1.0) for UI bar display
    pub fn hp_fraction(&self) -> f32 {
        if self.max_hp == 0 {
            return 1.0;
        }
        self.current_hp as f32 / self.max_hp as f32
    }

    // ========================================================================
    // Anticipation Animation
    // ========================================================================

    /// Start an anticipation animation (purely visual, no input delay)
    ///
    /// This creates a brief squash effect that gives actions more weight
    /// without actually delaying the gameplay mechanics.
    pub fn start_anticipation(&mut self, anticipation_type: AnticipationType) {
        // Don't override if already in anticipation
        if self.anticipation_timer > 0.0 {
            return;
        }

        self.anticipation_type = anticipation_type;
        self.anticipation_timer = ANTICIPATION_DURATION;

        // Apply immediate visual effect based on type
        match anticipation_type {
            AnticipationType::Jump => {
                // Crouch squash before jump
                self.visual_scale_y = 0.85;
            }
            AnticipationType::WallJump => {
                // Slight compression against wall
                self.visual_scale_y = 0.9;
            }
            AnticipationType::JetDive => {
                // Pull up before dive
                self.visual_scale_y = 1.1;
            }
            AnticipationType::None => {}
        }
    }

    /// Get the anticipation scale modifier (returns 1.0 if no anticipation active)
    pub fn get_anticipation_scale(&self) -> (f32, f32) {
        if self.anticipation_timer <= 0.0 || self.anticipation_type == AnticipationType::None {
            return (1.0, 1.0);
        }

        // Calculate progress (0 = start, 1 = end)
        let progress = 1.0 - (self.anticipation_timer / ANTICIPATION_DURATION);

        match self.anticipation_type {
            AnticipationType::Jump => {
                // Squash horizontally, stretch vertically (crouch)
                let squash = 1.0 + 0.12 * (1.0 - progress); // 1.12 -> 1.0
                let stretch = 1.0 - 0.15 * (1.0 - progress); // 0.85 -> 1.0
                (squash, stretch)
            }
            AnticipationType::WallJump => {
                // Compress against wall
                let compress = 1.0 - 0.1 * (1.0 - progress); // 0.9 -> 1.0
                let stretch = 1.0 + 0.05 * (1.0 - progress); // 1.05 -> 1.0
                (compress, stretch)
            }
            AnticipationType::JetDive => {
                // Pull up before dive
                let compress = 1.0 - 0.05 * (1.0 - progress); // 0.95 -> 1.0
                let stretch = 1.0 + 0.12 * (1.0 - progress); // 1.12 -> 1.0
                (compress, stretch)
            }
            AnticipationType::None => (1.0, 1.0),
        }
    }
}

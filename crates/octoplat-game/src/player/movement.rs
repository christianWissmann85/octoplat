//! Player movement mechanics
//!
//! Contains jump, wall jump, and physics methods.

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::input::InputState;

use super::state::PlayerState;
use super::Player;

impl Player {
    /// Apply gravity to vertical velocity, clamped to terminal velocity
    #[inline]
    fn apply_gravity(&mut self, config: &GameConfig, dt: f32) {
        self.velocity.y = (self.velocity.y + config.gravity * dt).min(config.terminal_velocity);
    }

    /// Execute a regular jump from ground
    pub(super) fn execute_jump(&mut self, config: &GameConfig) {
        self.velocity.y = config.jump_velocity;
        self.state = PlayerState::Jumping;
        self.coyote_timer = 0.0;
    }

    /// Execute a wall jump - requires intentional input direction
    /// Returns true if jump was executed, false if input was invalid
    pub(super) fn execute_wall_jump(&mut self, input: &InputState, config: &GameConfig) -> bool {
        // Wall Kick: pressing away from wall - full horizontal push
        let pressing_away = (input.move_dir.x > config.input_deadzone && self.wall_direction < 0)
            || (input.move_dir.x < -config.input_deadzone && self.wall_direction > 0);

        // Climb Jump: pressing up - strong vertical, small horizontal clearance
        let pressing_up = input.move_dir.y < -config.input_deadzone;

        let (horizontal_push, vertical_push) = if pressing_away {
            // Wall kick: full push away (for jumping between walls or launching off)
            (config.wall_jump_velocity.x, config.wall_jump_velocity.y)
        } else if pressing_up {
            // Climb jump: strong vertical boost, moderate horizontal for a natural arc
            // The horizontal push creates clearance from the wall to prevent immediate re-grab
            (
                config.wall_jump_velocity.x * config.wall_jump_climb_horizontal,
                config.wall_jump_velocity.y * config.wall_jump_climb_vertical,
            )
        } else {
            // No valid input - don't execute wall jump
            return false;
        };

        // Record the wall position before clearing wall_direction
        // This is the X position of the wall we're jumping from
        let wall_x = self.position.x + (self.wall_direction as f32 * self.hitbox.width / 2.0);
        self.last_wall_jump_x = Some(wall_x);

        self.velocity = vec2(
            horizontal_push * -self.wall_direction as f32,
            vertical_push,
        );
        self.state = PlayerState::Jumping;
        self.wall_direction = 0;

        // Set cooldowns to prevent immediate re-grab
        self.wall_jump_cooldown = config.wall_jump_cooldown;
        self.same_wall_cooldown = config.same_wall_cooldown;

        // Change facing when doing a wall kick
        if pressing_away {
            self.facing_right = self.velocity.x > 0.0;
        }

        true
    }

    /// Apply physics based on current state
    pub(super) fn apply_state_physics(&mut self, input: &InputState, config: &GameConfig, dt: f32) {
        // Apply landing recovery slowdown to ground movement
        let recovery_factor = self.landing_recovery_factor_with_config(config);

        match self.state {
            PlayerState::Idle => {
                // Decelerate to stop
                self.velocity.x = move_toward(self.velocity.x, 0.0, config.deceleration * dt);
                self.apply_gravity(config, dt);
            }

            PlayerState::Running => {
                // Use sprint speed/acceleration if sprinting
                let effective_speed = if self.is_sprinting { config.sprint_speed } else { config.move_speed };
                let effective_accel = if self.is_sprinting { config.sprint_acceleration } else { config.acceleration };

                // Accelerate toward target speed (reduced during landing recovery)
                let target_vx = input.move_dir.x * effective_speed * recovery_factor;
                let accel = effective_accel * recovery_factor;
                self.velocity.x = move_toward(self.velocity.x, target_vx, accel * dt);
                self.apply_gravity(config, dt);
            }

            PlayerState::Jumping | PlayerState::Falling => {
                // Air control (reduced acceleration) with partial sprint bonus
                let base_air_speed = config.move_speed * config.air_control;
                let sprint_air_bonus = if self.is_sprinting {
                    (config.sprint_speed - config.move_speed) * config.sprint_air_bonus * config.air_control
                } else {
                    0.0
                };
                let target_vx = input.move_dir.x * (base_air_speed + sprint_air_bonus);
                self.velocity.x =
                    move_toward(self.velocity.x, target_vx, config.air_acceleration * dt);
                self.apply_gravity(config, dt);
            }

            PlayerState::WallGrip => {
                // Cling in place - no movement
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }

            // Abilities handle their own physics
            PlayerState::JetBoosting | PlayerState::Swinging => {
                self.apply_ability_physics(input, config, dt);
            }
        }
    }
}

/// Helper: move a value toward a target by a maximum delta
pub fn move_toward(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else {
        current + (target - current).signum() * max_delta
    }
}

//! Player state machine and timer management
//!
//! Handles state transitions between Idle, Running, Jumping, Falling, WallGrip,
//! JetBoosting, and Swinging states. Also manages all gameplay timers.

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::input::InputState;

use super::Player;
use super::PlayerState;

impl Player {
    /// Update all timers based on ground state
    pub(super) fn update_timers(&mut self, on_ground: bool, config: &GameConfig, dt: f32) {
        if on_ground {
            self.coyote_timer = config.coyote_time;
            self.wall_stamina = (self.wall_stamina + config.wall_stamina_regen_rate * dt)
                .min(config.wall_stamina_max);
            self.wall_jumps_remaining = config.wall_jumps_max;
            self.wall_jump_cooldown = 0.0;
            self.same_wall_cooldown = 0.0;
            self.last_wall_jump_x = None;
        } else {
            self.coyote_timer = (self.coyote_timer - dt).max(0.0);
        }

        // Jet charge passive regen
        if self.jet_charges < config.jet_max_charges {
            self.jet_regen_timer += dt;
            if self.jet_regen_timer >= config.jet_regen_rate {
                self.jet_regen_timer = 0.0;
                self.jet_charges += 1;
            }
        } else {
            self.jet_regen_timer = 0.0;
        }

        // Tick down wall jump cooldowns
        if self.wall_jump_cooldown > 0.0 {
            self.wall_jump_cooldown = (self.wall_jump_cooldown - dt).max(0.0);
        }
        if self.same_wall_cooldown > 0.0 {
            self.same_wall_cooldown = (self.same_wall_cooldown - dt).max(0.0);
            // Clear wall memory when same-wall cooldown expires
            if self.same_wall_cooldown <= 0.0 {
                self.last_wall_jump_x = None;
            }
        }

        // Update ink timer
        if self.ink_timer > 0.0 {
            self.ink_timer -= dt;
            if self.ink_timer <= 0.0 {
                self.is_inked = false;
            }
        }

        // Update jet timer
        if self.jet_timer > 0.0 {
            self.jet_timer -= dt;
        }

        // Update landing recovery timer
        if self.landing_recovery_timer > 0.0 {
            self.landing_recovery_timer = (self.landing_recovery_timer - dt).max(0.0);
        }
    }

    /// Handle ability input checks (ink, jet, grapple)
    pub(super) fn handle_ability_inputs(
        &mut self,
        input: &mut InputState,
        grapple_points: &[Vec2],
        config: &GameConfig,
    ) {
        // Ink Cloud (Q)
        if input.ink_pressed && self.ink_charges > 0 && !self.is_inked {
            self.activate_ink(config);
        }

        // Jet Boost (E) - works on ground, in air, or wall gripping
        if input.jet_boost_pressed
            && self.jet_charges > 0
            && self.state != PlayerState::JetBoosting
            && self.state != PlayerState::Swinging
        {
            self.execute_jet_boost(input, config);
        }

        // Grapple (F or Right Mouse) - toggle-based, no stamina cost
        let was_swinging = self.state == PlayerState::Swinging;

        if input.grapple_buffer_active && !was_swinging {
            #[cfg(debug_assertions)]
            eprintln!(
                "[Grapple] Attempting: buffer={}, state={:?}, points={}",
                input.grapple_buffer_active, self.state, grapple_points.len()
            );

            if let Some(target) = self.find_nearest_grapple_point(grapple_points, config) {
                #[cfg(debug_assertions)]
                eprintln!("[Grapple] Found target at {:?}, starting grapple", target);

                self.start_grapple(target, config);
                input.consume_grapple_buffer();
            } else {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[Grapple] No valid target found. Player pos={:?}, range={}",
                    self.position, config.grapple_range
                );
            }
        }

        // Release grapple when F pressed again while swinging
        // Only if we were ALREADY swinging (not if we just started this frame)
        if was_swinging && input.grapple_pressed {
            #[cfg(debug_assertions)]
            eprintln!("[Grapple] Releasing: F pressed again while swinging");
            self.release_grapple(config);
            input.consume_grapple_buffer();
        }
    }

    /// Handle state transitions based on environment and input
    pub(super) fn handle_state_transitions(
        &mut self,
        input: &mut InputState,
        on_ground: bool,
        wall_dir: i8,
        config: &GameConfig,
        dt: f32,
    ) {
        match self.state {
            PlayerState::Idle | PlayerState::Running => {
                self.handle_grounded_transitions(input, on_ground, config, dt);
            }
            PlayerState::Jumping | PlayerState::Falling => {
                self.handle_airborne_transitions(input, on_ground, wall_dir, config);
            }
            PlayerState::WallGrip => {
                self.handle_wall_transitions(input, wall_dir, config, dt);
            }
            PlayerState::JetBoosting => {
                self.handle_jet_transitions(input, on_ground, wall_dir, config);
            }
            PlayerState::Swinging => {
                self.handle_swing_transitions(input, on_ground, wall_dir, config);
            }
        }
    }

    /// Handle transitions from grounded states (Idle, Running)
    fn handle_grounded_transitions(
        &mut self,
        input: &mut InputState,
        on_ground: bool,
        config: &GameConfig,
        dt: f32,
    ) {
        let wants_jump = input.jump_buffer_active;

        // Update sprint state - drain stamina from the first frame of sprinting
        let wants_sprint =
            input.sprint_held && input.move_dir.x.abs() > config.input_deadzone && self.wall_stamina > 0.0;
        if wants_sprint {
            self.wall_stamina = (self.wall_stamina - config.sprint_stamina_drain * dt).max(0.0);
            self.is_sprinting = self.wall_stamina > 0.0;
        } else {
            self.is_sprinting = false;
        }

        if !on_ground {
            self.state = PlayerState::Falling;
        } else if wants_jump {
            self.execute_jump(config);
            input.consume_jump_buffer();
        } else if input.move_dir.x.abs() > config.input_deadzone {
            self.state = PlayerState::Running;
        } else {
            self.state = PlayerState::Idle;
            self.is_sprinting = false;
        }
    }

    /// Handle transitions from airborne states (Jumping, Falling)
    fn handle_airborne_transitions(
        &mut self,
        input: &mut InputState,
        on_ground: bool,
        wall_dir: i8,
        config: &GameConfig,
    ) {
        let can_coyote_jump = self.coyote_timer > 0.0;
        let wants_jump = input.jump_buffer_active;

        if self.can_grab_wall(wall_dir, input.grapple_held, config) {
            self.transition_to_wall(wall_dir);
        } else if wants_jump && can_coyote_jump && self.state == PlayerState::Falling {
            self.execute_jump(config);
            input.consume_jump_buffer();
        } else if on_ground && self.velocity.y >= 0.0 {
            self.state = if input.move_dir.x.abs() > config.input_deadzone {
                PlayerState::Running
            } else {
                PlayerState::Idle
            };
            if wants_jump {
                self.execute_jump(config);
                input.consume_jump_buffer();
            }
        } else if self.state == PlayerState::Jumping && self.velocity.y >= 0.0 {
            self.state = PlayerState::Falling;
        }
    }

    /// Handle transitions from wall grip state
    fn handle_wall_transitions(
        &mut self,
        input: &mut InputState,
        wall_dir: i8,
        config: &GameConfig,
        dt: f32,
    ) {
        let wants_jump = input.jump_buffer_active;

        self.wall_stamina = (self.wall_stamina - config.swing_stamina_drain * 0.5 * dt).max(0.0);

        // Release if: no wall contact, F released, or out of stamina
        let should_release = wall_dir == 0 || !input.grapple_held || self.wall_stamina <= 0.0;

        if wants_jump && self.wall_jumps_remaining > 0 {
            // Wall jump requires intentional input (away for kick, up for climb)
            if self.execute_wall_jump(input, config) {
                self.wall_jumps_remaining -= 1;
                input.consume_jump_buffer();
            }
        } else if should_release {
            self.state = PlayerState::Falling;
            self.wall_direction = 0;
        }
    }

    /// Handle transitions from jet boosting state
    fn handle_jet_transitions(
        &mut self,
        input: &InputState,
        on_ground: bool,
        wall_dir: i8,
        config: &GameConfig,
    ) {
        if self.jet_timer <= 0.0 {
            // Jet boost ended - transition based on current position
            if on_ground {
                self.state = PlayerState::Idle;
            } else if self.velocity.y >= 0.0 {
                self.state = PlayerState::Falling;
            } else {
                self.state = PlayerState::Jumping;
            };
        } else if self.can_grab_wall(wall_dir, input.grapple_held, config) {
            self.transition_to_wall(wall_dir);
            self.jet_timer = 0.0;
        } else if on_ground && self.jet_direction.y > 0.3 {
            // Only cancel on ground if jetting downward (dive attack landing)
            // Horizontal jets continue through ground contact
            self.state = PlayerState::Idle;
            self.jet_timer = 0.0;
        }
    }

    /// Handle transitions from swinging state
    fn handle_swing_transitions(
        &mut self,
        input: &mut InputState,
        on_ground: bool,
        wall_dir: i8,
        config: &GameConfig,
    ) {
        let wants_jump = input.jump_buffer_active;

        #[cfg(debug_assertions)]
        eprintln!(
            "[Swing] Checking transitions: wants_jump={}, on_ground={}, wall_dir={}, grapple_pressed={}",
            wants_jump, on_ground, wall_dir, input.grapple_pressed
        );

        // No stamina drain - swing is free to use
        // Release via: Jump (for boosted release), ground contact, or wall grab
        if wants_jump {
            #[cfg(debug_assertions)]
            eprintln!("[Swing] Releasing: jump pressed");
            self.release_grapple(config);
            self.velocity.y = config.jump_velocity * 0.8;
            self.state = PlayerState::Jumping;
            input.consume_jump_buffer();
        } else if on_ground {
            #[cfg(debug_assertions)]
            eprintln!("[Swing] Releasing: on ground");
            self.release_grapple(config);
            self.state = PlayerState::Idle;
        } else if self.can_grab_wall(wall_dir, input.grapple_pressed, config) {
            #[cfg(debug_assertions)]
            eprintln!("[Swing] Releasing: grabbing wall (wall_dir={})", wall_dir);
            self.release_grapple(config);
            self.transition_to_wall(wall_dir);
        }
    }

    /// Helper to transition to wall grip state
    pub(super) fn transition_to_wall(&mut self, wall_dir: i8) {
        self.state = PlayerState::WallGrip;
        self.wall_direction = wall_dir;
        self.velocity.y = 0.0;
    }

    /// Check if the player can grab the wall
    /// Requires: wall contact, grapple input active, stamina available, cooldown expired, not same wall
    #[inline]
    fn can_grab_wall(&self, wall_dir: i8, grapple_active: bool, config: &GameConfig) -> bool {
        wall_dir != 0
            && grapple_active
            && self.wall_stamina > 0.0
            && self.wall_jump_cooldown <= 0.0
            && !self.is_same_wall(wall_dir, config)
    }

    /// Check if the current wall is the same wall we recently jumped from
    pub(super) fn is_same_wall(&self, wall_dir: i8, config: &GameConfig) -> bool {
        if self.same_wall_cooldown <= 0.0 {
            return false;
        }
        if let Some(last_x) = self.last_wall_jump_x {
            let current_wall_x = self.position.x + (wall_dir as f32 * self.hitbox.width / 2.0);
            (current_wall_x - last_x).abs() < config.same_wall_threshold
        } else {
            false
        }
    }
}

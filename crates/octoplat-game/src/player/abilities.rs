//! Player ability mechanics
//!
//! Contains dive, jet boost, ink cloud, and grapple methods.

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::input::InputState;

use super::state::PlayerState;
use super::Player;
use super::AnticipationType;

impl Player {
    /// Execute a jet boost in input direction
    /// Downward jets are faster and can break blocks/kill enemies
    pub(super) fn execute_jet_boost(&mut self, input: &InputState, config: &GameConfig) {
        self.state = PlayerState::JetBoosting;
        self.jet_charges -= 1;
        self.jet_regen_timer = 0.0; // Reset regen timer on use
        self.jet_timer = config.jet_boost_duration;

        // Direction: use input direction, or facing direction if no input
        let dir = if input.move_dir.length_squared() > 0.1 {
            input.move_dir
        } else {
            vec2(if self.facing_right { 1.0 } else { -1.0 }, 0.0)
        };
        self.jet_direction = dir.normalize();

        // Add anticipation for downward dives (looks like a wind-up)
        if self.jet_direction.y > 0.5 {
            self.start_anticipation(AnticipationType::JetDive);
        }

        // Update facing based on jet direction
        if self.jet_direction.x.abs() > 0.1 {
            self.facing_right = self.jet_direction.x > 0.0;
        }
    }

    /// Check if current jet is primarily downward (for block breaking)
    pub fn is_jet_downward(&self) -> bool {
        self.state == PlayerState::JetBoosting && self.jet_direction.y > 0.5
    }

    /// Activate ink cloud (invincibility)
    pub(super) fn activate_ink(&mut self, config: &GameConfig) {
        self.ink_charges -= 1;
        self.ink_timer = config.ink_duration;
        self.is_inked = true;
    }

    /// Refill ink charges (call at water sources)
    /// Jet charges now regen passively
    pub fn refill_charges(&mut self, config: &GameConfig) {
        self.ink_charges = config.ink_max_charges;
    }

    /// Find the nearest grapple point within range
    pub(super) fn find_nearest_grapple_point(
        &self,
        points: &[Vec2],
        config: &GameConfig,
    ) -> Option<Vec2> {
        let mut nearest: Option<(Vec2, f32)> = None;

        for &point in points {
            let dist = (point - self.position).length();

            // Must be within range
            if dist > config.grapple_range {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[Grapple] Point {:?} rejected: dist={:.1} > range={:.1}",
                    point, dist, config.grapple_range
                );
                continue;
            }

            // Must be above or at player level (can't grapple below)
            if point.y > self.position.y + 20.0 {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[Grapple] Point {:?} rejected: point.y={:.1} > player.y+20={:.1} (point is below player)",
                    point, point.y, self.position.y + 20.0
                );
                continue;
            }

            #[cfg(debug_assertions)]
            eprintln!(
                "[Grapple] Point {:?} is valid candidate: dist={:.1}",
                point, dist
            );

            // Check if this is the nearest
            match nearest {
                None => nearest = Some((point, dist)),
                Some((_, best_dist)) if dist < best_dist => nearest = Some((point, dist)),
                _ => {}
            }
        }

        nearest.map(|(p, _)| p)
    }

    /// Start swinging from a grapple point
    pub(super) fn start_grapple(&mut self, target: Vec2, _config: &GameConfig) {
        self.grapple_point = Some(target);
        self.rope_length = (target - self.position).length();
        self.state = PlayerState::Swinging;

        // Initial angular velocity based on current velocity
        // Angle is from vertical, so tangent direction is (cos(angle), -sin(angle))
        let to_player = self.position - target;
        let angle = to_player.x.atan2(to_player.y);
        let tangent = vec2(angle.cos(), -angle.sin());
        self.swing_angular_velocity = self.velocity.dot(tangent) / self.rope_length;
    }

    /// Release the grapple and preserve momentum
    pub(super) fn release_grapple(&mut self, config: &GameConfig) {
        // Boost velocity slightly on release
        self.velocity *= config.swing_release_boost;
        self.grapple_point = None;
        self.swing_angular_velocity = 0.0;

        // Transition to appropriate air state
        self.state = if self.velocity.y < 0.0 {
            PlayerState::Jumping
        } else {
            PlayerState::Falling
        };
    }

    /// Apply physics for ability states (jet, swing)
    pub(super) fn apply_ability_physics(
        &mut self,
        input: &InputState,
        config: &GameConfig,
        dt: f32,
    ) {
        match self.state {
            PlayerState::JetBoosting => {
                // Move in jet direction at jet speed (no gravity during boost)
                // Downward jets are faster (like the old dive)
                let speed = if self.jet_direction.y > 0.5 {
                    config.jet_boost_speed * config.jet_downward_speed_mult
                } else {
                    config.jet_boost_speed
                };
                self.velocity = self.jet_direction * speed;
            }

            PlayerState::Swinging => {
                if let Some(grapple) = self.grapple_point {
                    // Pendulum physics - angle measured from vertical (straight down = 0)
                    let to_player = self.position - grapple;

                    // atan2(x, y) gives angle from vertical: 0 = straight down, positive = right
                    let angle = to_player.x.atan2(to_player.y);

                    // Pendulum angular acceleration: α = -g/L * sin(θ)
                    let angular_accel = -(config.swing_gravity / self.rope_length) * angle.sin();

                    // Update angular velocity
                    self.swing_angular_velocity += angular_accel * dt;

                    // Apply frame-rate-independent damping
                    // damping^(dt*60) makes the damping factor behave consistently regardless of frame rate
                    // (swing_damping is calibrated for 60fps, so we scale by dt*60)
                    self.swing_angular_velocity *= config.swing_damping.powf(dt * 60.0);

                    // Player can pump the swing with left/right input
                    self.swing_angular_velocity += input.move_dir.x * config.swing_pump_strength * dt;

                    // Clamp angular velocity to prevent extreme swings (max ~180 degrees per second)
                    const MAX_ANGULAR_VELOCITY: f32 = std::f32::consts::PI;
                    self.swing_angular_velocity = self.swing_angular_velocity.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);

                    // Retract/extend rope with up/down
                    if input.move_dir.y < -config.input_deadzone {
                        self.rope_length =
                            (self.rope_length - config.rope_retract_speed * dt).max(config.rope_min_length);
                    } else if input.move_dir.y > config.input_deadzone {
                        self.rope_length = (self.rope_length + config.rope_retract_speed * dt)
                            .min(config.grapple_range);
                    }

                    // Calculate new angle
                    let new_angle = angle + self.swing_angular_velocity * dt;

                    // Calculate new position from angle
                    let new_pos = grapple + vec2(new_angle.sin(), new_angle.cos()) * self.rope_length;

                    // Derive velocity from position change (for momentum when releasing)
                    // Clamp dt to prevent division by near-zero
                    let safe_dt = dt.max(0.001); // Minimum 1ms
                    self.velocity = (new_pos - self.position) / safe_dt;
                    // Safety clamp - angular velocity clamping above should prevent this from triggering
                    // in normal gameplay, but protects against edge cases
                    self.velocity.x = self.velocity.x.clamp(-2000.0, 2000.0);
                    self.velocity.y = self.velocity.y.clamp(-2000.0, 2000.0);

                    // Update position
                    self.position = new_pos;
                }
            }

            _ => {}
        }
    }
}

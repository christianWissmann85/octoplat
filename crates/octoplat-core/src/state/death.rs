//! Death and respawn state management
//!
//! Handles the death animation state machine.

use crate::Vec2;

/// Manages player death state and respawn timing
#[derive(Clone, Debug, Default)]
pub struct DeathState {
    /// Whether the player is currently dead
    pub is_dead: bool,
    /// Time remaining in death animation
    pub timer: f32,
    /// Position where the player died (for death effect)
    pub position: Option<Vec2>,
}

impl DeathState {
    pub fn new() -> Self {
        Self {
            is_dead: false,
            timer: 0.0,
            position: None,
        }
    }

    /// Trigger death at the given position
    pub fn trigger(&mut self, position: Vec2, animation_time: f32) {
        self.is_dead = true;
        self.timer = animation_time;
        self.position = Some(position);
    }

    /// Update death timer, returns true when ready to respawn
    pub fn update(&mut self, dt: f32) -> bool {
        if self.is_dead {
            self.timer -= dt;
            if self.timer <= 0.0 {
                return true;
            }
        }
        false
    }

    /// Reset death state after respawn
    pub fn respawn(&mut self) {
        self.is_dead = false;
        self.position = None;
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn animation_progress(&self, animation_time: f32) -> f32 {
        if animation_time > 0.0 {
            1.0 - (self.timer / animation_time)
        } else {
            1.0
        }
    }
}

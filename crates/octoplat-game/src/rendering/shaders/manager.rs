//! Unified shader management
//!
//! Manages all shader effects and their states.

use macroquad::prelude::*;

use super::ChromaticAberration;

/// Manages all shader effects
pub struct ShaderManager {
    /// Chromatic aberration on hit
    pub chromatic: ChromaticAberration,
}

impl ShaderManager {
    /// Create a new shader manager, loading all available shaders
    pub fn new() -> Self {
        Self {
            chromatic: ChromaticAberration::new(),
        }
    }

    /// Update chromatic aberration effect
    pub fn update_chromatic(&mut self, dt: f32) {
        self.chromatic.update(dt);
    }

    /// Trigger chromatic aberration from a hit at the given screen position
    pub fn trigger_chromatic_hit(&mut self, screen_pos: Vec2, intensity: f32) {
        self.chromatic.trigger(screen_pos, intensity);
    }

    /// Apply chromatic aberration post-process (call after all rendering)
    pub fn apply_chromatic(&self) {
        self.chromatic.apply();
    }
}

impl Default for ShaderManager {
    fn default() -> Self {
        Self::new()
    }
}

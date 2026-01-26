//! Gameplay engine subsystem
//!
//! Coordinates player, level environment, camera, and core gameplay systems.

use macroquad::prelude::*;

use crate::camera::GameCamera;
use crate::compat::ToCoreVec2;
use crate::config::GameConfig;
use crate::level::{LevelEnvironment, LevelManager};
use crate::player::Player;
use octoplat_core::state::DeathState;

/// Core gameplay engine managing player, level, and camera.
///
/// This subsystem handles:
/// - Player entity and movement
/// - Level environment (gems, enemies, platforms)
/// - Camera tracking
/// - Death/respawn state
/// - Game configuration
pub struct GameplayEngine {
    /// Game configuration
    pub config: GameConfig,

    /// Player entity
    pub player: Player,

    /// Camera tracking
    pub camera: GameCamera,

    /// Level environment (gems, enemies, platforms, markers)
    pub level_env: LevelEnvironment,

    /// Death/respawn state
    pub death: DeathState,
}

impl GameplayEngine {
    /// Create a new GameplayEngine with default configuration
    pub fn new() -> Self {
        let config = GameConfig::default();

        Self {
            player: Player::new(vec2(100.0, 100.0), &config),
            camera: GameCamera::new(),
            level_env: LevelEnvironment::new(),
            death: DeathState::new(),
            config,
        }
    }

    /// Create a new GameplayEngine with specific configuration
    pub fn with_config(config: GameConfig) -> Self {
        Self {
            player: Player::new(vec2(100.0, 100.0), &config),
            camera: GameCamera::new(),
            level_env: LevelEnvironment::new(),
            death: DeathState::new(),
            config,
        }
    }

    /// Set up the level from level manager
    ///
    /// Resets player, level environment, and camera based on the current tilemap.
    pub fn setup_level(&mut self, level_manager: &LevelManager) {
        if let Some(tilemap) = level_manager.tilemap() {
            // Get spawn position (checkpoint or level start)
            let spawn = level_manager.get_spawn_position();

            // Reset player
            self.player = Player::new(spawn, &self.config);

            // Set up level environment from tilemap
            self.level_env.setup_from_tilemap(tilemap, &self.config);

            // Snap camera to player
            self.camera.snap_to(spawn);
        }
    }

    /// Respawn player at checkpoint or level start
    pub fn respawn_player(&mut self, level_manager: &LevelManager) {
        let spawn = level_manager.get_spawn_position();
        self.player = Player::new(spawn, &self.config);
        // Give brief invincibility after respawn
        self.player.start_invincibility(1.5);
        self.camera.snap_to(spawn);
        self.death.respawn();
        self.level_env.destroyed_blocks.clear();
        self.level_env.reset_enemies();
        self.level_env.reset_platforms();
    }

    /// Trigger player death
    ///
    /// Returns true if death was triggered (player wasn't already dead/invincible)
    pub fn trigger_death(&mut self) -> bool {
        // Check all invincibility sources
        if !self.death.is_dead && !self.player.is_inked && !self.player.is_invincible() {
            self.death.trigger(
                self.player.position.to_core_vec2(),
                self.config.death_animation_time,
            );
            self.player.trigger_hit_flash(&self.config);
            true
        } else {
            false
        }
    }

    /// Check if player can be hurt (not dead, not invincible)
    pub fn can_player_be_hurt(&self) -> bool {
        !self.death.is_dead && !self.player.is_inked && !self.player.is_invincible()
    }

    /// Update camera to follow player
    pub fn update_camera(&mut self, dt: f32, bounds: Rect) {
        self.camera.update(
            self.player.position,
            self.player.velocity,
            dt,
            bounds,
            &self.config,
        );
    }

    /// Get player position
    pub fn player_position(&self) -> Vec2 {
        self.player.position
    }

    /// Get screen center for effects (based on camera)
    pub fn screen_center(&self) -> Vec2 {
        vec2(screen_width() / 2.0, screen_height() / 2.0)
    }
}

impl Default for GameplayEngine {
    fn default() -> Self {
        Self::new()
    }
}

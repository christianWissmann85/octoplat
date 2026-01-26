use macroquad::prelude::*;

use crate::compat::vec2_from_mq;
use crate::config::GameConfig;
use octoplat_core::TileMap;

/// Crab enemy - patrols horizontally on platforms, turns at edges/walls
pub struct Crab {
    pub position: Vec2,
    pub start_position: Vec2,
    pub velocity: f32,
    pub facing_right: bool,
    pub alive: bool,

    // Animation state
    /// Timer for idle bob animation
    pub idle_bob_timer: f32,
    /// Timer for walk cycle animation
    pub walk_cycle: f32,
    /// Alert intensity (0-1) when player is near
    pub alert_intensity: f32,
    /// Timer for hit flash effect
    pub hit_flash_timer: f32,
}

impl Crab {
    pub fn new(position: Vec2, config: &GameConfig) -> Self {
        Self {
            position,
            start_position: position,
            velocity: config.crab_speed,
            facing_right: true,
            alive: true,
            idle_bob_timer: 0.0,
            walk_cycle: 0.0,
            alert_intensity: 0.0,
            hit_flash_timer: 0.0,
        }
    }

    pub fn update(&mut self, tilemap: &TileMap, config: &GameConfig, dt: f32) {
        if !self.alive {
            return;
        }

        // Update animation timers
        self.idle_bob_timer += dt * 3.0;
        self.walk_cycle += dt * 8.0;
        if self.hit_flash_timer > 0.0 {
            self.hit_flash_timer -= dt;
        }
        // Decay alert intensity when not updated externally
        self.alert_intensity = (self.alert_intensity - dt * 2.0).max(0.0);

        // Apply enemy speed multiplier from difficulty settings
        let speed_mult = config.enemy_speed_multiplier;
        let direction = if self.facing_right { 1.0 } else { -1.0 };
        let new_x = self.position.x + direction * self.velocity * speed_mult * dt;

        // Check for wall collision
        let check_pos = vec2(
            new_x + direction * 12.0,
            self.position.y,
        );
        let nearby_tiles = tilemap.get_nearby_solid_rects(vec2_from_mq(check_pos), 16.0);
        let hits_wall = nearby_tiles.iter().any(|tile| {
            let crab_rect = Rect::new(check_pos.x - 10.0, check_pos.y - 8.0, 20.0, 16.0);
            let tile_rect = Rect::new(tile.x, tile.y, tile.w, tile.h);
            crab_rect.overlaps(&tile_rect)
        });

        // Check for edge (no ground ahead)
        let ground_check_pos = vec2(
            new_x + direction * 14.0,
            self.position.y + 20.0,
        );
        let ground_tiles = tilemap.get_nearby_solid_rects(vec2_from_mq(ground_check_pos), 8.0);
        let has_ground = !ground_tiles.is_empty();

        if hits_wall || !has_ground {
            // Turn around
            self.facing_right = !self.facing_right;
        } else {
            self.position.x = new_x;
        }
    }

    /// Set alert state when player is near (called by game logic)
    pub fn set_alert(&mut self, intensity: f32) {
        self.alert_intensity = intensity.clamp(0.0, 1.0);
    }

    /// Trigger hit flash (called when crab is stomped)
    pub fn trigger_hit_flash(&mut self) {
        self.hit_flash_timer = 0.15;
    }

    /// Get the vertical bob offset for idle animation
    pub fn get_bob_offset(&self) -> f32 {
        self.idle_bob_timer.sin() * 1.5
    }

    /// Get the leg animation phase (0-1 for walk cycle)
    pub fn get_walk_phase(&self) -> f32 {
        (self.walk_cycle % std::f32::consts::TAU) / std::f32::consts::TAU
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(
            self.position.x - 12.0,
            self.position.y - 10.0,
            24.0,
            20.0,
        )
    }

    pub fn reset(&mut self) {
        self.position = self.start_position;
        self.facing_right = true;
        self.alive = true;
        self.idle_bob_timer = 0.0;
        self.walk_cycle = 0.0;
        self.alert_intensity = 0.0;
        self.hit_flash_timer = 0.0;
    }
}

/// Movement pattern for pufferfish
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PufferfishPattern {
    Stationary,
    Horizontal,
    Vertical,
}

/// Pufferfish enemy - floats in patterns, hazardous to touch
pub struct Pufferfish {
    pub position: Vec2,
    pub start_position: Vec2,
    pub pattern: PufferfishPattern,
    pub phase: f32,
    pub alive: bool,

    // Animation state
    /// Current visual scale (for puff up/deflate)
    pub visual_scale: f32,
    /// Target scale when threatened
    pub target_scale: f32,
    /// Rotation wobble phase
    pub wobble_phase: f32,
    /// Hit flash timer
    pub hit_flash_timer: f32,
}

impl Pufferfish {
    pub fn new(position: Vec2, pattern: PufferfishPattern) -> Self {
        Self {
            position,
            start_position: position,
            pattern,
            phase: 0.0,
            alive: true,
            visual_scale: 1.0,
            target_scale: 1.0,
            wobble_phase: 0.0,
            hit_flash_timer: 0.0,
        }
    }

    pub fn update(&mut self, config: &GameConfig, dt: f32) {
        if !self.alive {
            return;
        }

        // Apply enemy speed multiplier from difficulty settings
        let speed_mult = config.enemy_speed_multiplier;
        self.phase += dt * config.pufferfish_speed * speed_mult;

        // Update animation state
        self.wobble_phase += dt * 4.0;
        if self.hit_flash_timer > 0.0 {
            self.hit_flash_timer -= dt;
        }

        // Smoothly interpolate visual scale toward target
        let scale_speed = 4.0;
        let diff = self.target_scale - self.visual_scale;
        self.visual_scale += diff * scale_speed * dt;

        // Decay target scale back to normal when not actively threatened
        self.target_scale = 1.0 + (self.target_scale - 1.0) * 0.95_f32.powf(dt * 60.0);

        match self.pattern {
            PufferfishPattern::Stationary => {
                // Slight bobbing motion
                self.position.y = self.start_position.y + (self.phase * 2.0).sin() * 4.0;
            }
            PufferfishPattern::Horizontal => {
                self.position.x = self.start_position.x + self.phase.sin() * config.pufferfish_amplitude;
                self.position.y = self.start_position.y + (self.phase * 2.0).sin() * 4.0;
            }
            PufferfishPattern::Vertical => {
                self.position.x = self.start_position.x + (self.phase * 2.0).sin() * 4.0;
                self.position.y = self.start_position.y + self.phase.sin() * config.pufferfish_amplitude;
            }
        }
    }

    /// Puff up when threatened (player is near)
    pub fn puff_up(&mut self) {
        self.target_scale = 1.3; // Puff up to 130% size
    }

    /// Trigger hit flash (when defeated)
    pub fn trigger_hit_flash(&mut self) {
        self.hit_flash_timer = 0.15;
    }

    /// Get the current wobble rotation (radians)
    pub fn get_wobble_rotation(&self) -> f32 {
        self.wobble_phase.sin() * 0.1 // Subtle wobble
    }

    /// Get the pulse scale for gentle breathing effect
    pub fn get_pulse_scale(&self) -> f32 {
        1.0 + (self.phase * 1.5).sin() * 0.05
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(
            self.position.x - 14.0,
            self.position.y - 14.0,
            28.0,
            28.0,
        )
    }

    pub fn reset(&mut self) {
        self.position = self.start_position;
        self.phase = 0.0;
        self.alive = true;
        self.visual_scale = 1.0;
        self.target_scale = 1.0;
        self.wobble_phase = 0.0;
        self.hit_flash_timer = 0.0;
    }
}

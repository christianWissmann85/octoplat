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
}

impl Crab {
    pub fn new(position: Vec2, config: &GameConfig) -> Self {
        Self {
            position,
            start_position: position,
            velocity: config.crab_speed,
            facing_right: true,
            alive: true,
        }
    }

    pub fn update(&mut self, tilemap: &TileMap, _config: &GameConfig, dt: f32) {
        if !self.alive {
            return;
        }

        let direction = if self.facing_right { 1.0 } else { -1.0 };
        let new_x = self.position.x + direction * self.velocity * dt;

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
}

impl Pufferfish {
    pub fn new(position: Vec2, pattern: PufferfishPattern) -> Self {
        Self {
            position,
            start_position: position,
            pattern,
            phase: 0.0,
            alive: true,
        }
    }

    pub fn update(&mut self, config: &GameConfig, dt: f32) {
        if !self.alive {
            return;
        }

        self.phase += dt * config.pufferfish_speed;

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
    }
}

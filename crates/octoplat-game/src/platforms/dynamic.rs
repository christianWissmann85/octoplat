use macroquad::prelude::*;

use crate::config::GameConfig;

/// Moving platform that travels between two points
pub struct MovingPlatform {
    pub position: Vec2,
    pub start: Vec2,
    pub end: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    phase: f32,        // 0.0 to 1.0, position along path
    direction: f32,    // 1.0 = toward end, -1.0 = toward start
}

impl MovingPlatform {
    pub fn new(start: Vec2, end: Vec2, size: Vec2) -> Self {
        Self {
            position: start,
            start,
            end,
            size,
            velocity: Vec2::ZERO,
            phase: 0.0,
            direction: 1.0,
        }
    }

    pub fn update(&mut self, config: &GameConfig, dt: f32) {
        let path_length = (self.end - self.start).length();
        if path_length < 1.0 {
            return; // No movement needed
        }

        // Calculate phase change based on speed
        let phase_speed = config.moving_platform_speed / path_length;
        self.phase += self.direction * phase_speed * dt;

        // Reverse direction at endpoints
        if self.phase >= 1.0 {
            self.phase = 1.0;
            self.direction = -1.0;
        } else if self.phase <= 0.0 {
            self.phase = 0.0;
            self.direction = 1.0;
        }

        // Calculate new position
        let new_position = self.start.lerp(self.end, self.phase);

        // Calculate velocity for player riding
        self.velocity = (new_position - self.position) / dt;

        self.position = new_position;
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }
}

/// State of a crumbling platform
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CrumblingState {
    Stable,     // Normal, waiting for player
    Shaking,    // Player standing on it, about to fall
    Falling,    // Falling down
    Respawning, // Waiting to respawn
}

/// Crumbling platform that falls when stood on
pub struct CrumblingPlatform {
    pub position: Vec2,
    pub start_position: Vec2,
    pub size: Vec2,
    pub state: CrumblingState,
    pub timer: f32,
    fall_velocity: f32,
}

impl CrumblingPlatform {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            start_position: position,
            size,
            state: CrumblingState::Stable,
            timer: 0.0,
            fall_velocity: 0.0,
        }
    }

    pub fn update(&mut self, config: &GameConfig, dt: f32) {
        match self.state {
            CrumblingState::Stable => {
                // Waiting for player to stand on it
            }
            CrumblingState::Shaking => {
                self.timer -= dt;
                if self.timer <= 0.0 {
                    self.state = CrumblingState::Falling;
                    self.fall_velocity = 0.0;
                }
            }
            CrumblingState::Falling => {
                // Apply gravity and fall
                self.fall_velocity += config.gravity * dt;
                self.position.y += self.fall_velocity * dt;

                // After falling off screen, start respawn timer
                if self.position.y > self.start_position.y + 500.0 {
                    self.state = CrumblingState::Respawning;
                    self.timer = config.crumble_respawn_time;
                }
            }
            CrumblingState::Respawning => {
                self.timer -= dt;
                if self.timer <= 0.0 {
                    self.reset();
                }
            }
        }
    }

    /// Start crumbling when player stands on platform
    pub fn trigger(&mut self, config: &GameConfig) {
        if self.state == CrumblingState::Stable {
            self.state = CrumblingState::Shaking;
            self.timer = config.crumble_shake_time;
        }
    }

    pub fn reset(&mut self) {
        self.position = self.start_position;
        self.state = CrumblingState::Stable;
        self.timer = 0.0;
        self.fall_velocity = 0.0;
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }

    /// Check if platform is currently solid (can be stood on)
    pub fn is_solid(&self) -> bool {
        matches!(self.state, CrumblingState::Stable | CrumblingState::Shaking)
    }

    /// Get shake offset for rendering
    pub fn shake_offset(&self) -> Vec2 {
        if self.state == CrumblingState::Shaking {
            let intensity = 3.0 * (1.0 - self.timer / 0.5); // Intensify as timer runs out
            vec2(
                (self.timer * 50.0).sin() * intensity,
                (self.timer * 60.0).cos() * intensity * 0.5,
            )
        } else {
            Vec2::ZERO
        }
    }
}

use macroquad::prelude::*;

use crate::config::GameConfig;

pub struct GameCamera {
    pub position: Vec2,
    pub target: Vec2,
    pub zoom: f32,

    // Lookahead direction (smoothed)
    lookahead_direction: Vec2,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            target: Vec2::ZERO,
            zoom: 1.0,
            lookahead_direction: Vec2::ZERO,
        }
    }

    /// Update camera to follow a target position
    pub fn update(
        &mut self,
        target_pos: Vec2,
        target_velocity: Vec2,
        dt: f32,
        bounds: Rect,
        config: &GameConfig,
    ) {
        // Calculate lookahead based on movement direction
        if target_velocity.length_squared() > 100.0 {
            self.lookahead_direction = self
                .lookahead_direction
                .lerp(target_velocity.normalize(), dt * 3.0);
        }

        // Dynamic lookahead - increases with speed
        let speed = target_velocity.length();
        let dynamic_lookahead = config.camera_lookahead
            + speed * config.camera_lookahead_speed_mult;
        let lookahead_offset = self.lookahead_direction * dynamic_lookahead;

        // Vertical bias - look further down when falling
        let vertical_bias = if target_velocity.y > 100.0 {
            // Smoothly ramp up vertical bias based on fall speed
            let fall_factor = ((target_velocity.y - 100.0) / 400.0).clamp(0.0, 1.0);
            config.camera_vertical_bias * fall_factor
        } else {
            0.0
        };

        let desired_target = target_pos + lookahead_offset + vec2(0.0, vertical_bias);

        // Apply deadzone - only move if target leaves deadzone
        let diff = desired_target - self.target;
        let mut adjusted_target = self.target;

        if diff.x.abs() > config.camera_deadzone.x {
            adjusted_target.x = desired_target.x - config.camera_deadzone.x * diff.x.signum();
        }
        if diff.y.abs() > config.camera_deadzone.y {
            adjusted_target.y = desired_target.y - config.camera_deadzone.y * diff.y.signum();
        }

        // Smooth follow with exponential decay (frame-rate independent)
        let t = 1.0 - (-config.camera_smoothing * dt).exp();
        self.target = self.target.lerp(adjusted_target, t);

        // Clamp to level bounds
        let half_width = screen_width() / (2.0 * self.zoom);
        let half_height = screen_height() / (2.0 * self.zoom);

        // Only clamp if the viewport is smaller than the bounds
        self.position.x = if half_width * 2.0 < bounds.w {
            self.target
                .x
                .clamp(bounds.x + half_width, bounds.x + bounds.w - half_width)
        } else {
            bounds.x + bounds.w / 2.0 // Center in bounds
        };

        self.position.y = if half_height * 2.0 < bounds.h {
            self.target
                .y
                .clamp(bounds.y + half_height, bounds.y + bounds.h - half_height)
        } else {
            bounds.y + bounds.h / 2.0 // Center in bounds
        };
    }

    /// Get the macroquad Camera2D for rendering
    pub fn to_camera2d(&self) -> Camera2D {
        Camera2D {
            target: self.position,
            zoom: vec2(
                self.zoom * 2.0 / screen_width(),
                self.zoom * 2.0 / screen_height(), // Positive Y (screen coords)
            ),
            ..Default::default()
        }
    }

    /// Initialize camera to center on a position immediately
    pub fn snap_to(&mut self, position: Vec2) {
        self.position = position;
        self.target = position;
    }
}

impl Default for GameCamera {
    fn default() -> Self {
        Self::new()
    }
}

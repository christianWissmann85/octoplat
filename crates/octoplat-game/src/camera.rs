use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::rendering::easing::ease_in_out_cubic;

/// Zoom transition state
struct ZoomTransition {
    from: f32,
    to: f32,
    duration: f32,
    elapsed: f32,
}

pub struct GameCamera {
    pub position: Vec2,
    pub target: Vec2,
    pub zoom: f32,

    // Lookahead direction (smoothed)
    lookahead_direction: Vec2,

    // Zoom transition
    zoom_transition: Option<ZoomTransition>,

    // Base zoom level (to return to after effects)
    base_zoom: f32,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            target: Vec2::ZERO,
            zoom: 1.0,
            lookahead_direction: Vec2::ZERO,
            zoom_transition: None,
            base_zoom: 1.0,
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

    /// Start a zoom transition to a target zoom level
    ///
    /// The zoom will smoothly transition over the given duration using
    /// ease-in-out easing for a polished feel.
    pub fn zoom_to(&mut self, target_zoom: f32, duration: f32) {
        self.zoom_transition = Some(ZoomTransition {
            from: self.zoom,
            to: target_zoom,
            duration,
            elapsed: 0.0,
        });
    }

    /// Zoom in slightly (for dramatic moments like level complete)
    pub fn zoom_in_dramatic(&mut self) {
        self.zoom_to(1.15, 0.4);
    }

    /// Zoom out slightly (for death or wide view)
    pub fn zoom_out_dramatic(&mut self) {
        self.zoom_to(0.9, 0.3);
    }

    /// Return to base zoom level
    pub fn reset_zoom(&mut self) {
        self.zoom_to(self.base_zoom, 0.3);
    }

    /// Set the base zoom level (default zoom to return to)
    #[allow(dead_code)]
    pub fn set_base_zoom(&mut self, zoom: f32) {
        self.base_zoom = zoom;
    }

    /// Update zoom transition (call every frame)
    pub fn update_zoom(&mut self, dt: f32) {
        if let Some(ref mut transition) = self.zoom_transition {
            transition.elapsed += dt;
            let t = (transition.elapsed / transition.duration).min(1.0);

            // Use ease-in-out for smooth zoom
            let eased_t = ease_in_out_cubic(t);
            self.zoom = transition.from + (transition.to - transition.from) * eased_t;

            if t >= 1.0 {
                self.zoom = transition.to;
                self.zoom_transition = None;
            }
        }
    }

    /// Check if a zoom transition is in progress
    #[allow(dead_code)]
    pub fn is_zooming(&self) -> bool {
        self.zoom_transition.is_some()
    }
}

impl Default for GameCamera {
    fn default() -> Self {
        Self::new()
    }
}

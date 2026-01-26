//! Screen transition effects

use macroquad::prelude::*;

/// Screen transition state
pub struct Transition {
    /// Progress (0.0 = start, 1.0 = complete)
    pub progress: f32,
    /// Total duration
    pub duration: f32,
    /// Whether the midpoint has been reached (for state switch)
    pub midpoint_reached: bool,
}

impl Transition {
    pub fn new(duration: f32) -> Self {
        Self {
            progress: 0.0,
            duration,
            midpoint_reached: false,
        }
    }

    /// Update transition, returns true when complete
    pub fn update(&mut self, dt: f32) -> bool {
        self.progress += dt / self.duration;

        if self.progress >= 0.5 && !self.midpoint_reached {
            self.midpoint_reached = true;
        }

        self.progress >= 1.0
    }

    /// Get fade alpha (0.0 = visible, 1.0 = black)
    pub fn fade_alpha(&self) -> f32 {
        if self.progress < 0.5 {
            // Fade out
            self.progress * 2.0
        } else {
            // Fade in
            (1.0 - self.progress) * 2.0
        }
    }

    /// Check if we should switch states (at midpoint)
    pub fn should_switch(&self) -> bool {
        self.midpoint_reached
    }
}

/// Draw a fade overlay (black with alpha)
pub fn draw_fade_overlay(alpha: f32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, alpha),
    );
}

//! Visual effects system including particles and screen shake
//!
//! Provides satisfying visual feedback for game actions.
//!
//! The `EffectsController` provides a unified interface for all audio
//! and visual effects systems.

mod controller;
mod particles;
mod speed_lines;

pub use controller::EffectsController;
pub use particles::{BurstConfig, ParticleSystem};
pub use speed_lines::SpeedLines;

use macroquad::prelude::*;

/// Screen shake state with smooth dampening
#[derive(Default)]
pub struct ScreenShake {
    /// Current shake intensity
    intensity: f32,
    /// Time remaining for shake
    timer: f32,
    /// Maximum duration (for decay calculation)
    max_duration: f32,
    /// Accumulated time for smooth noise
    noise_time: f32,
    /// Current offset applied to camera
    pub offset: Vec2,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add screen shake with given intensity and duration
    pub fn add(&mut self, intensity: f32, duration: f32) {
        // Take the maximum intensity if already shaking
        if intensity > self.intensity {
            self.intensity = intensity;
            self.max_duration = duration;
            self.timer = duration;
        } else if duration > self.timer {
            // Extend duration if new shake is longer
            self.timer = duration;
            self.max_duration = self.max_duration.max(duration);
        }
    }

    /// Update shake state and calculate new offset with smooth dampening
    pub fn update(&mut self, dt: f32) {
        if self.timer > 0.0 {
            self.timer -= dt;
            self.noise_time += dt;

            // Calculate decay progress (0 = start, 1 = end)
            let progress = 1.0 - (self.timer / self.max_duration).max(0.0);

            // Use ease-out quadratic for smooth decay (fast initial shake, smooth fade)
            let decay = 1.0 - crate::rendering::easing::ease_out_quad(progress);

            // Current intensity with decay applied
            let current_intensity = self.intensity * decay;

            // Use layered sine waves for smooth, organic motion instead of pure random
            // This creates a more natural shake that doesn't feel jittery
            let freq1 = 25.0; // Primary shake frequency
            let freq2 = 41.0; // Secondary frequency (different to avoid patterns)
            let freq3 = 67.0; // Tertiary for added variation

            let noise_x = (self.noise_time * freq1).sin() * 0.6
                + (self.noise_time * freq2).cos() * 0.3
                + (self.noise_time * freq3).sin() * 0.1;

            let noise_y = (self.noise_time * freq1 * 1.1).cos() * 0.6
                + (self.noise_time * freq2 * 0.9).sin() * 0.3
                + (self.noise_time * freq3 * 1.2).cos() * 0.1;

            self.offset = vec2(
                noise_x * current_intensity,
                noise_y * current_intensity,
            );

            if self.timer <= 0.0 {
                self.intensity = 0.0;
                self.max_duration = 0.0;
                self.offset = Vec2::ZERO;
            }
        } else {
            self.offset = Vec2::ZERO;
        }
    }
}

/// Visual effect manager - holds all effect systems
pub struct EffectsManager {
    pub particles: ParticleSystem,
    pub shake: ScreenShake,
    pub speed_lines: SpeedLines,
}

impl EffectsManager {
    pub fn new() -> Self {
        Self {
            particles: ParticleSystem::new(),
            shake: ScreenShake::new(),
            speed_lines: SpeedLines::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.particles.update(dt);
        self.shake.update(dt);
        // Note: speed_lines.update() is called separately with player velocity
    }

    /// Update speed lines based on player velocity
    pub fn update_speed_lines(&mut self, dt: f32, player_vel: Vec2, is_jet_boosting: bool) {
        self.speed_lines.update(dt, player_vel, is_jet_boosting);
    }

    pub fn draw(&self) {
        self.particles.draw();
    }

    /// Draw speed lines in screen space (call after resetting camera)
    pub fn draw_speed_lines(&self) {
        self.speed_lines.draw();
    }

    // ========================================================================
    // Convenience methods for spawning common effects
    // ========================================================================

    /// Spawn jump dust puff
    pub fn spawn_jump(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 6,
                speed_range: (30.0, 80.0),
                angle_range: (std::f32::consts::PI * 0.6, std::f32::consts::PI * 1.4), // Downward spread
                lifetime: 0.3,
                size_range: (3.0, 6.0),
                color: Color::new(0.6, 0.5, 0.4, 0.8),
                gravity: 100.0,
                fade: true,
                shrink: true,
            },
        );
    }

    /// Spawn landing impact dust
    pub fn spawn_land(&mut self, pos: Vec2, intensity: f32) {
        let count = (6.0 + intensity * 8.0) as usize;
        self.particles.burst(
            pos,
            BurstConfig {
                count,
                speed_range: (40.0, 120.0 * intensity.min(1.5)),
                angle_range: (std::f32::consts::PI * 0.7, std::f32::consts::PI * 1.3), // Mostly horizontal
                lifetime: 0.4,
                size_range: (2.0, 5.0),
                color: Color::new(0.6, 0.5, 0.4, 0.7),
                gravity: 150.0,
                fade: true,
                shrink: true,
            },
        );

        // Add screen shake for hard landings
        if intensity > 0.5 {
            self.shake.add(intensity * 4.0, 0.1);
        }
    }

    /// Spawn wall jump burst
    pub fn spawn_wall_jump(&mut self, pos: Vec2, wall_dir: i8) {
        let angle_base = if wall_dir < 0 {
            0.0 // Burst going right (away from left wall)
        } else {
            std::f32::consts::PI // Burst going left (away from right wall)
        };

        self.particles.burst(
            pos,
            BurstConfig {
                count: 8,
                speed_range: (60.0, 120.0),
                angle_range: (angle_base - 0.8, angle_base + 0.8),
                lifetime: 0.35,
                size_range: (3.0, 6.0),
                color: Color::new(0.7, 0.8, 0.9, 0.8),
                gravity: 80.0,
                fade: true,
                shrink: true,
            },
        );
    }

    /// Spawn grapple attach splash
    pub fn spawn_grapple_attach(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 8,
                speed_range: (40.0, 100.0),
                angle_range: (0.0, std::f32::consts::TAU), // All directions
                lifetime: 0.3,
                size_range: (3.0, 6.0),
                color: Color::new(0.5, 0.8, 0.6, 0.9), // Tentacle green
                gravity: 60.0,
                fade: true,
                shrink: true,
            },
        );
    }

    /// Spawn gem collection sparkles
    pub fn spawn_gem_collect(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 12,
                speed_range: (60.0, 140.0),
                angle_range: (0.0, std::f32::consts::TAU), // All directions
                lifetime: 0.5,
                size_range: (2.0, 5.0),
                color: Color::new(1.0, 0.9, 0.3, 1.0), // Gold
                gravity: -20.0, // Float upward slightly
                fade: true,
                shrink: false,
            },
        );
    }

    /// Spawn checkpoint activation effect
    pub fn spawn_checkpoint(&mut self, pos: Vec2) {
        // Ring of particles
        self.particles.burst(
            pos,
            BurstConfig {
                count: 16,
                speed_range: (80.0, 120.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.6,
                size_range: (3.0, 5.0),
                color: Color::new(0.3, 1.0, 0.5, 0.9), // Green
                gravity: 0.0,
                fade: true,
                shrink: true,
            },
        );
    }

    /// Spawn bounce pad effect
    pub fn spawn_bounce(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 10,
                speed_range: (100.0, 200.0),
                angle_range: (-std::f32::consts::PI * 0.8, -std::f32::consts::PI * 0.2), // Upward
                lifetime: 0.4,
                size_range: (3.0, 6.0),
                color: Color::new(1.0, 0.5, 0.7, 0.9), // Pink
                gravity: 200.0,
                fade: true,
                shrink: true,
            },
        );

        self.shake.add(2.0, 0.08);
    }

    /// Spawn dive impact effect
    pub fn spawn_dive_impact(&mut self, pos: Vec2) {
        // Radial dust cloud
        self.particles.burst(
            pos,
            BurstConfig {
                count: 14,
                speed_range: (80.0, 180.0),
                angle_range: (std::f32::consts::PI * 0.6, std::f32::consts::PI * 1.4), // Horizontal spread
                lifetime: 0.5,
                size_range: (4.0, 8.0),
                color: Color::new(0.5, 0.4, 0.3, 0.8),
                gravity: 100.0,
                fade: true,
                shrink: true,
            },
        );

        self.shake.add(4.0, 0.1);
    }

    /// Spawn death effect (enhanced)
    pub fn spawn_death(&mut self, pos: Vec2) {
        // Explosion of particles
        self.particles.burst(
            pos,
            BurstConfig {
                count: 20,
                speed_range: (100.0, 250.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.8,
                size_range: (4.0, 10.0),
                color: Color::new(0.9, 0.3, 0.3, 0.9), // Red
                gravity: 150.0,
                fade: true,
                shrink: true,
            },
        );

        // Secondary splash
        self.particles.burst(
            pos,
            BurstConfig {
                count: 12,
                speed_range: (50.0, 120.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.6,
                size_range: (2.0, 5.0),
                color: Color::new(0.4, 0.6, 0.8, 0.7), // Blue ink
                gravity: 100.0,
                fade: true,
                shrink: true,
            },
        );

        self.shake.add(10.0, 0.25);
    }

    /// Spawn jet boost trail
    pub fn spawn_jet_boost(&mut self, pos: Vec2, direction: Vec2) {
        let angle = direction.y.atan2(direction.x) + std::f32::consts::PI; // Opposite direction

        self.particles.burst(
            pos,
            BurstConfig {
                count: 6,
                speed_range: (80.0, 150.0),
                angle_range: (angle - 0.4, angle + 0.4),
                lifetime: 0.3,
                size_range: (3.0, 7.0),
                color: Color::new(0.3, 0.7, 1.0, 0.8), // Cyan
                gravity: 50.0,
                fade: true,
                shrink: true,
            },
        );
    }

    /// Spawn ink cloud effect
    pub fn spawn_ink_cloud(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 15,
                speed_range: (30.0, 80.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.8,
                size_range: (5.0, 12.0),
                color: Color::new(0.1, 0.1, 0.2, 0.6), // Dark ink
                gravity: -30.0, // Float slightly
                fade: true,
                shrink: false,
            },
        );
    }

    /// Spawn player hurt flash effect
    pub fn spawn_hurt(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 8,
                speed_range: (60.0, 120.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.25,
                size_range: (3.0, 6.0),
                color: Color::new(1.0, 0.4, 0.4, 0.9),
                gravity: 80.0,
                fade: true,
                shrink: true,
            },
        );

        self.shake.add(6.0, 0.15);
    }

    /// Spawn extra life (1UP) effect - golden particles bursting upward
    pub fn spawn_extra_life(&mut self, pos: Vec2) {
        self.particles.burst(
            pos,
            BurstConfig {
                count: 20,
                speed_range: (80.0, 160.0),
                angle_range: (-std::f32::consts::PI * 0.8, -std::f32::consts::PI * 0.2), // Upward burst
                lifetime: 0.6,
                size_range: (4.0, 8.0),
                color: Color::new(1.0, 0.85, 0.2, 1.0), // Golden
                gravity: 150.0,
                fade: true,
                shrink: false,
            },
        );

        // Light celebratory screen shake
        self.shake.add(3.0, 0.15);
    }

    /// Spawn crab defeat effect - orange shell fragments bursting outward
    pub fn spawn_crab_defeat(&mut self, pos: Vec2) {
        // Primary burst - orange/red shell fragments
        self.particles.burst(
            pos,
            BurstConfig {
                count: 14,
                speed_range: (80.0, 180.0),
                angle_range: (0.0, std::f32::consts::TAU), // All directions
                lifetime: 0.5,
                size_range: (4.0, 8.0),
                color: Color::new(0.9, 0.4, 0.2, 0.95), // Orange-red
                gravity: 200.0,
                fade: true,
                shrink: true,
            },
        );

        // Secondary burst - smaller debris
        self.particles.burst(
            pos,
            BurstConfig {
                count: 8,
                speed_range: (40.0, 100.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.4,
                size_range: (2.0, 4.0),
                color: Color::new(0.7, 0.3, 0.1, 0.8), // Darker orange
                gravity: 150.0,
                fade: true,
                shrink: true,
            },
        );

        // Satisfying defeat screen shake
        self.shake.add(4.0, 0.12);
    }

    /// Spawn pufferfish defeat effect - blue deflation burst with pop
    pub fn spawn_pufferfish_defeat(&mut self, pos: Vec2) {
        // Primary burst - blue/purple deflation particles spiraling out
        self.particles.burst(
            pos,
            BurstConfig {
                count: 16,
                speed_range: (100.0, 200.0),
                angle_range: (0.0, std::f32::consts::TAU), // All directions
                lifetime: 0.45,
                size_range: (5.0, 10.0),
                color: Color::new(0.4, 0.6, 0.9, 0.9), // Blue
                gravity: 80.0,
                fade: true,
                shrink: true,
            },
        );

        // Secondary "pop" particles - small white sparkles
        self.particles.burst(
            pos,
            BurstConfig {
                count: 10,
                speed_range: (60.0, 140.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.3,
                size_range: (2.0, 4.0),
                color: Color::new(0.9, 0.95, 1.0, 0.95), // White-ish
                gravity: -20.0, // Float slightly
                fade: true,
                shrink: false,
            },
        );

        // Tertiary - spines flying off
        self.particles.burst(
            pos,
            BurstConfig {
                count: 6,
                speed_range: (120.0, 200.0),
                angle_range: (0.0, std::f32::consts::TAU),
                lifetime: 0.5,
                size_range: (3.0, 5.0),
                color: Color::new(0.6, 0.5, 0.8, 0.85), // Purple-ish
                gravity: 180.0,
                fade: true,
                shrink: true,
            },
        );

        // Satisfying pop shake
        self.shake.add(5.0, 0.1);
    }
}

impl Default for EffectsManager {
    fn default() -> Self {
        Self::new()
    }
}

//! Particle system for visual effects

use std::collections::VecDeque;
use macroquad::prelude::*;

/// Configuration for a particle burst
#[derive(Clone)]
pub struct BurstConfig {
    /// Number of particles to spawn
    pub count: usize,
    /// Speed range (min, max)
    pub speed_range: (f32, f32),
    /// Angle range in radians (min, max)
    pub angle_range: (f32, f32),
    /// Lifetime in seconds
    pub lifetime: f32,
    /// Size range (min, max)
    pub size_range: (f32, f32),
    /// Particle color
    pub color: Color,
    /// Gravity (positive = down, negative = up)
    pub gravity: f32,
    /// Whether particles fade out over lifetime
    pub fade: bool,
    /// Whether particles shrink over lifetime
    pub shrink: bool,
}

impl Default for BurstConfig {
    fn default() -> Self {
        Self {
            count: 10,
            speed_range: (50.0, 100.0),
            angle_range: (0.0, std::f32::consts::TAU),
            lifetime: 0.5,
            size_range: (3.0, 6.0),
            color: WHITE,
            gravity: 100.0,
            fade: true,
            shrink: true,
        }
    }
}

/// A single particle
#[derive(Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub initial_size: f32,
    pub color: Color,
    pub initial_alpha: f32,
    pub gravity: f32,
    pub fade: bool,
    pub shrink: bool,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, config: &BurstConfig) -> Self {
        let size = rand::gen_range(config.size_range.0, config.size_range.1);
        Self {
            position,
            velocity,
            lifetime: config.lifetime,
            max_lifetime: config.lifetime,
            size,
            initial_size: size,
            color: config.color,
            initial_alpha: config.color.a,
            gravity: config.gravity,
            fade: config.fade,
            shrink: config.shrink,
        }
    }

    /// Update particle, returns true if still alive
    pub fn update(&mut self, dt: f32) -> bool {
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            return false;
        }

        // Apply gravity
        self.velocity.y += self.gravity * dt;

        // Apply velocity
        self.position += self.velocity * dt;

        // Calculate life ratio (1.0 at start, 0.0 at end)
        let life_ratio = self.lifetime / self.max_lifetime;

        // Fade out
        if self.fade {
            self.color.a = self.initial_alpha * life_ratio;
        }

        // Shrink
        if self.shrink {
            self.size = self.initial_size * life_ratio;
        }

        true
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
    }
}

/// Particle system that manages multiple particles
///
/// Uses VecDeque for O(1) removal of oldest particles when at capacity.
pub struct ParticleSystem {
    particles: VecDeque<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: VecDeque::with_capacity(500),
            max_particles: 500,
        }
    }

    /// Spawn a burst of particles at a position
    pub fn burst(&mut self, position: Vec2, config: BurstConfig) {
        for _ in 0..config.count {
            // Don't exceed max particles - remove oldest (O(1) with VecDeque)
            if self.particles.len() >= self.max_particles {
                self.particles.pop_front();
            }

            // Random angle within range
            let angle = rand::gen_range(config.angle_range.0, config.angle_range.1);
            let speed = rand::gen_range(config.speed_range.0, config.speed_range.1);

            let velocity = vec2(angle.cos() * speed, angle.sin() * speed);

            self.particles.push_back(Particle::new(position, velocity, &config));
        }
    }

    /// Spawn a single particle (for trails)
    #[allow(dead_code)]
    pub fn spawn(&mut self, position: Vec2, velocity: Vec2, config: &BurstConfig) {
        // Remove oldest if at capacity (O(1) with VecDeque)
        if self.particles.len() >= self.max_particles {
            self.particles.pop_front();
        }

        self.particles.push_back(Particle::new(position, velocity, config));
    }

    /// Update all particles
    pub fn update(&mut self, dt: f32) {
        // VecDeque doesn't have retain_mut, so we update in place and collect dead indices
        let mut i = 0;
        while i < self.particles.len() {
            if !self.particles[i].update(dt) {
                self.particles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Draw all particles
    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    /// Get current particle count
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.particles.len()
    }

    /// Clear all particles
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.particles.clear();
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

//! Speed lines effect for fast movement and jet boost
//!
//! Creates directional streak lines that appear at screen edges when
//! the player moves fast, enhancing the sense of speed.

use macroquad::prelude::*;

/// A single speed line
struct SpeedLine {
    /// Start position (screen space)
    start: Vec2,
    /// Direction the line moves (normalized)
    direction: Vec2,
    /// Current length of the line
    length: f32,
    /// Maximum length
    max_length: f32,
    /// Current alpha (0.0 to 1.0)
    alpha: f32,
    /// Time remaining
    lifetime: f32,
    /// Total lifetime for fade calculation
    max_lifetime: f32,
    /// Line thickness
    thickness: f32,
}

impl SpeedLine {
    fn update(&mut self, dt: f32) -> bool {
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            return false;
        }

        // Calculate life ratio (1.0 at start, 0.0 at end)
        let life_ratio = self.lifetime / self.max_lifetime;

        // Grow quickly at start, then maintain
        let grow_progress = (1.0 - life_ratio) * 3.0; // Grow in first third
        self.length = self.max_length * grow_progress.min(1.0);

        // Fade out in last half
        self.alpha = if life_ratio < 0.5 {
            life_ratio * 2.0 // Fade from 1.0 to 0.0 in last half
        } else {
            1.0
        };

        // Move the line along its direction
        let speed = 800.0; // Pixels per second
        self.start += self.direction * speed * dt;

        true
    }

    fn draw(&self) {
        if self.alpha < 0.01 || self.length < 1.0 {
            return;
        }

        let end = self.start + self.direction * self.length;
        let color = Color::new(1.0, 1.0, 1.0, self.alpha * 0.6);

        draw_line(
            self.start.x,
            self.start.y,
            end.x,
            end.y,
            self.thickness,
            color,
        );
    }
}

/// Speed lines effect manager
pub struct SpeedLines {
    lines: Vec<SpeedLine>,
    spawn_timer: f32,
    /// Whether speed lines are currently active
    active: bool,
    /// Current movement direction (for spawning)
    movement_direction: Vec2,
}

impl SpeedLines {
    pub fn new() -> Self {
        Self {
            lines: Vec::with_capacity(50),
            spawn_timer: 0.0,
            active: false,
            movement_direction: Vec2::ZERO,
        }
    }

    /// Update speed lines based on player velocity
    ///
    /// Call this every frame with player velocity. Speed lines will automatically
    /// activate when speed threshold is exceeded or during jet boost.
    pub fn update(&mut self, dt: f32, player_vel: Vec2, is_jet_boosting: bool) {
        // Threshold for speed lines (pixels/second)
        const SPEED_THRESHOLD: f32 = 350.0;
        const JET_THRESHOLD: f32 = 200.0; // Lower threshold during jet boost

        let speed = player_vel.length();
        let threshold = if is_jet_boosting { JET_THRESHOLD } else { SPEED_THRESHOLD };

        // Determine if we should be active
        self.active = speed > threshold || is_jet_boosting;

        if self.active && speed > 10.0 {
            // Store normalized movement direction
            self.movement_direction = player_vel.normalize();

            // Spawn rate based on speed (faster = more lines)
            let spawn_rate = if is_jet_boosting {
                0.02 // Very fast spawning during jet
            } else {
                0.05 + (1.0 - speed / 800.0).max(0.0) * 0.1 // 0.05 to 0.15 based on speed
            };

            self.spawn_timer -= dt;
            if self.spawn_timer <= 0.0 {
                self.spawn_line(speed, is_jet_boosting);
                self.spawn_timer = spawn_rate;
            }
        }

        // Update existing lines
        self.lines.retain_mut(|line| line.update(dt));
    }

    /// Spawn a new speed line at screen edge
    fn spawn_line(&mut self, speed: f32, is_jet_boosting: bool) {
        let sw = screen_width();
        let sh = screen_height();

        // Lines come from the direction we're moving toward
        // (appear ahead of player and streak past)
        let dir = self.movement_direction;

        // Determine spawn edge based on movement direction
        let (start, line_dir) = self.calculate_spawn_position(dir, sw, sh);

        // Line properties based on speed
        let speed_factor = (speed / 600.0).min(1.5);
        let base_length = if is_jet_boosting { 80.0 } else { 40.0 };

        let line = SpeedLine {
            start,
            direction: line_dir,
            length: 0.0,
            max_length: base_length + rand::gen_range(20.0, 60.0) * speed_factor,
            alpha: 1.0,
            lifetime: 0.3 + rand::gen_range(0.0, 0.2),
            max_lifetime: 0.3 + rand::gen_range(0.0, 0.2),
            thickness: if is_jet_boosting {
                rand::gen_range(2.0, 4.0)
            } else {
                rand::gen_range(1.0, 2.5)
            },
        };

        // Limit total lines
        if self.lines.len() < 50 {
            self.lines.push(line);
        }
    }

    /// Calculate spawn position and direction for a speed line
    fn calculate_spawn_position(&self, move_dir: Vec2, sw: f32, sh: f32) -> (Vec2, Vec2) {
        // Margin from screen center where lines can spawn
        let margin = 50.0;

        // Determine primary axis of movement
        let abs_x = move_dir.x.abs();
        let abs_y = move_dir.y.abs();

        if abs_x > abs_y {
            // Horizontal movement dominant - spawn from left/right edges
            let spawn_x = if move_dir.x > 0.0 {
                sw + rand::gen_range(10.0, 50.0) // Spawn off right edge
            } else {
                -rand::gen_range(10.0, 50.0) // Spawn off left edge
            };
            let spawn_y = rand::gen_range(margin, sh - margin);
            let line_dir = vec2(-move_dir.x.signum(), rand::gen_range(-0.2, 0.2)).normalize();
            (vec2(spawn_x, spawn_y), line_dir)
        } else {
            // Vertical movement dominant - spawn from top/bottom edges
            let spawn_x = rand::gen_range(margin, sw - margin);
            let spawn_y = if move_dir.y > 0.0 {
                sh + rand::gen_range(10.0, 50.0) // Spawn off bottom edge
            } else {
                -rand::gen_range(10.0, 50.0) // Spawn off top edge
            };
            let line_dir = vec2(rand::gen_range(-0.2, 0.2), -move_dir.y.signum()).normalize();
            (vec2(spawn_x, spawn_y), line_dir)
        }
    }

    /// Draw all speed lines (call in screen space, after resetting camera)
    pub fn draw(&self) {
        for line in &self.lines {
            line.draw();
        }
    }

    /// Check if speed lines are currently active
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.active || !self.lines.is_empty()
    }
}

impl Default for SpeedLines {
    fn default() -> Self {
        Self::new()
    }
}

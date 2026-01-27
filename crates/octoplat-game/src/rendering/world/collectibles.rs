//! Collectible and interactive element rendering

use macroquad::prelude::*;

use crate::collectibles::Gem;
use crate::config::GameConfig;
use crate::player::{Player, PlayerState};
use crate::rendering::shaders::{draw_bloom, draw_bloom_pulsing};

/// Draw grapple points (hooks)
pub fn draw_grapple_points(points: &[Vec2], player: &Player, config: &GameConfig) {
    for &point in points {
        let dist = (point - player.position).length();
        let in_range = dist <= config.grapple_range;

        // Color based on whether in range
        let color = if in_range {
            Color::new(1.0, 0.8, 0.2, 1.0)
        } else {
            Color::new(0.5, 0.4, 0.2, 0.5)
        };

        // Draw hook shape (circle with inner circle)
        draw_circle(point.x, point.y, 8.0, color);
        draw_circle(point.x, point.y, 5.0, Color::new(0.3, 0.25, 0.1, 1.0));

        // Draw range indicator when close
        if in_range && player.state != PlayerState::Swinging {
            draw_circle_lines(point.x, point.y, 12.0, 1.0, Color::new(1.0, 0.8, 0.2, 0.5));
        }
    }
}

/// Draw a gem as a diamond shape with bloom effect
pub fn draw_gem(gem: &Gem, time: f32) {
    if !gem.collected {
        let pos = gem.render_position(time);

        // Sparkle effect
        let sparkle = (time * 4.0).sin() * 0.3 + 0.7;
        let gem_color = Color::new(0.3 * sparkle, 0.8 * sparkle, 1.0 * sparkle, 1.0);

        // Enhanced bloom effect (increased for better visibility)
        draw_bloom_pulsing(
            pos,
            27.0,
            Color::new(0.3, 0.8, 1.0, 1.0),
            1.0,
            time,
            4.0,
        );

        // Diamond shape (rotated square)
        let size = 14.0;
        draw_poly(pos.x, pos.y, 4, size, 45.0, gem_color);

        // Inner highlight with bloom
        draw_poly(pos.x - 2.0, pos.y - 2.0, 4, size * 0.4, 45.0, WHITE);

        // Extra sparkle bloom
        let sparkle_offset = vec2((time * 3.0).cos() * 4.0, (time * 2.5).sin() * 4.0);
        draw_bloom(pos + sparkle_offset, 4.0, Color::new(1.0, 1.0, 1.0, 0.6 * sparkle), 0.5);
    }
}

/// Draw checkpoints (save points) with enhanced bloom
pub fn draw_checkpoints(positions: &[Vec2], active_checkpoint: Option<Vec2>, time: f32) {
    for &pos in positions {
        let is_active = active_checkpoint.map(|cp| (cp - pos).length() < 5.0).unwrap_or(false);

        // Enhanced bloom when active (draw first, behind flag)
        if is_active {
            let glow_center = vec2(pos.x + 8.0, pos.y - 10.0);

            // Use bloom effect for active checkpoint
            draw_bloom_pulsing(
                glow_center,
                28.0,
                Color::new(0.2, 1.0, 0.4, 0.9),
                1.0,
                time,
                3.0,
            );
        }

        // Flag pole
        draw_rectangle(pos.x - 2.0, pos.y - 20.0, 4.0, 24.0, Color::new(0.4, 0.3, 0.2, 1.0));

        // Flag (animated wave when active)
        let wave = if is_active {
            (time * 5.0).sin() * 3.0
        } else {
            0.0
        };

        let flag_color = if is_active {
            Color::new(0.2, 0.9, 0.3, 1.0) // Green when active
        } else {
            Color::new(0.7, 0.7, 0.7, 0.8) // Gray when inactive
        };

        draw_triangle(
            vec2(pos.x + 2.0, pos.y - 18.0),
            vec2(pos.x + 2.0, pos.y - 6.0),
            vec2(pos.x + 16.0 + wave, pos.y - 12.0),
            flag_color,
        );
    }
}

/// Draw level exit with enhanced bloom effect
pub fn draw_exit(position: Option<Vec2>, time: f32) {
    if let Some(pos) = position {
        // Pulsing portal effect
        let pulse = (time * 3.0).sin() * 0.2 + 0.8;
        let size = 16.0 * pulse;

        // Enhanced bloom effect (golden glow)
        draw_bloom_pulsing(
            pos,
            40.0,
            Color::new(1.0, 0.85, 0.3, 0.9),
            1.2,
            time,
            3.0,
        );

        // Outer glow ring
        draw_circle(pos.x, pos.y, size + 8.0, Color::new(0.9, 0.7, 0.2, 0.4));

        // Main portal
        draw_circle(pos.x, pos.y, size, Color::new(1.0, 0.8, 0.2, 0.9));

        // Inner bright core with bloom
        draw_bloom(pos, size * 0.4, Color::new(1.0, 1.0, 0.9, 1.0), 0.8);
        draw_circle(pos.x, pos.y, size * 0.5, Color::new(1.0, 1.0, 0.8, 1.0));

        // Rotating particles with individual bloom
        for ring in 0..2 {
            let ring_radius = size + 4.0 + ring as f32 * 8.0;
            let particle_count = 4 + ring * 2;
            let ring_speed = 2.0 - ring as f32 * 0.5;
            for i in 0..particle_count {
                let angle = time * ring_speed + (i as f32) * std::f32::consts::TAU / particle_count as f32;
                let particle_pos = pos + vec2(angle.cos(), angle.sin()) * ring_radius;
                let particle_pulse = (time * 3.0 + i as f32 * 0.5).sin() * 0.3 + 0.7;

                // Add bloom to orbiting particles
                draw_bloom(particle_pos, 5.0, Color::new(1.0, 0.95, 0.6, 0.7), 0.4 * particle_pulse);
                draw_circle(particle_pos.x, particle_pos.y, 3.0 * particle_pulse, Color::new(1.0, 0.95, 0.6, 0.8 * particle_pulse));
            }
        }
    }
}

/// Draw water pools (charge refill points)
pub fn draw_water_pools(positions: &[Vec2], time: f32) {
    for &pos in positions {
        // Water surface with wave animation
        let wave1 = (time * 2.0).sin() * 2.0;
        let wave2 = (time * 2.5 + 1.0).sin() * 1.5;

        // Pool base
        draw_rectangle(
            pos.x - 14.0,
            pos.y - 4.0,
            28.0,
            12.0,
            Color::new(0.1, 0.3, 0.5, 0.8),
        );

        // Water surface waves
        draw_ellipse(pos.x, pos.y + wave1, 12.0, 4.0, 0.0, Color::new(0.3, 0.6, 0.9, 0.7));
        draw_ellipse(pos.x + 4.0, pos.y - 2.0 + wave2, 8.0, 3.0, 0.0, Color::new(0.4, 0.7, 1.0, 0.6));

        // Sparkle
        let sparkle = ((time * 4.0).sin() + 1.0) * 0.5;
        draw_circle(
            pos.x - 4.0,
            pos.y - 2.0,
            2.0 * sparkle,
            Color::new(0.8, 0.9, 1.0, sparkle),
        );
    }
}

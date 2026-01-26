//! Visual effects rendering

use macroquad::prelude::*;

use octoplat_core::procgen::BiomeTheme;

use crate::compat::ToMqColor;

/// Draw death effect (expanding ring and particles)
pub fn draw_death_effect(position: Vec2, progress: f32) {
    let alpha = 1.0 - progress;

    // Expanding ring
    let radius = progress * 60.0;
    draw_circle_lines(
        position.x,
        position.y,
        radius,
        3.0,
        Color::new(1.0, 0.3, 0.3, alpha),
    );

    // Inner ring
    let inner_radius = progress * 40.0;
    draw_circle_lines(
        position.x,
        position.y,
        inner_radius,
        2.0,
        Color::new(1.0, 0.5, 0.5, alpha * 0.7),
    );

    // Particle burst
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::PI / 4.0;
        let dist = progress * 50.0;
        let particle_pos = position + vec2(angle.cos(), angle.sin()) * dist;
        let particle_size = 5.0 * (1.0 - progress);
        draw_circle(
            particle_pos.x,
            particle_pos.y,
            particle_size,
            Color::new(1.0, 0.4, 0.4, alpha),
        );
    }
}

/// Draw a biome-themed background gradient
pub fn draw_biome_background(theme: &BiomeTheme) {
    let sw = screen_width();
    let sh = screen_height();

    // Draw vertical gradient using theme's bg_color_at helper
    let steps = 20;
    let step_height = sh / steps as f32;

    for i in 0..steps {
        let y = i as f32 * step_height;
        let t = i as f32 / steps as f32;
        let color = theme.bg_color_at(t).to_mq_color();
        draw_rectangle(0.0, y, sw, step_height + 1.0, color);
    }
}

/// Draw ambient particles based on biome theme
pub fn draw_biome_particles(theme: &BiomeTheme, time: f32, particle_count: u32) {
    let sw = screen_width();
    let sh = screen_height();

    for i in 0..particle_count {
        // Create deterministic but moving positions
        let seed = i as f32 * 123.456;
        let x = ((seed * 7.89 + time * 20.0) % sw).abs();
        let y = ((seed * 3.21 + time * 10.0 + (seed * 0.5).sin() * 50.0) % sh).abs();
        let size = 1.0 + (seed * 0.1).sin().abs() * 2.0;
        let alpha = 0.3 + (seed + time).sin().abs() * 0.4;

        let color = Color::new(
            theme.particle_color.r,
            theme.particle_color.g,
            theme.particle_color.b,
            theme.particle_color.a * alpha,
        );

        draw_circle(x, y, size, color);
    }
}

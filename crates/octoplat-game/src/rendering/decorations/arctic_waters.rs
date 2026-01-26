//! Arctic Waters biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_ice_shard(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let shimmer = (time * 2.0 + deco.phase * std::f32::consts::TAU).sin() * 0.15 + 0.85;
    let color = Color::new(
        (solid.r * shimmer).min(1.0),
        (solid.g * shimmer).min(1.0),
        solid.b,
        0.85,
    );

    // Main ice spike
    let height = (15.0 + deco.variant as f32 * 4.0) * scale;
    let width = 5.0 * scale;
    draw_triangle(
        vec2(pos.x - width, pos.y),
        vec2(pos.x + width, pos.y),
        vec2(pos.x + (deco.variant as f32 * 0.5).sin() * 2.0, pos.y - height),
        color,
    );

    // Highlight facet
    let highlight = Color::new(1.0, 1.0, 1.0, 0.4 * shimmer);
    draw_triangle(
        vec2(pos.x - width * 0.3, pos.y - height * 0.1),
        vec2(pos.x + width * 0.2, pos.y - height * 0.1),
        vec2(pos.x - width * 0.1, pos.y - height * 0.9),
        highlight,
    );
}

pub fn draw_snowflake(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let particle = color_to_mq(theme.particle_color);
    let color = Color::new(particle.r, particle.g, particle.b, 0.7);

    // Floating animation
    let drift_x = (time * 0.5 + deco.phase * std::f32::consts::TAU).sin() * 5.0 * scale;
    let drift_y = ((time * 0.3 + deco.phase) % 2.0) * 15.0 * scale;
    let rotation = time * 0.5 + deco.phase * std::f32::consts::PI;

    let pos = vec2(base.x + drift_x, base.y + drift_y);
    let size = (3.0 + deco.variant as f32) * scale;

    // 6-pointed snowflake
    for i in 0..6 {
        let angle = rotation + (i as f32 / 6.0) * std::f32::consts::TAU;
        let end_x = pos.x + angle.cos() * size;
        let end_y = pos.y + angle.sin() * size;
        draw_line(pos.x, pos.y, end_x, end_y, 1.0 * scale, color);

        // Small branches
        let mid_x = pos.x + angle.cos() * size * 0.6;
        let mid_y = pos.y + angle.sin() * size * 0.6;
        let branch_angle1 = angle + 0.5;
        let branch_angle2 = angle - 0.5;
        draw_line(mid_x, mid_y, mid_x + branch_angle1.cos() * size * 0.3, mid_y + branch_angle1.sin() * size * 0.3, 0.8 * scale, color);
        draw_line(mid_x, mid_y, mid_x + branch_angle2.cos() * size * 0.3, mid_y + branch_angle2.sin() * size * 0.3, 0.8 * scale, color);
    }
}

pub fn draw_frosted_rock(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let rock_color = Color::new(
        solid.r * 0.7,
        solid.g * 0.75,
        solid.b * 0.8,
        0.9,
    );

    // Rock base
    let width = (10.0 + deco.variant as f32 * 2.0) * scale;
    let height = (6.0 + deco.variant as f32) * scale;
    draw_ellipse(pos.x, pos.y, width, height, 0.0, rock_color);

    // Frost coating on top
    let frost_color = Color::new(0.9, 0.95, 1.0, 0.6);
    draw_ellipse(pos.x, pos.y - height * 0.3, width * 0.9, height * 0.4, 0.0, frost_color);

    // Ice crystals on top
    let crystal_color = Color::new(0.8, 0.9, 1.0, 0.5);
    draw_triangle(
        vec2(pos.x - width * 0.3, pos.y - height * 0.5),
        vec2(pos.x - width * 0.15, pos.y - height * 0.5),
        vec2(pos.x - width * 0.22, pos.y - height * 1.2),
        crystal_color,
    );
    draw_triangle(
        vec2(pos.x + width * 0.15, pos.y - height * 0.5),
        vec2(pos.x + width * 0.3, pos.y - height * 0.5),
        vec2(pos.x + width * 0.22, pos.y - height * 1.0),
        crystal_color,
    );
}

pub fn draw_ice_crystal(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let shimmer = (time * 2.5 + deco.phase * std::f32::consts::TAU).sin() * 0.2 + 0.8;
    let color = Color::new(
        accent.r * shimmer,
        accent.g * shimmer,
        (accent.b * 1.1).min(1.0),
        0.8,
    );

    // Multi-faceted crystal cluster
    let heights = [12.0, 8.0, 10.0];
    let offsets = [-4.0, 0.0, 5.0];

    for i in 0..3 {
        let height = heights[i] * scale * (0.8 + deco.variant as f32 * 0.1);
        let offset = offsets[i] * scale;
        let width = 3.0 * scale;

        draw_triangle(
            vec2(pos.x + offset - width, pos.y),
            vec2(pos.x + offset + width, pos.y),
            vec2(pos.x + offset, pos.y - height),
            color,
        );

        // Highlight
        let highlight = Color::new(1.0, 1.0, 1.0, 0.35 * shimmer);
        draw_triangle(
            vec2(pos.x + offset - width * 0.4, pos.y - height * 0.15),
            vec2(pos.x + offset + width * 0.2, pos.y - height * 0.15),
            vec2(pos.x + offset - width * 0.1, pos.y - height * 0.85),
            highlight,
        );
    }

    // Aurora glow at base
    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        let pulse = shimmer * 0.5;
        draw_circle(pos.x, pos.y, 6.0 * scale, Color::new(glow_mq.r, glow_mq.g, glow_mq.b, pulse * 0.3));
    }
}

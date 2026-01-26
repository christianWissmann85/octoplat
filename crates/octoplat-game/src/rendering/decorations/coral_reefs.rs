//! Coral Reefs biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_coral_branch(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r,
        accent.g * 0.7,
        accent.b * 0.9,
        0.9,
    );

    // Main branch
    let sway = (time * 0.8).sin() * 1.0;
    let height = (15.0 + deco.variant as f32 * 3.0) * scale;
    draw_line(base.x, base.y, base.x + sway, base.y - height, 3.0 * scale, color);

    // Sub-branches
    let branch_count = 2 + (deco.variant % 2) as i32;
    for i in 0..branch_count {
        let by = base.y - height * (0.4 + i as f32 * 0.25);
        let dir = if i % 2 == 0 { 1.0 } else { -1.0 };
        let bx = base.x + sway + dir * 8.0 * scale;
        draw_line(base.x + sway * 0.5, by, bx, by - 5.0 * scale, 2.0 * scale, color);
        draw_circle(bx, by - 5.0 * scale, 2.5 * scale, color);
    }

    // Tip
    draw_circle(base.x + sway, base.y - height, 3.0 * scale, color);
}

pub fn draw_anemone(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r,
        accent.g * 0.6,
        accent.b,
        0.85,
    );

    // Base disc
    draw_ellipse(base.x, base.y, 8.0 * scale, 4.0 * scale, 0.0, color);

    // Tentacles
    let tentacle_count = 5 + deco.variant as i32;
    for i in 0..tentacle_count {
        let angle = (i as f32 / tentacle_count as f32) * std::f32::consts::PI - std::f32::consts::PI / 2.0;
        let sway = (time * 2.0 + i as f32 * 0.5).sin() * 0.2;
        let tentacle_angle = angle + sway;

        let length = (10.0 + (i as f32 * 1.3) % 4.0) * scale;
        let end_x = base.x + tentacle_angle.cos() * length;
        let end_y = base.y - tentacle_angle.sin().abs() * length;

        let tentacle_color = Color::new(color.r, color.g * 1.2, color.b, color.a * 0.8);
        draw_line(base.x, base.y - 2.0 * scale, end_x, end_y, 1.5 * scale, tentacle_color);
    }
}

pub fn draw_shell(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r * 0.9,
        accent.g * 0.8,
        accent.b * 0.7,
        0.9,
    );

    // Spiral shell shape
    let size = (6.0 + deco.variant as f32 * 1.5) * scale;
    draw_ellipse(pos.x, pos.y, size, size * 0.7, 0.3 * deco.variant as f32, color);

    // Inner spiral suggestion
    let inner = Color::new(color.r * 0.7, color.g * 0.7, color.b * 0.7, 0.6);
    draw_ellipse(pos.x + size * 0.2, pos.y, size * 0.4, size * 0.3, 0.3 * deco.variant as f32, inner);
}

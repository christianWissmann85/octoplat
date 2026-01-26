//! Sunken Ruins biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_broken_column(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 0.95,
        solid.g * 0.95,
        solid.b * 0.9,
        0.9,
    );

    // Column shaft
    let width = (8.0 + deco.variant as f32 * 2.0) * scale;
    let height = (18.0 + deco.variant as f32 * 3.0) * scale;

    // Broken top (jagged)
    let break_height = height * (0.6 + (deco.variant as f32 * 0.1) % 0.3);
    draw_rectangle(pos.x - width / 2.0, pos.y - break_height, width, break_height, color);

    // Capital (ornate top piece, partially broken)
    let cap_color = Color::new(color.r * 1.05, color.g * 1.05, color.b, color.a);
    draw_rectangle(pos.x - width * 0.7, pos.y - break_height - 3.0 * scale, width * 1.2, 3.0 * scale, cap_color);

    // Fluting (vertical grooves)
    let groove_color = Color::new(color.r * 0.8, color.g * 0.8, color.b * 0.75, 0.5);
    for i in 0..3 {
        let gx = pos.x - width * 0.3 + i as f32 * width * 0.3;
        draw_line(gx, pos.y, gx, pos.y - break_height + 2.0, 1.0, groove_color);
    }

    // Cracks
    let crack_color = Color::new(0.3, 0.28, 0.25, 0.4);
    draw_line(pos.x - width * 0.2, pos.y - break_height * 0.3, pos.x + width * 0.1, pos.y - break_height * 0.5, 1.0, crack_color);
}

pub fn draw_ancient_tile(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 0.9,
        solid.g * 0.85,
        solid.b * 0.8,
        0.85,
    );

    // Tile shape (slightly rotated square)
    let size = (10.0 + deco.variant as f32 * 2.0) * scale;
    let rotation = (deco.variant as f32 - 1.5) * 0.15;

    draw_rectangle_ex(
        pos.x - size / 2.0,
        pos.y - size / 2.0,
        size,
        size,
        DrawRectangleParams {
            rotation,
            color,
            ..Default::default()
        },
    );

    // Carved pattern (mosaic-like)
    let pattern_color = Color::new(
        (color.r * 1.15).min(1.0),
        (color.g * 1.1).min(1.0),
        color.b,
        0.6,
    );
    draw_rectangle(pos.x - size * 0.3, pos.y - size * 0.3, size * 0.25, size * 0.25, pattern_color);
    draw_rectangle(pos.x + size * 0.05, pos.y + size * 0.05, size * 0.25, size * 0.25, pattern_color);

    // Weathering
    let wear_color = Color::new(0.4, 0.35, 0.3, 0.3);
    draw_circle(pos.x + size * 0.2, pos.y - size * 0.15, size * 0.15, wear_color);
}

pub fn draw_mystic_orb(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;

    // Floating motion
    let float_y = (time * 0.8 + deco.phase * std::f32::consts::TAU).sin() * 5.0 * scale;
    let float_x = (time * 0.5 + deco.phase * std::f32::consts::PI).cos() * 3.0 * scale;
    let pos = vec2(base.x + float_x, base.y + float_y);

    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        let pulse = (time * 1.5 + deco.phase * std::f32::consts::TAU).sin() * 0.3 + 0.7;
        let size = (5.0 + deco.variant as f32) * scale;

        // Outer glow rings
        for i in 0..4 {
            let ring_size = size * (1.0 + i as f32 * 0.5);
            let alpha = (0.35 - i as f32 * 0.08) * pulse;
            draw_circle(pos.x, pos.y, ring_size, Color::new(glow_mq.r, glow_mq.g, glow_mq.b, alpha));
        }

        // Core
        let core_color = Color::new(
            (glow_mq.r * 1.2).min(1.0),
            (glow_mq.g * 1.2).min(1.0),
            glow_mq.b,
            0.9 * pulse,
        );
        draw_circle(pos.x, pos.y, size * 0.4, core_color);

        // Sparkle
        let sparkle = ((time * 4.0 + deco.phase * 10.0) % 1.0) < 0.3;
        if sparkle {
            draw_circle(pos.x - size * 0.2, pos.y - size * 0.2, size * 0.15, Color::new(1.0, 1.0, 1.0, 0.6));
        }
    }
}

pub fn draw_vine_growth(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;
    let segments = 4 + deco.variant as i32;
    let seg_height = 8.0 * scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r * 0.4,
        accent.g * 0.8,
        accent.b * 0.5,
        0.8,
    );

    let mut prev_x = base.x;
    let mut prev_y = base.y;

    for i in 0..segments {
        let sway = (time * 0.8 + i as f32 * 0.5).sin() * 3.0 * scale;
        let y = base.y - ((i + 1) as f32 * seg_height);
        let x = base.x + sway;
        let width = (2.5 - i as f32 * 0.3).max(1.0) * scale;

        draw_line(prev_x, prev_y, x, y, width, color);

        // Small leaves
        if i % 2 == 1 {
            let leaf_color = Color::new(color.r * 1.2, color.g * 1.1, color.b, color.a * 0.7);
            let dir = if i % 4 == 1 { 1.0 } else { -1.0 };
            draw_ellipse(x + dir * 5.0 * scale, y, 4.0 * scale, 2.5 * scale, dir * 0.5, leaf_color);
        }

        prev_x = x;
        prev_y = y;
    }

    // Tendril curl at top
    let curl_color = Color::new(color.r, color.g * 0.9, color.b, color.a * 0.6);
    let curl_x = prev_x + (time * 1.2).sin() * 2.0 * scale;
    draw_circle(curl_x + 3.0 * scale, prev_y - 2.0 * scale, 2.0 * scale, curl_color);
}

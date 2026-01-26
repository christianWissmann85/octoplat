//! Tropical Shore biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_palm_frond(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;
    let segments = 5 + deco.variant as i32;
    let seg_height = 10.0 * scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r * 0.3,
        accent.g * 1.0,
        accent.b * 0.4,
        0.9,
    );

    let mut prev_x = base.x;
    let mut prev_y = base.y;

    // Main frond stem
    for i in 0..segments {
        let sway = (time * 1.0 + i as f32 * 0.3).sin() * 4.0 * scale;
        let y = base.y - ((i + 1) as f32 * seg_height);
        let x = base.x + sway;
        let width = (3.0 - i as f32 * 0.4).max(1.0) * scale;
        draw_line(prev_x, prev_y, x, y, width, color);

        // Leaflets on each segment
        if i > 0 {
            let leaf_color = Color::new(color.r * 1.1, color.g * 1.05, color.b, color.a * 0.8);
            let leaf_sway = (time * 1.5 + i as f32).sin() * 2.0;
            // Left leaf
            draw_line(prev_x, prev_y, prev_x - 8.0 * scale + leaf_sway, prev_y - 3.0 * scale, 1.5 * scale, leaf_color);
            // Right leaf
            draw_line(prev_x, prev_y, prev_x + 8.0 * scale + leaf_sway, prev_y - 3.0 * scale, 1.5 * scale, leaf_color);
        }

        prev_x = x;
        prev_y = y;
    }
}

pub fn draw_coconut(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 0.6,
        solid.g * 0.45,
        solid.b * 0.3,
        0.95,
    );

    // Coconut body (brown oval)
    let size = (6.0 + deco.variant as f32 * 1.5) * scale;
    draw_ellipse(pos.x, pos.y, size, size * 0.85, 0.0, color);

    // Three "eyes" on the coconut
    let eye_color = Color::new(0.2, 0.15, 0.1, 0.8);
    draw_circle(pos.x - size * 0.25, pos.y - size * 0.2, size * 0.15, eye_color);
    draw_circle(pos.x + size * 0.25, pos.y - size * 0.2, size * 0.15, eye_color);
    draw_circle(pos.x, pos.y + size * 0.1, size * 0.12, eye_color);

    // Highlight
    let highlight = Color::new(1.0, 0.95, 0.8, 0.3);
    draw_circle(pos.x - size * 0.3, pos.y - size * 0.35, size * 0.2, highlight);
}

pub fn draw_tropical_flower(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    // Bright tropical colors - cycle through variants
    let petal_colors = [
        Color::new(1.0, 0.4, 0.5, 0.9),  // Pink
        Color::new(1.0, 0.8, 0.2, 0.9),  // Yellow
        Color::new(1.0, 0.5, 0.2, 0.9),  // Orange
        Color::new(0.9, 0.3, 0.6, 0.9),  // Magenta
    ];
    let petal_color = petal_colors[(deco.variant % 4) as usize];

    let num_petals = 5;
    let petal_size = (5.0 + deco.variant as f32) * scale;

    // Petals arranged in circle
    for i in 0..num_petals {
        let angle = (i as f32 / num_petals as f32) * std::f32::consts::TAU;
        let sway = (time * 1.5 + i as f32 * 0.3).sin() * 0.1;
        let petal_angle = angle + sway;
        let px = pos.x + petal_angle.cos() * petal_size * 0.8;
        let py = pos.y + petal_angle.sin() * petal_size * 0.8;
        draw_ellipse(px, py, petal_size * 0.6, petal_size * 0.4, angle, petal_color);
    }

    // Center
    let center_color = Color::new(accent.r, accent.g * 0.8, accent.b * 0.2, 1.0);
    draw_circle(pos.x, pos.y, petal_size * 0.35, center_color);
}

pub fn draw_starfish(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(
        accent.r * 0.9,
        accent.g * 0.5,
        accent.b * 0.3,
        0.9,
    );

    let arm_length = (8.0 + deco.variant as f32 * 2.0) * scale;
    let num_arms = 5;
    let rotation = deco.variant as f32 * 0.3;

    // Draw 5 arms
    for i in 0..num_arms {
        let angle = rotation + (i as f32 / num_arms as f32) * std::f32::consts::TAU;
        let end_x = pos.x + angle.cos() * arm_length;
        let end_y = pos.y + angle.sin() * arm_length;

        // Tapered arm
        draw_triangle(
            vec2(pos.x + angle.cos() * 3.0 * scale, pos.y + angle.sin() * 3.0 * scale),
            vec2(pos.x + (angle + 0.3).cos() * 3.0 * scale, pos.y + (angle + 0.3).sin() * 3.0 * scale),
            vec2(end_x, end_y),
            color,
        );
        draw_triangle(
            vec2(pos.x + angle.cos() * 3.0 * scale, pos.y + angle.sin() * 3.0 * scale),
            vec2(pos.x + (angle - 0.3).cos() * 3.0 * scale, pos.y + (angle - 0.3).sin() * 3.0 * scale),
            vec2(end_x, end_y),
            color,
        );
    }

    // Center body
    draw_circle(pos.x, pos.y, 4.0 * scale, color);

    // Texture dots
    let dot_color = Color::new(color.r * 0.8, color.g * 0.7, color.b * 0.6, 0.6);
    for i in 0..num_arms {
        let angle = rotation + (i as f32 / num_arms as f32) * std::f32::consts::TAU;
        let dx = pos.x + angle.cos() * arm_length * 0.5;
        let dy = pos.y + angle.sin() * arm_length * 0.5;
        draw_circle(dx, dy, 1.5 * scale, dot_color);
    }
}

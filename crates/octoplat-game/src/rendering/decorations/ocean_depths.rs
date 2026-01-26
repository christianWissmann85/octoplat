//! Ocean Depths biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::primitives::{draw_segmented_sway, draw_segmented_sway_with_leaves, SwayParams};
use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_seaweed(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(accent.r * 0.6, accent.g * 0.9, accent.b * 0.5, 0.8);

    draw_segmented_sway(base, deco.scale, deco.variant, time, color, &SwayParams::seaweed());
}

pub fn draw_kelp(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(accent.r * 0.4, accent.g * 0.8, accent.b * 0.3, 0.85);
    let leaf_color = Color::new(color.r * 1.2, color.g * 1.1, color.b, color.a * 0.7);

    draw_segmented_sway_with_leaves(
        base,
        deco.scale,
        deco.variant,
        time,
        color,
        &SwayParams::kelp(),
        |i, x, y, scale| {
            // Draw kelp leaves on alternating segments
            if i % 2 == 1 {
                let leaf_dir = if i % 4 == 1 { 1.0 } else { -1.0 };
                draw_circle(x + leaf_dir * 6.0 * scale, y, 3.0 * scale, leaf_color);
            }
        },
    );
}

pub fn draw_bubbles(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;
    let count = 2 + deco.variant as i32;

    let particle = color_to_mq(theme.particle_color);
    let color = Color::new(
        particle.r,
        particle.g,
        particle.b,
        0.4,
    );

    for i in 0..count {
        let phase = deco.phase + i as f32 * 0.3;
        let rise = ((time * 0.5 + phase) % 2.0) * 30.0 * scale;
        let wobble = (time * 2.0 + i as f32).sin() * 3.0 * scale;
        let size = (2.0 + (i as f32 * 0.5)) * scale;

        draw_circle(base.x + wobble, base.y - rise, size, color);
        // Highlight
        draw_circle(
            base.x + wobble - size * 0.3,
            base.y - rise - size * 0.3,
            size * 0.3,
            Color::new(1.0, 1.0, 1.0, 0.3),
        );
    }
}

pub fn draw_small_rock(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 0.8,
        solid.g * 0.8,
        solid.b * 0.8,
        0.9,
    );

    // Simple rounded rock shape
    let width = (8.0 + deco.variant as f32 * 2.0) * scale;
    let height = (5.0 + deco.variant as f32) * scale;
    draw_ellipse(pos.x, pos.y, width, height, 0.0, color);

    // Highlight
    let highlight = Color::new(color.r * 1.3, color.g * 1.3, color.b * 1.3, 0.4);
    draw_ellipse(pos.x - width * 0.2, pos.y - height * 0.2, width * 0.4, height * 0.4, 0.0, highlight);
}

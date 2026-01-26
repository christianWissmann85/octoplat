//! Abyss biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::primitives::{draw_pulsing_glow, draw_segmented_sway, GlowParams, SwayParams};
use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_crystal(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let accent = color_to_mq(theme.accent_color);
    let base_color = Color::new(
        accent.r * 0.8,
        accent.g * 0.7,
        accent.b,
        0.85,
    );

    // Crystal spire
    let height = (12.0 + deco.variant as f32 * 4.0) * scale;
    let width = 4.0 * scale;

    draw_triangle(
        vec2(pos.x - width, pos.y),
        vec2(pos.x + width, pos.y),
        vec2(pos.x, pos.y - height),
        base_color,
    );

    // Glow effect
    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        let pulse = (time * 2.0 + deco.phase * std::f32::consts::TAU).sin() * 0.3 + 0.7;
        let glow_color = Color::new(glow_mq.r, glow_mq.g, glow_mq.b, glow_mq.a * pulse * 0.5);
        draw_circle(pos.x, pos.y - height * 0.5, height * 0.3, glow_color);
    }

    // Highlight facet
    let highlight = Color::new(1.0, 1.0, 1.0, 0.25);
    draw_triangle(
        vec2(pos.x - width * 0.3, pos.y - height * 0.2),
        vec2(pos.x + width * 0.2, pos.y - height * 0.2),
        vec2(pos.x - width * 0.1, pos.y - height * 0.8),
        highlight,
    );
}

pub fn draw_bio_glow(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let size = (4.0 + deco.variant as f32) * deco.scale;

    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        draw_pulsing_glow(pos, size, time, deco.phase, glow_mq, &GlowParams::bio_glow());
    }
}

pub fn draw_tendril(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let accent = color_to_mq(theme.accent_color);
    let color = Color::new(accent.r * 0.6, accent.g * 0.5, accent.b * 0.8, 0.7);

    // Draw the tendril segments
    let tip = draw_segmented_sway(
        base,
        deco.scale,
        deco.variant,
        time,
        color,
        &SwayParams::tendril(),
    );

    // Tip glow at the end
    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        let pulse = (time * 2.0 + deco.phase * std::f32::consts::TAU).sin() * 0.3 + 0.5;
        draw_circle(tip.x, tip.y, 2.0 * deco.scale, Color::new(glow_mq.r, glow_mq.g, glow_mq.b, pulse));
    }
}

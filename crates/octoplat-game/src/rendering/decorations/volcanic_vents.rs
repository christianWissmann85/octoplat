//! Volcanic Vents biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_lava_rock(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let base_color = Color::new(
        solid.r * 0.6,
        solid.g * 0.5,
        solid.b * 0.4,
        0.9,
    );

    // Irregular rock shape using multiple circles
    let size = (8.0 + deco.variant as f32 * 2.0) * scale;
    draw_circle(pos.x, pos.y, size, base_color);
    draw_circle(pos.x - size * 0.5, pos.y + size * 0.3, size * 0.6, base_color);
    draw_circle(pos.x + size * 0.4, pos.y - size * 0.2, size * 0.5, base_color);

    // Glowing cracks
    if let Some(glow) = theme.glow_color {
        let glow_mq = color_to_mq(glow);
        let pulse = (time * 2.0).sin() * 0.3 + 0.7;
        let crack_color = Color::new(glow_mq.r, glow_mq.g, glow_mq.b, glow_mq.a * pulse);
        draw_line(pos.x - size * 0.3, pos.y, pos.x + size * 0.2, pos.y + size * 0.4, 1.5, crack_color);
    }
}

pub fn draw_steam_vent(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;

    // Vent opening
    let solid = color_to_mq(theme.solid_color);
    let vent_color = Color::new(
        solid.r * 0.5,
        solid.g * 0.4,
        solid.b * 0.3,
        0.9,
    );
    draw_ellipse(base.x, base.y, 6.0 * scale, 3.0 * scale, 0.0, vent_color);

    // Steam puffs rising
    let steam_color = Color::new(0.8, 0.8, 0.8, 0.3);
    let puff_count = 3;
    for i in 0..puff_count {
        let phase = (time * 0.8 + i as f32 * 0.5 + deco.phase) % 2.0;
        let rise = phase * 20.0 * scale;
        let spread = phase * 5.0 * scale;
        let alpha = (1.0 - phase / 2.0) * 0.4;
        let size = (3.0 + phase * 4.0) * scale;

        draw_circle(
            base.x + (i as f32 - 1.0) * spread,
            base.y - rise - 5.0 * scale,
            size,
            Color::new(steam_color.r, steam_color.g, steam_color.b, alpha),
        );
    }
}

pub fn draw_ash(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;
    let count = 3 + deco.variant as i32;

    let particle = color_to_mq(theme.particle_color);
    let color = Color::new(
        particle.r * 0.5,
        particle.g * 0.4,
        particle.b * 0.4,
        0.5,
    );

    for i in 0..count {
        let phase = deco.phase + i as f32 * 0.25;
        let fall = ((time * 0.3 + phase) % 2.0) * 25.0 * scale;
        let drift = (time * 1.5 + i as f32).sin() * 4.0 * scale;
        let size = (1.5 + (i as f32 * 0.3)) * scale;

        draw_circle(base.x + drift, base.y + fall, size, color);
    }
}

//! Shipwreck biome decorations

use macroquad::prelude::*;

use octoplat_core::level::Decoration;
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::to_mq_vec2;
use crate::compat::color_to_mq;

pub fn draw_wood_debris(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 1.2,
        solid.g * 1.1,
        solid.b * 0.9,
        0.85,
    );

    // Broken plank shapes
    let width = (12.0 + deco.variant as f32 * 3.0) * scale;
    let height = 3.0 * scale;
    let angle = (deco.variant as f32 - 1.5) * 0.3;

    draw_rectangle_ex(
        pos.x - width / 2.0,
        pos.y - height / 2.0,
        width,
        height,
        DrawRectangleParams {
            rotation: angle,
            color,
            ..Default::default()
        },
    );

    // Wood grain lines
    let grain = Color::new(color.r * 0.7, color.g * 0.65, color.b * 0.6, 0.5);
    draw_line(
        pos.x - width * 0.3,
        pos.y,
        pos.x + width * 0.3,
        pos.y,
        1.0,
        grain,
    );
}

pub fn draw_barrel(deco: &Decoration, theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let solid = color_to_mq(theme.solid_color);
    let color = Color::new(
        solid.r * 1.3,
        solid.g * 1.1,
        solid.b * 0.8,
        0.9,
    );

    // Barrel body
    let width = 10.0 * scale;
    let height = 14.0 * scale;
    draw_ellipse(pos.x, pos.y, width, height, 0.0, color);

    // Metal bands
    let band_color = Color::new(0.4, 0.35, 0.3, 0.7);
    draw_ellipse(pos.x, pos.y - height * 0.6, width * 1.1, 2.0 * scale, 0.0, band_color);
    draw_ellipse(pos.x, pos.y + height * 0.6, width * 1.1, 2.0 * scale, 0.0, band_color);
}

pub fn draw_chain(deco: &Decoration, _theme: &BiomeTheme, time: f32) {
    let base = to_mq_vec2(deco.position);
    let scale = deco.scale;
    let links = 4 + deco.variant as i32;

    let color = Color::new(0.5, 0.45, 0.4, 0.8);

    for i in 0..links {
        let sway = (time * 1.0 + i as f32 * 0.2).sin() * 2.0 * scale;
        let y = base.y + i as f32 * 6.0 * scale;
        let x = base.x + sway;

        // Chain link (two overlapping circles)
        draw_circle_lines(x, y, 3.0 * scale, 1.5, color);
    }
}

pub fn draw_anchor(deco: &Decoration, _theme: &BiomeTheme) {
    let pos = to_mq_vec2(deco.position);
    let scale = deco.scale;

    let color = Color::new(0.4, 0.38, 0.35, 0.9);

    // Anchor shaft
    draw_line(pos.x, pos.y - 15.0 * scale, pos.x, pos.y + 5.0 * scale, 3.0 * scale, color);

    // Cross bar
    draw_line(pos.x - 8.0 * scale, pos.y - 10.0 * scale, pos.x + 8.0 * scale, pos.y - 10.0 * scale, 2.5 * scale, color);

    // Flukes (curved arms at bottom)
    draw_line(pos.x, pos.y + 5.0 * scale, pos.x - 8.0 * scale, pos.y - 2.0 * scale, 2.5 * scale, color);
    draw_line(pos.x, pos.y + 5.0 * scale, pos.x + 8.0 * scale, pos.y - 2.0 * scale, 2.5 * scale, color);

    // Ring at top
    draw_circle_lines(pos.x, pos.y - 18.0 * scale, 4.0 * scale, 2.0, color);
}

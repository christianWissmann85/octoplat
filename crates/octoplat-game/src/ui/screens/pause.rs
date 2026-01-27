//! Pause menu screen rendering

use macroquad::prelude::*;
use octoplat_core::state::PauseMenuItem;

use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_list};

/// Draw the pause menu overlay
pub fn draw_pause_menu(menu: &MenuState<PauseMenuItem>, overlay_texture: Option<&Texture2D>) {
    let sw = screen_width();
    let sh = screen_height();

    // Darken background first
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.05, 0.8));

    // Draw overlay texture if available (centered frame/panel)
    if let Some(texture) = overlay_texture {
        draw_centered_overlay(texture, sw, sh);
    }

    // Pause title
    draw_centered_text("PAUSED", sh * 0.25, 48.0, Color::new(1.0, 1.0, 1.0, 0.9));

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 60.0;
    let start_y = sh * 0.4;
    let line_height = 45.0;

    draw_menu_list(menu, &labels, center_x, start_y, 28.0, line_height);

    // Hint
    draw_centered_text(
        "ESC to resume",
        sh - 40.0,
        16.0,
        Color::new(0.5, 0.6, 0.7, 0.75),
    );
}

/// Draw a texture as a centered overlay panel
fn draw_centered_overlay(texture: &Texture2D, sw: f32, sh: f32) {
    let tex_w = texture.width();
    let tex_h = texture.height();

    // Scale to fit within screen while maintaining aspect ratio (max 80% of screen)
    let max_w = sw * 0.8;
    let max_h = sh * 0.8;
    let scale = (max_w / tex_w).min(max_h / tex_h).min(1.0);

    let draw_w = tex_w * scale;
    let draw_h = tex_h * scale;

    // Center on screen
    let draw_x = (sw - draw_w) / 2.0;
    let draw_y = (sh - draw_h) / 2.0;

    draw_texture_ex(
        texture,
        draw_x,
        draw_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(draw_w, draw_h)),
            ..Default::default()
        },
    );
}

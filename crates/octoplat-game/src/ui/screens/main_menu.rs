//! Main menu screen rendering
//!
//! Displays the main menu with optional FLUX-generated background.

use macroquad::prelude::*;
use octoplat_core::state::MainMenuItem;

use crate::rendering::UiTextureManager;
use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_list};

/// Draw the main menu
///
/// If ui_textures is provided and has a main menu background, uses it.
/// Otherwise falls back to procedural background.
pub fn draw_main_menu(
    menu: &MenuState<MainMenuItem>,
    time: f32,
    ui_textures: Option<&UiTextureManager>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Draw background (textured or procedural)
    let has_bg = ui_textures
        .and_then(|t| t.menus.main_menu.as_ref())
        .is_some();

    if let Some(bg) = ui_textures.and_then(|t| t.menus.main_menu.as_ref()) {
        // Draw textured background
        draw_texture_ex(
            bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(sw, sh)),
                ..Default::default()
            },
        );
    } else {
        // Procedural background
        clear_background(Color::new(0.05, 0.08, 0.15, 1.0));
    }

    // Draw animated bubbles (subtle)
    let bubble_opacity = if has_bg { 0.3 } else { 0.5 };
    draw_menu_bubbles(time, bubble_opacity);

    // Semi-transparent overlay for text readability if we have a background
    if has_bg {
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.3));
    }

    // Title (smaller than title screen) with shadow
    draw_centered_text(
        "OCTOPLAT",
        sh * 0.15 + 2.0,
        60.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text("OCTOPLAT", sh * 0.15, 60.0, Color::new(0.3, 0.8, 0.9, 1.0));

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 80.0;
    let start_y = sh * 0.35;
    let line_height = 50.0;

    draw_menu_list(menu, &labels, center_x, start_y, 32.0, line_height);

    // Description of selected item with shadow
    let description = menu.selected_item().description();
    draw_centered_text(
        description,
        sh * 0.85 + 1.0,
        20.0,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );
    draw_centered_text(
        description,
        sh * 0.85,
        20.0,
        Color::new(0.5, 0.6, 0.7, 0.7),
    );

    // Navigation hint with shadow
    draw_centered_text(
        "W/S or Arrow Keys to navigate  |  SPACE/ENTER to select",
        sh - 29.0,
        16.0,
        Color::new(0.0, 0.0, 0.0, 0.3),
    );
    draw_centered_text(
        "W/S or Arrow Keys to navigate  |  SPACE/ENTER to select",
        sh - 30.0,
        16.0,
        Color::new(0.4, 0.5, 0.6, 0.5),
    );
}

/// Draw floating bubbles for menu background
fn draw_menu_bubbles(time: f32, opacity_mult: f32) {
    let sw = screen_width();
    let sh = screen_height();

    for i in 0..12 {
        let seed = i as f32 * 97.3;
        let x = ((seed * 5.7).sin() * 0.5 + 0.5) * sw;
        let base_y = ((seed * 2.9).cos() * 0.5 + 0.5) * sh;
        let speed = 15.0 + (seed * 0.08).sin().abs() * 20.0;
        let y = (base_y + time * speed) % (sh + 40.0) - 20.0;
        let size = 2.0 + (seed * 0.5).sin().abs() * 5.0;
        let alpha = (0.08 + (seed * 0.2).cos().abs() * 0.1) * opacity_mult;

        draw_circle(x, sh - y, size, Color::new(0.7, 0.85, 1.0, alpha));
    }
}

//! Settings screen rendering
//!
//! Displays settings menu with optional FLUX-generated background.

use macroquad::prelude::*;
use octoplat_core::state::SettingsMenuItem;

use crate::rendering::UiTextureManager;
use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_volume_slider};

/// Draw settings screen
///
/// If ui_textures is provided and has a settings background, uses it.
/// Otherwise falls back to procedural background.
pub fn draw_settings(
    menu: &MenuState<SettingsMenuItem>,
    sfx_volume: f32,
    music_volume: f32,
    screen_shake_enabled: bool,
    minimap_size: f32,
    minimap_scale: f32,
    minimap_opacity: f32,
    ui_textures: Option<&UiTextureManager>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Draw background (textured or procedural)
    let has_bg = ui_textures
        .and_then(|t| t.menus.settings.as_ref())
        .is_some();

    if let Some(bg) = ui_textures.and_then(|t| t.menus.settings.as_ref()) {
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
        // Darken for readability
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.4));
    } else {
        // Procedural background
        clear_background(Color::new(0.08, 0.1, 0.12, 1.0));
    }

    // Title with shadow
    draw_centered_text(
        "SETTINGS",
        sh * 0.10 + 2.0,
        48.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text(
        "SETTINGS",
        sh * 0.10,
        48.0,
        Color::new(0.7, 0.8, 0.9, 1.0),
    );

    // Settings box (taller to fit more items)
    let box_x = sw / 2.0 - 220.0;
    let box_y = sh * 0.18;
    let box_w = 440.0;
    let box_h = 420.0;

    // Box with slightly higher opacity if we have background texture
    let box_alpha = if has_bg { 0.85 } else { 0.95 };
    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.1, 0.12, 0.15, box_alpha));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.4, 0.5, 0.8));

    // Draw each setting item
    let item_x = box_x + 30.0;
    let item_start_y = box_y + 50.0;
    let item_spacing = 55.0;

    for (i, item) in menu.items.iter().enumerate() {
        let y = item_start_y + i as f32 * item_spacing;
        let is_selected = i == menu.selected;

        // Selection indicator
        let text_color = if is_selected {
            Color::new(1.0, 0.9, 0.4, 1.0)
        } else {
            Color::new(0.7, 0.7, 0.7, 0.9)
        };

        if is_selected {
            // Draw selection highlight
            draw_rectangle(
                box_x + 10.0,
                y - 20.0,
                box_w - 20.0,
                45.0,
                Color::new(0.2, 0.25, 0.3, 0.5),
            );
            // Draw arrow
            draw_text(">", item_x - 20.0, y, 24.0, text_color);
        }

        // Draw label
        draw_text(item.label(), item_x, y, 24.0, text_color);

        // Draw value
        let value_x = box_x + 210.0;
        match item {
            SettingsMenuItem::SfxVolume => {
                draw_volume_slider(value_x, y - 8.0, 150.0, sfx_volume, is_selected);
            }
            SettingsMenuItem::MusicVolume => {
                draw_volume_slider(value_x, y - 8.0, 150.0, music_volume, is_selected);
            }
            SettingsMenuItem::ScreenShake => {
                let toggle_text = if screen_shake_enabled { "ON" } else { "OFF" };
                let toggle_color = if screen_shake_enabled {
                    Color::new(0.3, 1.0, 0.5, 1.0)
                } else {
                    Color::new(0.6, 0.4, 0.4, 0.8)
                };
                draw_text(toggle_text, value_x + 50.0, y, 24.0, toggle_color);
            }
            SettingsMenuItem::MinimapSize => {
                // Size slider: 100-250 pixels, displayed as percentage
                let size_pct = ((minimap_size - 100.0) / 150.0).clamp(0.0, 1.0);
                draw_volume_slider(value_x, y - 8.0, 150.0, size_pct, is_selected);
                // Show actual pixel value
                draw_text(
                    &format!("{}px", minimap_size as i32),
                    value_x + 165.0,
                    y,
                    16.0,
                    Color::new(0.5, 0.6, 0.7, 0.7),
                );
            }
            SettingsMenuItem::MinimapZoom => {
                // Zoom/scale slider: 1.0-6.0
                let zoom_pct = ((minimap_scale - 1.0) / 5.0).clamp(0.0, 1.0);
                draw_volume_slider(value_x, y - 8.0, 150.0, zoom_pct, is_selected);
                // Show actual zoom value
                draw_text(
                    &format!("{:.1}x", minimap_scale),
                    value_x + 165.0,
                    y,
                    16.0,
                    Color::new(0.5, 0.6, 0.7, 0.7),
                );
            }
            SettingsMenuItem::MinimapOpacity => {
                draw_volume_slider(value_x, y - 8.0, 150.0, minimap_opacity, is_selected);
            }
            SettingsMenuItem::Back => {
                // No value for back button
            }
        }
    }

    // Instructions with shadow
    draw_centered_text(
        "UP/DOWN: Select   LEFT/RIGHT: Adjust   ENTER: Confirm",
        sh * 0.88 + 1.0,
        18.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text(
        "UP/DOWN: Select   LEFT/RIGHT: Adjust   ENTER: Confirm",
        sh * 0.88,
        18.0,
        Color::new(0.5, 0.5, 0.5, 0.85),
    );
}

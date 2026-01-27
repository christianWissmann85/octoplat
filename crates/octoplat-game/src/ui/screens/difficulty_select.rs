//! Difficulty selection screen for RogueLite mode
//!
//! Displays difficulty options with HP, i-frame, and enemy speed stats.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use octoplat_core::state::DifficultyMenuItem;

use crate::rendering::UiTextureManager;
use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_item};

/// Draw the difficulty selection screen
pub fn draw_difficulty_select(
    menu: &MenuState<DifficultyMenuItem>,
    biome: BiomeId,
    _time: f32,
    _ui_textures: Option<&UiTextureManager>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Background
    clear_background(Color::new(0.05, 0.08, 0.15, 1.0));

    // Get biome color for accent
    let biome_def = biome.definition();
    let biome_color = Color::new(
        biome_def.theme.solid_color.r,
        biome_def.theme.solid_color.g,
        biome_def.theme.solid_color.b,
        1.0,
    );

    // Title with shadow
    draw_centered_text(
        "SELECT DIFFICULTY",
        sh * 0.10 + 2.0,
        48.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text("SELECT DIFFICULTY", sh * 0.10, 48.0, Color::new(0.3, 0.8, 0.9, 1.0));

    // Subtitle showing selected biome
    let subtitle = format!("for {} challenge", biome_def.name);
    draw_centered_text(
        &subtitle,
        sh * 0.17,
        22.0,
        biome_color,
    );

    // Draw difficulty options
    let center_x = sw / 2.0;
    let start_y = sh * 0.30;
    let line_height = 55.0;

    // Difficulty colors
    let difficulty_colors = [
        Color::new(0.4, 0.8, 0.5, 1.0),  // Drifting - green (easy)
        Color::new(0.5, 0.7, 0.9, 1.0),  // Treading Water - blue (normal)
        Color::new(0.9, 0.6, 0.2, 1.0),  // OctoHard - orange (hard)
        Color::new(0.9, 0.2, 0.2, 1.0),  // The Kraken - red (hardest)
        Color::new(0.5, 0.5, 0.6, 1.0),  // Back - gray
    ];

    for (i, item) in menu.items.iter().enumerate() {
        let y = start_y + i as f32 * line_height;
        let selected = i == menu.selected;

        // Draw difficulty indicator
        if i < 4 {
            let indicator_x = center_x - 200.0;
            let indicator_size = 16.0;

            // Draw colored diamond
            draw_poly(
                indicator_x,
                y,
                4,
                indicator_size / 2.0,
                45.0,
                difficulty_colors[i],
            );

            // Highlight if selected
            if selected {
                draw_poly_lines(
                    indicator_x,
                    y,
                    4,
                    indicator_size / 2.0 + 2.0,
                    45.0,
                    2.0,
                    Color::new(1.0, 0.9, 0.3, 1.0),
                );
            }
        }

        // Draw menu item
        let item_x = center_x - 170.0;
        draw_menu_item(
            item.label(),
            item_x,
            y,
            32.0,
            selected,
            menu.pulse_alpha(),
            if selected { menu.selection_anim() } else { 0.0 },
        );

        // Draw HP hearts on the right for difficulty items (not Back)
        if i < 4 && selected {
            let stats_x = center_x + 80.0;

            // HP indicator as hearts
            let hp = match i {
                0 => 5,
                1 => 3,
                2 => 2,
                _ => 1,
            };
            for h in 0..hp {
                let heart_x = stats_x + h as f32 * 18.0;
                draw_heart(heart_x, y - 5.0, 12.0, difficulty_colors[i]);
            }
        }
    }

    // Description of selected difficulty with shadow
    let description = menu.selected_item().description();
    draw_centered_text(
        description,
        sh * 0.78 + 1.0,
        18.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text(
        description,
        sh * 0.78,
        18.0,
        Color::new(0.5, 0.6, 0.7, 0.8),
    );

    // Draw recommendation text for selected
    let selected_idx = menu.selected;
    if selected_idx < 4 {
        let rec_text = match selected_idx {
            0 => "Recommended for newcomers",
            1 => "The classic experience",
            2 => "For skilled players",
            3 => "Only for the bravest",
            _ => "",
        };
        draw_centered_text(
            rec_text,
            sh * 0.83,
            16.0,
            difficulty_colors[selected_idx],
        );
    }

    // Navigation hints with shadow
    draw_centered_text(
        "W/S to navigate  |  SPACE to select  |  ESC to go back",
        sh - 29.0,
        16.0,
        Color::new(0.0, 0.0, 0.0, 0.3),
    );
    draw_centered_text(
        "W/S to navigate  |  SPACE to select  |  ESC to go back",
        sh - 30.0,
        16.0,
        Color::new(0.4, 0.5, 0.6, 0.75),
    );
}

/// Draw a simple heart shape
fn draw_heart(x: f32, y: f32, size: f32, color: Color) {
    let half = size / 2.0;
    let quarter = size / 4.0;

    // Two circles for the top bumps
    draw_circle(x + quarter, y, quarter, color);
    draw_circle(x + quarter * 3.0, y, quarter, color);

    // Triangle for the bottom point
    draw_triangle(
        vec2(x, y),
        vec2(x + size, y),
        vec2(x + half, y + size * 0.8),
        color,
    );
}

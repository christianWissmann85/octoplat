//! Biome selection screen for RogueLite mode
//!
//! Displays biome options with optional FLUX-generated thumbnails.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use octoplat_core::state::BiomeMenuItem;

use crate::rendering::UiTextureManager;
use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_item};

/// Draw the biome selection screen
///
/// If ui_textures is provided and has biome thumbnails, uses them.
/// Otherwise falls back to procedural color squares.
pub fn draw_biome_select(
    menu: &MenuState<BiomeMenuItem>,
    _time: f32,
    ui_textures: Option<&UiTextureManager>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Background
    clear_background(Color::new(0.05, 0.08, 0.15, 1.0));

    // Title with shadow
    draw_centered_text(
        "ROGUELITE",
        sh * 0.10 + 2.0,
        48.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text("ROGUELITE", sh * 0.10, 48.0, Color::new(0.3, 0.8, 0.9, 1.0));

    // Subtitle
    draw_centered_text(
        "Select a biome for endless linked-segment levels",
        sh * 0.17,
        22.0,
        Color::new(0.5, 0.6, 0.7, 0.8),
    );

    // Draw biomes in a grid-like layout for better visibility
    let center_x = sw / 2.0;
    let start_y = sh * 0.30;
    let line_height = 45.0;

    // Get biome colors for visual distinction
    let biome_colors = [
        Color::new(0.2, 0.4, 0.7, 1.0),  // Ocean Depths - deep blue
        Color::new(0.9, 0.4, 0.5, 1.0),  // Coral Reefs - coral pink
        Color::new(0.4, 0.7, 0.5, 1.0),  // Tropical Shore - turquoise/green
        Color::new(0.5, 0.35, 0.2, 1.0), // Shipwreck - brown
        Color::new(0.6, 0.8, 0.95, 1.0), // Arctic Waters - ice blue
        Color::new(0.9, 0.3, 0.1, 1.0),  // Volcanic Vents - orange-red
        Color::new(0.5, 0.5, 0.45, 1.0), // Sunken Ruins - stone gray
        Color::new(0.1, 0.05, 0.15, 1.0), // Abyss - near black
        Color::new(0.4, 0.5, 0.6, 1.0),  // Back - neutral gray
    ];

    // Map menu index to BiomeId for thumbnail lookup
    let biome_ids = [
        BiomeId::OceanDepths,
        BiomeId::CoralReefs,
        BiomeId::TropicalShore,
        BiomeId::Shipwreck,
        BiomeId::ArcticWaters,
        BiomeId::VolcanicVents,
        BiomeId::SunkenRuins,
        BiomeId::Abyss,
    ];

    // Check if we have any thumbnails
    let has_thumbnails = ui_textures
        .map(|t| t.biome_thumbnails.has_any())
        .unwrap_or(false);

    // Adjust thumbnail size based on whether we have textures
    let thumbnail_size = if has_thumbnails { 40.0 } else { 16.0 };

    for (i, item) in menu.items.iter().enumerate() {
        let y = start_y + i as f32 * line_height;
        let selected = i == menu.selected;

        // Draw biome indicator (thumbnail or color square)
        if i < 8 {
            let indicator_x = center_x - 150.0;
            let indicator_y = y - thumbnail_size + 4.0;

            // Try to draw thumbnail texture
            let mut drew_thumbnail = false;
            if let Some(textures) = ui_textures {
                if let Some(thumbnail) = textures.biome_thumbnails.get(biome_ids[i]) {
                    // Draw thumbnail with rounded appearance
                    draw_texture_ex(
                        thumbnail,
                        indicator_x,
                        indicator_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(thumbnail_size, thumbnail_size)),
                            ..Default::default()
                        },
                    );
                    drew_thumbnail = true;
                }
            }

            // Fallback to procedural color square
            if !drew_thumbnail {
                draw_rectangle(
                    indicator_x,
                    indicator_y,
                    thumbnail_size,
                    thumbnail_size,
                    biome_colors[i],
                );
            }

            // Draw border (highlight if selected)
            let border_color = if selected {
                Color::new(1.0, 0.9, 0.3, 1.0)
            } else {
                Color::new(0.5, 0.5, 0.5, 0.5)
            };
            let border_width = if selected { 2.0 } else { 1.0 };
            draw_rectangle_lines(
                indicator_x,
                indicator_y,
                thumbnail_size,
                thumbnail_size,
                border_width,
                border_color,
            );
        }

        // Draw menu item (adjust x position based on thumbnail size)
        let item_x = center_x - 150.0 + thumbnail_size + 15.0;
        draw_menu_item(
            item.label(),
            item_x,
            y,
            28.0,
            selected,
            menu.pulse_alpha(),
            if selected { menu.selection_anim() } else { 0.0 },
        );
    }

    // Description of selected biome with shadow
    let description = menu.selected_item().description();
    draw_centered_text(
        description,
        sh * 0.78 + 1.0,
        20.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_centered_text(
        description,
        sh * 0.78,
        20.0,
        Color::new(0.5, 0.6, 0.7, 0.8),
    );

    // Draw biome difficulty indicator for selected
    let selected_idx = menu.selected;
    if selected_idx < 8 {
        let difficulty_text = match selected_idx {
            0 => "Difficulty: Easy",
            1 => "Difficulty: Easy-Medium",
            2 => "Difficulty: Easy-Medium",
            3 => "Difficulty: Medium",
            4 => "Difficulty: Medium",
            5 => "Difficulty: Hard",
            6 => "Difficulty: Hard",
            7 => "Difficulty: Extreme",
            _ => "",
        };
        draw_centered_text(
            difficulty_text,
            sh * 0.83,
            16.0,
            biome_colors[selected_idx],
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

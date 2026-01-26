//! Level complete screen rendering

use macroquad::prelude::*;
use octoplat_core::state::LevelCompleteMenuItem;

use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_list};

/// Draw level complete screen
pub fn draw_level_complete(
    menu: &MenuState<LevelCompleteMenuItem>,
    gems_collected: u32,
    total_gems: u32,
    level_time: f32,
    deaths: u32,
    best_time: Option<f32>,
    best_gems: Option<u32>,
    banner_texture: Option<&Texture2D>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Background - use texture if available
    if let Some(texture) = banner_texture {
        draw_textured_fullscreen_background(texture, sw, sh);
    } else {
        clear_background(Color::new(0.05, 0.1, 0.15, 1.0));
    }

    // Title
    draw_centered_text(
        "LEVEL COMPLETE!",
        sh * 0.15,
        48.0,
        Color::new(0.3, 1.0, 0.5, 1.0),
    );

    // Stats box
    let box_x = sw / 2.0 - 180.0;
    let box_y = sh * 0.25;
    let box_w = 360.0;
    let box_h = 220.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.1, 0.15, 0.2, 0.9));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.5, 0.6, 0.8));

    // Stats
    let stat_x = box_x + 30.0;
    let stat_y = box_y + 40.0;
    let stat_spacing = 36.0;

    // Time
    let minutes = (level_time / 60.0) as u32;
    let seconds = level_time % 60.0;
    let time_str = format!("Time:   {:02}:{:05.2}", minutes, seconds);

    // Check for new record
    let is_new_time_record = best_time.map(|bt| level_time < bt).unwrap_or(true);
    let time_color = if is_new_time_record {
        Color::new(0.3, 1.0, 0.5, 1.0) // Green for new record
    } else {
        Color::new(0.8, 0.9, 1.0, 0.9)
    };

    draw_text(&time_str, stat_x, stat_y, 22.0, time_color);

    // Show best time comparison
    if let Some(bt) = best_time {
        let bt_min = (bt / 60.0) as u32;
        let bt_sec = bt % 60.0;
        let best_str = format!("(Best: {:02}:{:05.2})", bt_min, bt_sec);
        draw_text(
            &best_str,
            stat_x + 200.0,
            stat_y,
            18.0,
            Color::new(0.5, 0.6, 0.7, 0.7),
        );
    }
    if is_new_time_record && best_time.is_some() {
        draw_text("NEW!", stat_x + 320.0, stat_y, 18.0, Color::new(1.0, 0.8, 0.2, 1.0));
    }

    // Gems
    let is_new_gem_record = best_gems.map(|bg| gems_collected > bg).unwrap_or(gems_collected > 0);
    let gem_color = if is_new_gem_record {
        Color::new(1.0, 0.9, 0.3, 1.0) // Gold for new record
    } else {
        Color::new(1.0, 0.9, 0.3, 0.9)
    };

    draw_text(
        &format!("Gems:   {} / {}", gems_collected, total_gems),
        stat_x,
        stat_y + stat_spacing,
        22.0,
        gem_color,
    );

    // Show best gems comparison
    if let Some(bg) = best_gems {
        draw_text(
            &format!("(Best: {})", bg),
            stat_x + 200.0,
            stat_y + stat_spacing,
            18.0,
            Color::new(0.5, 0.6, 0.7, 0.7),
        );
    }
    if is_new_gem_record && best_gems.is_some() {
        draw_text("NEW!", stat_x + 320.0, stat_y + stat_spacing, 18.0, Color::new(1.0, 0.8, 0.2, 1.0));
    }

    // Deaths
    draw_text(
        &format!("Deaths: {}", deaths),
        stat_x,
        stat_y + stat_spacing * 2.0,
        22.0,
        Color::new(0.9, 0.5, 0.5, 0.9),
    );

    // Rating (simple calculation)
    let gem_ratio = gems_collected as f32 / total_gems.max(1) as f32;
    let (rating, rating_color) = if deaths == 0 && gem_ratio >= 1.0 {
        ("S", Color::new(1.0, 0.85, 0.0, 1.0)) // Gold
    } else if deaths <= 1 && gem_ratio >= 0.8 {
        ("A", Color::new(0.3, 1.0, 0.5, 1.0)) // Green
    } else if deaths <= 3 && gem_ratio >= 0.6 {
        ("B", Color::new(0.5, 0.8, 1.0, 1.0)) // Blue
    } else if deaths <= 5 && gem_ratio >= 0.4 {
        ("C", Color::new(0.9, 0.7, 0.3, 1.0)) // Orange
    } else {
        ("D", Color::new(0.7, 0.5, 0.5, 1.0)) // Gray-red
    };

    draw_text(
        &format!("Rating: {}", rating),
        stat_x,
        stat_y + stat_spacing * 3.0,
        22.0,
        rating_color,
    );

    // Draw rating stars
    let star_x = stat_x + 120.0;
    let star_y = stat_y + stat_spacing * 3.0 - 5.0;
    let num_stars = match rating {
        "S" => 5,
        "A" => 4,
        "B" => 3,
        "C" => 2,
        _ => 1,
    };
    for i in 0..5 {
        let color = if i < num_stars {
            rating_color
        } else {
            Color::new(0.3, 0.3, 0.3, 0.5)
        };
        draw_text("*", star_x + i as f32 * 18.0, star_y, 24.0, color);
    }

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 60.0;
    let start_y = sh * 0.72;
    let line_height = 40.0;

    draw_menu_list(menu, &labels, center_x, start_y, 26.0, line_height);
}

/// Draw a texture as fullscreen background, covering the screen
fn draw_textured_fullscreen_background(texture: &Texture2D, sw: f32, sh: f32) {
    let tex_w = texture.width();
    let tex_h = texture.height();

    // Scale to cover screen while maintaining aspect ratio
    let scale_x = sw / tex_w;
    let scale_y = sh / tex_h;
    let scale = scale_x.max(scale_y);

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

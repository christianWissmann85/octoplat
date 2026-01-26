//! Game over screen rendering

use macroquad::prelude::*;
use octoplat_core::state::GameOverMenuItem;

use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_list};

/// Draw game over screen
pub fn draw_game_over(
    menu: &MenuState<GameOverMenuItem>,
    deaths: u32,
    gems_collected: u32,
    lives: u32,
    background_texture: Option<&Texture2D>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Background - use texture if available
    if let Some(texture) = background_texture {
        draw_textured_fullscreen_background(texture, sw, sh);
    } else {
        clear_background(Color::new(0.1, 0.05, 0.08, 1.0));
    }

    // Title
    draw_centered_text(
        "GAME OVER",
        sh * 0.25,
        56.0,
        Color::new(0.9, 0.3, 0.3, 1.0),
    );

    // Brief stats
    draw_centered_text(
        &format!("Gems collected: {}", gems_collected),
        sh * 0.42,
        24.0,
        Color::new(1.0, 0.9, 0.3, 0.8),
    );

    draw_centered_text(
        &format!("Deaths: {}", deaths),
        sh * 0.49,
        24.0,
        Color::new(0.8, 0.6, 0.6, 0.8),
    );

    draw_centered_text(
        &format!("Lives remaining: {}", lives),
        sh * 0.56,
        24.0,
        Color::new(0.6, 0.6, 0.8, 0.8),
    );

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 50.0;
    let start_y = sh * 0.70;
    let line_height = 45.0;

    draw_menu_list(menu, &labels, center_x, start_y, 28.0, line_height);
}

/// Draw roguelite mode game over screen with run statistics
pub fn draw_roguelite_game_over(
    menu: &MenuState<GameOverMenuItem>,
    levels_completed: u32,
    total_gems: u32,
    deaths: u32,
    time: f32,
    background_texture: Option<&Texture2D>,
) {
    let sw = screen_width();
    let sh = screen_height();

    // Background - use texture if available
    if let Some(texture) = background_texture {
        draw_textured_fullscreen_background(texture, sw, sh);
    } else {
        clear_background(Color::new(0.08, 0.05, 0.12, 1.0));
    }

    // Title
    draw_centered_text(
        "RUN OVER",
        sh * 0.15,
        56.0,
        Color::new(0.9, 0.5, 0.3, 1.0),
    );

    // Subtitle
    draw_centered_text(
        "RogueLite Mode",
        sh * 0.22,
        24.0,
        Color::new(0.7, 0.6, 0.5, 0.8),
    );

    // Stats box
    let box_x = sw / 2.0 - 160.0;
    let box_y = sh * 0.28;
    let box_w = 320.0;
    let box_h = 180.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.12, 0.1, 0.15, 0.9));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.4, 0.3, 0.5, 0.8));

    // Run statistics
    let stat_x = box_x + 40.0;
    let stat_y = box_y + 45.0;
    let stat_spacing = 38.0;

    // Levels cleared
    draw_text(
        &format!("Levels Cleared: {}", levels_completed),
        stat_x,
        stat_y,
        24.0,
        Color::new(0.5, 0.9, 0.6, 1.0),
    );

    // Total gems
    draw_text(
        &format!("Total Gems: {}", total_gems),
        stat_x,
        stat_y + stat_spacing,
        24.0,
        Color::new(1.0, 0.9, 0.3, 1.0),
    );

    // Deaths
    draw_text(
        &format!("Deaths: {}", deaths),
        stat_x,
        stat_y + stat_spacing * 2.0,
        24.0,
        Color::new(0.9, 0.5, 0.5, 0.9),
    );

    // Time
    let minutes = (time / 60.0) as u32;
    let seconds = time % 60.0;
    draw_text(
        &format!("Time: {:02}:{:05.2}", minutes, seconds),
        stat_x,
        stat_y + stat_spacing * 3.0,
        24.0,
        Color::new(0.7, 0.8, 0.9, 0.9),
    );

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 50.0;
    let start_y = sh * 0.75;
    let line_height = 45.0;

    draw_menu_list(menu, &labels, center_x, start_y, 28.0, line_height);

    // Hint
    draw_centered_text(
        "Press ENTER to start a new run",
        sh - 40.0,
        16.0,
        Color::new(0.5, 0.5, 0.5, 0.5),
    );
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

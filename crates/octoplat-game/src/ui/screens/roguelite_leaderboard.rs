//! Roguelite leaderboard screen rendering

use macroquad::prelude::*;
use octoplat_core::save::EndlessRun;

use crate::ui::primitives::draw_centered_text;

/// Draw roguelite mode leaderboard
pub fn draw_roguelite_leaderboard(runs: &[EndlessRun], best_levels: u32, best_gems: u32) {
    let sw = screen_width();
    let sh = screen_height();

    // Background
    clear_background(Color::new(0.05, 0.08, 0.12, 1.0));

    // Title
    draw_centered_text(
        "ROGUELITE MODE - BEST RUNS",
        sh * 0.08,
        42.0,
        Color::new(0.4, 0.8, 0.9, 1.0),
    );

    // Best stats summary
    let summary_y = sh * 0.16;
    draw_centered_text(
        &format!("Best: {} levels  |  {} gems", best_levels, best_gems),
        summary_y,
        24.0,
        Color::new(1.0, 0.9, 0.4, 0.9),
    );

    // Leaderboard box
    let box_x = sw / 2.0 - 280.0;
    let box_y = sh * 0.22;
    let box_w = 560.0;
    let box_h = 360.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.1, 0.12, 0.95));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.4, 0.5, 0.6));

    // Header row
    let header_y = box_y + 30.0;
    let col_rank = box_x + 20.0;
    let col_levels = box_x + 70.0;
    let col_gems = box_x + 160.0;
    let col_deaths = box_x + 250.0;
    let col_time = box_x + 340.0;
    let col_seed = box_x + 440.0;

    let header_color = Color::new(0.6, 0.7, 0.8, 0.8);
    draw_text("#", col_rank, header_y, 18.0, header_color);
    draw_text("Levels", col_levels, header_y, 18.0, header_color);
    draw_text("Gems", col_gems, header_y, 18.0, header_color);
    draw_text("Deaths", col_deaths, header_y, 18.0, header_color);
    draw_text("Time", col_time, header_y, 18.0, header_color);
    draw_text("Seed", col_seed, header_y, 18.0, header_color);

    // Divider line
    draw_line(
        box_x + 10.0,
        header_y + 10.0,
        box_x + box_w - 10.0,
        header_y + 10.0,
        1.0,
        Color::new(0.3, 0.4, 0.5, 0.5),
    );

    // Leaderboard entries
    let row_start_y = header_y + 35.0;
    let row_height = 30.0;

    if runs.is_empty() {
        draw_centered_text(
            "No runs recorded yet. Start an endless run!",
            row_start_y + 80.0,
            20.0,
            Color::new(0.5, 0.5, 0.5, 0.7),
        );
    } else {
        for (i, run) in runs.iter().take(10).enumerate() {
            let y = row_start_y + i as f32 * row_height;

            // Highlight top 3
            let row_color = match i {
                0 => Color::new(1.0, 0.85, 0.3, 0.95), // Gold
                1 => Color::new(0.8, 0.8, 0.85, 0.9),  // Silver
                2 => Color::new(0.8, 0.6, 0.4, 0.85),  // Bronze
                _ => Color::new(0.7, 0.75, 0.8, 0.75),
            };

            // Rank
            draw_text(&format!("{}.", i + 1), col_rank, y, 18.0, row_color);

            // Levels completed
            draw_text(&format!("{}", run.levels_completed), col_levels, y, 18.0, row_color);

            // Gems collected
            draw_text(&format!("{}", run.gems_collected), col_gems, y, 18.0, row_color);

            // Deaths
            draw_text(&format!("{}", run.deaths), col_deaths, y, 18.0, row_color);

            // Time
            let minutes = (run.time / 60.0) as u32;
            let seconds = (run.time % 60.0) as u32;
            draw_text(&format!("{}:{:02}", minutes, seconds), col_time, y, 18.0, row_color);

            // Seed (truncated)
            let seed_str = format!("{}", run.seed);
            let seed_display = if seed_str.len() > 8 {
                format!("{}...", &seed_str[..8])
            } else {
                seed_str
            };
            draw_text(&seed_display, col_seed, y, 16.0, Color::new(0.5, 0.55, 0.6, 0.7));
        }
    }

    // Navigation hint
    draw_centered_text(
        "Press SPACE/ENTER to start  |  ESC to go back",
        sh - 40.0,
        18.0,
        Color::new(0.5, 0.5, 0.5, 0.6),
    );
}

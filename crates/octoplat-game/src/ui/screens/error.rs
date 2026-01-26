//! Error screen rendering

use macroquad::prelude::*;
use octoplat_core::state::ErrorMenuItem;

use crate::ui::menu_state::MenuState;
use crate::ui::primitives::{draw_centered_text, draw_menu_list};

/// Draw error screen with message and recovery options
pub fn draw_error_screen(menu: &MenuState<ErrorMenuItem>, error_message: &str) {
    let sw = screen_width();
    let sh = screen_height();

    // Background (dark red tint)
    clear_background(Color::new(0.12, 0.05, 0.05, 1.0));

    // Title
    draw_centered_text(
        "ERROR",
        sh * 0.20,
        56.0,
        Color::new(0.9, 0.3, 0.3, 1.0),
    );

    // Subtitle
    draw_centered_text(
        "Something went wrong",
        sh * 0.28,
        24.0,
        Color::new(0.7, 0.5, 0.5, 0.8),
    );

    // Error message box
    let box_x = sw / 2.0 - 280.0;
    let box_y = sh * 0.35;
    let box_w = 560.0;
    let box_h = 120.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.04, 0.04, 0.9));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.5, 0.2, 0.2, 0.8));

    // Error message (with word wrapping for long messages)
    let max_chars_per_line = 60;
    let words: Vec<&str> = error_message.split_whitespace().collect();
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_chars_per_line {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Limit to 4 lines max
    let display_lines = if lines.len() > 4 {
        let mut truncated = lines[..3].to_vec();
        truncated.push("...".to_string());
        truncated
    } else {
        lines
    };

    let text_y_start = box_y + 35.0;
    let line_height = 22.0;
    for (i, line) in display_lines.iter().enumerate() {
        draw_text(
            line,
            box_x + 20.0,
            text_y_start + (i as f32) * line_height,
            18.0,
            Color::new(0.9, 0.8, 0.8, 1.0),
        );
    }

    // Menu items
    let labels: Vec<&str> = menu.items.iter().map(|item| item.label()).collect();
    let center_x = sw / 2.0 - 50.0;
    let start_y = sh * 0.70;
    let menu_line_height = 45.0;

    draw_menu_list(menu, &labels, center_x, start_y, 28.0, menu_line_height);

    // Hint
    draw_centered_text(
        "Press ENTER to select",
        sh - 40.0,
        16.0,
        Color::new(0.5, 0.5, 0.5, 0.5),
    );
}

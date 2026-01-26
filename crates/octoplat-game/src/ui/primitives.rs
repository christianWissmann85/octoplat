//! UI primitive drawing functions

use macroquad::prelude::*;

use super::menu_state::MenuState;

/// Draw centered text
pub fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        (screen_width() - dims.width) / 2.0,
        y,
        font_size,
        color,
    );
}

/// Draw a menu item (text with optional selection highlight)
pub fn draw_menu_item(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    selected: bool,
    pulse_alpha: f32,
    selection_anim: f32,
) {
    let base_color = if selected {
        Color::new(1.0, 0.9, 0.3, pulse_alpha) // Golden highlight
    } else {
        Color::new(0.7, 0.8, 0.9, 0.8) // Muted blue-white
    };

    // Selection indicator
    if selected {
        let indicator_x = x - 30.0 - (1.0 - selection_anim) * 10.0;
        let indicator_alpha = selection_anim;
        draw_text(
            ">",
            indicator_x,
            y,
            font_size,
            Color::new(1.0, 0.9, 0.3, indicator_alpha),
        );
    }

    draw_text(text, x, y, font_size, base_color);
}

/// Draw a vertical menu list
pub fn draw_menu_list<T: Clone + Copy + PartialEq>(
    menu: &MenuState<T>,
    labels: &[&str],
    center_x: f32,
    start_y: f32,
    font_size: f32,
    line_height: f32,
) {
    for (i, label) in labels.iter().enumerate() {
        let y = start_y + i as f32 * line_height;
        let selected = i == menu.selected;
        draw_menu_item(
            label,
            center_x,
            y,
            font_size,
            selected,
            menu.pulse_alpha(),
            if selected { menu.selection_anim() } else { 0.0 },
        );
    }
}

/// Draw a volume slider
pub fn draw_volume_slider(x: f32, y: f32, width: f32, value: f32, is_selected: bool) {
    let height = 16.0;

    // Background track
    draw_rectangle(
        x,
        y,
        width,
        height,
        Color::new(0.2, 0.2, 0.25, 0.8),
    );

    // Filled portion
    let fill_color = if is_selected {
        Color::new(0.4, 0.7, 0.9, 0.9)
    } else {
        Color::new(0.3, 0.5, 0.6, 0.8)
    };
    draw_rectangle(
        x,
        y,
        width * value,
        height,
        fill_color,
    );

    // Border
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        1.0,
        Color::new(0.4, 0.5, 0.6, 0.6),
    );

    // Percentage text
    let pct = (value * 100.0) as i32;
    draw_text(
        &format!("{}%", pct),
        x + width + 10.0,
        y + 13.0,
        18.0,
        Color::new(0.6, 0.7, 0.8, 0.8),
    );
}

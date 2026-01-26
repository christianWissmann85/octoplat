//! Loading screen with animated octopus
//!
//! Displays a cute animated octopus while the game loads audio assets.
//! Supports FLUX-generated textures with graceful fallback to procedural rendering.

use macroquad::prelude::*;

use crate::rendering::UiTextureManager;

/// Funny Octoplat-themed loading messages
const LOADING_MESSAGES: &[&str] = &[
    "Waking up the octopus...",
    "Filling the ocean with water...",
    "Counting tentacles (1...2...8!)...",
    "Teaching crabs to walk sideways...",
    "Polishing the pearls...",
    "Inflating the pufferfish...",
    "Calibrating bubble physics...",
    "Brewing underwater coffee...",
    "Untangling tentacles...",
    "Loading ink cartridges...",
    "Consulting the ancient coral...",
    "Tuning the ocean currents...",
    "Warming up the volcanic vents...",
    "Defrosting the arctic waters...",
    "Reticulating splines (underwater edition)...",
];

/// Get a loading message that changes over time
pub fn get_loading_message(time: f32) -> &'static str {
    let index = ((time * 0.4) as usize) % LOADING_MESSAGES.len();
    LOADING_MESSAGES[index]
}

/// Draw the loading screen with animated octopus
///
/// If ui_textures is provided and has loaded textures, uses them.
/// Otherwise falls back to procedural rendering.
pub fn draw_loading_screen(
    progress: f32,
    message: &str,
    time: f32,
    ui_textures: Option<&UiTextureManager>,
) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let center_x = screen_w / 2.0;
    let center_y = screen_h / 2.0 - 30.0;

    // Draw background (textured or procedural)
    let has_bg_texture = ui_textures
        .and_then(|t| t.loading.background.as_ref())
        .is_some();

    if let Some(bg) = ui_textures.and_then(|t| t.loading.background.as_ref()) {
        // Draw textured background, scaled to fill screen
        draw_texture_ex(
            bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                ..Default::default()
            },
        );
    } else {
        // Procedural background
        clear_background(Color::from_rgba(15, 25, 45, 255));
    }

    // Draw ambient bubbles (always, but adjust opacity if we have background texture)
    let bubble_opacity = if has_bg_texture { 0.6 } else { 1.0 };
    draw_bubbles(time, bubble_opacity);

    // Draw the octopus (textured or procedural)
    if let Some(octo) = ui_textures.and_then(|t| t.loading.octopus.as_ref()) {
        draw_textured_octopus(octo, center_x, center_y, time);
    } else {
        draw_octopus(center_x, center_y, time);
    }

    // Draw loading text
    let text = "Loading...";
    let font_size = 32.0;
    let text_dim = measure_text(text, None, font_size as u16, 1.0);

    // Text shadow for better visibility on textured backgrounds
    draw_text(
        text,
        center_x - text_dim.width / 2.0 + 2.0,
        center_y + 120.0 + 2.0,
        font_size,
        Color::from_rgba(0, 0, 0, 100),
    );
    draw_text(
        text,
        center_x - text_dim.width / 2.0,
        center_y + 120.0,
        font_size,
        WHITE,
    );

    // Draw progress bar
    let bar_width = 300.0;
    let bar_height = 12.0;
    let bar_x = center_x - bar_width / 2.0;
    let bar_y = center_y + 140.0;

    // Bar background
    draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::from_rgba(40, 60, 90, 200));

    // Bar fill with animated shimmer effect
    let fill_width = bar_width * progress.clamp(0.0, 1.0);
    draw_rectangle(
        bar_x,
        bar_y,
        fill_width,
        bar_height,
        Color::from_rgba(100, 200, 255, 255),
    );

    // Animated shimmer on the progress bar
    if fill_width > 10.0 {
        let shimmer_x = bar_x + ((time * 100.0) % fill_width);
        let shimmer_alpha = ((time * 5.0).sin() * 0.5 + 0.5) * 100.0;
        draw_rectangle(
            shimmer_x,
            bar_y,
            20.0_f32.min(fill_width - (shimmer_x - bar_x)),
            bar_height,
            Color::from_rgba(255, 255, 255, shimmer_alpha as u8),
        );
    }

    // Bar border
    draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, Color::from_rgba(150, 200, 255, 200));

    // Draw spinning dots indicator (shows activity even during blocking loads)
    let dots_y = bar_y + 25.0;
    let num_dots = 3;
    let dot_spacing = 15.0;
    let dots_start_x = center_x - (num_dots as f32 - 1.0) * dot_spacing / 2.0;

    for i in 0..num_dots {
        let dot_phase = time * 3.0 - i as f32 * 0.3;
        let dot_alpha = ((dot_phase.sin() * 0.5 + 0.5) * 200.0 + 55.0) as u8;
        let dot_size = 4.0 + (dot_phase.sin() * 0.5 + 0.5) * 3.0;
        draw_circle(
            dots_start_x + i as f32 * dot_spacing,
            dots_y,
            dot_size,
            Color::from_rgba(150, 200, 255, dot_alpha),
        );
    }

    // Draw status message with shadow
    let msg_size = 18.0;
    let msg_dim = measure_text(message, None, msg_size as u16, 1.0);
    draw_text(
        message,
        center_x - msg_dim.width / 2.0 + 1.0,
        center_y + 175.0 + 1.0,
        msg_size,
        Color::from_rgba(0, 0, 0, 80),
    );
    draw_text(
        message,
        center_x - msg_dim.width / 2.0,
        center_y + 175.0,
        msg_size,
        Color::from_rgba(150, 180, 220, 255),
    );

    // Draw fun loading tip at the bottom
    let tips = [
        "Tip: Hold the grapple button to wall-slide!",
        "Tip: Jet boost can be aimed in any direction!",
        "Tip: Ink clouds make you temporarily invincible!",
        "Tip: Collect gems to increase your score!",
        "Tip: Bounce pads give you extra height!",
    ];
    let tip_index = ((time * 0.3) as usize) % tips.len();
    let tip = tips[tip_index];
    let tip_size = 16.0;
    let tip_dim = measure_text(tip, None, tip_size as u16, 1.0);
    draw_text(
        tip,
        center_x - tip_dim.width / 2.0 + 1.0,
        screen_h - 40.0 + 1.0,
        tip_size,
        Color::from_rgba(0, 0, 0, 60),
    );
    draw_text(
        tip,
        center_x - tip_dim.width / 2.0,
        screen_h - 40.0,
        tip_size,
        Color::from_rgba(100, 140, 180, 200),
    );
}

/// Draw textured octopus with animation
fn draw_textured_octopus(texture: &Texture2D, center_x: f32, center_y: f32, time: f32) {
    let bob = (time * 2.0).sin() * 8.0;
    let rotation = (time * 0.5).sin() * 0.08; // Gentle sway
    let squish = 1.0 + (time * 3.0).sin() * 0.03;

    // Scale the 512px sprite down to a reasonable size
    let base_size = 180.0;
    let dest_w = base_size * squish;
    let dest_h = base_size / squish;

    draw_texture_ex(
        texture,
        center_x - dest_w / 2.0,
        center_y + bob - dest_h / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest_w, dest_h)),
            rotation,
            pivot: Some(vec2(center_x, center_y + bob)),
            ..Default::default()
        },
    );
}

/// Draw ambient floating bubbles
fn draw_bubbles(time: f32, opacity_mult: f32) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    for i in 0..15 {
        let seed = i as f32 * 127.3;
        let x = ((seed * 7.3).sin() * 0.5 + 0.5) * screen_w;
        let base_y = ((seed * 3.7).cos() * 0.5 + 0.5) * screen_h;
        let y = (base_y + time * 30.0 * (0.5 + (seed * 0.1).sin() * 0.5)) % (screen_h + 50.0) - 25.0;
        let size = 3.0 + (seed * 0.7).sin().abs() * 8.0;
        let alpha = (0.1 + (seed * 0.3).cos().abs() * 0.2) * opacity_mult;

        draw_circle(
            x,
            screen_h - y,
            size,
            Color::from_rgba(150, 200, 255, (alpha * 255.0) as u8),
        );
    }
}

/// Draw a cute animated octopus (procedural fallback)
fn draw_octopus(x: f32, y: f32, time: f32) {
    let bob = (time * 2.0).sin() * 8.0;
    let squish = 1.0 + (time * 3.0).sin() * 0.05;

    // Body color - cute pink/purple
    let body_color = Color::from_rgba(220, 120, 180, 255);
    let dark_color = Color::from_rgba(180, 80, 140, 255);
    let light_color = Color::from_rgba(255, 180, 220, 255);

    // Draw 8 tentacles with wavy animation
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0 - std::f32::consts::PI / 2.0;
        let wave_offset = i as f32 * 0.5;

        draw_tentacle(
            x + angle.cos() * 25.0,
            y + bob + 35.0 + angle.sin().abs() * 10.0,
            angle,
            time,
            wave_offset,
            body_color,
            dark_color,
        );
    }

    // Draw body (mantle) - oval shape
    let body_w = 70.0 * squish;
    let body_h = 55.0 / squish;

    // Body shadow
    draw_ellipse(x + 3.0, y + bob + 3.0, body_w / 2.0, body_h / 2.0, 0.0, Color::from_rgba(0, 0, 0, 40));

    // Main body
    draw_ellipse(x, y + bob, body_w / 2.0, body_h / 2.0, 0.0, body_color);

    // Body highlight
    draw_ellipse(x - 10.0, y + bob - 10.0, body_w / 4.0, body_h / 4.0, 0.0, light_color);

    // Draw cute eyes
    let eye_y = y + bob - 5.0;
    let blink = if ((time * 0.5) % 3.0) < 0.15 { 0.1 } else { 1.0 };

    // Left eye
    draw_ellipse(x - 15.0, eye_y, 12.0, 14.0 * blink, 0.0, WHITE);
    draw_ellipse(x - 15.0 + (time * 0.5).sin() * 2.0, eye_y, 6.0, 7.0 * blink, 0.0, Color::from_rgba(30, 30, 50, 255));
    draw_circle(x - 17.0, eye_y - 3.0, 3.0, Color::from_rgba(255, 255, 255, 200));

    // Right eye
    draw_ellipse(x + 15.0, eye_y, 12.0, 14.0 * blink, 0.0, WHITE);
    draw_ellipse(x + 15.0 + (time * 0.5).sin() * 2.0, eye_y, 6.0, 7.0 * blink, 0.0, Color::from_rgba(30, 30, 50, 255));
    draw_circle(x + 13.0, eye_y - 3.0, 3.0, Color::from_rgba(255, 255, 255, 200));

    // Cute little smile
    let smile_y = y + bob + 12.0;
    draw_arc(x, smile_y, 8.0, 12.0, 0.0, std::f32::consts::PI, dark_color);

    // Rosy cheeks
    draw_ellipse(x - 28.0, eye_y + 8.0, 8.0, 5.0, 0.0, Color::from_rgba(255, 150, 180, 100));
    draw_ellipse(x + 28.0, eye_y + 8.0, 8.0, 5.0, 0.0, Color::from_rgba(255, 150, 180, 100));
}

/// Draw a single wavy tentacle
fn draw_tentacle(x: f32, y: f32, base_angle: f32, time: f32, wave_offset: f32, color: Color, dark: Color) {
    let segments = 8;
    let segment_len = 12.0;

    let mut prev_x = x;
    let mut prev_y = y;

    for i in 0..segments {
        let t = i as f32 / segments as f32;
        let wave = (time * 4.0 + wave_offset + t * 3.0).sin() * (1.0 - t) * 15.0;
        let angle = base_angle + wave * 0.05;

        let thickness = 8.0 * (1.0 - t * 0.7);

        let next_x = prev_x + angle.cos() * segment_len + wave * 0.3;
        let next_y = prev_y + segment_len * 0.8;

        // Draw segment
        draw_line(prev_x, prev_y, next_x, next_y, thickness, color);

        // Draw sucker on alternating segments
        if i % 2 == 1 && i < segments - 1 {
            let sucker_x = (prev_x + next_x) / 2.0;
            let sucker_y = (prev_y + next_y) / 2.0;
            draw_circle(sucker_x, sucker_y, thickness * 0.4, dark);
        }

        prev_x = next_x;
        prev_y = next_y;
    }
}

/// Draw an arc (for the smile)
fn draw_arc(x: f32, y: f32, radius_x: f32, radius_y: f32, start_angle: f32, end_angle: f32, color: Color) {
    let segments = 16;
    let angle_step = (end_angle - start_angle) / segments as f32;

    for i in 0..segments {
        let a1 = start_angle + angle_step * i as f32;
        let a2 = start_angle + angle_step * (i + 1) as f32;

        let x1 = x + a1.cos() * radius_x;
        let y1 = y + a1.sin() * radius_y;
        let x2 = x + a2.cos() * radius_x;
        let y2 = y + a2.sin() * radius_y;

        draw_line(x1, y1, x2, y2, 3.0, color);
    }
}

/// Draw an ellipse
fn draw_ellipse(x: f32, y: f32, radius_x: f32, radius_y: f32, _rotation: f32, color: Color) {
    let segments = 32;
    let angle_step = std::f32::consts::PI * 2.0 / segments as f32;

    // Draw filled ellipse using triangles
    for i in 0..segments {
        let a1 = angle_step * i as f32;
        let a2 = angle_step * (i + 1) as f32;

        let x1 = x + a1.cos() * radius_x;
        let y1 = y + a1.sin() * radius_y;
        let x2 = x + a2.cos() * radius_x;
        let y2 = y + a2.sin() * radius_y;

        draw_triangle(
            Vec2::new(x, y),
            Vec2::new(x1, y1),
            Vec2::new(x2, y2),
            color,
        );
    }
}

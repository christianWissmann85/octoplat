//! Title screen rendering
//!
//! Displays the game title with animated background effects.
//! Supports FLUX-generated textures with graceful fallback to procedural rendering.

use macroquad::prelude::*;

use crate::rendering::UiTextureManager;
use crate::ui::primitives::draw_centered_text;

/// Draw the title screen
///
/// If ui_textures is provided and has loaded textures, uses them.
/// Otherwise falls back to procedural rendering.
pub fn draw_title_screen(time: f32, ui_textures: Option<&UiTextureManager>) {
    let sw = screen_width();
    let sh = screen_height();

    // Draw background (textured or procedural)
    let has_bg_texture = ui_textures
        .and_then(|t| t.title.background.as_ref())
        .is_some();

    if let Some(bg) = ui_textures.and_then(|t| t.title.background.as_ref()) {
        // Draw textured background, scaled to fill screen
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
        // Procedural background gradient (dark blue)
        clear_background(Color::new(0.05, 0.08, 0.15, 1.0));
    }

    // Draw animated light rays (always, adjust opacity based on background)
    let ray_opacity = if has_bg_texture { 0.4 } else { 1.0 };
    draw_light_rays(time, ray_opacity);

    // Draw animated bubbles
    let bubble_opacity = if has_bg_texture { 0.5 } else { 0.8 };
    draw_animated_bubbles(time, bubble_opacity);

    // Draw logo or title text
    if let Some(logo) = ui_textures.and_then(|t| t.title.logo.as_ref()) {
        draw_logo_texture(logo, sw, sh, time);
    } else {
        // Procedural title text
        draw_title_text(sh, time);
    }

    // Pulsing "Press Start" text
    let pulse = ((time * 2.0).sin() + 1.0) / 2.0;
    let alpha = 0.5 + pulse * 0.5;

    // Shadow for better visibility
    draw_centered_text(
        "Press SPACE or ENTER",
        sh * 0.65 + 2.0,
        28.0,
        Color::new(0.0, 0.0, 0.0, alpha * 0.5),
    );
    draw_centered_text(
        "Press SPACE or ENTER",
        sh * 0.65,
        28.0,
        Color::new(1.0, 1.0, 1.0, alpha),
    );

    // Version/credits with shadow
    draw_text(
        "v0.1 - Made with Rust & Macroquad",
        11.0,
        sh - 9.0,
        16.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );
    draw_text(
        "v0.1 - Made with Rust & Macroquad",
        10.0,
        sh - 10.0,
        16.0,
        Color::new(0.4, 0.5, 0.6, 0.6),
    );
}

/// Draw animated light rays from above
fn draw_light_rays(time: f32, opacity_mult: f32) {
    let sw = screen_width();
    let sh = screen_height();

    for i in 0..5 {
        let base_x = sw * (0.15 + i as f32 * 0.18);
        let sway = (time * 0.2 + i as f32 * 0.7).sin() * 40.0;
        let base_alpha = (0.03 + (time * 0.15 + i as f32 * 0.5).sin().abs() * 0.025) * opacity_mult;

        // Ray width varies over time
        let ray_width = 60.0 + (time * 0.4 + i as f32 * 0.3).sin() * 20.0;

        // Draw light ray as gradient triangle from top
        let top_x = base_x + sway * 0.3;
        let bottom_left = base_x + sway - ray_width;
        let bottom_right = base_x + sway + ray_width;

        // Multiple passes for soft glow effect
        for pass in 0..3 {
            let pass_alpha = base_alpha * (1.0 - pass as f32 * 0.3);
            let pass_expand = pass as f32 * 15.0;

            draw_triangle(
                Vec2::new(top_x, 0.0),
                Vec2::new(bottom_left - pass_expand, sh),
                Vec2::new(bottom_right + pass_expand, sh),
                Color::new(1.0, 1.0, 0.9, pass_alpha),
            );
        }
    }
}

/// Draw floating animated bubbles
fn draw_animated_bubbles(time: f32, opacity_mult: f32) {
    let sw = screen_width();
    let sh = screen_height();

    for i in 0..20 {
        let seed = i as f32 * 127.3;
        let x = ((seed * 7.3).sin() * 0.5 + 0.5) * sw;
        let base_y = ((seed * 3.7).cos() * 0.5 + 0.5) * sh;
        let speed = 20.0 + (seed * 0.1).sin().abs() * 30.0;
        let y = (base_y + time * speed) % (sh + 50.0) - 25.0;
        let size = 2.0 + (seed * 0.7).sin().abs() * 6.0;
        let alpha = (0.12 + (seed * 0.3).cos().abs() * 0.12) * opacity_mult;

        // Main bubble
        draw_circle(x, sh - y, size, Color::new(0.8, 0.9, 1.0, alpha));

        // Highlight
        draw_circle(
            x - size * 0.3,
            sh - y - size * 0.3,
            size * 0.3,
            Color::new(1.0, 1.0, 1.0, alpha * 0.6),
        );
    }
}

/// Draw the logo texture with animation
fn draw_logo_texture(logo: &Texture2D, sw: f32, sh: f32, time: f32) {
    let bob = (time * 1.5).sin() * 5.0;
    let scale_pulse = 1.0 + (time * 2.0).sin() * 0.02;

    // Scale logo to reasonable size (max 60% of screen width)
    let logo_w = logo.width();
    let logo_h = logo.height();
    let max_width = sw * 0.6;
    let scale = (max_width / logo_w).min(1.0) * scale_pulse;
    let dest_w = logo_w * scale;
    let dest_h = logo_h * scale;

    let x = (sw - dest_w) / 2.0;
    let y = sh * 0.25 + bob - dest_h / 2.0;

    // Glow effect behind logo
    let glow_alpha = 0.15 + (time * 1.5).sin().abs() * 0.1;
    for glow in 1..=3 {
        let glow_expand = glow as f32 * 8.0;
        draw_rectangle(
            x - glow_expand,
            y - glow_expand,
            dest_w + glow_expand * 2.0,
            dest_h + glow_expand * 2.0,
            Color::new(0.3, 0.8, 1.0, glow_alpha / glow as f32),
        );
    }

    draw_texture_ex(
        logo,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest_w, dest_h)),
            ..Default::default()
        },
    );
}

/// Draw procedural title text (fallback)
fn draw_title_text(sh: f32, time: f32) {
    let bob = (time * 1.5).sin() * 3.0;

    // Title shadow
    draw_centered_text(
        "OCTOPLAT",
        sh * 0.3 + bob + 3.0,
        80.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );

    // Title with slight color animation
    let hue_shift = (time * 0.3).sin() * 0.1;
    draw_centered_text(
        "OCTOPLAT",
        sh * 0.3 + bob,
        80.0,
        Color::new(0.3 + hue_shift, 0.8, 0.9 - hue_shift, 1.0),
    );

    // Subtitle shadow
    draw_centered_text(
        "A Tentacle Platformer",
        sh * 0.3 + 50.0 + bob * 0.5 + 2.0,
        24.0,
        Color::new(0.0, 0.0, 0.0, 0.3),
    );

    // Subtitle
    draw_centered_text(
        "A Tentacle Platformer",
        sh * 0.3 + 50.0 + bob * 0.5,
        24.0,
        Color::new(0.5, 0.6, 0.7, 0.8),
    );
}

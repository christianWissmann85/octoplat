//! HUD rendering
//!
//! Draws the game HUD including gem counter, lives, and ability charges.
//! Supports optional FLUX-generated textures with procedural fallback.

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::player::Player;
use crate::rendering::ui_textures::HudTextures;

/// Draw the HUD (gem counter, lives, and ability charges)
///
/// If hud_textures is provided, uses texture-based rendering where available.
/// Otherwise falls back to procedural shapes.
pub fn draw_hud(
    gems_collected: u32,
    total_gems: u32,
    player: &Player,
    config: &GameConfig,
    lives: u32,
    is_endless: bool,
    hud_textures: Option<&HudTextures>,
) {
    // Lives display (top-left, above gems)
    draw_lives_display(lives, hud_textures);

    // Gem counter (shifted down to make room for lives)
    draw_gem_counter(gems_collected, total_gems, hud_textures);

    // Stamina bar (for wall grip and grapple swing)
    draw_stamina_bar(player, config, hud_textures);

    // Wall jump charges
    draw_wall_jump_charges(player, config);

    // Ability charges
    draw_jet_charges(player, config, hud_textures);
    draw_ink_charges(player, config, hud_textures);

    // Mode indicator in endless mode
    if is_endless {
        draw_text(
            "ENDLESS",
            screen_width() - 80.0,
            screen_height() - 30.0,
            16.0,
            Color::new(1.0, 0.8, 0.3, 0.7),
        );
    }

    // Controls hint (small text at bottom)
    draw_text(
        "WASD: Move | Space: Jump | Shift: Sprint | C: Slide | S: Dive | E: Jet | Q: Ink | F: Grab/Grapple",
        10.0,
        screen_height() - 10.0,
        16.0,
        Color::new(1.0, 1.0, 1.0, 0.5),
    );
}

/// Draw the lives display with heart icons
fn draw_lives_display(lives: u32, hud_textures: Option<&HudTextures>) {
    let start_x = 20.0;
    let y = 25.0;
    let heart_size = 24.0; // Larger for textures
    let heart_spacing = 28.0;

    if lives == u32::MAX {
        // Infinite lives
        draw_text("INF", start_x, y + 5.0, 20.0, Color::new(0.8, 0.8, 0.8, 0.8));
        return;
    }

    // Check if we have heart textures
    let has_heart_texture = hud_textures
        .and_then(|t| t.heart_full.as_ref())
        .is_some();

    // Draw hearts (max 5 displayed as icons)
    let hearts_to_draw = lives.min(5);
    for i in 0..hearts_to_draw {
        let x = start_x + i as f32 * heart_spacing;

        if let Some(textures) = hud_textures {
            if let Some(heart_tex) = &textures.heart_full {
                // Draw textured heart
                draw_texture_ex(
                    heart_tex,
                    x,
                    y - heart_size / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(heart_size, heart_size)),
                        ..Default::default()
                    },
                );
                continue;
            }
        }

        // Fallback to procedural heart
        draw_heart(x, y, if has_heart_texture { heart_size } else { 12.0 }, Color::new(0.9, 0.2, 0.3, 1.0));
    }

    // If more than 5 lives, show "x N" after the hearts
    if lives > 5 {
        let text_x = start_x + 5.0 * heart_spacing + 5.0;
        draw_text(
            &format!("x{}", lives),
            text_x,
            y + 5.0,
            18.0,
            Color::new(0.9, 0.2, 0.3, 1.0),
        );
    }
}

/// Draw a simple heart shape
fn draw_heart(x: f32, y: f32, size: f32, color: Color) {
    // Simple heart using circles and a triangle
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

/// Draw the gem counter with icon and text
fn draw_gem_counter(gems_collected: u32, total_gems: u32, hud_textures: Option<&HudTextures>) {
    let icon_x = 20.0;
    let icon_y = 45.0;
    let icon_size = 24.0;

    // Try to draw textured gem icon
    let mut drew_texture = false;
    if let Some(textures) = hud_textures {
        if let Some(gem_tex) = &textures.gem {
            draw_texture_ex(
                gem_tex,
                icon_x,
                icon_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(icon_size, icon_size)),
                    ..Default::default()
                },
            );
            drew_texture = true;
        }
    }

    // Fallback to procedural diamond
    if !drew_texture {
        draw_poly(30.0, 55.0, 4, 10.0, 45.0, Color::new(0.3, 0.8, 1.0, 1.0));
    }

    // Text counter
    let text = format!("{} / {}", gems_collected, total_gems);
    let text_x = if drew_texture { icon_x + icon_size + 8.0 } else { 50.0 };
    draw_text(&text, text_x, 63.0, 28.0, WHITE);
}

/// Draw the stamina bar
fn draw_stamina_bar(player: &Player, config: &GameConfig, hud_textures: Option<&HudTextures>) {
    let stamina_x = 20.0;
    let stamina_y = 80.0;
    let bar_width = 80.0;
    let bar_height = 8.0;
    let stamina_ratio = (player.wall_stamina / config.wall_stamina_max).clamp(0.0, 1.0);

    // Check if we have a stamina frame texture
    if let Some(textures) = hud_textures {
        if let Some(frame_tex) = &textures.stamina_frame {
            // Draw frame texture (assumes frame is larger than the bar fill area)
            let frame_width = bar_width + 8.0;
            let frame_height = bar_height + 8.0;
            draw_texture_ex(
                frame_tex,
                stamina_x - 4.0,
                stamina_y - 4.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(frame_width, frame_height)),
                    ..Default::default()
                },
            );
        }
    }

    // Background (only if no frame texture)
    if hud_textures.and_then(|t| t.stamina_frame.as_ref()).is_none() {
        draw_rectangle(
            stamina_x,
            stamina_y,
            bar_width,
            bar_height,
            Color::new(0.2, 0.2, 0.3, 0.8),
        );
    }

    // Stamina fill - color changes from green to yellow to red
    let fill_color = if stamina_ratio > 0.5 {
        Color::new(0.3, 0.8, 0.4, 1.0) // Green
    } else if stamina_ratio > 0.25 {
        Color::new(0.9, 0.8, 0.2, 1.0) // Yellow
    } else {
        Color::new(0.9, 0.3, 0.2, 1.0) // Red
    };
    draw_rectangle(
        stamina_x,
        stamina_y,
        bar_width * stamina_ratio,
        bar_height,
        fill_color,
    );

    // Border (only if no frame texture)
    if hud_textures.and_then(|t| t.stamina_frame.as_ref()).is_none() {
        draw_rectangle_lines(
            stamina_x,
            stamina_y,
            bar_width,
            bar_height,
            1.0,
            Color::new(0.5, 0.5, 0.6, 0.8),
        );
    }
}

/// Draw wall jump charge indicators (small wall icons)
fn draw_wall_jump_charges(player: &Player, config: &GameConfig) {
    let base_x = 110.0;
    let base_y = 80.0;

    for i in 0..config.wall_jumps_max {
        let x = base_x + i as f32 * 14.0;
        let has_charge = i < player.wall_jumps_remaining;

        let color = if has_charge {
            Color::new(0.9, 0.6, 0.3, 1.0) // Orange when available
        } else {
            Color::new(0.3, 0.3, 0.3, 0.5) // Gray when used
        };

        // Draw small wall/brick icon
        draw_rectangle(x, base_y, 10.0, 8.0, color);
        draw_rectangle_lines(x, base_y, 10.0, 8.0, 1.0, Color::new(0.2, 0.2, 0.2, 0.8));
    }
}

/// Draw jet charge indicators
fn draw_jet_charges(player: &Player, config: &GameConfig, hud_textures: Option<&HudTextures>) {
    let jet_x = screen_width() - 150.0;
    let icon_size = 20.0;

    // Check if we have jet icon texture
    let has_jet_icon = hud_textures
        .and_then(|t| t.jet_icon.as_ref())
        .is_some();

    // Draw label or icon
    if has_jet_icon {
        if let Some(textures) = hud_textures {
            if let Some(jet_tex) = &textures.jet_icon {
                draw_texture_ex(
                    jet_tex,
                    jet_x,
                    15.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    },
                );
            }
        }
    } else {
        draw_text("E:", jet_x, 35.0, 24.0, Color::new(0.5, 0.9, 1.0, 1.0));
    }

    let charges_start_x = if has_jet_icon { jet_x + icon_size + 10.0 } else { jet_x + 30.0 };

    for i in 0..config.jet_max_charges {
        let x = charges_start_x + i as f32 * 20.0;
        let has_charge = i < player.jet_charges;
        let color = if has_charge {
            Color::new(0.3, 0.8, 1.0, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.4, 0.5)
        };

        // Simple droplet shape (procedural)
        draw_circle(x, 28.0, 6.0, color);
        draw_triangle(
            vec2(x - 4.0, 28.0),
            vec2(x + 4.0, 28.0),
            vec2(x, 18.0),
            color,
        );
    }
}

/// Draw ink charge indicators
fn draw_ink_charges(player: &Player, config: &GameConfig, hud_textures: Option<&HudTextures>) {
    let ink_x = screen_width() - 150.0;
    let icon_size = 20.0;

    // Check if we have ink icon texture
    let has_ink_icon = hud_textures
        .and_then(|t| t.ink_icon.as_ref())
        .is_some();

    // Draw label or icon
    if has_ink_icon {
        if let Some(textures) = hud_textures {
            if let Some(ink_tex) = &textures.ink_icon {
                draw_texture_ex(
                    ink_tex,
                    ink_x,
                    45.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    },
                );
            }
        }
    } else {
        draw_text("Q:", ink_x, 65.0, 24.0, Color::new(0.5, 0.3, 0.6, 1.0));
    }

    let charges_start_x = if has_ink_icon { ink_x + icon_size + 10.0 } else { ink_x + 30.0 };

    for i in 0..config.ink_max_charges {
        let x = charges_start_x + i as f32 * 20.0;
        let has_charge = i < player.ink_charges;
        let color = if has_charge {
            Color::new(0.4, 0.2, 0.5, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.3, 0.5)
        };
        draw_circle(x, 58.0, 7.0, color);
    }
}

/// Draw debug info (only available in debug builds)
#[cfg(debug_assertions)]
pub fn draw_debug(player_state: &str, velocity: Vec2, fps: i32) {
    draw_text(&format!("FPS: {}", fps), 10.0, 70.0, 20.0, YELLOW);
    draw_text(&format!("State: {}", player_state), 10.0, 90.0, 20.0, YELLOW);
    draw_text(
        &format!("Vel: ({:.0}, {:.0})", velocity.x, velocity.y),
        10.0,
        110.0,
        20.0,
        YELLOW,
    );
}

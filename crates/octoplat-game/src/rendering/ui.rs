//! HUD rendering
//!
//! Draws the game HUD including gem counter, lives, and ability charges.
//! Supports optional FLUX-generated textures with procedural fallback.
//!
//! Layout:
//! - Top-left: Lives (octopus icons) - clean and visible
//! - Bottom-right: Gem counter, stamina, abilities - grouped together
//! - Bottom-left: Reserved for minimap (drawn separately)
//! - Bottom-center: Control hints

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::player::Player;
use crate::rendering::ui_textures::HudTextures;

/// Draw the HUD (gem counter, lives, HP, and ability charges)
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
    let sh = screen_height();
    let sw = screen_width();

    // === TOP-LEFT: Lives display and HP bar (clean, prominent) ===
    draw_lives_display(lives, hud_textures);

    // Draw HP bar below lives (only if max_hp > 1)
    if player.max_hp > 1 {
        draw_hp_bar(player, 20.0, 50.0);
    }

    // === BOTTOM-RIGHT: Gem counter, stamina, abilities ===
    let bottom_panel_x = sw - 200.0;
    let bottom_panel_y = sh - 100.0;

    // Gem counter
    draw_gem_counter(gems_collected, total_gems, bottom_panel_x, bottom_panel_y, hud_textures);

    // Stamina bar (for wall grip and grapple swing)
    draw_stamina_bar(player, config, bottom_panel_x, bottom_panel_y + 35.0, hud_textures);

    // Ability charges (jet and ink side by side)
    draw_jet_charges(player, config, bottom_panel_x, bottom_panel_y + 55.0, hud_textures);
    draw_ink_charges(player, config, bottom_panel_x + 90.0, bottom_panel_y + 55.0, hud_textures);

    // Wall jump charges (small, below abilities)
    draw_wall_jump_charges(player, config, bottom_panel_x, bottom_panel_y + 80.0);

    // Mode indicator in endless mode
    if is_endless {
        draw_text(
            "ENDLESS",
            sw - 80.0,
            sh - 15.0,
            14.0,
            Color::new(1.0, 0.8, 0.3, 0.7),
        );
    }

    // Controls hint (small text at bottom-center, avoid minimap area on left)
    let hint_text = "WASD:Move Space:Jump C:Slide S:Dive E:Jet Q:Ink F:Grapple";
    let hint_width = measure_text(hint_text, None, 13, 1.0).width;
    draw_text(
        hint_text,
        (sw - hint_width) / 2.0,
        sh - 8.0,
        13.0,
        Color::new(1.0, 1.0, 1.0, 0.4),
    );
}

/// Draw the lives display with octopus icons
fn draw_lives_display(lives: u32, hud_textures: Option<&HudTextures>) {
    let start_x = 20.0;
    let y = 25.0;
    let icon_size = 24.0;
    let icon_spacing = 28.0;

    if lives == u32::MAX {
        // Infinite lives
        draw_text("INF", start_x, y + 5.0, 20.0, Color::new(0.8, 0.8, 0.8, 0.8));
        return;
    }

    // Check if we have life icon texture
    let has_life_texture = hud_textures
        .and_then(|t| t.life_icon.as_ref())
        .is_some();

    // Draw octopus icons (max 5 displayed as icons)
    let icons_to_draw = lives.min(5);
    for i in 0..icons_to_draw {
        let x = start_x + i as f32 * icon_spacing;

        if let Some(textures) = hud_textures {
            if let Some(life_tex) = &textures.life_icon {
                // Draw textured octopus
                draw_texture_ex(
                    life_tex,
                    x,
                    y - icon_size / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    },
                );
                continue;
            }
        }

        // Fallback to procedural octopus
        let octo_color = Color::new(0.85, 0.4, 0.6, 1.0); // Pink/magenta octopus
        draw_octopus(x, y, if has_life_texture { icon_size } else { 14.0 }, octo_color);
    }

    // If more than 5 lives, show "x N" after the icons
    if lives > 5 {
        let text_x = start_x + 5.0 * icon_spacing + 5.0;
        draw_text(
            &format!("x{}", lives),
            text_x,
            y + 5.0,
            18.0,
            Color::new(0.85, 0.4, 0.6, 1.0),
        );
    }
}

/// Draw a simple octopus shape (procedural fallback)
fn draw_octopus(x: f32, y: f32, size: f32, color: Color) {
    let half = size / 2.0;
    let center_x = x + half;
    let center_y = y;

    // Head (oval/circle)
    let head_radius = size * 0.35;
    draw_circle(center_x, center_y, head_radius, color);

    // Eyes (small white circles with dark pupils)
    let eye_offset = head_radius * 0.4;
    let eye_radius = head_radius * 0.25;
    draw_circle(center_x - eye_offset, center_y - head_radius * 0.1, eye_radius, WHITE);
    draw_circle(center_x + eye_offset, center_y - head_radius * 0.1, eye_radius, WHITE);
    draw_circle(center_x - eye_offset, center_y - head_radius * 0.1, eye_radius * 0.5, Color::new(0.1, 0.1, 0.1, 1.0));
    draw_circle(center_x + eye_offset, center_y - head_radius * 0.1, eye_radius * 0.5, Color::new(0.1, 0.1, 0.1, 1.0));

    // Tentacles (wavy lines below the head)
    let tentacle_start_y = center_y + head_radius * 0.5;
    let tentacle_length = size * 0.4;
    let tentacle_width = size * 0.08;

    for i in 0..4 {
        let offset = (i as f32 - 1.5) * (size * 0.18);
        let tx = center_x + offset;
        // Draw tentacle as small overlapping circles
        for j in 0..4 {
            let ty = tentacle_start_y + j as f32 * (tentacle_length / 4.0);
            let wave = (j as f32 * 0.8).sin() * tentacle_width;
            draw_circle(tx + wave, ty, tentacle_width, color);
        }
    }
}

/// Draw the HP bar with colored segments
fn draw_hp_bar(player: &Player, x: f32, y: f32) {
    let bar_width = 120.0;
    let bar_height = 12.0;
    let segment_gap = 2.0;

    // Calculate segment width based on max HP
    let max_hp = player.max_hp as f32;
    let segment_width = (bar_width - (max_hp - 1.0) * segment_gap) / max_hp;

    // Background
    draw_rectangle(x - 2.0, y - 2.0, bar_width + 4.0, bar_height + 4.0, Color::new(0.1, 0.1, 0.15, 0.8));

    // HP fraction for color
    let hp_fraction = player.hp_fraction();

    // Color based on HP level: green (>60%), yellow (30-60%), red (<30%)
    let fill_color = if hp_fraction > 0.6 {
        Color::new(0.3, 0.85, 0.4, 1.0)  // Green
    } else if hp_fraction > 0.3 {
        Color::new(0.95, 0.8, 0.2, 1.0)  // Yellow
    } else {
        Color::new(0.95, 0.3, 0.2, 1.0)  // Red
    };

    // Draw HP segments
    for i in 0..player.max_hp {
        let segment_x = x + i as f32 * (segment_width + segment_gap);

        if i < player.current_hp {
            // Filled segment
            draw_rectangle(segment_x, y, segment_width, bar_height, fill_color);
        } else {
            // Empty segment
            draw_rectangle(segment_x, y, segment_width, bar_height, Color::new(0.2, 0.2, 0.25, 0.6));
        }

        // Segment border
        draw_rectangle_lines(segment_x, y, segment_width, bar_height, 1.0, Color::new(0.3, 0.3, 0.35, 0.8));
    }

    // HP text label
    draw_text("HP", x, y - 3.0, 11.0, Color::new(0.7, 0.7, 0.8, 0.7));
}

/// Draw the gem counter with icon and text
fn draw_gem_counter(gems_collected: u32, total_gems: u32, x: f32, y: f32, hud_textures: Option<&HudTextures>) {
    let icon_size = 22.0;

    // Try to draw textured gem icon
    let mut drew_texture = false;
    if let Some(textures) = hud_textures {
        if let Some(gem_tex) = &textures.gem {
            draw_texture_ex(
                gem_tex,
                x,
                y,
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
        draw_poly(x + 10.0, y + 10.0, 4, 9.0, 45.0, Color::new(0.3, 0.8, 1.0, 1.0));
    }

    // Text counter
    let text = format!("{} / {}", gems_collected, total_gems);
    let text_x = x + icon_size + 6.0;
    draw_text(&text, text_x, y + 18.0, 22.0, WHITE);
}

/// Draw the stamina bar
fn draw_stamina_bar(player: &Player, config: &GameConfig, x: f32, y: f32, hud_textures: Option<&HudTextures>) {
    let bar_width = 170.0;
    let bar_height = 10.0;
    let stamina_ratio = (player.wall_stamina / config.wall_stamina_max).clamp(0.0, 1.0);

    // Check if we have a stamina frame texture
    if let Some(textures) = hud_textures {
        if let Some(frame_tex) = &textures.stamina_frame {
            // Draw frame texture (assumes frame is larger than the bar fill area)
            let frame_width = bar_width + 8.0;
            let frame_height = bar_height + 8.0;
            draw_texture_ex(
                frame_tex,
                x - 4.0,
                y - 4.0,
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
        draw_rectangle(x, y, bar_width, bar_height, Color::new(0.2, 0.2, 0.3, 0.8));
    }

    // Stamina fill - color changes from green to yellow to red
    let fill_color = if stamina_ratio > 0.5 {
        Color::new(0.3, 0.8, 0.4, 1.0) // Green
    } else if stamina_ratio > 0.25 {
        Color::new(0.9, 0.8, 0.2, 1.0) // Yellow
    } else {
        Color::new(0.9, 0.3, 0.2, 1.0) // Red
    };
    draw_rectangle(x, y, bar_width * stamina_ratio, bar_height, fill_color);

    // Border (only if no frame texture)
    if hud_textures.and_then(|t| t.stamina_frame.as_ref()).is_none() {
        draw_rectangle_lines(x, y, bar_width, bar_height, 1.0, Color::new(0.5, 0.5, 0.6, 0.8));
    }

    // Label
    draw_text("Stamina", x, y - 2.0, 11.0, Color::new(0.7, 0.8, 0.9, 0.7));
}

/// Draw wall jump charge indicators (small wall icons)
fn draw_wall_jump_charges(player: &Player, config: &GameConfig, x: f32, y: f32) {
    // Label
    draw_text("Wall:", x, y + 7.0, 11.0, Color::new(0.7, 0.7, 0.8, 0.6));

    for i in 0..config.wall_jumps_max {
        let charge_x = x + 30.0 + i as f32 * 14.0;
        let has_charge = i < player.wall_jumps_remaining;

        let color = if has_charge {
            Color::new(0.9, 0.6, 0.3, 1.0) // Orange when available
        } else {
            Color::new(0.3, 0.3, 0.3, 0.5) // Gray when used
        };

        // Draw small wall/brick icon
        draw_rectangle(charge_x, y, 10.0, 8.0, color);
        draw_rectangle_lines(charge_x, y, 10.0, 8.0, 1.0, Color::new(0.2, 0.2, 0.2, 0.8));
    }
}

/// Draw jet charge indicators
fn draw_jet_charges(player: &Player, config: &GameConfig, x: f32, y: f32, hud_textures: Option<&HudTextures>) {
    let icon_size = 18.0;

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
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    },
                );
            }
        }
    } else {
        draw_text("Jet", x, y + 14.0, 12.0, Color::new(0.5, 0.9, 1.0, 0.8));
    }

    let charges_start_x = x + icon_size + 5.0;

    for i in 0..config.jet_max_charges {
        let charge_x = charges_start_x + i as f32 * 16.0;
        let has_charge = i < player.jet_charges;
        let color = if has_charge {
            Color::new(0.3, 0.8, 1.0, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.4, 0.5)
        };

        // Simple droplet shape (procedural)
        let center_y = y + 9.0;
        draw_circle(charge_x, center_y, 5.0, color);
        draw_triangle(
            vec2(charge_x - 3.5, center_y),
            vec2(charge_x + 3.5, center_y),
            vec2(charge_x, center_y - 8.0),
            color,
        );
    }
}

/// Draw ink charge indicators
fn draw_ink_charges(player: &Player, config: &GameConfig, x: f32, y: f32, hud_textures: Option<&HudTextures>) {
    let icon_size = 18.0;

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
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    },
                );
            }
        }
    } else {
        draw_text("Ink", x, y + 14.0, 12.0, Color::new(0.5, 0.3, 0.6, 0.8));
    }

    let charges_start_x = x + icon_size + 5.0;

    for i in 0..config.ink_max_charges {
        let charge_x = charges_start_x + i as f32 * 16.0;
        let has_charge = i < player.ink_charges;
        let color = if has_charge {
            Color::new(0.4, 0.2, 0.5, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.3, 0.5)
        };
        draw_circle(charge_x, y + 9.0, 5.0, color);
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

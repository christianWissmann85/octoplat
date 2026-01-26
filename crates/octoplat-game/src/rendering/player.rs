//! Player rendering
//!
//! Drawing functions for the octopus player character.

use macroquad::prelude::*;

use crate::config::GameConfig;
use crate::player::{Player, PlayerState};

/// Calculate a point on a cubic bezier curve
fn bezier_point(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    p0 * mt3 + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * t3
}

/// Draw a bezier curve tentacle with tapering thickness
#[allow(clippy::too_many_arguments)]
fn draw_bezier_tentacle(
    start: Vec2,
    c1: Vec2,
    c2: Vec2,
    end: Vec2,
    base_thickness: f32,
    color: Color,
    segments: usize,
    draw_suckers: bool,
) {
    let mut prev_point = start;

    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let point = bezier_point(start, c1, c2, end, t);

        // Tapering thickness (thicker at base, thinner at tip)
        let thickness = base_thickness * (1.0 - t * 0.7);

        draw_line(prev_point.x, prev_point.y, point.x, point.y, thickness, color);

        // Draw sucker circles along the tentacle
        if draw_suckers && i % 2 == 0 && i < segments - 1 {
            let sucker_size = thickness * 0.6;
            let sucker_color = Color::new(
                color.r * 0.7,
                color.g * 0.7,
                color.b * 0.7,
                color.a * 0.8,
            );
            draw_circle(point.x, point.y, sucker_size, sucker_color);
        }

        prev_point = point;
    }
}

/// Draw an ellipse (approximated with polygon)
fn draw_ellipse_shape(x: f32, y: f32, radius_x: f32, radius_y: f32, color: Color) {
    const SEGMENTS: usize = 16;
    let mut vertices = Vec::with_capacity(SEGMENTS);

    for i in 0..SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        vertices.push(vec2(
            x + angle.cos() * radius_x,
            y + angle.sin() * radius_y,
        ));
    }

    // Draw as triangle fan from center
    for i in 0..SEGMENTS {
        let next = (i + 1) % SEGMENTS;
        draw_triangle(
            vec2(x, y),
            vertices[i],
            vertices[next],
            color,
        );
    }
}

/// Draw the player as an octopus shape
pub fn draw_player(player: &Player, config: &GameConfig, time: f32) {
    let position = player.position;
    let facing_right = player.facing_right;

    // Invincibility blink - skip rendering every other frame
    if player.invincibility_timer > 0.0 {
        let blink_rate = 15.0;
        if (time * blink_rate) as i32 % 2 == 0 {
            return; // Skip this frame for blink effect
        }
    }

    // Base color depends on state
    let body_color = match player.state {
        PlayerState::JetBoosting if player.is_jet_downward() => Color::new(0.3, 0.5, 0.9, 1.0), // Blue downward jet
        PlayerState::JetBoosting => Color::new(0.2, 0.9, 0.9, 1.0), // Cyan jet
        _ => Color::new(0.8, 0.3, 0.6, 1.0),                        // Purple octopus
    };

    // Stamina affects color (purple -> blue -> cyan as stamina drains)
    let stamina_ratio = (player.wall_stamina / config.wall_stamina_max).clamp(0.0, 1.0);
    let mut final_color = Color::new(
        body_color.r * stamina_ratio + 0.3 * (1.0 - stamina_ratio),
        body_color.g * stamina_ratio + 0.5 * (1.0 - stamina_ratio),
        body_color.b * stamina_ratio + 0.8 * (1.0 - stamina_ratio),
        1.0,
    );

    // Hit flash - override color to white
    if player.hit_flash_timer > 0.0 {
        let flash_intensity = player.hit_flash_timer / 0.12;
        final_color = Color::new(
            final_color.r + (1.0 - final_color.r) * flash_intensity,
            final_color.g + (1.0 - final_color.g) * flash_intensity,
            final_color.b + (1.0 - final_color.b) * flash_intensity,
            1.0,
        );
    }

    // Ink cloud effect - semi-transparent dark purple
    if player.is_inked {
        let ink_alpha = (player.ink_timer / config.ink_duration).min(1.0);
        let ink_size = 30.0 + (time * 5.0).sin() * 5.0;
        draw_circle(
            position.x,
            position.y,
            ink_size,
            Color::new(0.2, 0.1, 0.3, ink_alpha * 0.6),
        );
        final_color = Color::new(0.4, 0.2, 0.5, 1.0);
    }

    // Body shape depends on state
    let (base_body_radius, body_y_offset) = match player.state {
        PlayerState::JetBoosting if player.is_jet_downward() => (12.0, 0.0),
        _ => (14.0, 0.0),
    };

    // Apply squash/stretch scaling with breathing wobble
    let breathing_wobble = if player.state == PlayerState::Idle {
        player.breathing_phase.sin() * 0.05
    } else {
        0.0
    };
    let scale_y = player.visual_scale_y + breathing_wobble;
    let scale_x = 1.0 / scale_y.sqrt(); // Inverse to preserve volume
    let body_radius_x = base_body_radius * scale_x;
    let body_radius_y = base_body_radius * scale_y;
    let body_radius = base_body_radius; // For compatibility with other calculations

    // Draw jet trail if boosting
    if player.state == PlayerState::JetBoosting {
        let trail_dir = -player.jet_direction;
        for i in 0..5 {
            let trail_pos = position + trail_dir * (i as f32 * 8.0 + 10.0);
            let trail_size = 6.0 - i as f32;
            let alpha = 0.6 - i as f32 * 0.1;
            draw_circle(
                trail_pos.x,
                trail_pos.y,
                trail_size,
                Color::new(0.5, 0.9, 1.0, alpha),
            );
        }
    }

    // Draw dive trail for downward jet
    if player.is_jet_downward() {
        for i in 0..4 {
            let trail_y = position.y - (i as f32 * 10.0 + 15.0);
            let alpha = 0.4 - i as f32 * 0.1;
            draw_circle(
                position.x,
                trail_y,
                5.0 - i as f32,
                Color::new(0.3, 0.5, 0.9, alpha),
            );
        }
    }

    // Body (ellipse for squash/stretch effect)
    draw_ellipse_shape(
        position.x,
        position.y + body_y_offset,
        body_radius_x,
        body_radius_y,
        final_color,
    );

    // Eyes
    let eye_offset_x = if facing_right { 4.0 } else { -4.0 };
    let eye_y = position.y + body_y_offset - 3.0;
    draw_circle(position.x + eye_offset_x - 3.0, eye_y, 3.0, WHITE);
    draw_circle(position.x + eye_offset_x + 3.0, eye_y, 3.0, WHITE);

    // Pupils - look in movement direction
    let pupil_shift = if facing_right { 1.0 } else { -1.0 };
    let pupil_y_shift = if player.is_jet_downward() {
        1.5
    } else {
        0.0
    };
    draw_circle(
        position.x + eye_offset_x - 3.0 + pupil_shift,
        eye_y + pupil_y_shift,
        1.5,
        BLACK,
    );
    draw_circle(
        position.x + eye_offset_x + 3.0 + pupil_shift,
        eye_y + pupil_y_shift,
        1.5,
        BLACK,
    );

    // Tentacles
    let tentacle_y = position.y + body_y_offset + body_radius;
    let tentacle_color = Color::new(
        final_color.r * 0.8,
        final_color.g * 0.8,
        final_color.b * 0.8,
        1.0,
    );

    draw_tentacles(player, position, tentacle_y, facing_right, tentacle_color, time);
}

/// Draw tentacles based on player state using bezier curves
fn draw_tentacles(
    player: &Player,
    position: Vec2,
    tentacle_y: f32,
    _facing_right: bool,
    tentacle_color: Color,
    time: f32,
) {
    match player.state {
        PlayerState::JetBoosting => {
            // Tentacles point opposite to jet direction
            let tent_dir = -player.jet_direction;
            let perp = vec2(-tent_dir.y, tent_dir.x);

            for i in 0..4 {
                let offset = (i as f32 - 1.5) * 5.0;
                let wave = (time * 10.0 + i as f32 * 0.9).sin() * 2.0;

                let start = position + perp * offset;
                let end = position + tent_dir * 22.0 + perp * (offset + wave);
                let c1 = position + tent_dir * 8.0 + perp * offset;
                let c2 = position + tent_dir * 16.0 + perp * (offset + wave * 0.5);

                draw_bezier_tentacle(start, c1, c2, end, 3.0, tentacle_color, 6, false);
            }
        }
        PlayerState::Swinging => {
            // Tentacles follow grapple direction
            if let Some(grapple) = player.grapple_point {
                let to_grapple = (grapple - position).normalize_or_zero();
                let perp = vec2(-to_grapple.y, to_grapple.x);

                for i in 0..4 {
                    let offset = (i as f32 - 1.5) * 5.0;
                    let wave = (time * 4.0 + i as f32 * 0.7).sin() * 4.0;

                    let start = position + perp * offset;
                    let end = position + to_grapple * 15.0 + perp * (offset * 0.5 + wave);
                    let c1 = position + to_grapple * 5.0 + perp * offset;
                    let c2 = position + to_grapple * 10.0 + perp * (offset * 0.7 + wave * 0.5);

                    draw_bezier_tentacle(start, c1, c2, end, 3.5, tentacle_color, 6, true);
                }
            } else {
                // Fallback to idle animation
                draw_idle_tentacles(position, tentacle_y, tentacle_color, time);
            }
        }
        _ => {
            // Idle/Running - flowing wave animation
            draw_idle_tentacles(position, tentacle_y, tentacle_color, time);
        }
    }
}

/// Draw idle tentacles with flowing wave animation
fn draw_idle_tentacles(position: Vec2, tentacle_y: f32, tentacle_color: Color, time: f32) {
    for i in 0..4 {
        let x_offset = (i as f32 - 1.5) * 6.0;
        // Flowing wave animation with phase offset per tentacle
        let wave_x = (time * 2.0 + i as f32 * 0.8).sin() * 3.0;
        let wave_y = (time * 2.5 + i as f32 * 0.6).cos() * 2.0;

        let start = vec2(position.x + x_offset, tentacle_y);
        let end = vec2(position.x + x_offset + wave_x, tentacle_y + 16.0 + wave_y);
        let c1 = vec2(position.x + x_offset + wave_x * 0.3, tentacle_y + 5.0);
        let c2 = vec2(position.x + x_offset + wave_x * 0.7, tentacle_y + 11.0 + wave_y * 0.5);

        draw_bezier_tentacle(start, c1, c2, end, 4.0, tentacle_color, 8, true);
    }
}

/// Draw the tentacle line when swinging
pub fn draw_tentacle(player: &Player) {
    if let Some(grapple) = player.grapple_point {
        let segments = 8;
        let start = player.position;
        let end = grapple;

        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            // Add some waviness
            let wave1 = (t1 * std::f32::consts::TAU).sin() * 3.0 * (1.0 - t1);
            let wave2 = (t2 * std::f32::consts::TAU).sin() * 3.0 * (1.0 - t2);

            let p1 = start.lerp(end, t1);
            let p2 = start.lerp(end, t2);

            // Perpendicular offset for wave
            let dir = (end - start).normalize();
            let perp = vec2(-dir.y, dir.x);

            let p1_wave = p1 + perp * wave1;
            let p2_wave = p2 + perp * wave2;

            // Thicker at player end, thinner at grapple
            let thickness = 4.0 * (1.0 - t1 * 0.5);

            draw_line(
                p1_wave.x,
                p1_wave.y,
                p2_wave.x,
                p2_wave.y,
                thickness,
                Color::new(0.7, 0.3, 0.5, 0.9),
            );
        }

        // Draw a small circle at the attachment point
        draw_circle(grapple.x, grapple.y, 4.0, Color::new(0.9, 0.4, 0.6, 1.0));
    }
}

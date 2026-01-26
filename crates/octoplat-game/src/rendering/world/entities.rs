//! Entity rendering (enemies and platforms)

use macroquad::prelude::*;

use crate::hazards::{Crab, Pufferfish, PufferfishPattern};
use crate::platforms::{CrumblingPlatform, CrumblingState, MovingPlatform};

/// Draw a crab enemy with two-segment animated legs
pub fn draw_crab(crab: &Crab, time: f32) {
    if !crab.alive {
        return;
    }

    let pos = crab.position;

    // Body (oval shape)
    let body_color = Color::new(0.9, 0.4, 0.2, 1.0);
    draw_ellipse(pos.x, pos.y, 14.0, 10.0, 0.0, body_color);

    // Shell pattern details
    draw_ellipse(pos.x - 3.0, pos.y - 3.0, 6.0, 4.0, 0.0, Color::new(1.0, 0.6, 0.4, 0.6));
    draw_ellipse(pos.x + 4.0, pos.y, 4.0, 3.0, 0.0, Color::new(1.0, 0.5, 0.3, 0.4));

    // Eyes (on stalks) with animation
    let eye_color = Color::new(0.1, 0.1, 0.1, 1.0);
    let eye_white = Color::new(1.0, 1.0, 1.0, 1.0);
    let eye_offset_x = if crab.facing_right { 6.0 } else { -6.0 };
    let eye_bob = (time * 4.0).sin() * 1.0;

    // Eye stalks
    let stalk_color = Color::new(0.8, 0.35, 0.15, 1.0);
    draw_line(pos.x - 6.0, pos.y - 6.0, pos.x - 6.0, pos.y - 10.0 + eye_bob, 3.0, stalk_color);
    draw_line(pos.x + 6.0, pos.y - 6.0, pos.x + 6.0, pos.y - 10.0 - eye_bob * 0.5, 3.0, stalk_color);

    // Left eye
    draw_circle(pos.x - 6.0, pos.y - 11.0 + eye_bob, 4.0, eye_white);
    draw_circle(pos.x - 6.0 + eye_offset_x * 0.2, pos.y - 11.0 + eye_bob, 2.0, eye_color);

    // Right eye
    draw_circle(pos.x + 6.0, pos.y - 11.0 - eye_bob * 0.5, 4.0, eye_white);
    draw_circle(pos.x + 6.0 + eye_offset_x * 0.2, pos.y - 11.0 - eye_bob * 0.5, 2.0, eye_color);

    // Claws with snapping animation
    let claw_color = Color::new(0.8, 0.3, 0.1, 1.0);
    let claw_inner = Color::new(0.95, 0.5, 0.2, 1.0);
    let claw_wave = (time * 4.0).sin() * 3.0;

    // Claw snapping effect - periodic snapping
    let snap_cycle = (time * 2.0) % std::f32::consts::TAU;
    let snap_left = if snap_cycle < 0.3 { snap_cycle * 10.0 } else { 0.0 };
    let snap_right = if snap_cycle > std::f32::consts::PI && snap_cycle < std::f32::consts::PI + 0.3 {
        (snap_cycle - std::f32::consts::PI) * 10.0
    } else {
        0.0
    };

    // Left claw (two parts - upper and lower)
    let left_claw_pos = vec2(pos.x - 18.0, pos.y + claw_wave);
    // Upper claw
    draw_ellipse(left_claw_pos.x, left_claw_pos.y - 2.0 - snap_left, 7.0, 4.0, 0.0, claw_color);
    // Lower claw
    draw_ellipse(left_claw_pos.x, left_claw_pos.y + 2.0 + snap_left * 0.5, 6.0, 3.0, 0.0, claw_inner);

    // Right claw (two parts)
    let right_claw_pos = vec2(pos.x + 18.0, pos.y - claw_wave);
    // Upper claw
    draw_ellipse(right_claw_pos.x, right_claw_pos.y - 2.0 - snap_right, 7.0, 4.0, 0.0, claw_color);
    // Lower claw
    draw_ellipse(right_claw_pos.x, right_claw_pos.y + 2.0 + snap_right * 0.5, 6.0, 3.0, 0.0, claw_inner);

    // Two-segment legs with animated joints
    let leg_color = Color::new(0.7, 0.3, 0.1, 1.0);
    let joint_color = Color::new(0.85, 0.4, 0.15, 1.0);
    for i in 0..3 {
        let leg_y = pos.y + 2.0 + (i as f32) * 3.0;
        // Phase-shifted walking motion per leg
        let leg_phase = (time * 8.0 + i as f32 * 1.2).sin();
        let leg_phase2 = (time * 8.0 + i as f32 * 1.2 + 0.5).sin();

        // Left leg - segment 1 (from body to joint)
        let l_joint_x = pos.x - 14.0 + leg_phase * 1.5;
        let l_joint_y = leg_y + 3.0 + leg_phase.abs() * 2.0;
        draw_line(pos.x - 10.0, leg_y, l_joint_x, l_joint_y, 2.5, leg_color);
        // Joint circle
        draw_circle(l_joint_x, l_joint_y, 1.5, joint_color);
        // Left leg - segment 2 (from joint to ground)
        let l_foot_x = pos.x - 20.0 + leg_phase2 * 2.0;
        let l_foot_y = leg_y + 8.0 + leg_phase2.abs() * 1.5;
        draw_line(l_joint_x, l_joint_y, l_foot_x, l_foot_y, 2.0, leg_color);

        // Right leg - segment 1
        let r_joint_x = pos.x + 14.0 - leg_phase * 1.5;
        let r_joint_y = leg_y + 3.0 + (-leg_phase).abs() * 2.0;
        draw_line(pos.x + 10.0, leg_y, r_joint_x, r_joint_y, 2.5, leg_color);
        // Joint circle
        draw_circle(r_joint_x, r_joint_y, 1.5, joint_color);
        // Right leg - segment 2
        let r_foot_x = pos.x + 20.0 - leg_phase2 * 2.0;
        let r_foot_y = leg_y + 8.0 + (-leg_phase2).abs() * 1.5;
        draw_line(r_joint_x, r_joint_y, r_foot_x, r_foot_y, 2.0, leg_color);
    }
}

/// Draw a pufferfish enemy with enhanced spike animation
pub fn draw_pufferfish(puffer: &Pufferfish, time: f32) {
    if !puffer.alive {
        return;
    }

    let pos = puffer.position;

    // Pulsing size
    let pulse = (time * 3.0).sin() * 0.1 + 1.0;
    let base_size = 14.0 * pulse;

    // Body (spiky circle)
    let body_color = Color::new(0.9, 0.8, 0.3, 1.0);
    draw_circle(pos.x, pos.y, base_size, body_color);

    // Body highlight
    draw_circle(pos.x - 3.0, pos.y - 3.0, base_size * 0.5, Color::new(1.0, 0.95, 0.6, 0.4));

    // Enhanced spikes as triangles with individual animation
    let spike_color = Color::new(0.7, 0.5, 0.2, 1.0);
    let spike_highlight = Color::new(0.9, 0.7, 0.3, 1.0);
    let num_spikes = 12;
    for i in 0..num_spikes {
        // Individual spike rotation
        let base_angle = (i as f32) * std::f32::consts::TAU / num_spikes as f32;
        let angle = base_angle + time * 0.5;

        // Per-spike breathing pulse
        let spike_pulse = (time * 4.0 + i as f32 * 0.5).sin() * 0.3 + 1.0;
        let spike_length = 8.0 * spike_pulse;

        // Triangle spike instead of line
        let dir = vec2(angle.cos(), angle.sin());
        let perp = vec2(-dir.y, dir.x);

        let spike_base = pos + dir * base_size;
        let spike_tip = pos + dir * (base_size + spike_length);
        let spike_width = 3.0 * spike_pulse;

        draw_triangle(
            vec2(spike_base.x + perp.x * spike_width, spike_base.y + perp.y * spike_width),
            vec2(spike_base.x - perp.x * spike_width, spike_base.y - perp.y * spike_width),
            spike_tip,
            spike_color,
        );

        // Spike highlight
        draw_triangle(
            vec2(spike_base.x + perp.x * spike_width * 0.5, spike_base.y + perp.y * spike_width * 0.5),
            spike_base,
            vec2(spike_tip.x - dir.x * 2.0, spike_tip.y - dir.y * 2.0),
            spike_highlight,
        );
    }

    // Face
    let face_offset = match puffer.pattern {
        PufferfishPattern::Horizontal => {
            if (time * 2.0).sin() > 0.0 { 2.0 } else { -2.0 }
        }
        _ => 0.0,
    };

    // Eyes (expressive based on pulsing)
    let eye_color = Color::new(0.1, 0.1, 0.1, 1.0);
    let eye_size = 3.0 + (time * 2.0).sin() * 0.3;
    draw_circle(pos.x - 5.0 + face_offset, pos.y - 3.0, eye_size, WHITE);
    draw_circle(pos.x + 5.0 + face_offset, pos.y - 3.0, eye_size, WHITE);
    draw_circle(pos.x - 5.0 + face_offset, pos.y - 3.0, 1.5, eye_color);
    draw_circle(pos.x + 5.0 + face_offset, pos.y - 3.0, 1.5, eye_color);

    // Mouth (surprised O) - pulsing
    let mouth_size = 4.0 + (time * 3.0).sin() * 0.5;
    draw_circle(pos.x + face_offset, pos.y + 4.0, mouth_size, Color::new(0.6, 0.3, 0.2, 1.0));
    draw_circle(pos.x + face_offset, pos.y + 4.0, mouth_size * 0.5, Color::new(0.3, 0.1, 0.1, 1.0));

    // Fins (small triangles on sides)
    let fin_color = Color::new(0.8, 0.6, 0.2, 0.8);
    let fin_wave = (time * 6.0).sin() * 5.0;

    // Left fin
    draw_triangle(
        vec2(pos.x - base_size, pos.y),
        vec2(pos.x - base_size - 8.0, pos.y - 4.0 + fin_wave),
        vec2(pos.x - base_size - 8.0, pos.y + 4.0 + fin_wave),
        fin_color,
    );

    // Right fin
    draw_triangle(
        vec2(pos.x + base_size, pos.y),
        vec2(pos.x + base_size + 8.0, pos.y - 4.0 - fin_wave),
        vec2(pos.x + base_size + 8.0, pos.y + 4.0 - fin_wave),
        fin_color,
    );
}

/// Draw a moving platform
pub fn draw_moving_platform(platform: &MovingPlatform, _time: f32) {
    let rect = platform.collision_rect();

    // Platform body
    let platform_color = Color::new(0.4, 0.5, 0.7, 1.0);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, platform_color);

    // Top surface highlight
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        4.0,
        Color::new(0.6, 0.7, 0.9, 1.0),
    );

    // Bottom shadow
    draw_rectangle(
        rect.x,
        rect.y + rect.h - 3.0,
        rect.w,
        3.0,
        Color::new(0.2, 0.3, 0.5, 1.0),
    );

    // Mechanical details (gears/rivets)
    let center_x = rect.x + rect.w / 2.0;
    let center_y = rect.y + rect.h / 2.0;
    draw_circle(center_x, center_y, 4.0, Color::new(0.3, 0.4, 0.5, 1.0));
    draw_circle(center_x, center_y, 2.0, Color::new(0.5, 0.6, 0.7, 1.0));
}

/// Draw a crumbling platform with pulsing crack animation
pub fn draw_crumbling_platform(platform: &CrumblingPlatform, time: f32) {
    // Don't draw if respawning
    if platform.state == CrumblingState::Respawning {
        return;
    }

    let shake = platform.shake_offset();
    let rect = platform.collision_rect();
    let pos = vec2(rect.x + shake.x, rect.y + shake.y);

    // Platform color based on state
    let (base_color, crack_color, danger_intensity) = match platform.state {
        CrumblingState::Stable => (
            Color::new(0.6, 0.5, 0.4, 1.0),
            Color::new(0.4, 0.35, 0.3, 0.5),
            0.0,
        ),
        CrumblingState::Shaking => (
            Color::new(0.7, 0.4, 0.3, 1.0),
            Color::new(0.5, 0.3, 0.2, 0.8),
            1.0,
        ),
        CrumblingState::Falling => (
            Color::new(0.5, 0.4, 0.3, 0.7),
            Color::new(0.3, 0.25, 0.2, 0.5),
            0.5,
        ),
        CrumblingState::Respawning => return,
    };

    // Danger glow when shaking (about to break)
    if danger_intensity > 0.0 {
        let pulse = (time * 6.0).sin() * 0.5 + 0.5;
        let glow_alpha = 0.3 * danger_intensity * pulse;
        // Red danger glow - outer
        draw_rectangle(
            pos.x - 4.0,
            pos.y - 4.0,
            rect.w + 8.0,
            rect.h + 8.0,
            Color::new(1.0, 0.3, 0.2, glow_alpha),
        );
    }

    // Main body
    draw_rectangle(pos.x, pos.y, rect.w, rect.h, base_color);

    // Pulsing crack width based on state
    let pulse = (time * 4.0).sin() * 0.5 + 1.5;
    let crack_width = if platform.state == CrumblingState::Shaking {
        2.0 * pulse // Pulsing cracks when about to break
    } else {
        2.0
    };

    // Enhanced cracks with animation
    let crack_offset = (time * 10.0).sin() * 0.5;

    // Main diagonal crack (animated)
    draw_line(
        pos.x + rect.w * 0.2,
        pos.y,
        pos.x + rect.w * 0.35 + crack_offset,
        pos.y + rect.h,
        crack_width,
        crack_color,
    );
    draw_line(
        pos.x + rect.w * 0.6,
        pos.y,
        pos.x + rect.w * 0.7 - crack_offset,
        pos.y + rect.h,
        crack_width,
        crack_color,
    );

    // Additional cracks when shaking
    if platform.state == CrumblingState::Shaking {
        let crack_pulse_offset = (time * 8.0).sin() * 1.0;

        // Horizontal cracks
        draw_line(
            pos.x + rect.w * 0.1,
            pos.y + rect.h * 0.4,
            pos.x + rect.w * 0.5 + crack_pulse_offset,
            pos.y + rect.h * 0.5,
            crack_width * 0.8,
            crack_color,
        );
        draw_line(
            pos.x + rect.w * 0.5,
            pos.y + rect.h * 0.6,
            pos.x + rect.w * 0.9 - crack_pulse_offset,
            pos.y + rect.h * 0.5,
            crack_width * 0.8,
            crack_color,
        );

        // Branching cracks
        draw_line(
            pos.x + rect.w * 0.35 + crack_offset,
            pos.y + rect.h * 0.5,
            pos.x + rect.w * 0.5,
            pos.y + rect.h * 0.7,
            crack_width * 0.6,
            crack_color,
        );
    }

    // Crumbling particles when shaking (enhanced)
    if platform.state == CrumblingState::Shaking {
        for i in 0..5 {
            let particle_x = pos.x + rect.w * (0.1 + (i as f32) * 0.2);
            let particle_y = pos.y + rect.h + (time * 20.0 + (i as f32) * 1.5).sin().abs() * 10.0;
            let particle_size = 2.0 + (time * 5.0 + i as f32).sin().abs() * 1.5;
            let particle_alpha = 0.4 + (time * 3.0 + i as f32 * 0.7).sin() * 0.3;
            draw_circle(particle_x, particle_y, particle_size, Color::new(0.5, 0.4, 0.3, particle_alpha));
        }
    }
}

//! Shared decoration animation primitives
//!
//! Reusable drawing functions for common decoration patterns:
//! - Segmented swaying plants (seaweed, kelp, vines, tendrils)
//! - Pulsing glow orbs (bio-luminescent effects)
//! - Multi-layer glow rings

use macroquad::prelude::*;

/// Parameters for segmented sway animation (seaweed, kelp, vines, palm fronds, tendrils)
#[derive(Clone, Debug)]
pub struct SwayParams {
    /// Animation speed multiplier (0.8 - 1.8 typical)
    pub sway_speed: f32,
    /// Phase offset between segments (0.3 - 0.5 typical)
    pub sway_phase_offset: f32,
    /// Maximum sway distance in pixels (3.0 - 5.0 typical)
    pub sway_amplitude: f32,
    /// Base number of segments before variant is added (4 - 6 typical)
    pub base_segments: i32,
    /// Height of each segment in pixels (6.0 - 12.0 typical)
    pub seg_height: f32,
    /// Width of the first segment (2.5 - 4.0 typical)
    pub starting_width: f32,
    /// Width reduction per segment (0.3 - 0.5 typical)
    pub width_taper: f32,
    /// Minimum width at the tip (0.5 - 1.5 typical)
    pub min_width: f32,
}

impl Default for SwayParams {
    fn default() -> Self {
        Self {
            sway_speed: 1.5,
            sway_phase_offset: 0.4,
            sway_amplitude: 3.0,
            base_segments: 4,
            seg_height: 8.0,
            starting_width: 3.0,
            width_taper: 0.3,
            min_width: 1.0,
        }
    }
}

/// Draw a segmented swaying plant decoration.
///
/// Returns the final position (tip) for adding decorations like leaves or glows.
pub fn draw_segmented_sway(
    base: Vec2,
    scale: f32,
    variant: u8,
    time: f32,
    color: Color,
    params: &SwayParams,
) -> Vec2 {
    let segments = params.base_segments + variant as i32;
    let seg_height = params.seg_height * scale;

    let mut prev_x = base.x;
    let mut prev_y = base.y;

    for i in 0..segments {
        let sway = (time * params.sway_speed + i as f32 * params.sway_phase_offset).sin()
            * params.sway_amplitude
            * scale;
        let y = base.y - ((i + 1) as f32 * seg_height);
        let x = base.x + sway;
        let width = (params.starting_width - i as f32 * params.width_taper).max(params.min_width) * scale;

        draw_line(prev_x, prev_y, x, y, width, color);

        prev_x = x;
        prev_y = y;
    }

    vec2(prev_x, prev_y)
}

/// Draw a segmented swaying plant with leaf callback.
///
/// The leaf callback receives (segment_index, x, y, scale) and can draw leaves at each segment.
pub fn draw_segmented_sway_with_leaves<F>(
    base: Vec2,
    scale: f32,
    variant: u8,
    time: f32,
    color: Color,
    params: &SwayParams,
    mut leaf_callback: F,
) -> Vec2
where
    F: FnMut(i32, f32, f32, f32),
{
    let segments = params.base_segments + variant as i32;
    let seg_height = params.seg_height * scale;

    let mut prev_x = base.x;
    let mut prev_y = base.y;

    for i in 0..segments {
        let sway = (time * params.sway_speed + i as f32 * params.sway_phase_offset).sin()
            * params.sway_amplitude
            * scale;
        let y = base.y - ((i + 1) as f32 * seg_height);
        let x = base.x + sway;
        let width = (params.starting_width - i as f32 * params.width_taper).max(params.min_width) * scale;

        draw_line(prev_x, prev_y, x, y, width, color);

        // Call leaf callback for each segment
        leaf_callback(i, x, y, scale);

        prev_x = x;
        prev_y = y;
    }

    vec2(prev_x, prev_y)
}

/// Parameters for pulsing glow effect (bio-luminescent orbs, crystals, mystic orbs)
#[derive(Clone, Debug)]
pub struct GlowParams {
    /// Pulse animation speed (1.5 - 2.0 typical)
    pub pulse_speed: f32,
    /// Pulse intensity variation (0.3 - 0.4 typical)
    pub pulse_amplitude: f32,
    /// Base intensity level (0.6 - 0.7 typical)
    pub base_intensity: f32,
    /// Number of glow layers (3 - 4 typical)
    pub layer_count: u8,
    /// Size multiplier for each outer layer (0.5 typical)
    pub layer_size_mult: f32,
    /// Alpha reduction per layer (0.08 typical)
    pub layer_alpha_decay: f32,
    /// Starting alpha for outermost layer (0.3 - 0.35 typical)
    pub initial_alpha: f32,
    /// Core size relative to base size (0.4 - 0.5 typical)
    pub core_size_mult: f32,
    /// Core alpha intensity (0.8 typical)
    pub core_alpha: f32,
}

impl Default for GlowParams {
    fn default() -> Self {
        Self {
            pulse_speed: 1.5,
            pulse_amplitude: 0.3,
            base_intensity: 0.7,
            layer_count: 3,
            layer_size_mult: 0.5,
            layer_alpha_decay: 0.08,
            initial_alpha: 0.3,
            core_size_mult: 0.5,
            core_alpha: 0.8,
        }
    }
}

/// Draw a multi-layer pulsing glow orb.
///
/// Returns the calculated pulse value for additional effects.
pub fn draw_pulsing_glow(
    pos: Vec2,
    base_size: f32,
    time: f32,
    phase: f32,
    glow_color: Color,
    params: &GlowParams,
) -> f32 {
    let pulse = (time * params.pulse_speed + phase * std::f32::consts::TAU).sin()
        * params.pulse_amplitude
        + params.base_intensity;

    // Outer glow layers
    for i in 0..params.layer_count {
        let layer_size = base_size * (1.0 + i as f32 * params.layer_size_mult);
        let alpha = (params.initial_alpha - i as f32 * params.layer_alpha_decay) * pulse;
        draw_circle(
            pos.x,
            pos.y,
            layer_size,
            Color::new(glow_color.r, glow_color.g, glow_color.b, alpha),
        );
    }

    // Core
    draw_circle(
        pos.x,
        pos.y,
        base_size * params.core_size_mult,
        Color::new(glow_color.r, glow_color.g, glow_color.b, params.core_alpha * pulse),
    );

    pulse
}

/// Calculate a pulse value for timing effects without drawing.
///
/// Useful when you need the pulse timing but want custom drawing.
pub fn calc_pulse(time: f32, phase: f32, speed: f32, amplitude: f32, base: f32) -> f32 {
    (time * speed + phase * std::f32::consts::TAU).sin() * amplitude + base
}

/// Helper to derive a color from a theme color with multipliers.
///
/// Common pattern in decoration rendering.
pub fn derive_color(base: Color, r_mult: f32, g_mult: f32, b_mult: f32, alpha: f32) -> Color {
    Color::new(
        (base.r * r_mult).min(1.0),
        (base.g * g_mult).min(1.0),
        (base.b * b_mult).min(1.0),
        alpha,
    )
}

/// Draw a simple highlight effect on a shape.
pub fn draw_highlight(pos: Vec2, width: f32, height: f32, intensity: f32) {
    let highlight = Color::new(1.0, 1.0, 1.0, intensity);
    draw_ellipse(
        pos.x - width * 0.2,
        pos.y - height * 0.2,
        width * 0.4,
        height * 0.4,
        0.0,
        highlight,
    );
}

// Pre-configured parameter sets for common decoration types

impl SwayParams {
    /// Seaweed-style sway: slow, gentle movement
    pub fn seaweed() -> Self {
        Self {
            sway_speed: 1.5,
            sway_phase_offset: 0.4,
            sway_amplitude: 3.0,
            base_segments: 4,
            seg_height: 8.0,
            starting_width: 3.0,
            width_taper: 0.3,
            min_width: 1.0,
        }
    }

    /// Kelp-style sway: taller, larger movement
    pub fn kelp() -> Self {
        Self {
            sway_speed: 1.2,
            sway_phase_offset: 0.3,
            sway_amplitude: 5.0,
            base_segments: 6,
            seg_height: 12.0,
            starting_width: 4.0,
            width_taper: 0.4,
            min_width: 1.5,
        }
    }

    /// Vine-style sway: slow, organic movement
    pub fn vine() -> Self {
        Self {
            sway_speed: 0.8,
            sway_phase_offset: 0.5,
            sway_amplitude: 3.0,
            base_segments: 4,
            seg_height: 8.0,
            starting_width: 2.5,
            width_taper: 0.3,
            min_width: 1.0,
        }
    }

    /// Palm frond-style sway: medium movement
    pub fn palm_frond() -> Self {
        Self {
            sway_speed: 1.0,
            sway_phase_offset: 0.3,
            sway_amplitude: 4.0,
            base_segments: 5,
            seg_height: 10.0,
            starting_width: 3.0,
            width_taper: 0.4,
            min_width: 1.0,
        }
    }

    /// Tendril-style sway: faster, wave-like movement
    pub fn tendril() -> Self {
        Self {
            sway_speed: 1.8,
            sway_phase_offset: 0.4,
            sway_amplitude: 4.0,
            base_segments: 5,
            seg_height: 6.0,
            starting_width: 2.5,
            width_taper: 0.3,
            min_width: 0.5,
        }
    }
}

impl GlowParams {
    /// Bio-glow style: subtle, organic pulsing
    pub fn bio_glow() -> Self {
        Self {
            pulse_speed: 1.5,
            pulse_amplitude: 0.4,
            base_intensity: 0.6,
            layer_count: 3,
            layer_size_mult: 0.5,
            layer_alpha_decay: 0.08,
            initial_alpha: 0.3,
            core_size_mult: 0.5,
            core_alpha: 0.8,
        }
    }

    /// Mystic orb style: brighter, more layers
    pub fn mystic_orb() -> Self {
        Self {
            pulse_speed: 1.5,
            pulse_amplitude: 0.3,
            base_intensity: 0.7,
            layer_count: 4,
            layer_size_mult: 0.5,
            layer_alpha_decay: 0.08,
            initial_alpha: 0.35,
            core_size_mult: 0.4,
            core_alpha: 0.9,
        }
    }

    /// Crystal glow style: faster, subtle
    pub fn crystal() -> Self {
        Self {
            pulse_speed: 2.0,
            pulse_amplitude: 0.3,
            base_intensity: 0.7,
            layer_count: 1,
            layer_size_mult: 0.3,
            layer_alpha_decay: 0.0,
            initial_alpha: 0.5,
            core_size_mult: 0.3,
            core_alpha: 0.5,
        }
    }
}

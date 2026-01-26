//! Biome-specific geometry rendering
//!
//! Provides drawing functions for different geometry styles per biome.

use macroquad::prelude::*;

use octoplat_core::procgen::biome::theme::GeometryStyle;

/// Draw a platform top edge with biome-specific style
pub fn draw_platform_edge(
    x: f32,
    y: f32,
    width: f32,
    style: GeometryStyle,
    color: Color,
    time: f32,
) {
    match style {
        GeometryStyle::Standard => draw_platform_wavy(x, y, width, color, time),
        GeometryStyle::Organic => draw_platform_coral(x, y, width, color, time),
        GeometryStyle::Tropical => draw_platform_tropical(x, y, width, color, time),
        GeometryStyle::Broken => draw_platform_broken(x, y, width, color, time),
        GeometryStyle::Icy => draw_platform_icy(x, y, width, color, time),
        GeometryStyle::Jagged => draw_platform_jagged(x, y, width, color, time),
        GeometryStyle::Ancient => draw_platform_ancient(x, y, width, color, time),
        GeometryStyle::Crystalline => draw_platform_crystal(x, y, width, color, time),
    }
}

/// Draw solid block decoration with biome style
pub fn draw_block_decoration(
    x: f32,
    y: f32,
    size: f32,
    style: GeometryStyle,
    base_color: Color,
    time: f32,
    glow_color: Option<Color>,
) {
    match style {
        GeometryStyle::Standard => draw_block_standard(x, y, size, base_color),
        GeometryStyle::Organic => draw_block_organic(x, y, size, base_color, time),
        GeometryStyle::Tropical => draw_block_tropical(x, y, size, base_color, time),
        GeometryStyle::Broken => draw_block_broken(x, y, size, base_color, time),
        GeometryStyle::Icy => draw_block_icy(x, y, size, base_color, time, glow_color),
        GeometryStyle::Jagged => draw_block_jagged(x, y, size, base_color, time),
        GeometryStyle::Ancient => draw_block_ancient(x, y, size, base_color, time, glow_color),
        GeometryStyle::Crystalline => draw_block_crystal(x, y, size, base_color, time, glow_color),
    }
}

// ============================================================================
// Standard Style (Ocean Depths) - wavy edges, rounded corners
// ============================================================================

fn draw_platform_wavy(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Undulating top edge
    let segments = (width / 8.0) as i32;
    let highlight = Color::new(
        (color.r * 1.2).min(1.0),
        (color.g * 1.2).min(1.0),
        (color.b * 1.2).min(1.0),
        color.a,
    );

    for i in 0..segments {
        let seg_x = x + i as f32 * 8.0;
        let wave = (time * 2.0 + i as f32 * 0.5).sin() * 1.5;
        draw_rectangle(seg_x, y + wave, 8.0, 3.0, highlight);
    }
}

fn draw_block_standard(x: f32, y: f32, size: f32, color: Color) {
    // Rounded corner effect with lighter edges
    let highlight = Color::new(
        (color.r * 1.15).min(1.0),
        (color.g * 1.15).min(1.0),
        (color.b * 1.15).min(1.0),
        color.a * 0.6,
    );

    // Top-left highlight
    draw_circle(x + 4.0, y + 4.0, 3.0, highlight);
    // Subtle edge glow
    draw_rectangle(x, y, size, 2.0, highlight);
    draw_rectangle(x, y, 2.0, size, highlight);
}

// ============================================================================
// Organic Style (Coral Reefs) - branching protrusions
// ============================================================================

fn draw_platform_coral(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Branching coral protrusions on top
    let branch_color = Color::new(
        (color.r * 1.3).min(1.0),
        (color.g * 1.1).min(1.0),
        (color.b * 1.2).min(1.0),
        color.a,
    );

    let num_branches = (width / 16.0) as i32;
    for i in 0..num_branches {
        let bx = x + 8.0 + i as f32 * 16.0 + (time * 0.5 + i as f32).sin() * 2.0;
        let height = 6.0 + (i as f32 * 2.3).sin().abs() * 4.0;
        let sway = (time * 1.5 + i as f32 * 0.7).sin() * 2.0;

        // Main branch
        draw_line(bx, y, bx + sway, y - height, 3.0, branch_color);
        // Branch tip (ball)
        draw_circle(bx + sway, y - height, 2.5, branch_color);

        // Sub-branches
        if i % 2 == 0 {
            let sub_height = height * 0.6;
            draw_line(bx, y - height * 0.4, bx + sway + 4.0, y - sub_height - 2.0, 2.0, branch_color);
            draw_circle(bx + sway + 4.0, y - sub_height - 2.0, 1.5, branch_color);
        }
    }
}

fn draw_block_organic(x: f32, y: f32, size: f32, color: Color, time: f32) {
    // Organic blob-like texture
    let num_blobs = 3;
    let blob_color = Color::new(
        (color.r * 1.2).min(1.0),
        (color.g * 1.1).min(1.0),
        color.b,
        color.a * 0.5,
    );

    for i in 0..num_blobs {
        let bx = x + size * 0.2 + (i as f32 / num_blobs as f32) * size * 0.6;
        let by = y + size * 0.3 + (time * 0.5 + i as f32).sin() * 2.0;
        let blob_size = 4.0 + (i as f32 * 1.7).sin().abs() * 3.0;
        draw_circle(bx, by, blob_size, blob_color);
    }
}

// ============================================================================
// Broken Style (Shipwreck) - tilted fragments, wood grain
// ============================================================================

fn draw_platform_broken(x: f32, y: f32, width: f32, color: Color, _time: f32) {
    // Wood plank lines
    let grain_color = Color::new(
        color.r * 0.7,
        color.g * 0.6,
        color.b * 0.5,
        color.a * 0.6,
    );

    let planks = (width / 12.0) as i32;
    for i in 0..planks {
        let px = x + i as f32 * 12.0 + 2.0;
        // Slightly tilted planks
        let tilt = ((i as f32 * 3.7) % 2.0 - 1.0) * 0.5;
        draw_line(px, y + 1.0, px, y + 6.0 + tilt, 1.5, grain_color);
        draw_line(px + 4.0, y + 2.0, px + 4.0, y + 5.0 - tilt, 1.0, grain_color);
    }

    // Nail heads
    let nail_color = Color::new(0.4, 0.35, 0.3, 0.8);
    for i in 0..(planks / 2) {
        let nx = x + 6.0 + i as f32 * 24.0;
        draw_circle(nx, y + 3.0, 1.5, nail_color);
    }
}

fn draw_block_broken(x: f32, y: f32, size: f32, color: Color, _time: f32) {
    // Wood grain texture
    let grain_color = Color::new(
        color.r * 0.75,
        color.g * 0.65,
        color.b * 0.55,
        color.a * 0.5,
    );

    // Horizontal wood grain lines
    for i in 0..4 {
        let ly = y + 4.0 + i as f32 * 7.0;
        let wave = ((i as f32 * 2.3) % 3.0) * 2.0;
        draw_line(x + 2.0, ly, x + size - 2.0, ly + wave, 1.5, grain_color);
    }

    // Hole/damage
    let hole_color = Color::new(0.1, 0.08, 0.06, 0.6);
    draw_circle(x + size * 0.7, y + size * 0.6, 4.0, hole_color);
}

// ============================================================================
// Jagged Style (Volcanic Vents) - sharp edges, lava drip
// ============================================================================

fn draw_platform_jagged(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Sharp spikes on top
    let spike_color = Color::new(
        (color.r * 0.9).min(1.0),
        color.g * 0.8,
        color.b * 0.7,
        color.a,
    );

    let num_spikes = (width / 10.0) as i32;
    for i in 0..num_spikes {
        let sx = x + 5.0 + i as f32 * 10.0;
        let height = 4.0 + (i as f32 * 2.1).sin().abs() * 3.0;
        draw_triangle(
            vec2(sx - 3.0, y),
            vec2(sx + 3.0, y),
            vec2(sx, y - height),
            spike_color,
        );
    }

    // Lava drip effect
    let drip_color = Color::new(1.0, 0.5, 0.1, 0.7);
    let drip_offset = (time * 2.0) % 20.0;
    if drip_offset < 10.0 {
        let drip_x = x + width * 0.3;
        draw_circle(drip_x, y + 4.0 + drip_offset, 2.0, drip_color);
    }
}

fn draw_block_jagged(x: f32, y: f32, size: f32, color: Color, time: f32) {
    // Darkened base edge highlight using the block color
    let edge_color = Color::new(
        color.r * 0.6,
        color.g * 0.5,
        color.b * 0.4,
        color.a * 0.5,
    );
    draw_rectangle(x, y, size, 2.0, edge_color);
    draw_rectangle(x, y, 2.0, size, edge_color);

    // Lava crack effect
    let crack_color = Color::new(1.0, 0.4, 0.1, 0.6 + (time * 3.0).sin() * 0.2);

    // Glowing cracks
    draw_line(x + size * 0.2, y + size * 0.3, x + size * 0.5, y + size * 0.6, 2.0, crack_color);
    draw_line(x + size * 0.5, y + size * 0.6, x + size * 0.8, y + size * 0.4, 2.0, crack_color);
    draw_line(x + size * 0.5, y + size * 0.6, x + size * 0.6, y + size * 0.9, 1.5, crack_color);

    // Hot spots
    let glow_alpha = 0.3 + (time * 2.0).sin() * 0.15;
    draw_circle(x + size * 0.5, y + size * 0.6, 4.0, Color::new(1.0, 0.6, 0.2, glow_alpha));
}

// ============================================================================
// Crystalline Style (Abyss) - angular facets, bioluminescent
// ============================================================================

fn draw_platform_crystal(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Crystal facets on top
    let facet_color = Color::new(
        (color.r * 1.4).min(1.0),
        (color.g * 1.3).min(1.0),
        (color.b * 1.5).min(1.0),
        color.a * 0.8,
    );

    let num_crystals = (width / 14.0) as i32;
    for i in 0..num_crystals {
        let cx = x + 7.0 + i as f32 * 14.0;
        let height = 5.0 + (i as f32 * 1.9).sin().abs() * 4.0;
        let glow = (time * 2.0 + i as f32 * 0.8).sin() * 0.3 + 0.7;

        // Crystal shape (diamond)
        let crystal_color = Color::new(
            facet_color.r * glow,
            facet_color.g * glow,
            facet_color.b * glow,
            facet_color.a,
        );

        draw_triangle(
            vec2(cx - 4.0, y),
            vec2(cx + 4.0, y),
            vec2(cx, y - height),
            crystal_color,
        );

        // Highlight facet
        draw_triangle(
            vec2(cx - 2.0, y - 1.0),
            vec2(cx + 1.0, y - 1.0),
            vec2(cx - 0.5, y - height + 2.0),
            Color::new(1.0, 1.0, 1.0, 0.3 * glow),
        );
    }
}

fn draw_block_crystal(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    time: f32,
    glow_color: Option<Color>,
) {
    // Bioluminescent glow center
    if let Some(glow) = glow_color {
        let pulse = (time * 2.0).sin() * 0.3 + 0.7;
        let glow_size = size * 0.3 * pulse;

        // Multiple glow layers
        for i in 0..3 {
            let layer_size = glow_size * (1.0 + i as f32 * 0.4);
            let alpha = glow.a * (0.3 - i as f32 * 0.08) * pulse;
            draw_circle(
                x + size * 0.5,
                y + size * 0.5,
                layer_size,
                Color::new(glow.r, glow.g, glow.b, alpha),
            );
        }
    }

    // Crystal facet lines
    let facet_color = Color::new(
        (color.r * 1.3).min(1.0),
        (color.g * 1.2).min(1.0),
        (color.b * 1.4).min(1.0),
        color.a * 0.4,
    );

    // Angular facet pattern
    draw_line(x + size * 0.2, y + size * 0.1, x + size * 0.5, y + size * 0.5, 1.0, facet_color);
    draw_line(x + size * 0.8, y + size * 0.2, x + size * 0.5, y + size * 0.5, 1.0, facet_color);
    draw_line(x + size * 0.5, y + size * 0.5, x + size * 0.3, y + size * 0.9, 1.0, facet_color);
    draw_line(x + size * 0.5, y + size * 0.5, x + size * 0.7, y + size * 0.85, 1.0, facet_color);
}

// ============================================================================
// Tropical Style (Tropical Shore) - palm fronds, sandy textures
// ============================================================================

fn draw_platform_tropical(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Palm frond-like protrusions on top
    let frond_color = Color::new(
        (color.r * 0.8).min(1.0),
        (color.g * 1.3).min(1.0),
        color.b * 0.6,
        color.a,
    );

    let num_fronds = (width / 20.0) as i32;
    for i in 0..num_fronds {
        let fx = x + 10.0 + i as f32 * 20.0;
        let sway = (time * 1.2 + i as f32 * 0.5).sin() * 2.0;
        let height = 8.0 + (i as f32 * 1.7).sin().abs() * 4.0;

        // Main frond stem
        draw_line(fx, y, fx + sway, y - height, 2.0, frond_color);

        // Frond leaves (alternating sides)
        for j in 0..3 {
            let leaf_y = y - height * (0.3 + j as f32 * 0.25);
            let leaf_sway = sway * (0.5 + j as f32 * 0.2);
            let dir = if j % 2 == 0 { 1.0 } else { -1.0 };
            draw_line(
                fx + leaf_sway * 0.5,
                leaf_y,
                fx + leaf_sway + dir * 6.0,
                leaf_y - 2.0,
                1.5,
                frond_color,
            );
        }
    }

    // Sandy sparkles
    let sand_color = Color::new(1.0, 0.95, 0.7, 0.4);
    for i in 0..(width / 15.0) as i32 {
        let sparkle_x = x + 7.5 + i as f32 * 15.0 + (time * 0.5 + i as f32).sin() * 2.0;
        if (time * 3.0 + i as f32 * 1.7).sin() > 0.5 {
            draw_circle(sparkle_x, y + 2.0, 1.5, sand_color);
        }
    }
}

fn draw_block_tropical(x: f32, y: f32, size: f32, color: Color, _time: f32) {
    // Sandy texture with shell-like patterns
    let shell_color = Color::new(
        (color.r * 1.1).min(1.0),
        (color.g * 1.05).min(1.0),
        color.b * 0.9,
        color.a * 0.5,
    );

    // Curved shell/wave patterns
    draw_ellipse(x + size * 0.3, y + size * 0.4, size * 0.2, size * 0.15, 0.0, shell_color);
    draw_ellipse(x + size * 0.7, y + size * 0.6, size * 0.15, size * 0.12, 0.0, shell_color);

    // Sandy grain dots
    let grain_color = Color::new(color.r * 0.85, color.g * 0.8, color.b * 0.7, 0.4);
    draw_circle(x + size * 0.2, y + size * 0.7, 2.0, grain_color);
    draw_circle(x + size * 0.6, y + size * 0.3, 1.5, grain_color);
    draw_circle(x + size * 0.8, y + size * 0.8, 2.0, grain_color);
}

// ============================================================================
// Icy Style (Arctic Waters) - ice crystals, frosted edges
// ============================================================================

fn draw_platform_icy(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Ice crystal formations on top
    let ice_color = Color::new(
        (color.r * 1.2).min(1.0),
        (color.g * 1.1).min(1.0),
        (color.b * 1.05).min(1.0),
        color.a * 0.9,
    );

    let num_crystals = (width / 12.0) as i32;
    for i in 0..num_crystals {
        let cx = x + 6.0 + i as f32 * 12.0;
        let height = 4.0 + (i as f32 * 2.3).sin().abs() * 5.0;
        let shimmer = (time * 2.5 + i as f32 * 0.6).sin() * 0.2 + 0.8;

        let crystal_color = Color::new(
            ice_color.r * shimmer,
            ice_color.g * shimmer,
            ice_color.b,
            ice_color.a,
        );

        // Ice spike
        draw_triangle(
            vec2(cx - 3.0, y),
            vec2(cx + 3.0, y),
            vec2(cx + (i as f32 * 0.3).sin(), y - height),
            crystal_color,
        );

        // Small highlight
        draw_triangle(
            vec2(cx - 1.0, y - 1.0),
            vec2(cx + 0.5, y - 1.0),
            vec2(cx - 0.3, y - height + 1.0),
            Color::new(1.0, 1.0, 1.0, 0.4 * shimmer),
        );
    }

    // Frost particles
    let frost_color = Color::new(0.9, 0.95, 1.0, 0.3);
    for i in 0..(width / 20.0) as i32 {
        let fx = x + 10.0 + i as f32 * 20.0;
        let drift = (time * 0.8 + i as f32 * 1.2).sin() * 3.0;
        draw_circle(fx + drift, y - 2.0 - (i as f32 * 0.5), 1.0, frost_color);
    }
}

fn draw_block_icy(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    time: f32,
    glow_color: Option<Color>,
) {
    // Northern lights glow effect
    if let Some(glow) = glow_color {
        let pulse = (time * 1.5).sin() * 0.3 + 0.5;
        let aurora_color = Color::new(glow.r, glow.g, glow.b, glow.a * pulse * 0.4);
        draw_circle(x + size * 0.5, y + size * 0.3, size * 0.25, aurora_color);
    }

    // Ice crack patterns
    let crack_color = Color::new(
        (color.r * 1.3).min(1.0),
        (color.g * 1.2).min(1.0),
        color.b,
        color.a * 0.5,
    );

    // Branching cracks
    draw_line(x + size * 0.2, y + size * 0.2, x + size * 0.5, y + size * 0.5, 1.0, crack_color);
    draw_line(x + size * 0.5, y + size * 0.5, x + size * 0.8, y + size * 0.3, 1.0, crack_color);
    draw_line(x + size * 0.5, y + size * 0.5, x + size * 0.4, y + size * 0.8, 1.0, crack_color);
    draw_line(x + size * 0.5, y + size * 0.5, x + size * 0.7, y + size * 0.9, 1.0, crack_color);

    // Frost sparkle
    let sparkle = (time * 3.0).sin() * 0.5 + 0.5;
    draw_circle(
        x + size * 0.3,
        y + size * 0.6,
        2.0,
        Color::new(1.0, 1.0, 1.0, 0.4 * sparkle),
    );
}

// ============================================================================
// Ancient Style (Sunken Ruins) - columns, carved patterns
// ============================================================================

fn draw_platform_ancient(x: f32, y: f32, width: f32, color: Color, time: f32) {
    // Column tops / carved stone edge
    let stone_color = Color::new(
        color.r * 0.9,
        color.g * 0.9,
        color.b * 0.85,
        color.a,
    );

    let num_columns = (width / 16.0) as i32;
    for i in 0..num_columns {
        let cx = x + 8.0 + i as f32 * 16.0;

        // Column capital (top decoration)
        draw_rectangle(cx - 5.0, y - 4.0, 10.0, 4.0, stone_color);
        draw_rectangle(cx - 6.0, y - 6.0, 12.0, 2.0, stone_color);

        // Carved groove
        let groove_color = Color::new(color.r * 0.7, color.g * 0.7, color.b * 0.65, 0.6);
        draw_line(cx - 3.0, y - 2.0, cx - 3.0, y + 2.0, 1.0, groove_color);
        draw_line(cx + 3.0, y - 2.0, cx + 3.0, y + 2.0, 1.0, groove_color);
    }

    // Mysterious glow particles
    let glow_color = Color::new(0.4, 0.8, 0.9, 0.3);
    for i in 0..(width / 30.0) as i32 {
        let gx = x + 15.0 + i as f32 * 30.0;
        let rise = ((time * 0.5 + i as f32 * 0.7) % 1.5) * 10.0;
        let alpha = (1.0 - rise / 15.0) * 0.4;
        draw_circle(gx, y - 8.0 - rise, 2.0, Color::new(glow_color.r, glow_color.g, glow_color.b, alpha));
    }
}

fn draw_block_ancient(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    time: f32,
    glow_color: Option<Color>,
) {
    // Mysterious glow center
    if let Some(glow) = glow_color {
        let pulse = (time * 1.8).sin() * 0.25 + 0.6;
        let glow_size = size * 0.2 * pulse;
        draw_circle(
            x + size * 0.5,
            y + size * 0.5,
            glow_size,
            Color::new(glow.r, glow.g, glow.b, glow.a * pulse * 0.5),
        );
    }

    // Carved stone patterns (ancient symbols)
    let carve_color = Color::new(
        color.r * 0.7,
        color.g * 0.7,
        color.b * 0.65,
        color.a * 0.5,
    );

    // Horizontal carved lines (like ancient text)
    draw_line(x + size * 0.15, y + size * 0.25, x + size * 0.45, y + size * 0.25, 1.5, carve_color);
    draw_line(x + size * 0.55, y + size * 0.25, x + size * 0.85, y + size * 0.25, 1.5, carve_color);
    draw_line(x + size * 0.2, y + size * 0.5, x + size * 0.8, y + size * 0.5, 1.5, carve_color);
    draw_line(x + size * 0.15, y + size * 0.75, x + size * 0.4, y + size * 0.75, 1.5, carve_color);
    draw_line(x + size * 0.6, y + size * 0.75, x + size * 0.85, y + size * 0.75, 1.5, carve_color);

    // Corner decorations (weathered)
    let corner_color = Color::new(color.r * 1.1, color.g * 1.1, color.b * 1.05, 0.4);
    draw_rectangle(x + 1.0, y + 1.0, 4.0, 4.0, corner_color);
    draw_rectangle(x + size - 5.0, y + 1.0, 4.0, 4.0, corner_color);
}

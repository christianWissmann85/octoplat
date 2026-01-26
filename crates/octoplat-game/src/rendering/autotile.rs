//! Auto-tiling system for visual tile variety
//!
//! Renders tiles differently based on their neighbors to create
//! natural-looking edges, corners, and interior sections.
//! Supports optional texture overlays for enhanced visuals.

use macroquad::prelude::*;

use octoplat_core::level::{TileMap, TileType};
use octoplat_core::procgen::BiomeTheme;
use crate::compat::ToMqColor;
use super::tile_textures::draw_tile_texture;

/// Neighbor bitmask constants
pub mod neighbors {
    pub const TOP: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const BOTTOM: u8 = 4;
    pub const LEFT: u8 = 8;

    // Common configurations
    pub const ISOLATED: u8 = 0;                              // 0000
    pub const INTERIOR: u8 = TOP | RIGHT | BOTTOM | LEFT;   // 1111 = 15

    // Single edge exposed (3 neighbors)
    pub const EDGE_TOP: u8 = RIGHT | BOTTOM | LEFT;         // 1110 = 14
    pub const EDGE_RIGHT: u8 = TOP | BOTTOM | LEFT;         // 1101 = 13
    pub const EDGE_BOTTOM: u8 = TOP | RIGHT | LEFT;         // 1011 = 11
    pub const EDGE_LEFT: u8 = TOP | RIGHT | BOTTOM;         // 0111 = 7

    // Corner exposed (2 neighbors, adjacent)
    pub const CORNER_TL: u8 = RIGHT | BOTTOM;               // 0110 = 6
    pub const CORNER_TR: u8 = BOTTOM | LEFT;                // 1100 = 12
    pub const CORNER_BL: u8 = TOP | RIGHT;                  // 0011 = 3
    pub const CORNER_BR: u8 = TOP | LEFT;                   // 1001 = 9

    // Tunnel (2 neighbors, opposite)
    pub const TUNNEL_H: u8 = LEFT | RIGHT;                  // 1010 = 10
    pub const TUNNEL_V: u8 = TOP | BOTTOM;                  // 0101 = 5

    // End caps (1 neighbor)
    pub const END_TOP: u8 = BOTTOM;                         // 0100 = 4
    pub const END_RIGHT: u8 = LEFT;                         // 1000 = 8
    pub const END_BOTTOM: u8 = TOP;                         // 0001 = 1
    pub const END_LEFT: u8 = RIGHT;                         // 0010 = 2
}

/// Get the neighbor bitmask for a tile position
///
/// Returns a bitmask indicating which cardinal neighbors are solid:
/// - Bit 0 (1): Top neighbor is solid
/// - Bit 1 (2): Right neighbor is solid
/// - Bit 2 (4): Bottom neighbor is solid
/// - Bit 3 (8): Left neighbor is solid
pub fn get_tile_neighbors(tilemap: &TileMap, x: usize, y: usize) -> u8 {
    let mut mask = 0u8;

    // Check if neighbor positions are solid
    let is_solid = |nx: i32, ny: i32| -> bool {
        if nx < 0 || ny < 0 {
            return true; // Treat out of bounds as solid (for edge tiles)
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if nx >= tilemap.width || ny >= tilemap.height {
            return true; // Treat out of bounds as solid
        }
        let tile = tilemap.get(nx, ny);
        matches!(tile, TileType::Solid | TileType::Breakable)
    };

    let ix = x as i32;
    let iy = y as i32;

    if is_solid(ix, iy - 1) {
        mask |= neighbors::TOP;
    }
    if is_solid(ix + 1, iy) {
        mask |= neighbors::RIGHT;
    }
    if is_solid(ix, iy + 1) {
        mask |= neighbors::BOTTOM;
    }
    if is_solid(ix - 1, iy) {
        mask |= neighbors::LEFT;
    }

    mask
}

/// Draw a solid tile with auto-tiling based on neighbors
///
/// If a tile texture is provided, it will be drawn as an overlay using multiply blending
/// after the procedural base shape.
pub fn draw_autotile(
    px: f32,
    py: f32,
    size: f32,
    neighbor_mask: u8,
    theme: &BiomeTheme,
    tile_seed: u32,
    tile_texture: Option<&Texture2D>,
) {
    // Calculate color variations based on tile position
    let variation = ((tile_seed % 100) as f32 / 100.0) * 0.1 - 0.05; // -0.05 to +0.05

    match neighbor_mask {
        neighbors::INTERIOR => {
            draw_interior_tile(px, py, size, theme, variation);
        }
        neighbors::ISOLATED => {
            draw_isolated_tile(px, py, size, theme, variation);
        }
        // Single edges
        neighbors::EDGE_TOP => {
            draw_edge_tile(px, py, size, theme, variation, true, false, false, false);
        }
        neighbors::EDGE_RIGHT => {
            draw_edge_tile(px, py, size, theme, variation, false, true, false, false);
        }
        neighbors::EDGE_BOTTOM => {
            draw_edge_tile(px, py, size, theme, variation, false, false, true, false);
        }
        neighbors::EDGE_LEFT => {
            draw_edge_tile(px, py, size, theme, variation, false, false, false, true);
        }
        // Corners (outer)
        neighbors::CORNER_TL => {
            draw_corner_tile(px, py, size, theme, variation, true, false, false, false);
        }
        neighbors::CORNER_TR => {
            draw_corner_tile(px, py, size, theme, variation, false, true, false, false);
        }
        neighbors::CORNER_BL => {
            draw_corner_tile(px, py, size, theme, variation, false, false, true, false);
        }
        neighbors::CORNER_BR => {
            draw_corner_tile(px, py, size, theme, variation, false, false, false, true);
        }
        // Tunnels
        neighbors::TUNNEL_H => {
            draw_tunnel_tile(px, py, size, theme, variation, true);
        }
        neighbors::TUNNEL_V => {
            draw_tunnel_tile(px, py, size, theme, variation, false);
        }
        // End caps
        neighbors::END_TOP | neighbors::END_RIGHT | neighbors::END_BOTTOM | neighbors::END_LEFT => {
            draw_end_tile(px, py, size, theme, variation, neighbor_mask);
        }
        // Default: treat as partial edge based on which sides are exposed
        _ => {
            let top_exposed = neighbor_mask & neighbors::TOP == 0;
            let right_exposed = neighbor_mask & neighbors::RIGHT == 0;
            let bottom_exposed = neighbor_mask & neighbors::BOTTOM == 0;
            let left_exposed = neighbor_mask & neighbors::LEFT == 0;
            draw_edge_tile(px, py, size, theme, variation, top_exposed, right_exposed, bottom_exposed, left_exposed);
        }
    }

    // Draw texture overlay if available
    if let Some(texture) = tile_texture {
        let base_color = theme.solid_color.to_mq_color();
        draw_tile_texture(texture, px, py, size, base_color, 0.6);
    }
}

/// Draw an interior tile (fully surrounded by other tiles)
fn draw_interior_tile(px: f32, py: f32, size: f32, theme: &BiomeTheme, variation: f32) {
    // Darker base color for interior
    let base = theme.solid_color.to_mq_color();
    let dark_factor = 0.75 + variation;
    let color = Color::new(
        base.r * dark_factor,
        base.g * dark_factor,
        base.b * dark_factor,
        base.a,
    );

    draw_rectangle(px, py, size, size, color);

    // Subtle texture pattern based on variation
    let pattern_alpha = 0.08 + variation.abs() * 0.04;
    let pattern_color = Color::new(0.0, 0.0, 0.0, pattern_alpha);

    // Grid texture for interior
    draw_line(px + size * 0.5, py, px + size * 0.5, py + size, 1.0, pattern_color);
    draw_line(px, py + size * 0.5, px + size, py + size * 0.5, 1.0, pattern_color);
}

/// Draw an isolated tile (no neighbors)
fn draw_isolated_tile(px: f32, py: f32, size: f32, theme: &BiomeTheme, variation: f32) {
    let base = theme.solid_color.to_mq_color();
    let margin = 2.0;

    // Slightly lighter base
    let color = Color::new(
        (base.r * 1.1 + variation).min(1.0),
        (base.g * 1.1 + variation).min(1.0),
        (base.b * 1.1 + variation).min(1.0),
        base.a,
    );

    // Main body with inset
    draw_rectangle(px + margin, py + margin, size - margin * 2.0, size - margin * 2.0, color);

    // Highlight on top-left edges
    let highlight = Color::new(
        (base.r * 1.4).min(1.0),
        (base.g * 1.4).min(1.0),
        (base.b * 1.4).min(1.0),
        0.6,
    );
    draw_line(px + margin, py + margin, px + size - margin, py + margin, 2.0, highlight);
    draw_line(px + margin, py + margin, px + margin, py + size - margin, 2.0, highlight);

    // Shadow on bottom-right edges
    let shadow = Color::new(
        base.r * 0.5,
        base.g * 0.5,
        base.b * 0.5,
        0.6,
    );
    draw_line(px + size - margin, py + margin, px + size - margin, py + size - margin, 2.0, shadow);
    draw_line(px + margin, py + size - margin, px + size - margin, py + size - margin, 2.0, shadow);

    // Outer border
    let border = theme.solid_border_color().to_mq_color();
    draw_rectangle_lines(px, py, size, size, 1.0, border);
}

/// Draw a tile with exposed edges
#[allow(clippy::too_many_arguments)]
fn draw_edge_tile(
    px: f32,
    py: f32,
    size: f32,
    theme: &BiomeTheme,
    variation: f32,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
) {
    let base = theme.solid_color.to_mq_color();

    // Base fill - slightly darker toward interior
    let color = Color::new(
        base.r * (0.9 + variation),
        base.g * (0.9 + variation),
        base.b * (0.9 + variation),
        base.a,
    );
    draw_rectangle(px, py, size, size, color);

    // Highlight gradient on exposed edges
    let highlight = Color::new(
        (base.r * 1.3).min(1.0),
        (base.g * 1.3).min(1.0),
        (base.b * 1.3).min(1.0),
        0.5,
    );

    let edge_width = 4.0;

    if top {
        // Top edge highlight with gradient
        draw_rectangle(px, py, size, edge_width, highlight);
        // Lighter line at very top
        let light = Color::new(highlight.r, highlight.g, highlight.b, 0.7);
        draw_line(px, py + 1.0, px + size, py + 1.0, 2.0, light);
    }

    if right {
        draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);
    }

    if bottom {
        // Bottom edge - darker shadow
        let shadow = Color::new(
            base.r * 0.6,
            base.g * 0.6,
            base.b * 0.6,
            0.5,
        );
        draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
    }

    if left {
        draw_rectangle(px, py, edge_width, size, highlight);
    }

    // Subtle border on exposed edges only
    let border = theme.solid_border_color().to_mq_color();
    let border_alpha = Color::new(border.r, border.g, border.b, 0.4);

    if top {
        draw_line(px, py, px + size, py, 1.0, border_alpha);
    }
    if right {
        draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
    }
    if bottom {
        draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
    }
    if left {
        draw_line(px, py, px, py + size, 1.0, border_alpha);
    }
}

/// Draw a corner tile (two adjacent edges exposed)
#[allow(clippy::too_many_arguments)]
fn draw_corner_tile(
    px: f32,
    py: f32,
    size: f32,
    theme: &BiomeTheme,
    variation: f32,
    top_left: bool,
    top_right: bool,
    bottom_left: bool,
    bottom_right: bool,
) {
    let base = theme.solid_color.to_mq_color();

    // Base fill
    let color = Color::new(
        base.r * (0.9 + variation),
        base.g * (0.9 + variation),
        base.b * (0.9 + variation),
        base.a,
    );
    draw_rectangle(px, py, size, size, color);

    let highlight = Color::new(
        (base.r * 1.35).min(1.0),
        (base.g * 1.35).min(1.0),
        (base.b * 1.35).min(1.0),
        0.6,
    );

    let shadow = Color::new(
        base.r * 0.55,
        base.g * 0.55,
        base.b * 0.55,
        0.5,
    );

    let edge_width = 5.0;
    let corner_size = 8.0;

    if top_left {
        // Top and left edges exposed
        draw_rectangle(px, py, size, edge_width, highlight);
        draw_rectangle(px, py, edge_width, size, highlight);
        // Corner accent
        draw_triangle(
            vec2(px, py),
            vec2(px + corner_size, py),
            vec2(px, py + corner_size),
            Color::new(highlight.r, highlight.g, highlight.b, 0.8),
        );
    }

    if top_right {
        // Top and right edges exposed
        draw_rectangle(px, py, size, edge_width, highlight);
        draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);
        // Corner accent
        draw_triangle(
            vec2(px + size, py),
            vec2(px + size - corner_size, py),
            vec2(px + size, py + corner_size),
            Color::new(highlight.r, highlight.g, highlight.b, 0.8),
        );
    }

    if bottom_left {
        // Bottom and left edges
        draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
        draw_rectangle(px, py, edge_width, size, highlight);
    }

    if bottom_right {
        // Bottom and right edges
        draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
        draw_rectangle(px + size - edge_width, py, edge_width, size, shadow);
        // Dark corner
        draw_triangle(
            vec2(px + size, py + size),
            vec2(px + size - corner_size, py + size),
            vec2(px + size, py + size - corner_size),
            Color::new(shadow.r, shadow.g, shadow.b, 0.8),
        );
    }

    // Border on exposed edges
    let border = theme.solid_border_color().to_mq_color();
    let border_alpha = Color::new(border.r, border.g, border.b, 0.5);

    if top_left {
        draw_line(px, py, px + size, py, 1.0, border_alpha);
        draw_line(px, py, px, py + size, 1.0, border_alpha);
    }
    if top_right {
        draw_line(px, py, px + size, py, 1.0, border_alpha);
        draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
    }
    if bottom_left {
        draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
        draw_line(px, py, px, py + size, 1.0, border_alpha);
    }
    if bottom_right {
        draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
        draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
    }
}

/// Draw a tunnel tile (opposite edges exposed)
fn draw_tunnel_tile(
    px: f32,
    py: f32,
    size: f32,
    theme: &BiomeTheme,
    variation: f32,
    horizontal: bool,
) {
    let base = theme.solid_color.to_mq_color();

    // Darker center for tunnel effect
    let color = Color::new(
        base.r * (0.85 + variation),
        base.g * (0.85 + variation),
        base.b * (0.85 + variation),
        base.a,
    );
    draw_rectangle(px, py, size, size, color);

    let highlight = Color::new(
        (base.r * 1.25).min(1.0),
        (base.g * 1.25).min(1.0),
        (base.b * 1.25).min(1.0),
        0.5,
    );

    let edge_width = 4.0;

    if horizontal {
        // Left and right edges exposed
        draw_rectangle(px, py, edge_width, size, highlight);
        draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);

        // Central groove
        let groove = Color::new(base.r * 0.7, base.g * 0.7, base.b * 0.7, 0.3);
        draw_rectangle(px + size * 0.4, py, size * 0.2, size, groove);
    } else {
        // Top and bottom edges exposed
        draw_rectangle(px, py, size, edge_width, highlight);
        let shadow = Color::new(base.r * 0.6, base.g * 0.6, base.b * 0.6, 0.5);
        draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);

        // Central groove
        let groove = Color::new(base.r * 0.7, base.g * 0.7, base.b * 0.7, 0.3);
        draw_rectangle(px, py + size * 0.4, size, size * 0.2, groove);
    }

    // Borders
    let border = theme.solid_border_color().to_mq_color();
    let border_alpha = Color::new(border.r, border.g, border.b, 0.4);

    if horizontal {
        draw_line(px, py, px, py + size, 1.0, border_alpha);
        draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
    } else {
        draw_line(px, py, px + size, py, 1.0, border_alpha);
        draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
    }
}

/// Draw an end cap tile (only one neighbor)
fn draw_end_tile(
    px: f32,
    py: f32,
    size: f32,
    theme: &BiomeTheme,
    variation: f32,
    neighbor_mask: u8,
) {
    let base = theme.solid_color.to_mq_color();

    let color = Color::new(
        base.r * (0.95 + variation),
        base.g * (0.95 + variation),
        base.b * (0.95 + variation),
        base.a,
    );
    draw_rectangle(px, py, size, size, color);

    let highlight = Color::new(
        (base.r * 1.3).min(1.0),
        (base.g * 1.3).min(1.0),
        (base.b * 1.3).min(1.0),
        0.55,
    );

    let shadow = Color::new(
        base.r * 0.6,
        base.g * 0.6,
        base.b * 0.6,
        0.5,
    );

    let edge_width = 4.0;
    let border = theme.solid_border_color().to_mq_color();
    let border_alpha = Color::new(border.r, border.g, border.b, 0.5);

    // Determine which side has the neighbor and highlight the other three
    match neighbor_mask {
        neighbors::END_TOP => {
            // Only bottom neighbor - highlight top, left, right
            draw_rectangle(px, py, size, edge_width, highlight);
            draw_rectangle(px, py, edge_width, size, highlight);
            draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);
            draw_line(px, py, px + size, py, 1.0, border_alpha);
            draw_line(px, py, px, py + size, 1.0, border_alpha);
            draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
        }
        neighbors::END_BOTTOM => {
            // Only top neighbor - highlight bottom, left, right
            draw_rectangle(px, py, edge_width, size, highlight);
            draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);
            draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
            draw_line(px, py, px, py + size, 1.0, border_alpha);
            draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
            draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
        }
        neighbors::END_LEFT => {
            // Only right neighbor - highlight top, bottom, left
            draw_rectangle(px, py, size, edge_width, highlight);
            draw_rectangle(px, py, edge_width, size, highlight);
            draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
            draw_line(px, py, px + size, py, 1.0, border_alpha);
            draw_line(px, py, px, py + size, 1.0, border_alpha);
            draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
        }
        neighbors::END_RIGHT => {
            // Only left neighbor - highlight top, bottom, right
            draw_rectangle(px, py, size, edge_width, highlight);
            draw_rectangle(px + size - edge_width, py, edge_width, size, highlight);
            draw_rectangle(px, py + size - edge_width, size, edge_width, shadow);
            draw_line(px, py, px + size, py, 1.0, border_alpha);
            draw_line(px + size, py, px + size, py + size, 1.0, border_alpha);
            draw_line(px, py + size, px + size, py + size, 1.0, border_alpha);
        }
        _ => {}
    }
}

/// Generate a deterministic seed for a tile position
pub fn tile_seed(x: usize, y: usize) -> u32 {
    // Simple hash combining x and y
    let mut hash = x as u32;
    hash = hash.wrapping_mul(1664525).wrapping_add(1013904223);
    hash ^= y as u32;
    hash = hash.wrapping_mul(1664525).wrapping_add(1013904223);
    hash
}

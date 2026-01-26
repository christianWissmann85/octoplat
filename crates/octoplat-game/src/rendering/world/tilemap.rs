//! Tilemap rendering

use macroquad::prelude::*;
use std::collections::HashSet;

use octoplat_core::level::{TileMap, TileType};
use octoplat_core::procgen::BiomeTheme;

use crate::compat::ToMqColor;
use crate::rendering::tile_textures::{draw_platform_texture, draw_spike_texture};

/// Draw the tilemap
pub fn draw_tilemap(tilemap: &TileMap, destroyed_blocks: &HashSet<(usize, usize)>) {
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let tile = tilemap.get(x, y);

            // Skip destroyed breakable blocks
            if tile == TileType::Breakable && destroyed_blocks.contains(&(x, y)) {
                continue;
            }

            if tile != TileType::Empty {
                let px = x as f32 * tilemap.tile_size;
                let py = y as f32 * tilemap.tile_size;
                let size = tilemap.tile_size;

                match tile {
                    TileType::Spike => {
                        // Draw spike as triangle pointing up
                        let spike_color = Color::new(0.9, 0.2, 0.2, 1.0);
                        draw_triangle(
                            vec2(px + 2.0, py + size),           // Bottom left
                            vec2(px + size - 2.0, py + size),   // Bottom right
                            vec2(px + size / 2.0, py + 4.0),    // Top point
                            spike_color,
                        );
                        // Highlight
                        draw_triangle(
                            vec2(px + size / 2.0 - 4.0, py + size - 4.0),
                            vec2(px + size / 2.0, py + 8.0),
                            vec2(px + size / 2.0 + 2.0, py + size - 6.0),
                            Color::new(1.0, 0.5, 0.5, 0.6),
                        );
                    }
                    TileType::OneWay => {
                        // Draw one-way platform as thin bar at top with arrow hints
                        let platform_color = Color::new(0.4, 0.7, 0.5, 0.9);
                        // Main platform surface (just the top portion)
                        draw_rectangle(px, py, size, 8.0, platform_color);
                        // Dashed/striped pattern to indicate pass-through
                        for i in 0..3 {
                            let stripe_x = px + 4.0 + (i as f32) * 10.0;
                            draw_rectangle(stripe_x, py + 10.0, 6.0, 3.0, Color::new(0.4, 0.7, 0.5, 0.4));
                        }
                    }
                    TileType::BouncePad => {
                        // Draw bounce pad as spring/mushroom shape
                        let spring_color = Color::new(0.9, 0.3, 0.4, 1.0);
                        let cap_color = Color::new(1.0, 0.5, 0.5, 1.0);

                        // Spring base
                        draw_rectangle(px + 4.0, py + size - 8.0, size - 8.0, 8.0, spring_color);

                        // Spring coil lines
                        for i in 0..3 {
                            let coil_y = py + size - 10.0 - (i as f32) * 6.0;
                            draw_rectangle(px + 6.0, coil_y, size - 12.0, 2.0, Color::new(0.7, 0.2, 0.3, 1.0));
                        }

                        // Bouncy cap on top
                        draw_ellipse(
                            px + size / 2.0,
                            py + size - 22.0,
                            (size - 4.0) / 2.0,
                            6.0,
                            0.0,
                            cap_color,
                        );
                    }
                    TileType::Breakable => {
                        // Draw breakable block with cracked appearance
                        let block_color = Color::new(0.6, 0.5, 0.35, 1.0);
                        let crack_color = Color::new(0.3, 0.25, 0.15, 1.0);

                        // Base block
                        draw_rectangle(px, py, size, size, block_color);

                        // Crack lines (X pattern to indicate fragility)
                        draw_line(
                            px + 4.0, py + 4.0,
                            px + size / 2.0, py + size / 2.0,
                            2.0, crack_color,
                        );
                        draw_line(
                            px + size - 4.0, py + 4.0,
                            px + size / 2.0, py + size / 2.0,
                            2.0, crack_color,
                        );
                        draw_line(
                            px + size / 2.0, py + size / 2.0,
                            px + 4.0, py + size - 4.0,
                            2.0, crack_color,
                        );
                        draw_line(
                            px + size / 2.0, py + size / 2.0,
                            px + size - 4.0, py + size - 4.0,
                            2.0, crack_color,
                        );

                        // Border
                        draw_rectangle_lines(px, py, size, size, 2.0, crack_color);
                    }
                    _ => {
                        // Draw filled rectangle for other tiles
                        draw_rectangle(px, py, size, size, tile.color().to_mq_color());
                        // Draw border for depth
                        draw_rectangle_lines(px, py, size, size, 2.0, Color::new(0.2, 0.3, 0.4, 0.5));
                    }
                }
            }
        }
    }
}

/// Draw the tilemap with biome theme colors and geometry style
///
/// If a tile texture is provided, it will be applied as an overlay on solid blocks
/// and platforms using multiply blending.
/// If a spike texture is provided, spikes will be rendered with that texture.
pub fn draw_tilemap_themed(
    tilemap: &TileMap,
    destroyed_blocks: &HashSet<(usize, usize)>,
    theme: &BiomeTheme,
    time: f32,
    tile_texture: Option<&Texture2D>,
    spike_texture: Option<&Texture2D>,
) {
    use crate::rendering::autotile;
    use crate::rendering::geometry;

    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let tile = tilemap.get(x, y);

            // Skip destroyed breakable blocks
            if tile == TileType::Breakable && destroyed_blocks.contains(&(x, y)) {
                continue;
            }

            if tile != TileType::Empty {
                let px = x as f32 * tilemap.tile_size;
                let py = y as f32 * tilemap.tile_size;
                let size = tilemap.tile_size;

                match tile {
                    TileType::Spike => {
                        let core_color = theme.hazard_color;
                        let spike_color = core_color.to_mq_color();

                        // Use texture if available, otherwise procedural
                        if let Some(tex) = spike_texture {
                            draw_spike_texture(tex, px, py, size, spike_color);
                        } else {
                            // Procedural spike (triangle pointing up)
                            draw_triangle(
                                vec2(px + 2.0, py + size),
                                vec2(px + size - 2.0, py + size),
                                vec2(px + size / 2.0, py + 4.0),
                                spike_color,
                            );
                            // Highlight
                            let highlight = Color::new(
                                (core_color.r * 1.3).min(1.0),
                                (core_color.g * 1.3).min(1.0),
                                (core_color.b * 1.3).min(1.0),
                                0.6,
                            );
                            draw_triangle(
                                vec2(px + size / 2.0 - 4.0, py + size - 4.0),
                                vec2(px + size / 2.0, py + 8.0),
                                vec2(px + size / 2.0 + 2.0, py + size - 6.0),
                                highlight,
                            );
                        }
                    }
                    TileType::OneWay => {
                        // Draw one-way platform with biome platform color
                        let core_platform_color = theme.platform_color;
                        let platform_color = core_platform_color.to_mq_color();
                        draw_rectangle(px, py, size, 8.0, platform_color);
                        // Border using theme helper
                        let border_color = theme.platform_border_color().to_mq_color();
                        draw_rectangle_lines(px, py, size, 8.0, 1.0, border_color);

                        // Apply texture overlay to platform (subtle, 40% opacity)
                        if let Some(texture) = tile_texture {
                            draw_platform_texture(texture, px, py, size, 8.0, platform_color, 0.4);
                        }

                        // Biome-specific platform edge decoration
                        geometry::draw_platform_edge(px, py, size, theme.geometry_style, theme.accent_color.to_mq_color(), time);

                        // Dashed pattern
                        let stripe_color = Color::new(
                            core_platform_color.r,
                            core_platform_color.g,
                            core_platform_color.b,
                            0.4,
                        );
                        for i in 0..3 {
                            let stripe_x = px + 4.0 + (i as f32) * 10.0;
                            draw_rectangle(stripe_x, py + 10.0, 6.0, 3.0, stripe_color);
                        }
                    }
                    TileType::BouncePad => {
                        // Draw bounce pad with accent color
                        let core_spring_color = theme.accent_color;
                        let spring_color = core_spring_color.to_mq_color();
                        let cap_color = Color::new(
                            (core_spring_color.r * 1.2).min(1.0),
                            (core_spring_color.g * 1.2).min(1.0),
                            (core_spring_color.b * 1.2).min(1.0),
                            1.0,
                        );
                        draw_rectangle(px + 4.0, py + size - 8.0, size - 8.0, 8.0, spring_color);
                        for i in 0..3 {
                            let coil_y = py + size - 10.0 - (i as f32) * 6.0;
                            draw_rectangle(
                                px + 6.0,
                                coil_y,
                                size - 12.0,
                                2.0,
                                Color::new(core_spring_color.r * 0.7, core_spring_color.g * 0.7, core_spring_color.b * 0.7, 1.0),
                            );
                        }
                        draw_ellipse(px + size / 2.0, py + size - 22.0, (size - 4.0) / 2.0, 6.0, 0.0, cap_color);
                    }
                    TileType::Breakable => {
                        // Draw breakable with highlight and border from theme helpers
                        let block_color = theme.solid_highlight_color().to_mq_color();
                        let crack_color = theme.solid_border_color().to_mq_color();
                        draw_rectangle(px, py, size, size, block_color);
                        draw_line(px + 4.0, py + 4.0, px + size / 2.0, py + size / 2.0, 2.0, crack_color);
                        draw_line(px + size - 4.0, py + 4.0, px + size / 2.0, py + size / 2.0, 2.0, crack_color);
                        draw_line(px + size / 2.0, py + size / 2.0, px + 4.0, py + size - 4.0, 2.0, crack_color);
                        draw_line(px + size / 2.0, py + size / 2.0, px + size - 4.0, py + size - 4.0, 2.0, crack_color);
                        draw_rectangle_lines(px, py, size, size, 2.0, crack_color);
                    }
                    TileType::Solid => {
                        // Use auto-tiling for solid blocks
                        let neighbor_mask = autotile::get_tile_neighbors(tilemap, x, y);
                        let seed = autotile::tile_seed(x, y);
                        autotile::draw_autotile(px, py, size, neighbor_mask, theme, seed, tile_texture);

                        // Biome-specific block decoration on top (for non-interior tiles)
                        if neighbor_mask != autotile::neighbors::INTERIOR {
                            let glow_color = theme.glow_color.map(|c| c.to_mq_color());
                            geometry::draw_block_decoration(px, py, size, theme.geometry_style, theme.solid_color.to_mq_color(), time, glow_color);
                        }
                    }
                    _ => {
                        // Fallback for any other tile types
                        draw_rectangle(px, py, size, size, theme.solid_color.to_mq_color());
                        let core_border_color = theme.solid_border_color();
                        draw_rectangle_lines(px, py, size, size, 2.0, Color::new(
                            core_border_color.r,
                            core_border_color.g,
                            core_border_color.b,
                            0.5,
                        ));
                    }
                }
            }
        }
    }
}

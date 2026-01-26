//! Minimap rendering for the HUD
//!
//! Displays a player-centered minimap showing the surrounding level area.

use std::collections::HashSet;
use macroquad::prelude::*;

use crate::level::LevelEnvironment;
use crate::player::Player;
use octoplat_core::level::{TileMap, TileType};

/// Draw the minimap in the bottom-left corner of the screen
pub fn draw_minimap(
    tilemap: &TileMap,
    player: &Player,
    level_env: &LevelEnvironment,
    minimap_size: f32,
    minimap_scale: f32,
    minimap_opacity: f32,
    time: f32,
    frame_texture: Option<&Texture2D>,
) {
    let minimap_width = minimap_size;
    let minimap_height = minimap_size * 0.75; // 4:3 aspect ratio
    let scale = minimap_scale;
    let opacity = minimap_opacity;

    // Position in bottom-left corner (away from HUD elements)
    let margin = 10.0;
    let minimap_x = margin;
    let minimap_y = screen_height() - minimap_height - margin - 25.0; // Leave room for hint text

    // Draw frame texture behind (if available) or procedural background
    if let Some(frame) = frame_texture {
        draw_minimap_frame_texture(frame, minimap_x, minimap_y, minimap_width, minimap_height, opacity);
    } else {
        // Draw background with oceanic tint
        draw_minimap_background(minimap_x, minimap_y, minimap_width, minimap_height, opacity);
    }

    // Calculate world viewport (centered on player)
    let tiles_visible_x = minimap_width / scale;
    let tiles_visible_y = minimap_height / scale;
    let world_view_width = tiles_visible_x * tilemap.tile_size;
    let world_view_height = tiles_visible_y * tilemap.tile_size;

    // Center viewport on player
    let view_center = player.position;
    let view_left = view_center.x - world_view_width / 2.0;
    let view_top = view_center.y - world_view_height / 2.0;

    // Clamp to level bounds
    let level_bounds = tilemap.bounds();
    let view_left = view_left.max(0.0).min(level_bounds.w - world_view_width);
    let view_top = view_top.max(0.0).min(level_bounds.h - world_view_height);

    // Draw tiles
    draw_minimap_tiles(
        tilemap,
        &level_env.destroyed_blocks,
        minimap_x,
        minimap_y,
        minimap_width,
        minimap_height,
        view_left,
        view_top,
        world_view_width,
        world_view_height,
    );

    // Draw dynamic platforms
    draw_minimap_platforms(
        level_env,
        minimap_x,
        minimap_y,
        minimap_width,
        minimap_height,
        view_left,
        view_top,
        world_view_width,
        world_view_height,
    );

    // Draw markers (gems, checkpoints, exit, enemies)
    draw_minimap_markers(
        level_env,
        minimap_x,
        minimap_y,
        minimap_width,
        minimap_height,
        view_left,
        view_top,
        world_view_width,
        world_view_height,
        time,
    );

    // Draw player dot (pulsing)
    draw_minimap_player(
        player,
        minimap_x,
        minimap_y,
        minimap_width,
        minimap_height,
        view_left,
        view_top,
        world_view_width,
        world_view_height,
        time,
    );

    // Draw border (only if no frame texture)
    if frame_texture.is_none() {
        draw_minimap_border(minimap_x, minimap_y, minimap_width, minimap_height);
    }

    // Draw toggle hint
    draw_minimap_hint(minimap_x, minimap_y + minimap_height + 2.0);
}

/// Draw the minimap background
fn draw_minimap_background(x: f32, y: f32, width: f32, height: f32, opacity: f32) {
    // Dark oceanic background
    draw_rectangle(
        x,
        y,
        width,
        height,
        Color::new(0.05, 0.12, 0.18, opacity),
    );
}

/// Draw the minimap with a textured frame
fn draw_minimap_frame_texture(
    texture: &Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    opacity: f32,
) {
    // The frame texture should be slightly larger than the map area to include decorative border
    let frame_padding = 8.0;
    let frame_x = x - frame_padding;
    let frame_y = y - frame_padding;
    let frame_w = width + frame_padding * 2.0;
    let frame_h = height + frame_padding * 2.0;

    // Draw frame texture with opacity
    draw_texture_ex(
        texture,
        frame_x,
        frame_y,
        Color::new(1.0, 1.0, 1.0, opacity),
        DrawTextureParams {
            dest_size: Some(vec2(frame_w, frame_h)),
            ..Default::default()
        },
    );

    // Draw dark background inside the frame for map content
    draw_rectangle(
        x,
        y,
        width,
        height,
        Color::new(0.03, 0.08, 0.12, opacity * 0.9),
    );
}

/// Draw the tilemap on the minimap
#[allow(clippy::too_many_arguments)]
fn draw_minimap_tiles(
    tilemap: &TileMap,
    destroyed_blocks: &HashSet<(usize, usize)>,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    view_left: f32,
    view_top: f32,
    view_width: f32,
    view_height: f32,
) {
    let tile_size = tilemap.tile_size;

    // Calculate which tiles are visible
    let start_tile_x = (view_left / tile_size).floor() as i32;
    let start_tile_y = (view_top / tile_size).floor() as i32;
    let end_tile_x = ((view_left + view_width) / tile_size).ceil() as i32;
    let end_tile_y = ((view_top + view_height) / tile_size).ceil() as i32;

    for ty in start_tile_y..=end_tile_y {
        for tx in start_tile_x..=end_tile_x {
            if tx < 0 || ty < 0 {
                continue;
            }
            let ux = tx as usize;
            let uy = ty as usize;
            if ux >= tilemap.width || uy >= tilemap.height {
                continue;
            }

            let tile = tilemap.get(ux, uy);
            if tile == TileType::Empty {
                continue;
            }

            // Skip destroyed breakable blocks
            if tile == TileType::Breakable && destroyed_blocks.contains(&(ux, uy)) {
                continue;
            }

            // Convert world position to minimap position
            let world_x = tx as f32 * tile_size;
            let world_y = ty as f32 * tile_size;
            let (mini_x, mini_y) = world_to_minimap(
                world_x,
                world_y,
                map_x,
                map_y,
                map_width,
                map_height,
                view_left,
                view_top,
                view_width,
                view_height,
            );

            // Calculate tile size on minimap
            let mini_tile_w = (tile_size / view_width) * map_width;
            let mini_tile_h = (tile_size / view_height) * map_height;

            // Get color based on tile type (slightly muted for minimap)
            let color = get_minimap_tile_color(tile);

            draw_rectangle(mini_x, mini_y, mini_tile_w, mini_tile_h, color);
        }
    }
}

/// Get muted color for tile on minimap
fn get_minimap_tile_color(tile: TileType) -> Color {
    match tile {
        TileType::Empty => BLANK,
        TileType::Solid => Color::new(0.25, 0.4, 0.5, 0.9),
        TileType::Platform => Color::new(0.35, 0.5, 0.45, 0.9),
        TileType::Spike => Color::new(0.7, 0.25, 0.25, 0.9),
        TileType::OneWay => Color::new(0.4, 0.55, 0.5, 0.6),
        TileType::BouncePad => Color::new(0.8, 0.35, 0.45, 0.9),
        TileType::Breakable => Color::new(0.5, 0.4, 0.25, 0.9),
    }
}

/// Draw dynamic platforms on the minimap
#[allow(clippy::too_many_arguments)]
fn draw_minimap_platforms(
    level_env: &LevelEnvironment,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    view_left: f32,
    view_top: f32,
    view_width: f32,
    view_height: f32,
) {
    // Moving platforms
    for platform in level_env.moving_platforms.values() {
        let pos = platform.position;
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x - platform.size.x / 2.0,
            pos.y - platform.size.y / 2.0,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        let mini_w = (platform.size.x / view_width) * map_width;
        let mini_h = (platform.size.y / view_height) * map_height;

        draw_rectangle(mini_x, mini_y, mini_w.max(2.0), mini_h.max(1.0), Color::new(0.5, 0.6, 0.5, 0.9));
    }

    // Crumbling platforms (only if solid)
    for platform in level_env.crumbling_platforms.values() {
        if !platform.is_solid() {
            continue;
        }

        let pos = platform.position;
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x - platform.size.x / 2.0,
            pos.y - platform.size.y / 2.0,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        let mini_w = (platform.size.x / view_width) * map_width;
        let mini_h = (platform.size.y / view_height) * map_height;

        draw_rectangle(mini_x, mini_y, mini_w.max(2.0), mini_h.max(1.0), Color::new(0.6, 0.5, 0.4, 0.8));
    }
}

/// Draw level markers on the minimap
#[allow(clippy::too_many_arguments)]
fn draw_minimap_markers(
    level_env: &LevelEnvironment,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    view_left: f32,
    view_top: f32,
    view_width: f32,
    view_height: f32,
    time: f32,
) {
    // Checkpoints (green markers)
    for &pos in &level_env.checkpoint_positions {
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x,
            pos.y,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        draw_circle(mini_x, mini_y, 2.5, Color::new(0.3, 0.8, 0.4, 0.9));
    }

    // Exit (gold diamond, slightly animated)
    if let Some(exit_pos) = level_env.exit_position {
        if is_in_view(exit_pos, view_left, view_top, view_width, view_height) {
            let (mini_x, mini_y) = world_to_minimap(
                exit_pos.x,
                exit_pos.y,
                map_x,
                map_y,
                map_width,
                map_height,
                view_left,
                view_top,
                view_width,
                view_height,
            );

            let pulse = 1.0 + (time * 3.0).sin() * 0.2;
            draw_poly(mini_x, mini_y, 4, 4.0 * pulse, 45.0, Color::new(1.0, 0.85, 0.3, 1.0));
        }
    }

    // Uncollected gems (yellow dots)
    for gem in level_env.gems.values() {
        if gem.collected {
            continue;
        }

        let pos = gem.position;
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x,
            pos.y,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        draw_circle(mini_x, mini_y, 2.0, Color::new(1.0, 0.9, 0.3, 0.9));
    }

    // Enemies (red dots)
    for crab in level_env.crabs.values() {
        let pos = crab.position;
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x,
            pos.y,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        draw_circle(mini_x, mini_y, 2.5, Color::new(0.9, 0.3, 0.3, 0.9));
    }

    for puffer in level_env.pufferfish.values() {
        let pos = puffer.position;
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x,
            pos.y,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        draw_circle(mini_x, mini_y, 2.5, Color::new(0.85, 0.4, 0.5, 0.9));
    }

    // Grapple points (small cyan markers)
    for &pos in &level_env.grapple_points {
        if !is_in_view(pos, view_left, view_top, view_width, view_height) {
            continue;
        }

        let (mini_x, mini_y) = world_to_minimap(
            pos.x,
            pos.y,
            map_x,
            map_y,
            map_width,
            map_height,
            view_left,
            view_top,
            view_width,
            view_height,
        );

        draw_circle(mini_x, mini_y, 1.5, Color::new(0.4, 0.8, 0.9, 0.7));
    }
}

/// Draw the player on the minimap
#[allow(clippy::too_many_arguments)]
fn draw_minimap_player(
    player: &Player,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    view_left: f32,
    view_top: f32,
    view_width: f32,
    view_height: f32,
    time: f32,
) {
    let pos = player.position;

    let (mini_x, mini_y) = world_to_minimap(
        pos.x,
        pos.y,
        map_x,
        map_y,
        map_width,
        map_height,
        view_left,
        view_top,
        view_width,
        view_height,
    );

    // Clamp player dot to minimap bounds
    let mini_x = mini_x.max(map_x + 3.0).min(map_x + map_width - 3.0);
    let mini_y = mini_y.max(map_y + 3.0).min(map_y + map_height - 3.0);

    // Pulsing effect
    let pulse = 1.0 + (time * 4.0).sin() * 0.25;
    let radius = 4.0 * pulse;

    // Outer glow
    draw_circle(mini_x, mini_y, radius + 1.5, Color::new(0.3, 0.8, 1.0, 0.4));

    // Inner bright dot
    draw_circle(mini_x, mini_y, radius, Color::new(0.5, 0.95, 1.0, 1.0));

    // Center highlight
    draw_circle(mini_x, mini_y, radius * 0.5, Color::new(1.0, 1.0, 1.0, 0.8));
}

/// Draw the minimap border
fn draw_minimap_border(x: f32, y: f32, width: f32, height: f32) {
    // Outer border
    draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.3, 0.5, 0.6, 0.9));

    // Inner subtle border
    draw_rectangle_lines(
        x + 1.0,
        y + 1.0,
        width - 2.0,
        height - 2.0,
        1.0,
        Color::new(0.2, 0.35, 0.45, 0.5),
    );
}

/// Draw the toggle hint below the minimap
fn draw_minimap_hint(x: f32, y: f32) {
    draw_text(
        "M:Toggle +/-:Zoom",
        x,
        y + 10.0,
        11.0,
        Color::new(0.5, 0.6, 0.7, 0.5),
    );
}

/// Convert world coordinates to minimap coordinates
#[allow(clippy::too_many_arguments)]
fn world_to_minimap(
    world_x: f32,
    world_y: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    view_left: f32,
    view_top: f32,
    view_width: f32,
    view_height: f32,
) -> (f32, f32) {
    let rel_x = (world_x - view_left) / view_width;
    let rel_y = (world_y - view_top) / view_height;

    let mini_x = map_x + rel_x * map_width;
    let mini_y = map_y + rel_y * map_height;

    (mini_x, mini_y)
}

/// Check if a position is within the current view
fn is_in_view(pos: Vec2, view_left: f32, view_top: f32, view_width: f32, view_height: f32) -> bool {
    pos.x >= view_left - 32.0
        && pos.x <= view_left + view_width + 32.0
        && pos.y >= view_top - 32.0
        && pos.y <= view_top + view_height + 32.0
}

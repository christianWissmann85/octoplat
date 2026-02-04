//! Freeform layout implementation
//!
//! Creates organic, snake-like level layouts where segments are connected
//! naturally without rectangular padding. Empty space is open (background)
//! rather than solid walls, creating a more open, less claustrophobic feel.

use super::corridors::{
    carve_horizontal_corridor, carve_vertical_corridor, find_entry_row, find_exit_row,
    find_vertical_entry_col, find_vertical_exit_col, punch_through_wall,
};
use super::placement::ensure_spawn_exit;
use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::{LayoutStrategy, LinkDirection, LinkedLevel, SegmentLinkerConfig};
use octoplat_core::Rng;

/// Background tile for open/empty space (not solid walls)
const BACKGROUND_TILE: char = ' ';

/// Freeform layout: Organic snake-like arrangement with open background
pub fn link_freeform(
    parsed: &mut [ParsedSegment],
    config: &SegmentLinkerConfig,
) -> LinkedLevel {
    let segment_count = parsed.len();
    let mut rng = Rng::new(config.seed);

    // Process segments - strip spawn/exit from middle segments
    for (i, seg) in parsed.iter_mut().enumerate() {
        if i == 0 {
            seg.strip_exit();
        } else if i == segment_count - 1 {
            seg.strip_spawn();
        } else {
            seg.strip_spawn();
            seg.strip_exit();
        }
    }

    // Build placement by walking through segments, choosing direction at each step
    // Directions: 0=right, 1=down, 2=left, 3=up
    let mut placements: Vec<SegmentPlacement> = Vec::new();
    let mut connections: Vec<(usize, usize, LinkDirection)> = Vec::new(); // (from_idx, to_idx, direction)

    // Track occupied regions to avoid overlap
    let mut occupied: Vec<(i32, i32, i32, i32)> = Vec::new(); // (min_x, min_y, max_x, max_y)

    // Start first segment at origin (we'll normalize later)
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;

    placements.push(SegmentPlacement {
        segment_idx: 0,
        x: 0, // Will be adjusted later
        y: 0,
    });
    occupied.push((0, 0, parsed[0].width as i32, parsed[0].height as i32));

    // Place remaining segments
    for i in 1..segment_count {
        let prev_seg = &parsed[i - 1];
        let curr_seg = &parsed[i];

        // Try directions in random order, favoring vertical movement for snake-like patterns
        // 55% vertical (down/up), 45% horizontal (right/left)
        let directions = if rng.next_float() < 0.55 {
            // Prefer vertical - creates more interesting snake patterns
            if rng.next_float() < 0.6 {
                [1, 3, 0, 2] // down, up, right, left (down preferred)
            } else {
                [3, 1, 0, 2] // up, down, right, left (up preferred)
            }
        } else {
            // Horizontal movement
            if rng.next_float() < 0.5 {
                [0, 1, 2, 3] // right, down, left, up
            } else {
                [2, 1, 0, 3] // left, down, right, up
            }
        };

        let mut placed = false;
        for &dir in &directions {
            let (new_x, new_y, link_dir) = match dir {
                0 => { // Right
                    let x = current_x + prev_seg.width as i32 + config.corridor_width as i32;
                    let y = current_y + (prev_seg.height as i32 - curr_seg.height as i32) / 2;
                    (x, y, LinkDirection::Right)
                }
                1 => { // Down
                    let x = current_x + (prev_seg.width as i32 - curr_seg.width as i32) / 2;
                    let y = current_y + prev_seg.height as i32 + config.corridor_height as i32;
                    (x, y, LinkDirection::Down)
                }
                2 => { // Left
                    let x = current_x - curr_seg.width as i32 - config.corridor_width as i32;
                    let y = current_y + (prev_seg.height as i32 - curr_seg.height as i32) / 2;
                    (x, y, LinkDirection::Left)
                }
                3 => { // Up
                    let x = current_x + (prev_seg.width as i32 - curr_seg.width as i32) / 2;
                    let y = current_y - curr_seg.height as i32 - config.corridor_height as i32;
                    (x, y, LinkDirection::Up)
                }
                _ => unreachable!(),
            };

            // Check for overlap with existing segments (with small margin)
            let margin = 2i32;
            let new_bounds = (
                new_x - margin,
                new_y - margin,
                new_x + curr_seg.width as i32 + margin,
                new_y + curr_seg.height as i32 + margin,
            );

            let overlaps = occupied.iter().any(|&(ox1, oy1, ox2, oy2)| {
                !(new_bounds.2 <= ox1 || new_bounds.0 >= ox2 ||
                  new_bounds.3 <= oy1 || new_bounds.1 >= oy2)
            });

            if !overlaps {
                placements.push(SegmentPlacement {
                    segment_idx: i,
                    x: new_x as usize, // Will be normalized
                    y: new_y as usize,
                });
                occupied.push((
                    new_x,
                    new_y,
                    new_x + curr_seg.width as i32,
                    new_y + curr_seg.height as i32,
                ));
                connections.push((i - 1, i, link_dir));
                current_x = new_x;
                current_y = new_y;
                placed = true;
                break;
            }
        }

        // Fallback: force placement to the right if all directions blocked
        if !placed {
            let new_x = current_x + prev_seg.width as i32 + config.corridor_width as i32;
            let new_y = current_y;
            placements.push(SegmentPlacement {
                segment_idx: i,
                x: new_x as usize,
                y: new_y as usize,
            });
            occupied.push((
                new_x,
                new_y,
                new_x + curr_seg.width as i32,
                new_y + curr_seg.height as i32,
            ));
            connections.push((i - 1, i, LinkDirection::Right));
            current_x = new_x;
            current_y = new_y;
        }
    }

    // Normalize coordinates to positive values
    let min_x = occupied.iter().map(|b| b.0).min().unwrap_or(0);
    let min_y = occupied.iter().map(|b| b.1).min().unwrap_or(0);

    // Adjust all placements
    for (idx, placement) in placements.iter_mut().enumerate() {
        let orig_x = occupied[idx].0;
        let orig_y = occupied[idx].1;
        placement.x = (orig_x - min_x) as usize;
        placement.y = (orig_y - min_y) as usize;
    }

    // Calculate total dimensions
    let total_width = occupied.iter()
        .map(|b| (b.2 - min_x) as usize)
        .max()
        .unwrap_or(0);
    let total_height = occupied.iter()
        .map(|b| (b.3 - min_y) as usize)
        .max()
        .unwrap_or(0);

    // Create combined tilemap with BACKGROUND (not walls!)
    let mut combined = vec![vec![BACKGROUND_TILE; total_width]; total_height];
    let segment_names: Vec<String> = parsed.iter().map(|s| s.name.clone()).collect();

    // Place all segment tiles
    for placement in &placements {
        let seg = &parsed[placement.segment_idx];
        for (sy, row) in seg.tiles.iter().enumerate() {
            let ty = placement.y + sy;
            if ty >= total_height {
                continue;
            }
            for (sx, &ch) in row.iter().enumerate() {
                let tx = placement.x + sx;
                if tx < total_width {
                    combined[ty][tx] = ch;
                }
            }
        }
    }

    // Connect segments based on recorded connections
    for &(from_idx, to_idx, direction) in &connections {
        let from_seg = &parsed[from_idx];
        let to_seg = &parsed[to_idx];
        let from_placement = &placements[from_idx];
        let to_placement = &placements[to_idx];

        match direction {
            LinkDirection::Right => {
                let exit_y = find_exit_row(from_seg, from_placement.y, total_height);
                let entry_y = find_entry_row(to_seg, to_placement.y, total_height);

                punch_through_wall(
                    &mut combined,
                    from_placement.x + from_seg.width - 1,
                    exit_y,
                    LinkDirection::Right,
                    config.corridor_height,
                );
                punch_through_wall(
                    &mut combined,
                    to_placement.x,
                    entry_y,
                    LinkDirection::Left,
                    config.corridor_height,
                );

                let corridor_start_x = from_placement.x + from_seg.width;
                let corridor_len = to_placement.x.saturating_sub(corridor_start_x);
                if corridor_len > 0 {
                    carve_horizontal_corridor(
                        &mut combined,
                        corridor_start_x,
                        exit_y,
                        corridor_len,
                        entry_y,
                        config.corridor_height,
                    );
                }
            }
            LinkDirection::Left => {
                let exit_y = find_entry_row(from_seg, from_placement.y, total_height);
                let entry_y = find_exit_row(to_seg, to_placement.y, total_height);

                punch_through_wall(
                    &mut combined,
                    from_placement.x,
                    exit_y,
                    LinkDirection::Left,
                    config.corridor_height,
                );
                punch_through_wall(
                    &mut combined,
                    to_placement.x + to_seg.width - 1,
                    entry_y,
                    LinkDirection::Right,
                    config.corridor_height,
                );

                let corridor_start_x = to_placement.x + to_seg.width;
                let corridor_len = from_placement.x.saturating_sub(corridor_start_x);
                if corridor_len > 0 {
                    carve_horizontal_corridor(
                        &mut combined,
                        corridor_start_x,
                        entry_y,
                        corridor_len,
                        exit_y,
                        config.corridor_height,
                    );
                }
            }
            LinkDirection::Down => {
                let exit_x = find_vertical_exit_col(from_seg, from_placement.x, total_width);
                let entry_x = find_vertical_entry_col(to_seg, to_placement.x, total_width);

                punch_through_wall(
                    &mut combined,
                    exit_x,
                    from_placement.y + from_seg.height - 1,
                    LinkDirection::Down,
                    config.corridor_height,
                );
                punch_through_wall(
                    &mut combined,
                    entry_x,
                    to_placement.y,
                    LinkDirection::Up,
                    config.corridor_height,
                );

                let corridor_start_y = from_placement.y + from_seg.height;
                let corridor_len = to_placement.y.saturating_sub(corridor_start_y);
                if corridor_len > 0 {
                    carve_vertical_corridor(
                        &mut combined,
                        exit_x,
                        corridor_start_y,
                        corridor_len,
                        to_placement.y,
                        entry_x,
                    );
                }
            }
            LinkDirection::Up => {
                let exit_x = find_vertical_entry_col(from_seg, from_placement.x, total_width);
                let entry_x = find_vertical_exit_col(to_seg, to_placement.x, total_width);

                punch_through_wall(
                    &mut combined,
                    exit_x,
                    from_placement.y,
                    LinkDirection::Up,
                    config.corridor_height,
                );
                punch_through_wall(
                    &mut combined,
                    entry_x,
                    to_placement.y + to_seg.height - 1,
                    LinkDirection::Down,
                    config.corridor_height,
                );

                let corridor_start_y = to_placement.y + to_seg.height;
                let corridor_len = from_placement.y.saturating_sub(corridor_start_y);
                if corridor_len > 0 {
                    carve_vertical_corridor(
                        &mut combined,
                        entry_x,
                        corridor_start_y,
                        corridor_len,
                        from_placement.y,
                        exit_x,
                    );
                }
            }
        }
    }

    // Ensure spawn and exit
    ensure_spawn_exit(&mut combined, parsed, total_height, &placements);

    // Convert to string
    let tilemap = combined
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    LinkedLevel {
        tilemap,
        width: total_width,
        height: total_height,
        segment_names,
        success: true,
        layout: LayoutStrategy::Freeform,
    }
}

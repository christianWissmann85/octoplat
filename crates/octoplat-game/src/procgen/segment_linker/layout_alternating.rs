//! Alternating (zig-zag) layout implementation

use super::corridors::{
    carve_horizontal_corridor, carve_vertical_corridor, find_entry_row, find_exit_row,
    find_vertical_entry_col, find_vertical_exit_col, punch_through_wall,
};
use super::placement::ensure_spawn_exit;
use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::{LayoutStrategy, LinkDirection, LinkedLevel, SegmentLinkerConfig};

/// Alternating layout: zig-zag pattern (right, up, right, up, ...)
pub fn link_alternating(
    parsed: &mut [ParsedSegment],
    config: &SegmentLinkerConfig,
) -> LinkedLevel {
    let segment_count = parsed.len();

    // Process segments
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

    // Calculate dimensions for zig-zag: alternating horizontal and vertical
    let mut total_width = 0;
    let mut total_height = 0;
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let mut current_x = 0;
    let mut current_y = 0;

    for (i, seg) in parsed.iter().enumerate() {
        positions.push((current_x, current_y));

        if i % 2 == 0 {
            // Even indices: move right
            current_x += seg.width + config.corridor_width;
        } else {
            // Odd indices: move up
            current_y += seg.height + config.corridor_height;
        }

        total_width = total_width.max(current_x + seg.width);
        total_height = total_height.max(current_y + seg.height);
    }

    // Create combined tilemap
    let mut combined = vec![vec!['#'; total_width]; total_height];
    let segment_names: Vec<String> = parsed.iter().map(|s| s.name.clone()).collect();

    // Calculate placements with y-inversion (game has y=0 at top)
    let mut placements: Vec<SegmentPlacement> = Vec::new();
    for (i, seg) in parsed.iter().enumerate() {
        let (x, y) = positions[i];
        // Invert Y so segments stack upward
        let inv_y = total_height.saturating_sub(y + seg.height);
        placements.push(SegmentPlacement {
            segment_idx: i,
            x,
            y: inv_y,
        });
    }

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

    // Carve corridors based on alternating directions
    for i in 0..parsed.len() - 1 {
        let seg = &parsed[i];
        let next_seg = &parsed[i + 1];
        let placement = &placements[i];
        let next_placement = &placements[i + 1];

        if i % 2 == 0 {
            // Horizontal connection (right)
            let exit_y = find_exit_row(seg, placement.y, total_height);
            let entry_y = find_entry_row(next_seg, next_placement.y, total_height);

            // Punch through walls FIRST to break into segments
            punch_through_wall(
                &mut combined,
                placement.x + seg.width - 1,
                exit_y,
                LinkDirection::Right,
                config.corridor_height,
            );
            punch_through_wall(
                &mut combined,
                next_placement.x,
                entry_y,
                LinkDirection::Left,
                config.corridor_height,
            );

            // Carve corridor AFTER punching so platforms aren't overwritten
            carve_horizontal_corridor(
                &mut combined,
                placement.x + seg.width,
                exit_y,
                config.corridor_width,
                entry_y,
                config.corridor_height,
            );
        } else {
            // Vertical connection (up)
            let exit_x = find_vertical_exit_col(seg, placement.x, total_width);
            let entry_x = find_vertical_entry_col(next_seg, next_placement.x, total_width);

            // Punch through walls FIRST
            punch_through_wall(
                &mut combined,
                exit_x,
                placement.y,
                LinkDirection::Up,
                config.corridor_height,
            );
            punch_through_wall(
                &mut combined,
                entry_x,
                next_placement.y + next_seg.height - 1,
                LinkDirection::Down,
                config.corridor_height,
            );

            // Carve corridor AFTER punching so platforms aren't overwritten
            carve_vertical_corridor(
                &mut combined,
                exit_x,
                next_placement.y + next_seg.height,
                config.corridor_height,
                placement.y,
                entry_x,
            );
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
        layout: LayoutStrategy::Alternating,
    }
}

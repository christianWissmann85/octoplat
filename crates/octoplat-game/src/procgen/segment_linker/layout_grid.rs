//! Grid layout implementation

use super::corridors::{
    carve_horizontal_corridor, carve_vertical_corridor, find_entry_row, find_exit_row,
    find_vertical_entry_col, find_vertical_exit_col, punch_through_wall,
};
use super::placement::ensure_spawn_exit;
use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::{LayoutStrategy, LinkDirection, LinkedLevel, SegmentLinkerConfig};

/// Grid layout: 2D arrangement
pub fn link_grid(
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

    // Determine grid dimensions (try to make it square-ish)
    let grid_cols = ((segment_count as f32).sqrt().ceil() as usize).max(2);
    let grid_rows = segment_count.div_ceil(grid_cols);

    // Calculate cell sizes
    let max_seg_width = parsed.iter().map(|s| s.width).max().unwrap_or(0);
    let max_seg_height = parsed.iter().map(|s| s.height).max().unwrap_or(0);

    let cell_width = max_seg_width + config.corridor_width;
    let cell_height = max_seg_height + config.corridor_height;

    let total_width = cell_width * grid_cols;
    let total_height = cell_height * grid_rows;

    // Create combined tilemap
    let mut combined = vec![vec!['#'; total_width]; total_height];
    let segment_names: Vec<String> = parsed.iter().map(|s| s.name.clone()).collect();

    // Calculate placements in grid
    let mut placements: Vec<SegmentPlacement> = Vec::new();
    for (i, seg) in parsed.iter().enumerate() {
        let col = i % grid_cols;
        let row = i / grid_cols;

        // Center segment in cell
        let cell_x = col * cell_width;
        let cell_y = row * cell_height;
        let x = cell_x + (cell_width - seg.width) / 2;
        let y = cell_y + (cell_height - seg.height) / 2;

        placements.push(SegmentPlacement {
            segment_idx: i,
            x,
            y,
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

    // Connect adjacent cells in the grid
    for i in 0..segment_count {
        let col = i % grid_cols;
        let row = i / grid_cols;

        // Connect to right neighbor
        if col + 1 < grid_cols && i + 1 < segment_count {
            let seg = &parsed[i];
            let next_seg = &parsed[i + 1];
            let placement = &placements[i];
            let next_placement = &placements[i + 1];

            let exit_y = find_exit_row(seg, placement.y, total_height);
            let entry_y = find_entry_row(next_seg, next_placement.y, total_height);

            // Punch through walls FIRST
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
            let corridor_len = next_placement.x.saturating_sub(placement.x + seg.width);
            carve_horizontal_corridor(
                &mut combined,
                placement.x + seg.width,
                exit_y,
                corridor_len,
                entry_y,
                config.corridor_height,
            );
        }

        // Connect to below neighbor
        let below_idx = i + grid_cols;
        if row + 1 < grid_rows && below_idx < segment_count {
            let seg = &parsed[i];
            let next_seg = &parsed[below_idx];
            let placement = &placements[i];
            let next_placement = &placements[below_idx];

            let exit_x = find_vertical_exit_col(seg, placement.x, total_width);
            let entry_x = find_vertical_entry_col(next_seg, next_placement.x, total_width);

            // Punch through walls FIRST
            punch_through_wall(
                &mut combined,
                exit_x,
                placement.y + seg.height - 1,
                LinkDirection::Down,
                config.corridor_height,
            );
            punch_through_wall(
                &mut combined,
                entry_x,
                next_placement.y,
                LinkDirection::Up,
                config.corridor_height,
            );

            // Carve corridor AFTER punching so platforms aren't overwritten
            let corridor_len = next_placement.y.saturating_sub(placement.y + seg.height);
            carve_vertical_corridor(
                &mut combined,
                exit_x,
                placement.y + seg.height,
                corridor_len,
                next_placement.y,
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
        layout: LayoutStrategy::Grid,
    }
}

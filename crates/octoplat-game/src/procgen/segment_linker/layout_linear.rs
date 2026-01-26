//! Linear (horizontal) layout implementation

use super::corridors::{
    carve_horizontal_corridor, find_entry_row, find_exit_row, punch_through_wall,
};
use super::placement::ensure_spawn_exit;
use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::{LayoutStrategy, LinkDirection, LinkedLevel, SegmentLinkerConfig};

/// Linear (horizontal) layout: [Seg1] -> [Seg2] -> [Seg3]
pub fn link_linear(
    parsed: &mut [ParsedSegment],
    config: &SegmentLinkerConfig,
) -> LinkedLevel {
    let segment_count = parsed.len();

    // Process segments: strip markers from middle segments
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

    // Calculate dimensions
    let max_height = parsed.iter().map(|s| s.height).max().unwrap_or(0);
    let total_width: usize = parsed.iter().map(|s| s.width).sum::<usize>()
        + (parsed.len() - 1) * config.corridor_width;

    // Create combined tilemap
    let mut combined = vec![vec!['#'; total_width]; max_height];
    let segment_names: Vec<String> = parsed.iter().map(|s| s.name.clone()).collect();

    // Calculate segment positions
    let mut placements: Vec<SegmentPlacement> = Vec::new();
    let mut x_offset = 0;
    for (i, seg) in parsed.iter().enumerate() {
        let y_offset = (max_height - seg.height) / 2;
        placements.push(SegmentPlacement {
            segment_idx: i,
            x: x_offset,
            y: y_offset,
        });
        x_offset += seg.width + config.corridor_width;
    }

    // Place all segment tiles
    for placement in &placements {
        let seg = &parsed[placement.segment_idx];
        for (sy, row) in seg.tiles.iter().enumerate() {
            let ty = placement.y + sy;
            if ty >= max_height {
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

    // Carve horizontal corridors between segments
    for i in 0..parsed.len() - 1 {
        let seg = &parsed[i];
        let next_seg = &parsed[i + 1];
        let placement = &placements[i];
        let next_placement = &placements[i + 1];

        let corridor_start_x = placement.x + seg.width;
        let exit_y = find_exit_row(seg, placement.y, max_height);
        let entry_y = find_entry_row(next_seg, next_placement.y, max_height);

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
            corridor_start_x,
            exit_y,
            config.corridor_width,
            entry_y,
            config.corridor_height,
        );
    }

    // Ensure spawn and exit
    ensure_spawn_exit(&mut combined, parsed, max_height, &placements);

    // Convert to string
    let tilemap = combined
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    LinkedLevel {
        tilemap,
        width: total_width,
        height: max_height,
        segment_names,
        success: true,
        layout: LayoutStrategy::Linear,
    }
}

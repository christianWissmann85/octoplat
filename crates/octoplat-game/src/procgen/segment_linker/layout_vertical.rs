//! Vertical layout implementation

use super::corridors::{
    carve_vertical_corridor, find_vertical_entry_col, find_vertical_exit_col, punch_through_wall,
};
use super::placement::ensure_spawn_exit;
use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::{LayoutStrategy, LinkDirection, LinkedLevel, SegmentLinkerConfig};

/// Vertical layout: segments stacked bottom to top
pub fn link_vertical(
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

    // Calculate dimensions
    let max_width = parsed.iter().map(|s| s.width).max().unwrap_or(0);
    let total_height: usize = parsed.iter().map(|s| s.height).sum::<usize>()
        + (parsed.len() - 1) * config.corridor_height;

    // Create combined tilemap
    let mut combined = vec![vec!['#'; max_width]; total_height];
    let segment_names: Vec<String> = parsed.iter().map(|s| s.name.clone()).collect();

    // Calculate segment positions (bottom to top)
    let mut placements: Vec<SegmentPlacement> = Vec::new();
    let mut y_offset = total_height;
    for (i, seg) in parsed.iter().enumerate() {
        y_offset = y_offset.saturating_sub(seg.height);
        let x_offset = (max_width - seg.width) / 2;
        placements.push(SegmentPlacement {
            segment_idx: i,
            x: x_offset,
            y: y_offset,
        });
        y_offset = y_offset.saturating_sub(config.corridor_height);
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
                if tx < max_width {
                    combined[ty][tx] = ch;
                }
            }
        }
    }

    // Carve vertical corridors between segments
    for i in 0..parsed.len() - 1 {
        let seg = &parsed[i];
        let next_seg = &parsed[i + 1];
        let placement = &placements[i];
        let next_placement = &placements[i + 1];

        let exit_x = find_vertical_exit_col(seg, placement.x, max_width);
        let entry_x = find_vertical_entry_col(next_seg, next_placement.x, max_width);

        // Punch through floors/ceilings FIRST
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

        // Carve vertical shaft AFTER punching so platforms aren't overwritten
        carve_vertical_corridor(
            &mut combined,
            exit_x,
            next_placement.y + next_seg.height,
            config.corridor_height,
            placement.y,
            entry_x,
        );
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
        width: max_width,
        height: total_height,
        segment_names,
        success: true,
        layout: LayoutStrategy::Vertical,
    }
}

//! Corridor carving and connection operations

use octoplat_core::constants::PROCGEN;

use super::segment::ParsedSegment;
use super::types::LinkDirection;

/// Find a suitable exit row on the right edge of a segment (prefers mid-height)
/// Prioritizes positions near the vertical center of the segment to ensure
/// corridors connect through the main gameplay area, not the edges.
pub fn find_exit_row(seg: &ParsedSegment, y_offset: usize, max_height: usize) -> usize {
    let mid_y = seg.height / 2;

    // Search DEEP into segment (up to half width) to find actual playable areas
    let max_col_search = seg.width / 2;

    // Search for ANY open space, starting from the middle and moving outward
    // This ensures we connect through the center of the segment where gameplay is
    // Limit search to middle 60% of segment height to avoid edge floors
    let search_range = seg.height * 3 / 10; // 30% above and below middle

    for col_offset in PROCGEN.corridor_column_search_min..max_col_search {
        let right_col = seg.width.saturating_sub(col_offset);
        if right_col == 0 {
            break;
        }

        // Search from middle outward, but stay within the central region
        for offset in 0..search_range {
            for direction in &[1i32, -1i32] {
                let y_signed = mid_y as i32 + (offset as i32 * direction);
                if y_signed < 0 {
                    continue;
                }
                let y = y_signed as usize;
                if y >= seg.height {
                    continue;
                }

                let global_y = y_offset + y;
                if global_y >= max_height {
                    continue;
                }

                let tile = seg.get_tile(right_col, y);

                // Found open space (not a wall) - we'll create platforms as needed
                if tile == ' ' {
                    return global_y;
                }
            }
        }
    }

    // Fallback to middle of segment
    y_offset + mid_y
}

/// Find a suitable entry row on the left edge of a segment (prefers mid-height)
/// Prioritizes positions near the vertical center of the segment to ensure
/// corridors connect through the main gameplay area, not the edges.
pub fn find_entry_row(seg: &ParsedSegment, y_offset: usize, max_height: usize) -> usize {
    let mid_y = seg.height / 2;

    // Search DEEP into segment (up to half width) to find actual playable areas
    let max_col_search = seg.width / 2;

    // Search for ANY open space, starting from the middle and moving outward
    // This ensures we connect through the center of the segment where gameplay is
    // Limit search to middle 60% of segment height to avoid edge floors
    let search_range = seg.height * 3 / 10; // 30% above and below middle

    for left_col in (PROCGEN.corridor_column_search_min - 1)..max_col_search {
        if left_col >= seg.width {
            break;
        }

        // Search from middle outward, but stay within the central region
        for offset in 0..search_range {
            for direction in &[1i32, -1i32] {
                let y_signed = mid_y as i32 + (offset as i32 * direction);
                if y_signed < 0 {
                    continue;
                }
                let y = y_signed as usize;
                if y >= seg.height {
                    continue;
                }

                let global_y = y_offset + y;
                if global_y >= max_height {
                    continue;
                }

                let tile = seg.get_tile(left_col, y);

                // Found open space (not a wall) - we'll create platforms as needed
                if tile == ' ' {
                    return global_y;
                }
            }
        }
    }

    // Fallback to middle of segment
    y_offset + mid_y
}

/// Find a suitable exit column on the bottom edge of a segment
pub fn find_vertical_exit_col(seg: &ParsedSegment, x_offset: usize, max_width: usize) -> usize {
    // Look for open space near bottom
    for row_offset in 1..5 {
        let check_row = seg.height.saturating_sub(row_offset);
        if check_row == 0 {
            break;
        }

        for x in seg.width / 3..seg.width * 2 / 3 {
            let global_x = x_offset + x;
            if global_x >= max_width {
                continue;
            }

            let tile = seg.get_tile(x, check_row);
            if tile == ' ' || tile == 'P' || tile == '>' {
                return global_x;
            }
        }
    }

    x_offset + seg.width / 2
}

/// Find a suitable entry column on the top edge of a segment
pub fn find_vertical_entry_col(seg: &ParsedSegment, x_offset: usize, max_width: usize) -> usize {
    // Look for open space near top
    for row_offset in 1..5 {
        if row_offset >= seg.height {
            break;
        }

        for x in seg.width / 3..seg.width * 2 / 3 {
            let global_x = x_offset + x;
            if global_x >= max_width {
                continue;
            }

            let tile = seg.get_tile(x, row_offset);
            if tile == ' ' || tile == 'P' || tile == '>' {
                return global_x;
            }
        }
    }

    x_offset + seg.width / 2
}

/// Punch through a wall to create passage
/// Punches in BOTH directions (into corridor AND into segment) to ensure connectivity
/// Creates extended floor/platform to ensure walkable path in open arena segments
/// Clears a large vertical range (both above AND below) to handle segments with different heights
pub fn punch_through_wall(
    tilemap: &mut [Vec<char>],
    x: usize,
    y: usize,
    direction: LinkDirection,
    corridor_height: usize,
) {
    let height = tilemap.len();
    let width = if height > 0 { tilemap[0].len() } else { 0 };

    // Clear a much larger vertical range to handle segments with different heights
    // We clear both above AND below the punch point to ensure connectivity
    let clearance_above = corridor_height + 8; // Extra clearance above
    let clearance_below = 4; // Also clear below the punch point

    match direction {
        LinkDirection::Left | LinkDirection::Right => {
            // Horizontal punch: clear vertically
            // Punch DEEP into segment to break through walls and reach interior
            let punch_depth_forward = 8; // Into corridor (increased)
            let punch_depth_back = 10; // Into segment interior (increased significantly)

            // Punch forward (into corridor)
            for dx in 0..punch_depth_forward {
                let punch_x = match direction {
                    LinkDirection::Right => x.saturating_add(dx),
                    LinkDirection::Left => x.saturating_sub(dx),
                    _ => x,
                };

                if punch_x >= width {
                    continue;
                }

                // Clear ABOVE the punch point
                for dy in 0..clearance_above {
                    let punch_y = y.saturating_sub(dy);
                    if punch_y < height {
                        tilemap[punch_y][punch_x] = ' ';
                    }
                }

                // Clear BELOW the punch point (for segments at different heights)
                for dy in 1..=clearance_below {
                    let punch_y = y + dy;
                    if punch_y < height {
                        tilemap[punch_y][punch_x] = ' ';
                    }
                }

                // Ensure floor - create platform for traversal
                let floor_y = y + clearance_below + 1;
                if floor_y < height {
                    tilemap[floor_y][punch_x] = '_';
                }
            }

            // Punch backward (into segment interior) to break through border wall
            // and create a walkable path into the segment
            for dx in 1..=punch_depth_back {
                let punch_x = match direction {
                    LinkDirection::Right => x.saturating_sub(dx),
                    LinkDirection::Left => x.saturating_add(dx),
                    _ => x,
                };

                if punch_x >= width {
                    continue;
                }

                // Clear ABOVE the punch point
                for dy in 0..clearance_above {
                    let punch_y = y.saturating_sub(dy);
                    if punch_y < height {
                        tilemap[punch_y][punch_x] = ' ';
                    }
                }

                // Clear BELOW the punch point
                for dy in 1..=clearance_below {
                    let punch_y = y + dy;
                    if punch_y < height {
                        tilemap[punch_y][punch_x] = ' ';
                    }
                }

                // Create platforms into segment to ensure traversability
                // Add platforms at multiple heights to create a path from segment interior to corridor
                let floor_y = y + 1;
                if floor_y < height {
                    // Add main floor platform
                    if tilemap[floor_y][punch_x] == ' ' {
                        tilemap[floor_y][punch_x] = '_';
                    }
                }

                // Add platforms at intervals going deeper into the segment
                // This creates stepping stones from the segment interior to the corridor
                if dx % 3 == 0 {
                    // Add platform at a higher position (for jumping up)
                    let high_platform_y = y.saturating_sub(2);
                    if high_platform_y < height && tilemap[high_platform_y][punch_x] == ' ' {
                        tilemap[high_platform_y][punch_x] = '_';
                    }
                    // Add platform at a lower position (for falling down)
                    let low_platform_y = y + 3;
                    if low_platform_y < height && tilemap[low_platform_y][punch_x] == ' ' {
                        tilemap[low_platform_y][punch_x] = '_';
                    }
                }
            }
        }
        LinkDirection::Up | LinkDirection::Down => {
            // Vertical punch: clear horizontally
            // Punch DEEP into segment to break through walls and reach interior
            let punch_depth_forward = 8; // Into corridor (increased)
            let punch_depth_back = 8; // Into segment interior (increased)
            let clearance = corridor_height + 4;

            // Punch forward (into corridor)
            for dy in 0..punch_depth_forward {
                let punch_y = match direction {
                    LinkDirection::Down => y.saturating_add(dy),
                    LinkDirection::Up => y.saturating_sub(dy),
                    _ => y,
                };

                if punch_y >= height {
                    continue;
                }

                for dx in 0..clearance {
                    let left_x = x.saturating_sub(clearance / 2) + dx;
                    if left_x < width {
                        tilemap[punch_y][left_x] = ' ';
                    }
                }
            }

            // Punch backward (into segment interior)
            for dy in 1..=punch_depth_back {
                let punch_y = match direction {
                    LinkDirection::Down => y.saturating_sub(dy),
                    LinkDirection::Up => y.saturating_add(dy),
                    _ => y,
                };

                if punch_y >= height {
                    continue;
                }

                for dx in 0..clearance {
                    let left_x = x.saturating_sub(clearance / 2) + dx;
                    if left_x < width {
                        tilemap[punch_y][left_x] = ' ';
                    }
                }
            }
        }
    }
}

/// Carve a horizontal corridor with platforms for traversal (not flat floors)
///
/// When start_y and end_y differ significantly (segments with different heights),
/// the corridor clears the entire vertical range to ensure connectivity.
#[allow(clippy::needless_range_loop)] // Iterating x indices for 2D grid access
pub fn carve_horizontal_corridor(
    tilemap: &mut [Vec<char>],
    start_x: usize,
    start_y: usize,
    corridor_len: usize,
    end_y: usize,
    corridor_height: usize,
) {
    let height = tilemap.len();
    let width = if height > 0 { tilemap[0].len() } else { 0 };

    // Calculate the vertical range that needs clearing
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);
    let height_diff = max_y.saturating_sub(min_y);

    // Ensure we clear enough vertical space - at least corridor_height, but more if there's
    // a significant height difference between segments
    let effective_clearance = corridor_height.max(height_diff + corridor_height);

    // Carve the corridor space (no floor - just open air)
    for x in start_x..start_x + corridor_len {
        if x >= width {
            continue;
        }

        // Interpolate y position for diagonal corridors
        let t = if corridor_len > 1 {
            (x - start_x) as f32 / (corridor_len - 1) as f32
        } else {
            0.0
        };
        let y = (start_y as f32 * (1.0 - t) + end_y as f32 * t) as usize;

        // Carve from min_y to the interpolated position plus clearance
        // This ensures the entire vertical range is passable
        let carve_top = min_y.saturating_sub(corridor_height);
        let carve_bottom = y + 1; // One below the corridor path for floor

        for carve_y in carve_top..=carve_bottom.min(height.saturating_sub(1)) {
            if carve_y < height && tilemap[carve_y][x] != '_' {
                tilemap[carve_y][x] = ' ';
            }
        }

        // Also clear the main corridor path with standard clearance
        for dy in 0..effective_clearance {
            let carve_y = y.saturating_sub(dy);
            if carve_y < height {
                tilemap[carve_y][x] = ' ';
            }
        }
    }

    // Add stepping platforms for traversal
    let platform_interval = 3;
    for (i, x) in (start_x..start_x + corridor_len).enumerate() {
        if x >= width {
            continue;
        }

        let t = if corridor_len > 1 {
            (x - start_x) as f32 / (corridor_len - 1) as f32
        } else {
            0.0
        };
        let base_y = (start_y as f32 * (1.0 - t) + end_y as f32 * t) as usize;

        // Add platforms along the corridor path
        if i % platform_interval == 1 {
            // Place platform at or slightly above the interpolated position
            let platform_y = if (i / platform_interval) % 2 == 0 {
                base_y
            } else {
                base_y.saturating_sub(2)
            };

            if platform_y < height {
                tilemap[platform_y][x] = '_';
                if x + 1 < width {
                    tilemap[platform_y][x + 1] = '_';
                }
            }
        }

        // If there's a significant height difference, add extra platforms
        // to help traverse the vertical distance
        if height_diff > 5 && i % (platform_interval * 2) == 0 {
            // Add intermediate platforms
            let mid_y = (min_y + max_y) / 2;
            if mid_y < height && mid_y != base_y {
                tilemap[mid_y][x] = '_';
            }
        }
    }
}

/// Carve a vertical corridor
#[allow(clippy::needless_range_loop)] // Iterating y indices for 2D grid access
pub fn carve_vertical_corridor(
    tilemap: &mut [Vec<char>],
    start_x: usize,
    start_y: usize,
    corridor_len: usize,
    end_y: usize,
    end_x: usize,
) {
    let height = tilemap.len();
    let width = if height > 0 { tilemap[0].len() } else { 0 };
    let corridor_width = 5; // Wide shaft for climbing

    let min_y = start_y.min(end_y);
    let max_y = (start_y.max(end_y) + corridor_len).min(height);

    // Carve main shaft
    for y in min_y..max_y {
        if y >= height {
            continue;
        }

        // Use interpolated x position
        let t = if max_y > min_y {
            (y - min_y) as f32 / (max_y - min_y) as f32
        } else {
            0.0
        };
        let shaft_x = (start_x as f32 * (1.0 - t) + end_x as f32 * t) as usize;

        for dx in 0..corridor_width {
            let cx = shaft_x.saturating_sub(corridor_width / 2) + dx;
            if cx < width {
                tilemap[y][cx] = ' ';
            }
        }
    }

    // Add platforms for climbing every few tiles
    let platform_interval = 4;
    for y in (min_y..max_y).step_by(platform_interval) {
        if y >= height {
            continue;
        }

        let t = if max_y > min_y {
            (y - min_y) as f32 / (max_y - min_y) as f32
        } else {
            0.0
        };
        let shaft_x = (start_x as f32 * (1.0 - t) + end_x as f32 * t) as usize;

        // Small platform on alternating sides
        let platform_side = if (y / platform_interval) % 2 == 0 { -2i32 } else { 2 };
        let platform_x = (shaft_x as i32 + platform_side) as usize;
        if platform_x > 0 && platform_x < width - 1 {
            tilemap[y][platform_x] = '_';
            if platform_x + 1 < width {
                tilemap[y][platform_x + 1] = '_';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_solid_tilemap(width: usize, height: usize) -> Vec<Vec<char>> {
        vec![vec!['#'; width]; height]
    }

    #[test]
    fn test_punch_through_wall_right_creates_bidirectional_opening() {
        // Create a solid 30x15 tilemap (larger to accommodate increased punch depths and clearance)
        let mut tilemap = make_solid_tilemap(30, 15);

        // Punch through at x=15 (middle), y=8, going RIGHT
        // This should clear tiles both to the right (into corridor) and left (into segment)
        punch_through_wall(&mut tilemap, 15, 8, LinkDirection::Right, 3);

        // Verify tiles to the right of x=15 are cleared (forward punch depth = 8)
        for dx in 0..8 {
            assert_eq!(tilemap[8][15 + dx], ' ', "Should clear tile at x={}", 15 + dx);
        }

        // Verify tiles to the left of x=15 are cleared (backward punch depth = 10)
        for dx in 1..=10 {
            if 15 >= dx {
                assert_eq!(tilemap[8][15 - dx], ' ', "Should clear tile at x={}", 15 - dx);
            }
        }

        // Verify floor/platform was added below the punch area
        // With clearance_below=4, floor is at y + clearance_below + 1 = 8 + 4 + 1 = 13
        assert_eq!(tilemap[13][15], '_', "OneWay platform should be added below punch point");
    }

    #[test]
    fn test_punch_through_wall_left_creates_bidirectional_opening() {
        let mut tilemap = make_solid_tilemap(30, 15);

        // Punch through at x=15, y=8, going LEFT
        punch_through_wall(&mut tilemap, 15, 8, LinkDirection::Left, 3);

        // Verify tiles to the left of x=15 are cleared (forward punch depth = 8)
        for dx in 0..8 {
            if 15 >= dx {
                assert_eq!(tilemap[8][15 - dx], ' ', "Should clear tile at x={}", 15 - dx);
            }
        }

        // Verify tiles to the right of x=15 are cleared (backward punch depth = 10)
        for dx in 1..=10 {
            if 15 + dx < 30 {
                assert_eq!(tilemap[8][15 + dx], ' ', "Should clear tile at x={}", 15 + dx);
            }
        }
    }

    #[test]
    fn test_punch_through_wall_clears_vertical_corridor_height() {
        let mut tilemap = make_solid_tilemap(30, 10);
        let corridor_height = 4;

        punch_through_wall(&mut tilemap, 15, 6, LinkDirection::Right, corridor_height);

        // Should clear corridor_height tiles vertically (above the punch point)
        for dy in 0..corridor_height {
            assert_eq!(tilemap[6 - dy][15], ' ', "Should clear tile at y={}", 6 - dy);
        }
    }

    #[test]
    fn test_punch_through_wall_vertical_down() {
        let mut tilemap = make_solid_tilemap(20, 20);

        // Punch through at x=10, y=10, going DOWN
        punch_through_wall(&mut tilemap, 10, 10, LinkDirection::Down, 3);

        // Verify tiles below y=10 are cleared (forward punch depth = 6)
        for dy in 0..6 {
            if 10 + dy < 20 {
                // Should clear horizontally for corridor width (clearance = 3)
                assert_eq!(tilemap[10 + dy][10], ' ', "Should clear tile at y={}", 10 + dy);
            }
        }

        // Verify tiles above y=10 are cleared (backward punch depth = 6)
        for dy in 1..=6 {
            if 10 >= dy {
                assert_eq!(tilemap[10 - dy][10], ' ', "Should clear tile at y={}", 10 - dy);
            }
        }
    }
}

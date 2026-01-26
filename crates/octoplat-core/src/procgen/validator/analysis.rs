//! Bottleneck detection and structure analysis

use std::collections::HashSet;

use super::types::TilePos;

/// Minimal geometry constraints for bottleneck detection
#[derive(Clone, Debug)]
pub struct GeometryConstraints {
    /// Minimum height for horizontal passages (player needs 2 tiles + margin)
    pub min_passage_height: usize,
    /// Minimum width for vertical passages
    pub min_passage_width: usize,
}

impl Default for GeometryConstraints {
    fn default() -> Self {
        Self {
            min_passage_height: 3,
            min_passage_width: 2,
        }
    }
}

/// Find passage bottlenecks in the level
pub fn find_passage_bottlenecks(
    tiles: &[Vec<char>],
    constraints: &GeometryConstraints,
) -> Vec<(usize, usize, String)> {
    let mut bottlenecks = Vec::new();
    let height = tiles.len();
    if height < 2 {
        return bottlenecks;
    }
    let width = tiles[0].len();
    let min_height = constraints.min_passage_height;
    let min_width = constraints.min_passage_width;

    // Check horizontal passages
    for y in 1..height.saturating_sub(1) {
        for x in 0..width {
            if !is_passable_tile(tiles[y][x]) {
                continue;
            }

            let clearance = measure_vertical_clearance_at(tiles, x, y);

            if clearance > 0 && clearance < min_height {
                bottlenecks.push((
                    x,
                    y,
                    format!(
                        "horizontal passage with {} tile clearance (need {})",
                        clearance, min_height
                    ),
                ));
            }
        }
    }

    // Check vertical passages
    for (y, row) in tiles.iter().enumerate() {
        for x in 1..width.saturating_sub(1) {
            if !is_passable_tile(row[x]) {
                continue;
            }

            let left_solid = x > 0 && !is_passable_tile(row[x - 1]);
            let right_solid = x + 1 < width && !is_passable_tile(row[x + 1]);

            if left_solid && right_solid && min_width > 1 {
                bottlenecks.push((
                    x,
                    y,
                    format!("vertical passage with 1 tile width (need {})", min_width),
                ));
            }
        }
    }

    // Deduplicate nearby bottlenecks
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    bottlenecks.retain(|(x, y, _)| {
        let key = (*x / 3, *y / 3);
        if seen.contains(&key) {
            false
        } else {
            seen.insert(key);
            true
        }
    });

    bottlenecks
}

fn is_passable_tile(ch: char) -> bool {
    !matches!(ch, '#' | 'X')
}

fn measure_vertical_clearance_at(tiles: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut clearance = 0;

    for check_y in (0..=y).rev() {
        if x < tiles[check_y].len() && is_passable_tile(tiles[check_y][x]) {
            clearance += 1;
        } else {
            break;
        }
    }

    clearance
}

/// Find a specific marker in the tilemap
pub fn find_marker(tiles: &[Vec<char>], marker: char) -> Option<TilePos> {
    for (y, row) in tiles.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == marker {
                return Some(TilePos::new(x as i32, y as i32));
            }
        }
    }
    None
}

/// Find all instances of a specific marker
pub fn find_all_markers(tiles: &[Vec<char>], marker: char) -> Vec<TilePos> {
    let mut positions = Vec::new();
    for (y, row) in tiles.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == marker {
                positions.push(TilePos::new(x as i32, y as i32));
            }
        }
    }
    positions
}

/// Find all hazards in the tilemap
pub fn find_hazards(tiles: &[Vec<char>]) -> HashSet<TilePos> {
    let mut hazards = HashSet::new();
    for (y, row) in tiles.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '^' {
                hazards.insert(TilePos::new(x as i32, y as i32));
            }
        }
    }
    hazards
}

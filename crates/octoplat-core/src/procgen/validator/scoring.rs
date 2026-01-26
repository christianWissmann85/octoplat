//! Interest scoring and mechanics counting

use std::collections::HashSet;

use super::types::{MechanicsUsed, TilePos};

/// Calculate the interest score for a level
pub fn calculate_interest_score(
    tiles: &[Vec<char>],
    grapple_points: &[TilePos],
    bounce_pads: &[TilePos],
    hazards: &HashSet<TilePos>,
    path_length: usize,
    mechanics_used: &MechanicsUsed,
) -> f32 {
    let mut score = 0.0;

    // Path length score
    let path_score = (path_length as f32 / 20.0).min(1.0);
    score += path_score * 0.25;

    // Mechanics diversity score
    let mechanics_score = (mechanics_used.count() as f32 / 4.0).min(1.0);
    score += mechanics_score * 0.3;

    // Hazard score
    let hazard_score = if hazards.is_empty() {
        0.0
    } else {
        (hazards.len() as f32 / 10.0).min(1.0)
    };
    score += hazard_score * 0.2;

    // Element density score
    let elements = grapple_points.len() + bounce_pads.len();
    let elements_score = (elements as f32 / 5.0).min(1.0);
    score += elements_score * 0.15;

    // Tile density score (optimal around 30%)
    let total_tiles = tiles.len() * tiles.first().map(|r| r.len()).unwrap_or(0);
    let mut solid_count = 0;
    for row in tiles {
        for &ch in row {
            if ch == '#' || ch == 'X' {
                solid_count += 1;
            }
        }
    }
    let density = solid_count as f32 / total_tiles as f32;
    let density_score = 1.0 - (density - 0.3).abs() * 2.0;
    score += density_score.max(0.0) * 0.1;

    score.clamp(0.0, 1.0)
}

/// Count available mechanics in the level
pub fn count_available_mechanics(
    tiles: &[Vec<char>],
    grapple_points: &[TilePos],
    bounce_pads: &[TilePos],
) -> usize {
    let mut count = 2; // Walking and falling are always available
    let height = tiles.len();

    // Check for jumping (height variation)
    let mut min_ground = i32::MAX;
    let mut max_ground = 0;
    for (y, row) in tiles.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' && y > 0 && x < tiles[y - 1].len() && tiles[y - 1][x] != '#' {
                min_ground = min_ground.min(y as i32);
                max_ground = max_ground.max(y as i32);
            }
        }
    }
    if max_ground - min_ground >= 2 {
        count += 1;
    }

    // Check for wall jumping
    let mut has_walls_to_climb = false;
    for (y, row) in tiles.iter().enumerate() {
        if y < 2 || y >= height {
            continue;
        }
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' {
                let left_clear = x > 0
                    && x - 1 < tiles[y].len()
                    && tiles[y][x - 1] != '#'
                    && x - 1 < tiles[y - 1].len()
                    && tiles[y - 1][x - 1] != '#';
                let right_clear = x + 1 < row.len()
                    && x + 1 < tiles[y - 1].len()
                    && tiles[y][x + 1] != '#'
                    && tiles[y - 1][x + 1] != '#';
                if left_clear || right_clear {
                    has_walls_to_climb = true;
                    break;
                }
            }
        }
        if has_walls_to_climb {
            break;
        }
    }
    if has_walls_to_climb {
        count += 1;
    }

    // Grapple points
    if !grapple_points.is_empty() {
        count += 1;
    }

    // Bounce pads
    if !bounce_pads.is_empty() {
        count += 1;
    }

    count
}

//! Spawn/exit placement and segment selection utilities

use octoplat_core::constants::PROCGEN;
use octoplat_core::procgen::{BiomeId, LevelArchetype, PooledLevel};
use octoplat_core::state::DifficultyPreset;
use octoplat_core::Rng;

use super::segment::{ParsedSegment, SegmentPlacement};
use super::types::LayoutStrategy;

/// Ensure spawn exists in first segment, exit in last segment
/// Places them in interesting gameplay positions (mid-height, away from edges)
/// Falls back to creating a platform if no valid floor is found
pub fn ensure_spawn_exit(
    tilemap: &mut [Vec<char>],
    segments: &[ParsedSegment],
    _max_height: usize,
    placements: &[SegmentPlacement],
) {
    if segments.is_empty() || placements.is_empty() {
        return;
    }

    let height = tilemap.len();
    let width = if height > 0 { tilemap[0].len() } else { 0 };

    // Check for spawn
    let has_spawn = tilemap.iter().any(|row| row.contains(&'P'));
    if !has_spawn {
        let first_seg = &segments[0];
        let placement = &placements[0];

        if let Some((gx, gy)) = find_valid_marker_position(tilemap, first_seg, placement, height, width, true) {
            tilemap[gy][gx] = 'P';
            // Ensure there's a floor below if not already solid
            if gy + 1 < height && tilemap[gy + 1][gx] == ' ' {
                tilemap[gy + 1][gx] = '_';
            }
        } else {
            // Fallback: try to place spawn in center area of first segment
            // Search for a valid position near center
            if let Some((gx, gy)) = find_fallback_position(tilemap, placement, first_seg, height, width) {
                tilemap[gy][gx] = 'P';
                if gy + 1 < height && tilemap[gy + 1][gx] == ' ' {
                    tilemap[gy + 1][gx] = '_';
                }
            }
            // Note: If no fallback found, spawn will be missing and validator will catch it
        }
    }

    // Check for exit
    let has_exit = tilemap.iter().any(|row| row.contains(&'>'));
    if !has_exit {
        let last_idx = segments.len() - 1;
        let last_seg = &segments[last_idx];
        let placement = &placements[last_idx];

        if let Some((gx, gy)) = find_valid_marker_position(tilemap, last_seg, placement, height, width, false) {
            tilemap[gy][gx] = '>';
            // Ensure there's a floor below if not already solid
            if gy + 1 < height && tilemap[gy + 1][gx] == ' ' {
                tilemap[gy + 1][gx] = '_';
            }
        } else {
            // Fallback: try to place exit in center area of last segment
            // Search for a valid position near center
            if let Some((gx, gy)) = find_fallback_position(tilemap, placement, last_seg, height, width) {
                tilemap[gy][gx] = '>';
                if gy + 1 < height && tilemap[gy + 1][gx] == ' ' {
                    tilemap[gy + 1][gx] = '_';
                }
            }
            // Note: If no fallback found, exit will be missing and validator will catch it
        }
    }
}

/// Find a fallback position near center of segment
/// Searches in expanding rings from center until it finds a valid empty space
/// Returns global (x, y) coordinates if found
fn find_fallback_position(
    tilemap: &[Vec<char>],
    placement: &SegmentPlacement,
    seg: &ParsedSegment,
    height: usize,
    width: usize,
) -> Option<(usize, usize)> {
    let center_x = placement.x + seg.width / 2;
    let center_y = placement.y + seg.height / 2;

    // Search in expanding rings from center
    for radius in 0..=((seg.width.max(seg.height) / 2) as i32) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                // Only check edge of current ring (optimization)
                if radius > 0 && dx.abs() != radius && dy.abs() != radius {
                    continue;
                }

                // Check for negative result before converting to usize (prevents underflow)
                let gx_signed = center_x as i32 + dx;
                let gy_signed = center_y as i32 + dy;

                if gx_signed < 0 || gy_signed < 0 {
                    continue;
                }

                let gx = gx_signed as usize;
                let gy = gy_signed as usize;

                if gy >= height || gy + 1 >= height || gx >= width {
                    continue;
                }

                let tile = tilemap[gy][gx];
                let below = tilemap[gy + 1][gx];

                // Valid: empty space (not a wall) with solid floor or empty below (we'll add floor)
                if tile == ' ' && (matches!(below, '#' | '=' | '-') || below == ' ') {
                    return Some((gx, gy));
                }
            }
        }
    }

    None
}

/// Find a valid position for a marker (spawn or exit) within a segment
/// Returns global (x, y) coordinates if found
fn find_valid_marker_position(
    tilemap: &[Vec<char>],
    seg: &ParsedSegment,
    placement: &SegmentPlacement,
    height: usize,
    width: usize,
    prefer_left: bool,
) -> Option<(usize, usize)> {
    // Search the entire segment, not just upper half
    // Prefer areas away from edges for interesting gameplay

    let x_range: Vec<usize> = if prefer_left {
        (3..seg.width.saturating_sub(3)).collect()
    } else {
        (3..seg.width.saturating_sub(3)).rev().collect()
    };

    // First pass: look for existing floors
    for &x in &x_range {
        for y in 2..seg.height.saturating_sub(2) {
            let global_x = placement.x + x;
            let global_y = placement.y + y;

            if global_y >= height || global_y + 1 >= height || global_x >= width {
                continue;
            }

            let tile = tilemap[global_y][global_x];
            let below = tilemap[global_y + 1][global_x];

            // Valid: empty space with solid floor
            if tile == ' ' && matches!(below, '#' | '=' | '-') {
                return Some((global_x, global_y));
            }
        }
    }

    // Second pass: look for any empty space we can add a floor to
    for &x in &x_range {
        for y in 2..seg.height.saturating_sub(2) {
            let global_x = placement.x + x;
            let global_y = placement.y + y;

            if global_y >= height || global_y + 1 >= height || global_x >= width {
                continue;
            }

            let tile = tilemap[global_y][global_x];
            let below = tilemap[global_y + 1][global_x];

            // Can create: empty space with empty below (we'll add a platform)
            if tile == ' ' && below == ' ' {
                return Some((global_x, global_y));
            }
        }
    }

    None
}

/// Select a layout strategy based on progression and difficulty
pub fn select_layout_strategy(
    level_index: u32,
    preset: DifficultyPreset,
    seed: u64,
) -> LayoutStrategy {
    let mut rng = Rng::new(seed.wrapping_add(level_index as u64 * 7919));

    let complexity = match preset {
        DifficultyPreset::Casual => 0.3,
        DifficultyPreset::Standard => 0.6,
        DifficultyPreset::Challenge => 1.0,
    };

    // Asymptotic curve that continues scaling past level 20
    // L0=0.00, L10=0.33, L20=0.50, L50=0.71, L100=0.83
    let progress = 1.0 - 1.0 / (1.0 + level_index as f32 * PROCGEN.difficulty_scale_rate);
    let threshold = complexity * progress;

    let roll = rng.next_float();

    if roll < 0.35 {
        LayoutStrategy::Linear
    } else if roll < 0.55 + threshold * 0.1 {
        LayoutStrategy::Vertical
    } else if roll < 0.75 + threshold * 0.15 {
        LayoutStrategy::Alternating
    } else {
        LayoutStrategy::Grid
    }
}

/// Select segments for a linked level
pub fn select_segments<'a>(
    pool_levels: &[&'a PooledLevel],
    biome: BiomeId,
    segment_count: usize,
    min_tier: u8,
    max_tier: u8,
    seed: u64,
) -> Vec<&'a PooledLevel> {
    let mut rng = Rng::new(seed);
    let mut selected: Vec<&'a PooledLevel> = Vec::new();
    let mut used_archetypes: Vec<LevelArchetype> = Vec::new();

    let candidates: Vec<&&PooledLevel> = pool_levels
        .iter()
        .filter(|l| l.biome == biome && l.difficulty_tier >= min_tier && l.difficulty_tier <= max_tier)
        .collect();

    if candidates.is_empty() {
        return selected;
    }

    for i in 0..segment_count {
        let available: Vec<&&PooledLevel> = candidates
            .iter()
            .filter(|l| !used_archetypes.contains(&l.archetype))
            .copied()
            .collect();

        let pool = if available.is_empty() {
            &candidates
        } else {
            &available
        };

        if pool.is_empty() {
            break;
        }

        let progress = i as f32 / segment_count as f32;
        let target_tier = min_tier as f32 + (max_tier - min_tier) as f32 * progress;

        let mut best_idx = rng.range_usize(0, pool.len());
        let mut best_score = f32::MAX;

        for (idx, level) in pool.iter().enumerate() {
            let tier_diff = (level.difficulty_tier as f32 - target_tier).abs();
            let score = tier_diff + rng.next_float() * 0.5;

            if score < best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        let chosen = pool[best_idx];
        used_archetypes.push(chosen.archetype);
        selected.push(chosen);
    }

    selected
}

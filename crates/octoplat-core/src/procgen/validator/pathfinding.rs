//! BFS pathfinding and reachability algorithms

use std::collections::{HashMap, HashSet, VecDeque};

use crate::constants::PROCGEN;

use super::types::{
    get_tile, is_hazard, is_near_wall, is_solid, is_standable, MechanicsUsed, MoveType,
    MovementCaps, TilePos,
};

/// BFS pathfinding from spawn to exit with mechanic tracking
#[allow(clippy::too_many_arguments)]
pub fn bfs_with_mechanics_disabled(
    caps: &MovementCaps,
    tiles: &[Vec<char>],
    spawn: TilePos,
    exit: TilePos,
    grapple_points: &[TilePos],
    bounce_pads: &[TilePos],
    hazards: &HashSet<TilePos>,
    disable_wall_jump: bool,
    disable_dive: bool,
    disable_jet_boost: bool,
    _disable_basic_jump: bool,
) -> Option<(usize, MechanicsUsed)> {
    let mut visited: HashMap<TilePos, (usize, MechanicsUsed)> = HashMap::new();
    let mut queue: VecDeque<(TilePos, usize, MechanicsUsed)> = VecDeque::new();

    let mut start = spawn;
    for dy in 0..PROCGEN.spawn_search_height {
        let check = TilePos::new(spawn.x, spawn.y + dy);
        // Must be both standable (has ground below) AND not solid (player can occupy)
        // Previously used OR which incorrectly matched empty air tiles with no ground
        if is_standable(tiles, check, hazards) && !is_solid(tiles, check) {
            start = check;
            break;
        }
    }

    let initial_mechanics = MechanicsUsed::default();
    queue.push_back((start, 0, initial_mechanics.clone()));
    visited.insert(start, (0, initial_mechanics));

    while let Some((current, steps, mechanics)) = queue.pop_front() {
        // Check if we've reached the exit
        // Distance 0: exactly at exit
        // Distance 1: adjacent to exit - verify we can actually reach it
        let dist_to_exit = current.manhattan_distance(exit);
        if dist_to_exit == 0 {
            return Some((steps, mechanics));
        }
        if dist_to_exit == 1 {
            // Verify the adjacent exit is actually accessible (not blocked by wall/hazard)
            if can_reach_adjacent_exit(tiles, current, exit, hazards) {
                return Some((steps, mechanics));
            }
        }

        let neighbors = get_reachable_with_types_filtered(
            caps,
            tiles,
            current,
            grapple_points,
            bounce_pads,
            hazards,
            disable_wall_jump,
            disable_dive,
            disable_jet_boost,
        );

        for (next, move_type) in neighbors {
            let new_steps = steps + 1;
            let should_visit = match visited.get(&next) {
                Some((old_steps, _)) => new_steps < *old_steps,
                None => true,
            };

            if should_visit {
                let mut new_mechanics = mechanics.clone();
                match move_type {
                    MoveType::Walk => new_mechanics.walking = true,
                    MoveType::Jump => new_mechanics.jumping = true,
                    MoveType::WallJump => new_mechanics.wall_jumping = true,
                    MoveType::Grapple => new_mechanics.grappling = true,
                    MoveType::Bounce => new_mechanics.bouncing = true,
                    MoveType::Fall => new_mechanics.falling = true,
                    MoveType::Dive => new_mechanics.diving = true,
                    MoveType::JetBoost => new_mechanics.jet_boosting = true,
                }
                visited.insert(next, (new_steps, new_mechanics.clone()));
                queue.push_back((next, new_steps, new_mechanics));
            }
        }
    }

    None
}

#[allow(clippy::too_many_arguments)]
fn get_reachable_with_types_filtered(
    caps: &MovementCaps,
    tiles: &[Vec<char>],
    pos: TilePos,
    grapple_points: &[TilePos],
    bounce_pads: &[TilePos],
    hazards: &HashSet<TilePos>,
    disable_wall_jump: bool,
    disable_dive: bool,
    disable_jet_boost: bool,
) -> Vec<(TilePos, MoveType)> {
    let mut reachable = get_reachable_with_types(caps, tiles, pos, grapple_points, bounce_pads, hazards);

    reachable.retain(|(_, move_type)| match move_type {
        MoveType::WallJump if disable_wall_jump => false,
        MoveType::Dive if disable_dive => false,
        MoveType::JetBoost if disable_jet_boost => false,
        _ => true,
    });

    reachable
}

/// Calculates all positions reachable from the current position via any movement type.
pub fn get_reachable_with_types(
    caps: &MovementCaps,
    tiles: &[Vec<char>],
    pos: TilePos,
    grapple_points: &[TilePos],
    bounce_pads: &[TilePos],
    hazards: &HashSet<TilePos>,
) -> Vec<(TilePos, MoveType)> {
    let mut reachable = Vec::new();
    let height = tiles.len() as i32;
    let width = tiles[0].len() as i32;

    let on_ground = is_standable(tiles, pos, hazards);
    let on_bounce = bounce_pads.iter().any(|bp| bp.x == pos.x && bp.y == pos.y + 1);

    // Walking
    if on_ground {
        for dx in [-1, 1] {
            let new_pos = TilePos::new(pos.x + dx, pos.y);
            if !is_solid(tiles, new_pos) && !is_hazard(hazards, new_pos) {
                reachable.push((new_pos, MoveType::Walk));
            }
        }
    }

    // Falling
    for dy in 1..=caps.max_fall {
        let fall_pos = TilePos::new(pos.x, pos.y + dy);
        if fall_pos.y >= height {
            break;
        }
        if is_solid(tiles, fall_pos) {
            break;
        }
        if is_hazard(hazards, fall_pos) {
            continue;
        }
        if is_standable(tiles, fall_pos, hazards) {
            reachable.push((fall_pos, MoveType::Fall));
        }
    }

    // Bouncing
    if on_bounce {
        for dy in -caps.bounce_vertical..=0 {
            for dx in -caps.jump_horizontal..=caps.jump_horizontal {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let target = TilePos::new(pos.x + dx, pos.y + dy);
                if target.x < 0 || target.x >= width || target.y < 0 || target.y >= height {
                    continue;
                }
                if !is_solid(tiles, target)
                    && !is_hazard(hazards, target)
                    && is_standable(tiles, target, hazards)
                    && is_jump_arc_clear(tiles, pos, target, hazards)
                {
                    reachable.push((target, MoveType::Bounce));
                }
            }
        }
    } else if on_ground {
        // Jumping
        for dy in -caps.jump_vertical..=0 {
            for dx in -caps.jump_horizontal..=caps.jump_horizontal {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let target = TilePos::new(pos.x + dx, pos.y + dy);
                if target.x < 0 || target.x >= width || target.y < 0 || target.y >= height {
                    continue;
                }
                if !is_solid(tiles, target)
                    && !is_hazard(hazards, target)
                    && is_standable(tiles, target, hazards)
                    && is_jump_arc_clear(tiles, pos, target, hazards)
                {
                    reachable.push((target, MoveType::Jump));
                }
            }
        }
    }

    // Wall jumping
    if is_near_wall(tiles, pos) {
        for dy in -caps.wall_jump_vertical..=1 {
            for dx in -caps.wall_jump_horizontal..=caps.wall_jump_horizontal {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let target = TilePos::new(pos.x + dx, pos.y + dy);
                if target.x < 0 || target.x >= width || target.y < 0 || target.y >= height {
                    continue;
                }
                if !is_solid(tiles, target) && !is_hazard(hazards, target)
                    && (is_standable(tiles, target, hazards) || is_near_wall(tiles, target)) {
                        reachable.push((target, MoveType::WallJump));
                    }
            }
        }
    }

    // Grappling
    for gp in grapple_points {
        let dist = pos.distance_to(*gp);
        if dist <= caps.grapple_range as f32 {
            let rope_len = dist.max(1.0) as i32;
            for dy in 0..=rope_len + 2 {
                for dx in -rope_len..=rope_len {
                    let target = TilePos::new(gp.x + dx, gp.y + dy);
                    if target.x < 0 || target.x >= width || target.y < 0 || target.y >= height {
                        continue;
                    }
                    let swing_dist = gp.distance_to(target);
                    if swing_dist <= (rope_len + 2) as f32
                        && !is_solid(tiles, target)
                        && !is_hazard(hazards, target)
                        && has_line_of_sight(tiles, pos, *gp) {
                            reachable.push((target, MoveType::Grapple));
                        }
                }
            }
        }
    }

    // Downward jet (fast descent)
    for dy in 1..=4 {
        let dive_pos = TilePos::new(pos.x, pos.y + dy);
        if dive_pos.y >= height {
            break;
        }

        let tile_char = get_tile(tiles, dive_pos);
        if tile_char == 'X' {
            continue;
        }
        if is_solid(tiles, dive_pos) {
            break;
        }
        if is_hazard(hazards, dive_pos) {
            break;
        }
        if is_standable(tiles, dive_pos, hazards) {
            reachable.push((dive_pos, MoveType::Dive));
        }
    }

    // Jet Boost
    let water_pools = find_water_pools_in_range(tiles, pos, 3);
    if !water_pools.is_empty() || has_water_charge(tiles, pos) {
        let jet_horizontal: i32 = caps.jet_boost_horizontal;
        let jet_vertical: i32 = caps.jet_boost_vertical;

        for dy in -jet_vertical..=jet_vertical {
            for dx in -jet_horizontal..=jet_horizontal {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if dx.abs() < 5 && dy.abs() < 3 {
                    continue;
                }

                let target = TilePos::new(pos.x + dx, pos.y + dy);
                if target.x < 0 || target.x >= width || target.y < 0 || target.y >= height {
                    continue;
                }

                if !is_solid(tiles, target)
                    && !is_hazard(hazards, target)
                    && is_standable(tiles, target, hazards)
                    && has_line_of_sight(tiles, pos, target) {
                        reachable.push((target, MoveType::JetBoost));
                    }
            }
        }
    }

    // Deduplicate
    let mut seen: HashSet<TilePos> = HashSet::new();
    reachable.retain(|(pos, _)| {
        if seen.contains(pos) {
            false
        } else {
            seen.insert(*pos);
            true
        }
    });

    reachable
}

/// Checks if a jump arc from `from` to `to` is clear of obstacles.
fn is_jump_arc_clear(
    tiles: &[Vec<char>],
    from: TilePos,
    to: TilePos,
    hazards: &HashSet<TilePos>,
) -> bool {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let steps = dx.abs().max(dy.abs().max(1));

    let peak_height = (dx.abs() as f32 / 2.0).max(1.5);

    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let check_x = from.x + (dx as f32 * t) as i32;

        let parabola = -4.0 * peak_height * (t - 0.5).powi(2) + peak_height;
        let linear_dy = dy as f32 * t;
        let arc_offset = (parabola + linear_dy) as i32;

        let check_y = from.y - arc_offset.max(0);
        let check_pos = TilePos::new(check_x, check_y);

        if is_solid(tiles, check_pos) || is_hazard(hazards, check_pos) {
            return false;
        }

        if arc_offset > 0 {
            let check_pos_below = TilePos::new(check_x, from.y);
            if is_hazard(hazards, check_pos_below) && t > 0.5 {
                return false;
            }
        }
    }
    true
}

fn has_line_of_sight(tiles: &[Vec<char>], from: TilePos, to: TilePos) -> bool {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let steps = dx.abs().max(dy.abs()).max(1);

    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let check_x = from.x + (dx as f32 * t) as i32;
        let check_y = from.y + (dy as f32 * t) as i32;
        if is_solid(tiles, TilePos::new(check_x, check_y)) {
            return false;
        }
    }
    true
}

fn find_water_pools_in_range(tiles: &[Vec<char>], pos: TilePos, range: i32) -> Vec<TilePos> {
    let mut pools = Vec::new();

    for dy in -range..=range {
        for dx in -range..=range {
            let check_pos = TilePos::new(pos.x + dx, pos.y + dy);
            if get_tile(tiles, check_pos) == '~' {
                pools.push(check_pos);
            }
        }
    }

    pools
}

fn has_water_charge(tiles: &[Vec<char>], pos: TilePos) -> bool {
    for dy in -1..=1 {
        for dx in -1..=1 {
            let check_pos = TilePos::new(pos.x + dx, pos.y + dy);
            if get_tile(tiles, check_pos) == '~' {
                return true;
            }
        }
    }
    false
}

/// Check if an adjacent exit tile is actually reachable from the current position
///
/// This verifies that:
/// - The exit tile itself is not a hazard
/// - There's a clear path (no walls blocking the direct step)
fn can_reach_adjacent_exit(
    tiles: &[Vec<char>],
    current: TilePos,
    exit: TilePos,
    hazards: &HashSet<TilePos>,
) -> bool {
    // Exit should not be a hazard
    if is_hazard(hazards, exit) {
        return false;
    }

    // Exit should not be a solid block (sanity check)
    if is_solid(tiles, exit) {
        return false;
    }

    // Check the movement direction
    let dx = exit.x - current.x;
    let dy = exit.y - current.y;

    // Horizontal movement: should be on ground or able to walk
    if dy == 0 && dx.abs() == 1 {
        // Can walk to adjacent tile if we're on ground and exit is passable
        return is_standable(tiles, current, hazards) || is_standable(tiles, exit, hazards);
    }

    // Vertical movement up: need to be able to jump or wall jump
    if dx == 0 && dy == -1 {
        // Can reach one tile up by jumping from ground or wall jumping
        return is_standable(tiles, current, hazards) || is_near_wall(tiles, current);
    }

    // Vertical movement down: falling is always possible
    if dx == 0 && dy == 1 {
        return true;
    }

    // Diagonal: rare but possible via jump
    if dx.abs() == 1 && dy.abs() == 1 {
        return is_standable(tiles, current, hazards);
    }

    false
}

/// Quick flood-fill connectivity check
pub fn are_connected_flood_fill(tiles: &[Vec<char>], spawn: TilePos, exit: TilePos) -> bool {
    let height = tiles.len() as i32;
    let width = tiles.first().map(|r| r.len()).unwrap_or(0) as i32;

    if width == 0 || height == 0 {
        return false;
    }

    let is_passable = |x: i32, y: i32| -> bool {
        if x < 0 || y < 0 || x >= width || y >= height {
            return false;
        }
        let ch = tiles[y as usize][x as usize];
        !matches!(ch, '#' | 'X')
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(spawn);
    visited.insert(spawn);

    let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(current) = queue.pop_front() {
        if current == exit {
            return true;
        }

        for (dx, dy) in directions {
            let next = TilePos::new(current.x + dx, current.y + dy);
            if is_passable(next.x, next.y) && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
            }
        }
    }

    false
}

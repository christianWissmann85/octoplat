//! Type definitions for the level validation system

use std::collections::HashSet;

/// Player movement capabilities in tile units.
#[derive(Clone)]
pub struct MovementCaps {
    /// Maximum horizontal distance reachable from a standing jump (tiles left/right).
    pub jump_horizontal: i32,
    /// Maximum vertical height from a standing jump (tiles upward).
    pub jump_vertical: i32,
    /// Maximum horizontal distance from a wall jump (tiles away from wall).
    pub wall_jump_horizontal: i32,
    /// Maximum vertical gain from a wall jump (tiles upward).
    pub wall_jump_vertical: i32,
    /// Maximum distance to attach to a grapple point (tiles).
    pub grapple_range: i32,
    /// Maximum vertical height from a bounce pad (tiles upward).
    pub bounce_vertical: i32,
    /// Maximum fall distance to consider (tiles downward).
    pub max_fall: i32,
    /// Sprint speed multiplier relative to normal movement.
    #[allow(dead_code)] // Future: used for sprint jump calculations
    pub sprint_speed_mult: f32,
    /// Maximum horizontal distance from jet boost (tiles).
    pub jet_boost_horizontal: i32,
    /// Maximum vertical distance from jet boost (tiles).
    pub jet_boost_vertical: i32,
}

impl Default for MovementCaps {
    fn default() -> Self {
        // Default values derived from typical game physics
        // (32px tiles, jump_velocity=-800, gravity=2400, etc.)
        Self {
            jump_horizontal: 4,
            jump_vertical: 3,
            wall_jump_horizontal: 3,
            wall_jump_vertical: 2,
            grapple_range: 6,
            bounce_vertical: 5,
            max_fall: 50,
            sprint_speed_mult: 2.0,
            jet_boost_horizontal: 15,
            jet_boost_vertical: 8,
        }
    }
}

impl MovementCaps {
    /// Create MovementCaps from runtime physics configuration.
    #[allow(dead_code)] // Future: will be called by game code to sync with physics
    ///
    /// Converts pixel-based physics values to tile-based movement capabilities.
    /// This ensures the validator uses the same parameters as the actual game.
    ///
    /// # Arguments
    /// * `tile_size` - Size of a tile in pixels (typically 32.0)
    /// * `gravity` - Gravitational acceleration in pixels/sec²
    /// * `jump_velocity` - Initial jump velocity in pixels/sec (negative = upward)
    /// * `wall_jump_velocity` - Wall jump velocity (x, y) in pixels/sec
    /// * `bounce_velocity` - Bounce pad launch velocity in pixels/sec (negative = upward)
    /// * `move_speed` - Normal movement speed in pixels/sec
    /// * `sprint_speed` - Sprint movement speed in pixels/sec
    /// * `grapple_range_px` - Grapple range in pixels
    /// * `jet_boost_speed` - Jet boost speed in pixels/sec
    /// * `jet_boost_duration` - Jet boost duration in seconds
    #[allow(clippy::too_many_arguments)]
    pub fn from_runtime_config(
        tile_size: f32,
        gravity: f32,
        jump_velocity: f32,
        wall_jump_velocity: (f32, f32),
        bounce_velocity: f32,
        move_speed: f32,
        sprint_speed: f32,
        grapple_range_px: f32,
        jet_boost_speed: f32,
        jet_boost_duration: f32,
    ) -> Self {
        // Jump physics: h = v² / (2g) for max height
        // Time to peak: t = -v / g
        // Horizontal distance during jump: d = v_x * 2t (full arc)

        // Jump vertical height (tiles) - convert from pixel velocity
        let jump_height_px = (jump_velocity * jump_velocity) / (2.0 * gravity);
        let jump_vertical = (jump_height_px / tile_size).ceil() as i32;

        // Jump horizontal distance (tiles) - based on move speed and jump time
        let jump_time = (-jump_velocity / gravity) * 2.0; // Full arc time
        let jump_horizontal_px = move_speed * jump_time;
        let jump_horizontal = (jump_horizontal_px / tile_size).ceil() as i32;

        // Wall jump physics
        let wall_jump_height_px = (wall_jump_velocity.1 * wall_jump_velocity.1) / (2.0 * gravity);
        let wall_jump_vertical = (wall_jump_height_px / tile_size).ceil() as i32;

        let wall_jump_time = (-wall_jump_velocity.1 / gravity) * 2.0;
        let wall_jump_horizontal_px = wall_jump_velocity.0.abs() * wall_jump_time;
        let wall_jump_horizontal = (wall_jump_horizontal_px / tile_size).ceil() as i32;

        // Bounce physics (similar to jump but with bounce velocity)
        let bounce_height_px = (bounce_velocity * bounce_velocity) / (2.0 * gravity);
        let bounce_vertical = (bounce_height_px / tile_size).ceil() as i32;

        // Grapple range (direct conversion)
        let grapple_range = (grapple_range_px / tile_size).ceil() as i32;

        // Sprint multiplier
        let sprint_speed_mult = sprint_speed / move_speed;

        // Jet boost distance (speed * duration)
        let jet_distance_px = jet_boost_speed * jet_boost_duration;
        let jet_boost_horizontal = ((jet_distance_px * 1.5) / tile_size).ceil() as i32; // Allow some extra for angled boosts
        let jet_boost_vertical = (jet_distance_px / tile_size).ceil() as i32;

        Self {
            jump_horizontal,
            jump_vertical,
            wall_jump_horizontal,
            wall_jump_vertical,
            grapple_range,
            bounce_vertical,
            max_fall: 50, // Large value, rarely limiting
            sprint_speed_mult,
            jet_boost_horizontal,
            jet_boost_vertical,
        }
    }
}

/// A position in the tile grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: TilePos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: TilePos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Result of level validation with detailed feedback
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Is the level completable (path exists from spawn to exit)?
    pub is_completable: bool,
    /// Is the level interesting enough to be fun?
    pub is_interesting: bool,
    /// Minimum steps to complete the level
    pub path_length: usize,
    /// Detailed reasons if validation failed
    pub issues: Vec<String>,
    /// Mechanics used on the optimal path
    pub mechanics_used: MechanicsUsed,
    /// Mechanics that are REQUIRED to complete the level (no bypass paths)
    pub mechanics_required: MechanicsRequired,
    /// Interest score (0.0 - 1.0)
    pub interest_score: f32,
}

impl ValidationResult {
    pub fn failed(reason: &str) -> Self {
        Self {
            is_completable: false,
            is_interesting: false,
            path_length: 0,
            issues: vec![reason.to_string()],
            mechanics_used: MechanicsUsed::default(),
            mechanics_required: MechanicsRequired::none(),
            interest_score: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.is_completable && self.is_interesting
    }

    #[allow(dead_code)]
    pub fn requires_advanced_mechanics(&self) -> bool {
        self.mechanics_required.has_advanced()
    }
}

/// Track which mechanics are used in the level
#[derive(Debug, Clone, Default)]
pub struct MechanicsUsed {
    pub walking: bool,
    pub jumping: bool,
    pub wall_jumping: bool,
    pub grappling: bool,
    pub bouncing: bool,
    pub falling: bool,
    pub diving: bool,
    pub jet_boosting: bool,
}

impl MechanicsUsed {
    pub fn count(&self) -> usize {
        let mut count = 0;
        if self.walking { count += 1; }
        if self.jumping { count += 1; }
        if self.wall_jumping { count += 1; }
        if self.grappling { count += 1; }
        if self.bouncing { count += 1; }
        if self.falling { count += 1; }
        if self.diving { count += 1; }
        if self.jet_boosting { count += 1; }
        count
    }
}

/// Movement type used to reach a position
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveType {
    Walk,
    Jump,
    WallJump,
    Grapple,
    Bounce,
    Fall,
    Dive,
    JetBoost,
}

/// Tracks which mechanics are REQUIRED to complete a level
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MechanicsRequired {
    pub grapple: bool,
    pub wall_jump: bool,
    pub bounce: bool,
    pub dive: bool,
    pub jet_boost: bool,
}

impl MechanicsRequired {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn new(grapple: bool, wall_jump: bool, bounce: bool) -> Self {
        Self {
            grapple,
            wall_jump,
            bounce,
            dive: false,
            jet_boost: false,
        }
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        let mut count = 0;
        if self.grapple { count += 1; }
        if self.wall_jump { count += 1; }
        if self.bounce { count += 1; }
        if self.dive { count += 1; }
        if self.jet_boost { count += 1; }
        count
    }

    pub fn has_advanced(&self) -> bool {
        self.grapple || self.wall_jump || self.bounce || self.dive || self.jet_boost
    }
}

/// Helper functions for tile checking
pub fn is_solid(tiles: &[Vec<char>], pos: TilePos) -> bool {
    if pos.y < 0 || pos.y >= tiles.len() as i32 {
        return true;
    }
    if pos.x < 0 || pos.x >= tiles[pos.y as usize].len() as i32 {
        return true;
    }
    let ch = tiles[pos.y as usize][pos.x as usize];
    matches!(ch, '#' | 'X')
}

pub fn is_hazard(hazards: &HashSet<TilePos>, pos: TilePos) -> bool {
    hazards.contains(&pos)
}

pub fn is_standable(tiles: &[Vec<char>], pos: TilePos, hazards: &HashSet<TilePos>) -> bool {
    if is_solid(tiles, pos) || is_hazard(hazards, pos) {
        return false;
    }
    let below = TilePos::new(pos.x, pos.y + 1);
    if is_solid(tiles, below) {
        return true;
    }
    if below.y >= 0 && (below.y as usize) < tiles.len() {
        let below_row = &tiles[below.y as usize];
        if (below.x as usize) < below_row.len() {
            let ch = below_row[below.x as usize];
            if ch == '_' || ch == '!' || ch == '.' {
                return true;
            }
        }
    }
    false
}

pub fn is_near_wall(tiles: &[Vec<char>], pos: TilePos) -> bool {
    let left = TilePos::new(pos.x - 1, pos.y);
    let right = TilePos::new(pos.x + 1, pos.y);
    is_solid(tiles, left) || is_solid(tiles, right)
}

pub fn get_tile(tiles: &[Vec<char>], pos: TilePos) -> char {
    if pos.y < 0 || pos.y >= tiles.len() as i32 {
        return '#';
    }
    let row = &tiles[pos.y as usize];
    if pos.x < 0 || pos.x >= row.len() as i32 {
        return '#';
    }
    row[pos.x as usize]
}

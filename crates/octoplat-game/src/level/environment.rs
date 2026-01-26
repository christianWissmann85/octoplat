//! Level environment - dynamic level state
//!
//! Contains gems, enemies, platforms, and other dynamic level elements.
//! Entities are stored in HashMaps with typed IDs for stable references and easy debugging.

use std::collections::{HashMap, HashSet};
use macroquad::prelude::{vec2, Rect, Vec2};

use crate::collectibles::Gem;
use crate::compat::vec2_to_mq;
use crate::config::GameConfig;
use crate::hazards::{Crab, Pufferfish, PufferfishPattern};
use crate::platforms::{CrumblingPlatform, MovingPlatform};
use octoplat_core::level::{Decoration, MarkerType, TileMap};

// ============================================================================
// Entity ID Types
// ============================================================================

/// Unique identifier for a gem entity
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GemId(pub u32);

/// Unique identifier for a crab enemy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CrabId(pub u32);

/// Unique identifier for a pufferfish enemy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PufferfishId(pub u32);

/// Unique identifier for a moving platform
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MovingPlatformId(pub u32);

/// Unique identifier for a crumbling platform
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CrumblingPlatformId(pub u32);

// ============================================================================
// Level Environment
// ============================================================================

/// Dynamic level environment state
pub struct LevelEnvironment {
    // Collectibles (HashMap with typed IDs)
    pub gems: HashMap<GemId, Gem>,
    next_gem_id: u32,
    pub gems_collected: u32,
    pub total_gems: u32,

    // Markers/positions
    pub grapple_points: Vec<Vec2>,
    pub checkpoint_positions: Vec<Vec2>,
    pub water_pool_positions: Vec<Vec2>,
    pub exit_position: Option<Vec2>,

    // Enemies (HashMap with typed IDs)
    pub crabs: HashMap<CrabId, Crab>,
    next_crab_id: u32,
    pub pufferfish: HashMap<PufferfishId, Pufferfish>,
    next_pufferfish_id: u32,

    // Platforms (HashMap with typed IDs)
    pub moving_platforms: HashMap<MovingPlatformId, MovingPlatform>,
    next_moving_platform_id: u32,
    pub crumbling_platforms: HashMap<CrumblingPlatformId, CrumblingPlatform>,
    next_crumbling_platform_id: u32,

    // Block state
    pub destroyed_blocks: HashSet<(usize, usize)>,

    // Visual decorations
    pub decorations: Vec<Decoration>,

    // Level state
    pub level_complete: bool,
    pub show_level_text: f32,
    pub level_time: f32,
}

impl LevelEnvironment {
    pub fn new() -> Self {
        Self {
            gems: HashMap::new(),
            next_gem_id: 0,
            gems_collected: 0,
            total_gems: 0,
            grapple_points: Vec::new(),
            checkpoint_positions: Vec::new(),
            water_pool_positions: Vec::new(),
            exit_position: None,
            crabs: HashMap::new(),
            next_crab_id: 0,
            pufferfish: HashMap::new(),
            next_pufferfish_id: 0,
            moving_platforms: HashMap::new(),
            next_moving_platform_id: 0,
            crumbling_platforms: HashMap::new(),
            next_crumbling_platform_id: 0,
            destroyed_blocks: HashSet::new(),
            decorations: Vec::new(),
            level_complete: false,
            show_level_text: 3.0,
            level_time: 0.0,
        }
    }

    // ========================================================================
    // Entity Spawn Methods
    // ========================================================================

    /// Spawn a new gem and return its ID
    pub fn spawn_gem(&mut self, gem: Gem) -> GemId {
        let id = GemId(self.next_gem_id);
        self.next_gem_id += 1;
        self.gems.insert(id, gem);
        id
    }

    /// Spawn a new crab and return its ID
    pub fn spawn_crab(&mut self, crab: Crab) -> CrabId {
        let id = CrabId(self.next_crab_id);
        self.next_crab_id += 1;
        self.crabs.insert(id, crab);
        id
    }

    /// Spawn a new pufferfish and return its ID
    pub fn spawn_pufferfish(&mut self, puffer: Pufferfish) -> PufferfishId {
        let id = PufferfishId(self.next_pufferfish_id);
        self.next_pufferfish_id += 1;
        self.pufferfish.insert(id, puffer);
        id
    }

    /// Spawn a new moving platform and return its ID
    pub fn spawn_moving_platform(&mut self, platform: MovingPlatform) -> MovingPlatformId {
        let id = MovingPlatformId(self.next_moving_platform_id);
        self.next_moving_platform_id += 1;
        self.moving_platforms.insert(id, platform);
        id
    }

    /// Spawn a new crumbling platform and return its ID
    pub fn spawn_crumbling_platform(&mut self, platform: CrumblingPlatform) -> CrumblingPlatformId {
        let id = CrumblingPlatformId(self.next_crumbling_platform_id);
        self.next_crumbling_platform_id += 1;
        self.crumbling_platforms.insert(id, platform);
        id
    }

    // ========================================================================
    // Reset Methods
    // ========================================================================

    /// Reset enemies to their starting positions
    pub fn reset_enemies(&mut self) {
        for crab in self.crabs.values_mut() {
            crab.reset();
        }
        for puffer in self.pufferfish.values_mut() {
            puffer.reset();
        }
    }

    /// Reset crumbling platforms
    pub fn reset_platforms(&mut self) {
        for platform in self.crumbling_platforms.values_mut() {
            platform.reset();
        }
    }

    // ========================================================================
    // Query Methods
    // ========================================================================

    /// Update level time
    pub fn update_time(&mut self, dt: f32) {
        self.level_time += dt;
    }

    /// Get all solid crumbling platform collision rects
    pub fn solid_crumbling_rects(&self) -> Vec<Rect> {
        self.crumbling_platforms
            .values()
            .filter(|p| p.is_solid())
            .map(|p| p.collision_rect())
            .collect()
    }

    // ========================================================================
    // Setup Methods
    // ========================================================================

    /// Set up environment from a tilemap
    pub fn setup_from_tilemap(&mut self, tilemap: &TileMap, config: &GameConfig) {
        // Clear existing entities and reset ID counters
        self.gems.clear();
        self.next_gem_id = 0;
        self.crabs.clear();
        self.next_crab_id = 0;
        self.pufferfish.clear();
        self.next_pufferfish_id = 0;
        self.moving_platforms.clear();
        self.next_moving_platform_id = 0;
        self.crumbling_platforms.clear();
        self.next_crumbling_platform_id = 0;

        // Set up gems (convert core Vec2 to mq Vec2)
        for pos in tilemap.get_gem_positions() {
            self.spawn_gem(Gem::new(vec2_to_mq(pos)));
        }
        self.total_gems = self.gems.len() as u32;
        self.gems_collected = 0;

        // Set up markers (convert core Vec2 to mq Vec2)
        self.grapple_points = tilemap.get_grapple_points()
            .into_iter().map(vec2_to_mq).collect();
        self.checkpoint_positions = tilemap.get_checkpoint_positions()
            .into_iter().map(vec2_to_mq).collect();
        self.water_pool_positions = tilemap.get_water_pool_positions()
            .into_iter().map(vec2_to_mq).collect();
        self.exit_position = tilemap.get_exit_position().map(vec2_to_mq);

        // Set up enemies from markers
        for pos in tilemap.get_marker_positions(MarkerType::Crab) {
            self.spawn_crab(Crab::new(vec2_to_mq(pos), config));
        }

        for m in &tilemap.markers {
            match m.marker_type {
                MarkerType::PufferfishStationary => {
                    self.spawn_pufferfish(Pufferfish::new(vec2_to_mq(m.position), PufferfishPattern::Stationary));
                }
                MarkerType::PufferfishHorizontal => {
                    self.spawn_pufferfish(Pufferfish::new(vec2_to_mq(m.position), PufferfishPattern::Horizontal));
                }
                MarkerType::PufferfishVertical => {
                    self.spawn_pufferfish(Pufferfish::new(vec2_to_mq(m.position), PufferfishPattern::Vertical));
                }
                _ => {}
            }
        }

        // Set up moving platforms (pair start and end markers)
        for platform in Self::create_moving_platforms(tilemap) {
            self.spawn_moving_platform(platform);
        }

        // Set up crumbling platforms
        let platform_size = vec2(tilemap.tile_size, tilemap.tile_size * 0.5);
        for pos in tilemap.get_marker_positions(MarkerType::CrumblingPlatform) {
            self.spawn_crumbling_platform(CrumblingPlatform::new(vec2_to_mq(pos), platform_size));
        }

        // Reset level state
        self.level_complete = false;
        self.show_level_text = 3.0;
        self.destroyed_blocks.clear();
    }

    /// Create moving platforms by pairing start/end markers
    fn create_moving_platforms(tilemap: &TileMap) -> Vec<MovingPlatform> {
        let mut platforms = Vec::new();
        let platform_size = vec2(tilemap.tile_size * 2.0, tilemap.tile_size * 0.5);

        // Get all start and end positions (core Vec2)
        let h_starts = tilemap.get_marker_positions(MarkerType::MovingPlatformHorizontalStart);
        let h_ends = tilemap.get_marker_positions(MarkerType::MovingPlatformHorizontalEnd);
        let v_starts = tilemap.get_marker_positions(MarkerType::MovingPlatformVerticalStart);
        let v_ends = tilemap.get_marker_positions(MarkerType::MovingPlatformVerticalEnd);

        // Pair horizontal platforms (find nearest end marker on same row)
        for start in &h_starts {
            let mut best_end: Option<octoplat_core::Vec2> = None;
            let mut best_dist = f32::MAX;

            for end in &h_ends {
                // Must be on approximately the same row (within half tile)
                if (start.y - end.y).abs() < tilemap.tile_size * 0.5 {
                    let dist = (end.x - start.x).abs();
                    if dist < best_dist && end.x > start.x {
                        best_dist = dist;
                        best_end = Some(*end);
                    }
                }
            }

            if let Some(end) = best_end {
                platforms.push(MovingPlatform::new(vec2_to_mq(*start), vec2_to_mq(end), platform_size));
            }
        }

        // Pair vertical platforms (find nearest end marker on same column)
        for start in &v_starts {
            let mut best_end: Option<octoplat_core::Vec2> = None;
            let mut best_dist = f32::MAX;

            for end in &v_ends {
                // Must be on approximately the same column (within half tile)
                if (start.x - end.x).abs() < tilemap.tile_size * 0.5 {
                    let dist = (end.y - start.y).abs();
                    if dist < best_dist && end.y > start.y {
                        best_dist = dist;
                        best_end = Some(*end);
                    }
                }
            }

            if let Some(end) = best_end {
                platforms.push(MovingPlatform::new(vec2_to_mq(*start), vec2_to_mq(end), platform_size));
            }
        }

        platforms
    }
}

impl Default for LevelEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

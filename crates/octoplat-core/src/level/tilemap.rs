use std::collections::HashSet;

use crate::{vec2, Color, Rect, Vec2};

use super::markers::{LevelMarker, MarkerType};

/// Tile types with their properties
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Solid,
    Platform,
    Spike,
    OneWay,    // Can jump through from below
    BouncePad, // Launches player upward
    Breakable, // Destroyed by dive attack
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        matches!(
            self,
            TileType::Solid | TileType::BouncePad | TileType::Breakable
        )
    }

    pub fn is_hazard(&self) -> bool {
        matches!(self, TileType::Spike)
    }

    pub fn color(&self) -> Color {
        match self {
            TileType::Empty => Color::TRANSPARENT,
            TileType::Solid => Color::new(0.3, 0.5, 0.6, 1.0),
            TileType::Platform => Color::new(0.4, 0.6, 0.5, 1.0),
            TileType::Spike => Color::new(0.8, 0.3, 0.3, 1.0),
            TileType::OneWay => Color::new(0.5, 0.7, 0.6, 0.7), // Semi-transparent green
            TileType::BouncePad => Color::new(0.9, 0.4, 0.5, 1.0), // Pink/red spring
            TileType::Breakable => Color::new(0.6, 0.5, 0.3, 1.0), // Brown/tan cracked block
        }
    }
}

/// The level tilemap
#[derive(Clone, Debug)]
pub struct TileMap {
    pub tiles: Vec<Vec<TileType>>,
    pub tile_size: f32,
    pub width: usize,
    pub height: usize,
    pub markers: Vec<LevelMarker>,
}

impl TileMap {
    /// Parse from string representation
    pub fn from_string(data: &str, tile_size: f32) -> Self {
        let lines: Vec<&str> = data.lines().filter(|l| !l.is_empty()).collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

        let mut tiles = vec![vec![TileType::Empty; width]; height];
        let mut markers = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let position = vec2(
                    x as f32 * tile_size + tile_size / 2.0,
                    y as f32 * tile_size + tile_size / 2.0,
                );

                tiles[y][x] = match ch {
                    // Solid tiles
                    '#' | '\u{2588}' | '\u{2593}' => TileType::Solid,
                    // Platform
                    '=' | '\u{2580}' => TileType::Platform,
                    // Spike
                    '^' | '\u{25B2}' => TileType::Spike,
                    // One-way platform (jump through from below)
                    '_' => TileType::OneWay,
                    // Bounce pad (launches player up)
                    '!' => TileType::BouncePad,
                    // Breakable block (destroyed by dive)
                    'X' => TileType::Breakable,
                    // Player spawn
                    '\u{1F419}' | 'P' => {
                        markers.push(LevelMarker::new(position, MarkerType::PlayerSpawn));
                        TileType::Empty
                    }
                    // Gem
                    '\u{1F48E}' | '*' => {
                        markers.push(LevelMarker::new(position, MarkerType::Gem));
                        TileType::Empty
                    }
                    // Grapple point
                    '@' | 'o' | '\u{1FA9D}' => {
                        markers.push(LevelMarker::new(position, MarkerType::GrapplePoint));
                        TileType::Empty
                    }
                    // Checkpoint
                    'S' | '\u{1F4BE}' => {
                        markers.push(LevelMarker::new(position, MarkerType::Checkpoint));
                        TileType::Empty
                    }
                    // Level exit
                    '>' | '\u{1F3C1}' => {
                        markers.push(LevelMarker::new(position, MarkerType::LevelExit));
                        TileType::Empty
                    }
                    // Water pool (for charge refill)
                    '~' => {
                        markers.push(LevelMarker::new(position, MarkerType::WaterPool));
                        TileType::Empty
                    }
                    // Crab enemy (patrols horizontally)
                    'C' => {
                        markers.push(LevelMarker::new(position, MarkerType::Crab));
                        TileType::Empty
                    }
                    // Pufferfish enemies (different movement patterns)
                    'O' => {
                        markers.push(LevelMarker::new(position, MarkerType::PufferfishStationary));
                        TileType::Empty
                    }
                    '(' => {
                        markers.push(LevelMarker::new(position, MarkerType::PufferfishHorizontal));
                        TileType::Empty
                    }
                    ')' => {
                        markers.push(LevelMarker::new(position, MarkerType::PufferfishVertical));
                        TileType::Empty
                    }
                    // Moving platform markers (horizontal)
                    '[' => {
                        markers.push(LevelMarker::new(
                            position,
                            MarkerType::MovingPlatformHorizontalStart,
                        ));
                        TileType::Empty
                    }
                    ']' => {
                        markers.push(LevelMarker::new(
                            position,
                            MarkerType::MovingPlatformHorizontalEnd,
                        ));
                        TileType::Empty
                    }
                    // Moving platform markers (vertical)
                    '{' => {
                        markers.push(LevelMarker::new(
                            position,
                            MarkerType::MovingPlatformVerticalStart,
                        ));
                        TileType::Empty
                    }
                    '}' => {
                        markers.push(LevelMarker::new(
                            position,
                            MarkerType::MovingPlatformVerticalEnd,
                        ));
                        TileType::Empty
                    }
                    // Crumbling platform
                    '.' => {
                        markers.push(LevelMarker::new(position, MarkerType::CrumblingPlatform));
                        TileType::Empty
                    }
                    _ => TileType::Empty,
                };
            }
        }

        Self {
            tiles,
            tile_size,
            width,
            height,
            markers,
        }
    }

    /// Get tile at grid position
    pub fn get(&self, x: usize, y: usize) -> TileType {
        self.tiles
            .get(y)
            .and_then(|row| row.get(x).copied())
            .unwrap_or(TileType::Empty)
    }

    /// Get solid tiles near a position (for collision)
    pub fn get_nearby_solid_rects(&self, position: Vec2, radius: f32) -> Vec<Rect> {
        self.get_nearby_rects_matching(position, radius, |t| t.is_solid())
    }

    /// Get solid tiles near a position, excluding destroyed breakable blocks
    pub fn get_nearby_solid_rects_excluding(
        &self,
        position: Vec2,
        radius: f32,
        destroyed: &HashSet<(usize, usize)>,
    ) -> Vec<Rect> {
        let min_x = ((position.x - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_x = ((position.x + radius) / self.tile_size).ceil() as usize;
        let min_y = ((position.y - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_y = ((position.y + radius) / self.tile_size).ceil() as usize;

        let mut rects = Vec::new();
        for y in min_y..=max_y.min(self.height.saturating_sub(1)) {
            for x in min_x..=max_x.min(self.width.saturating_sub(1)) {
                let tile = self.tiles[y][x];
                // Skip destroyed breakable blocks
                if tile == TileType::Breakable && destroyed.contains(&(x, y)) {
                    continue;
                }
                if tile.is_solid() {
                    rects.push(Rect::new(
                        x as f32 * self.tile_size,
                        y as f32 * self.tile_size,
                        self.tile_size,
                        self.tile_size,
                    ));
                }
            }
        }
        rects
    }

    /// Get hazard tiles near a position (for damage detection)
    pub fn get_nearby_hazard_rects(&self, position: Vec2, radius: f32) -> Vec<Rect> {
        self.get_nearby_rects_matching(position, radius, |t| t.is_hazard())
    }

    /// Get one-way platform tiles near a position
    pub fn get_nearby_oneway_rects(&self, position: Vec2, radius: f32) -> Vec<Rect> {
        self.get_nearby_rects_matching(position, radius, |t| matches!(t, TileType::OneWay))
    }

    /// Get bounce pad tiles near a position
    pub fn get_nearby_bounce_rects(&self, position: Vec2, radius: f32) -> Vec<Rect> {
        self.get_nearby_rects_matching(position, radius, |t| matches!(t, TileType::BouncePad))
    }

    /// Get breakable block tiles near a position with their grid coordinates
    pub fn get_nearby_breakable_tiles(
        &self,
        position: Vec2,
        radius: f32,
    ) -> Vec<(usize, usize, Rect)> {
        let min_x = ((position.x - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_x = ((position.x + radius) / self.tile_size).ceil() as usize;
        let min_y = ((position.y - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_y = ((position.y + radius) / self.tile_size).ceil() as usize;

        let mut result = Vec::new();
        for y in min_y..=max_y.min(self.height.saturating_sub(1)) {
            for x in min_x..=max_x.min(self.width.saturating_sub(1)) {
                if matches!(self.tiles[y][x], TileType::Breakable) {
                    result.push((
                        x,
                        y,
                        Rect::new(
                            x as f32 * self.tile_size,
                            y as f32 * self.tile_size,
                            self.tile_size,
                            self.tile_size,
                        ),
                    ));
                }
            }
        }
        result
    }

    /// Generic helper to get tiles matching a predicate
    fn get_nearby_rects_matching<F>(&self, position: Vec2, radius: f32, predicate: F) -> Vec<Rect>
    where
        F: Fn(TileType) -> bool,
    {
        let min_x = ((position.x - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_x = ((position.x + radius) / self.tile_size).ceil() as usize;
        let min_y = ((position.y - radius) / self.tile_size).floor().max(0.0) as usize;
        let max_y = ((position.y + radius) / self.tile_size).ceil() as usize;

        let mut rects = Vec::new();
        for y in min_y..=max_y.min(self.height.saturating_sub(1)) {
            for x in min_x..=max_x.min(self.width.saturating_sub(1)) {
                if predicate(self.tiles[y][x]) {
                    rects.push(Rect::new(
                        x as f32 * self.tile_size,
                        y as f32 * self.tile_size,
                        self.tile_size,
                        self.tile_size,
                    ));
                }
            }
        }
        rects
    }

    /// Get player spawn position from markers, or default
    pub fn get_spawn_position(&self) -> Vec2 {
        self.markers
            .iter()
            .find(|m| m.marker_type == MarkerType::PlayerSpawn)
            .map(|m| m.position)
            .unwrap_or(vec2(100.0, 100.0))
    }

    /// Get all positions for a specific marker type
    pub fn get_marker_positions(&self, marker_type: MarkerType) -> Vec<Vec2> {
        self.markers
            .iter()
            .filter(|m| m.marker_type == marker_type)
            .map(|m| m.position)
            .collect()
    }

    /// Get all gem positions
    pub fn get_gem_positions(&self) -> Vec<Vec2> {
        self.get_marker_positions(MarkerType::Gem)
    }

    /// Get all grapple point positions
    pub fn get_grapple_points(&self) -> Vec<Vec2> {
        self.get_marker_positions(MarkerType::GrapplePoint)
    }

    /// Get all checkpoint positions
    pub fn get_checkpoint_positions(&self) -> Vec<Vec2> {
        self.get_marker_positions(MarkerType::Checkpoint)
    }

    /// Get level exit position (if any)
    pub fn get_exit_position(&self) -> Option<Vec2> {
        self.markers
            .iter()
            .find(|m| m.marker_type == MarkerType::LevelExit)
            .map(|m| m.position)
    }

    /// Get all water pool positions
    pub fn get_water_pool_positions(&self) -> Vec<Vec2> {
        self.get_marker_positions(MarkerType::WaterPool)
    }

    /// Level bounds as a Rect
    pub fn bounds(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            self.width as f32 * self.tile_size,
            self.height as f32 * self.tile_size,
        )
    }

    /// Convert tilemap back to string representation
    pub fn to_level_string(&self) -> String {
        let mut result = String::new();

        // Create a grid of characters
        let mut grid: Vec<Vec<char>> = self
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        TileType::Empty => ' ',
                        TileType::Solid => '#',
                        TileType::Platform => '=',
                        TileType::Spike => '^',
                        TileType::OneWay => '_',
                        TileType::BouncePad => '!',
                        TileType::Breakable => 'X',
                    })
                    .collect()
            })
            .collect();

        // Overlay markers onto the grid
        for marker in &self.markers {
            let x = ((marker.position.x - self.tile_size / 2.0) / self.tile_size) as usize;
            let y = ((marker.position.y - self.tile_size / 2.0) / self.tile_size) as usize;

            if y < grid.len() && x < grid[y].len() {
                grid[y][x] = match marker.marker_type {
                    MarkerType::PlayerSpawn => 'P',
                    MarkerType::Gem => '*',
                    MarkerType::GrapplePoint => '@',
                    MarkerType::Checkpoint => 'S',
                    MarkerType::LevelExit => '>',
                    MarkerType::WaterPool => '~',
                    MarkerType::Crab => 'C',
                    MarkerType::PufferfishStationary => 'O',
                    MarkerType::PufferfishHorizontal => '(',
                    MarkerType::PufferfishVertical => ')',
                    MarkerType::MovingPlatformHorizontalStart => '[',
                    MarkerType::MovingPlatformHorizontalEnd => ']',
                    MarkerType::MovingPlatformVerticalStart => '{',
                    MarkerType::MovingPlatformVerticalEnd => '}',
                    MarkerType::CrumblingPlatform => '.',
                };
            }
        }

        // Convert grid to string
        for row in grid {
            result.push_str(&row.iter().collect::<String>());
            result.push('\n');
        }

        result
    }
}

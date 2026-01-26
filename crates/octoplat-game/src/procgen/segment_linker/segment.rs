//! Segment parsing and manipulation

use octoplat_core::procgen::PooledLevel;

/// Placement of a segment in the combined level
#[derive(Clone, Debug)]
pub struct SegmentPlacement {
    /// Index into the parsed segments array
    pub segment_idx: usize,
    /// Global X position (tiles)
    pub x: usize,
    /// Global Y position (tiles)
    pub y: usize,
}

/// A parsed segment ready for linking
pub struct ParsedSegment {
    /// Segment name
    pub name: String,
    /// Tile grid (row-major)
    pub tiles: Vec<Vec<char>>,
    /// Width
    pub width: usize,
    /// Height
    pub height: usize,
    /// Spawn position if present
    pub spawn_pos: Option<(usize, usize)>,
    /// Exit position if present
    pub exit_pos: Option<(usize, usize)>,
}

impl ParsedSegment {
    pub fn from_pooled_level(level: &PooledLevel) -> Option<Self> {
        // Extract tilemap portion (after --- separator)
        let content = &level.content;
        let tilemap_str = if let Some(pos) = content.find("\n---\n") {
            &content[pos + 5..]
        } else if content.starts_with("---") {
            &content[4..]
        } else {
            content
        };

        let tilemap_str = tilemap_str.trim();
        if tilemap_str.is_empty() {
            return None;
        }

        let tiles: Vec<Vec<char>> = tilemap_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        if tiles.is_empty() {
            return None;
        }

        let height = tiles.len();
        let width = tiles.iter().map(|row| row.len()).max().unwrap_or(0);

        // Find spawn and exit positions
        let mut spawn_pos = None;
        let mut exit_pos = None;

        for (y, row) in tiles.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                match ch {
                    'P' => spawn_pos = Some((x, y)),
                    '>' => exit_pos = Some((x, y)),
                    _ => {}
                }
            }
        }

        Some(Self {
            name: level.name.clone(),
            tiles,
            width,
            height,
            spawn_pos,
            exit_pos,
        })
    }

    /// Remove spawn marker from tiles
    pub fn strip_spawn(&mut self) {
        if let Some((x, y)) = self.spawn_pos {
            if y < self.tiles.len() && x < self.tiles[y].len() {
                self.tiles[y][x] = ' ';
            }
            self.spawn_pos = None;
        }
    }

    /// Remove exit marker from tiles
    pub fn strip_exit(&mut self) {
        if let Some((x, y)) = self.exit_pos {
            if y < self.tiles.len() && x < self.tiles[y].len() {
                self.tiles[y][x] = ' ';
            }
            self.exit_pos = None;
        }
    }

    /// Get a tile at position (with bounds checking)
    pub fn get_tile(&self, x: usize, y: usize) -> char {
        if y < self.tiles.len() && x < self.tiles[y].len() {
            self.tiles[y][x]
        } else {
            '#' // Out of bounds = solid
        }
    }

    /// Normalize height by padding shorter rows
    pub fn normalize_width(&mut self) {
        for row in &mut self.tiles {
            while row.len() < self.width {
                row.push('#');
            }
        }
    }

    /// Pad segment to a minimum height by adding wall rows at top and bottom
    /// This helps ensure segments with different heights can be properly linked
    pub fn pad_to_min_height(&mut self, min_height: usize) {
        if self.height >= min_height {
            return;
        }

        let rows_to_add = min_height - self.height;
        let top_rows = rows_to_add / 2;
        let bottom_rows = rows_to_add - top_rows;

        // Create wall rows
        let wall_row: Vec<char> = vec!['#'; self.width];

        // Add rows at top
        let mut new_tiles = Vec::with_capacity(min_height);
        for _ in 0..top_rows {
            new_tiles.push(wall_row.clone());
        }

        // Add original tiles (shifted down)
        for row in self.tiles.drain(..) {
            new_tiles.push(row);
        }

        // Add rows at bottom
        for _ in 0..bottom_rows {
            new_tiles.push(wall_row.clone());
        }

        self.tiles = new_tiles;
        self.height = min_height;

        // Adjust spawn and exit positions
        if let Some((x, y)) = self.spawn_pos {
            self.spawn_pos = Some((x, y + top_rows));
        }
        if let Some((x, y)) = self.exit_pos {
            self.exit_pos = Some((x, y + top_rows));
        }
    }
}

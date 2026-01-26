//! Type definitions for the segment linker system

use octoplat_core::procgen::BiomeId;
use octoplat_core::state::DifficultyPreset;

/// Layout strategy for segment arrangement
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutStrategy {
    /// Horizontal chain: [Seg1] -> [Seg2] -> [Seg3]
    Linear,
    /// Vertical stack: Seg1 below Seg2 below Seg3 (bottom to top)
    Vertical,
    /// Alternating horizontal and vertical: zig-zag pattern
    Alternating,
    /// 2D grid arrangement with multiple connection directions
    Grid,
}

/// Direction of a connection between segments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LinkDirection {
    Right,
    Left,
    Up,
    Down,
}

/// Connection zone specification
#[derive(Clone, Debug)]
pub struct ConnectionZone {
    /// Start position (row for horizontal, column for vertical)
    pub start: usize,
    /// End position (row for horizontal, column for vertical)
    pub end: usize,
}

impl ConnectionZone {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn center(&self) -> usize {
        (self.start + self.end) / 2
    }

    pub fn size(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// Configuration for segment linking
#[derive(Clone, Debug)]
pub struct SegmentLinkerConfig {
    /// Random seed
    pub seed: u64,
    /// Target biome
    pub biome: BiomeId,
    /// Difficulty preset
    pub preset: DifficultyPreset,
    /// Number of segments to link (2-5 typical)
    pub segment_count: usize,
    /// Width of connecting corridors
    pub corridor_width: usize,
    /// Height clearance for corridors
    pub corridor_height: usize,
    /// Layout strategy to use
    pub layout: LayoutStrategy,
}

impl Default for SegmentLinkerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            biome: BiomeId::OceanDepths,
            preset: DifficultyPreset::Standard,
            segment_count: 3,
            corridor_width: 6,
            corridor_height: 5,
            layout: LayoutStrategy::Linear,
        }
    }
}

/// Result of segment linking
#[derive(Clone, Debug)]
pub struct LinkedLevel {
    /// Combined tilemap string
    pub tilemap: String,
    /// Width in tiles
    pub width: usize,
    /// Height in tiles
    pub height: usize,
    /// Segments used (for debugging)
    pub segment_names: Vec<String>,
    /// Whether linking succeeded
    pub success: bool,
    /// Layout strategy used
    pub layout: LayoutStrategy,
}

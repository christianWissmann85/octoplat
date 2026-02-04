//! Segment Linker: Creates levels by linking hand-crafted segments
//!
//! This system generates large, varied levels by stitching together
//! hand-crafted archetype levels as segments. Supports multiple layout
//! strategies: Freeform (default, snake-like with open background),
//! Linear (horizontal), Vertical (tower), and Alternating (zig-zag).

mod corridors;
mod layout_alternating;
mod layout_freeform;
mod layout_linear;
mod layout_vertical;
mod placement;
mod segment;
mod trimming;
mod types;

use octoplat_core::procgen::PooledLevel;
use octoplat_core::Rng;

pub use placement::{select_layout_strategy, select_segments};
pub use types::{
    ConnectionZone, LayoutStrategy, LinkedLevel, LinkDirection, SegmentLinkerConfig,
};

use layout_alternating::link_alternating;
use layout_freeform::link_freeform;
use layout_linear::link_linear;
use layout_vertical::link_vertical;
use segment::ParsedSegment;
use trimming::trim_tilemap_string;

/// Segment linker that combines archetype levels
pub struct SegmentLinker {
    config: SegmentLinkerConfig,
    #[allow(dead_code)] // Reserved for future randomization features
    rng: Rng,
}

impl SegmentLinker {
    pub fn new(config: SegmentLinkerConfig) -> Self {
        Self {
            rng: Rng::new(config.seed),
            config,
        }
    }

    /// Link segments together using the configured layout strategy
    pub fn link(&mut self, segments: &[&PooledLevel]) -> LinkedLevel {
        if segments.is_empty() {
            return LinkedLevel {
                tilemap: String::new(),
                width: 0,
                height: 0,
                segment_names: Vec::new(),
                success: false,
                layout: self.config.layout,
            };
        }

        // Parse all segments
        let mut parsed: Vec<ParsedSegment> = segments
            .iter()
            .filter_map(|s| ParsedSegment::from_pooled_level(s))
            .collect();

        if parsed.is_empty() {
            return LinkedLevel {
                tilemap: String::new(),
                width: 0,
                height: 0,
                segment_names: Vec::new(),
                success: false,
                layout: self.config.layout,
            };
        }

        // Normalize widths
        for seg in &mut parsed {
            seg.normalize_width();
        }

        // Normalize heights - find max height and pad shorter segments
        // This ensures segments with different heights (e.g., gauntlet at 12 vs maze at 22)
        // can be properly linked without connectivity issues
        let max_height = parsed.iter().map(|s| s.height).max().unwrap_or(20);
        let min_height = max_height.max(20); // Ensure at least 20 tiles tall for playability
        for seg in &mut parsed {
            seg.pad_to_min_height(min_height);
        }

        // Create layout based on strategy
        let mut result = match self.config.layout {
            LayoutStrategy::Linear => link_linear(&mut parsed, &self.config),
            LayoutStrategy::Vertical => link_vertical(&mut parsed, &self.config),
            LayoutStrategy::Alternating => link_alternating(&mut parsed, &self.config),
            LayoutStrategy::Freeform => link_freeform(&mut parsed, &self.config),
        };

        // Trim unnecessary wall padding from the result
        // Keep a margin of 1 tile for proper level boundaries
        if result.success {
            let (trimmed, new_width, new_height) = trim_tilemap_string(&result.tilemap, 1);
            result.tilemap = trimmed;
            result.width = new_width;
            result.height = new_height;
        }

        result
    }
}

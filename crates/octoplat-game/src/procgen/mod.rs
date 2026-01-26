//! Procedural generation module for Octoplat Game
//!
//! This module provides game-specific procedural generation functionality.
//! Core procgen types are re-exported from octoplat_core.
//!
//! Module structure:
//! - `generator` - Main ProcgenManager and level generation orchestration
//! - `difficulty` - Difficulty scaling parameters
//! - `segment_linker` - Segment linking and layout strategies
//! - `debug_export` - Debug file export for generated levels

pub mod generator;
pub mod difficulty;
pub mod segment_linker;
pub mod debug_export;

use octoplat_core::procgen::LevelArchetype;
use std::fmt;

// Re-export core procgen types
pub use octoplat_core::procgen::{BiomeId, BiomeTheme};
pub use octoplat_core::state::DifficultyPreset;

// Export local game-specific types
pub use generator::ProcgenManager;
pub use segment_linker::{
    select_layout_strategy, select_segments, ConnectionZone, LayoutStrategy,
    LinkedLevel, LinkDirection, SegmentLinker, SegmentLinkerConfig,
};

/// Errors that can occur during procedural level generation
#[derive(Debug, Clone)]
pub enum ProcgenError {
    /// Archetype pool has not been loaded or is empty
    PoolNotLoaded,

    /// No levels available for the specified biome
    NoLevelsForBiome {
        biome: BiomeId,
    },

    /// No levels match the specified criteria
    NoMatchingLevels {
        biome: BiomeId,
        archetype: Option<LevelArchetype>,
        min_tier: u8,
        max_tier: u8,
    },

    /// Archetype sequencer has not been initialized
    SequencerNotInitialized,

    /// Failed to select an appropriate archetype
    ArchetypeSelectionFailed,

    /// Failed to select segments for linking
    SegmentSelectionFailed {
        biome: BiomeId,
        min_tier: u8,
        max_tier: u8,
    },

    /// Segment linking process failed
    LinkingFailed,

    /// Generated level failed validation (not completable)
    ValidationFailed {
        issues: Vec<String>,
    },

    /// All generation retry attempts exhausted
    RetriesExhausted {
        attempts: u32,
    },
}

impl fmt::Display for ProcgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PoolNotLoaded => {
                write!(f, "Archetype pool not loaded or empty")
            }
            Self::NoLevelsForBiome { biome } => {
                write!(f, "No levels available for biome {:?}", biome)
            }
            Self::NoMatchingLevels { biome, archetype, min_tier, max_tier } => {
                if let Some(arch) = archetype {
                    write!(f, "No levels available for biome {:?}, archetype {:?}, tiers {}-{}",
                           biome, arch, min_tier, max_tier)
                } else {
                    write!(f, "No levels available for biome {:?}, tiers {}-{}",
                           biome, min_tier, max_tier)
                }
            }
            Self::SequencerNotInitialized => {
                write!(f, "Archetype sequencer not initialized")
            }
            Self::ArchetypeSelectionFailed => {
                write!(f, "Failed to select archetype")
            }
            Self::SegmentSelectionFailed { biome, min_tier, max_tier } => {
                write!(f, "Could not select segments for biome {:?}, tiers {}-{}",
                       biome, min_tier, max_tier)
            }
            Self::LinkingFailed => {
                write!(f, "Segment linking failed")
            }
            Self::ValidationFailed { issues } => {
                write!(f, "Level not completable: {}", issues.join(", "))
            }
            Self::RetriesExhausted { attempts } => {
                write!(f, "Generation failed after {} attempts", attempts)
            }
        }
    }
}

impl std::error::Error for ProcgenError {}

impl From<ProcgenError> for String {
    fn from(err: ProcgenError) -> Self {
        err.to_string()
    }
}

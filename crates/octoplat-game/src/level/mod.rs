//! Level management module
//!
//! Provides level loading, transitions, checkpoint management, and
//! dynamic level environment state.

mod environment;
mod manager;
mod visuals;

pub use environment::LevelEnvironment;
pub use manager::LevelManager;
pub use visuals::{read_level_content, setup_level_visuals, LevelVisuals};

// Re-export core level types for convenience
pub use octoplat_core::level::{
    generate_decorations_for_tilemap, generate_decorations_with_rng, Decoration, DecorationConfig, DecorationType,
    LevelData, LevelMarker, MarkerType, TileMap, TileType,
};

//! Level data structures and parsing
//!
//! Provides types for tilemaps, markers, decorations, and level loading.

pub mod decorations;
mod loader;
mod markers;
mod tilemap;

// Re-export main types
pub use decorations::{generate_decorations_for_tilemap, generate_decorations_with_rng, Decoration, DecorationConfig, DecorationType};
pub use loader::LevelData;
pub use markers::{LevelMarker, MarkerType};
pub use tilemap::{TileMap, TileType};

//! Octoplat Core - Pure logic for level generation and validation
//!
//! This crate contains platform-independent code for:
//! - Level generation (procedural, hybrid, WFC)
//! - Level validation and completability checking
//! - Core data types (Vec2, Rect, Color)
//! - Game state types
pub mod constants;
pub mod error;
pub mod level;
pub mod paths;
pub mod physics;
pub mod procgen;
pub mod rng;
pub mod save;
pub mod state;
pub mod types;

/// Default tile size in pixels used throughout the game
pub const DEFAULT_TILE_SIZE: f32 = 32.0;

// Re-export core types at crate root for convenience
pub use types::{Color, Rect, Vec2, vec2};
pub use rng::Rng;

// Re-export key types from submodules
pub use level::{
    generate_decorations_for_tilemap, generate_decorations_with_rng, Decoration, DecorationConfig, DecorationType, LevelData,
    LevelMarker, MarkerType, TileMap, TileType,
};
pub use procgen::{
    ArchetypePool, ArchetypeSequencer, BiomeId, BiomeProgression, BiomeTheme, LevelArchetype,
    LevelValidator, MechanicsRequired, MechanicsUsed, MoveType, PooledLevel, SimpleRng, TilePos,
    ValidationResult,
};
pub use state::{
    AppState, BiomeMenuItem, DeathState, DifficultyPreset, GameOverMenuItem,
    LevelCompleteMenuItem, LivesManager, MainMenuItem, PauseMenuItem, PlayerState,
    PlayMode, RogueliteRun, SettingsMenuItem,
};
pub use physics::{aabb_collision, check_ground, check_wall, CollisionResult, FeedbackTracker, Hitbox};
pub use save::{EndlessRun, SaveData, SaveManager};

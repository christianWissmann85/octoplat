//! Procedural generation module
//!
//! Contains level validation, archetype pooling, and biome systems.
//!
//! The actual level generation (segment linking) lives in octoplat-game.
//! This module provides the supporting infrastructure:
//! - Archetype pool for managing hand-crafted level segments
//! - Biome definitions for theming and progression
//! - Level validation for ensuring playability

pub mod archetype;
pub mod biome;
mod validator;

pub use biome::{BiomeId, BiomeProgression, BiomeTheme};
pub use validator::{
    LevelValidator, MechanicsRequired, MechanicsUsed, MoveType, TilePos, ValidationResult,
};
pub use archetype::{ArchetypePool, ArchetypeSequencer, LevelArchetype, PooledLevel, SimpleRng};

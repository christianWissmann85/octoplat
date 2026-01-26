//! Rendering module for Octoplat Game
//!
//! This module contains rendering utilities for the game.
//!
//! Current contents:
//! - autotile: Auto-tiling system for tile variety
//! - background: Parallax background with shaders
//! - background_objects: Biome-specific background objects (procedural)
//! - background_textures: Texture-based parallax backgrounds
//! - easing: Animation easing functions
//! - geometry: Biome-specific geometry rendering
//! - decorations: Level decoration rendering
//! - player: Player character rendering
//! - shaders: Shader management
//! - ui: HUD and debug rendering
//! - minimap: Minimap rendering
//! - world: World entity rendering

pub mod autotile;
pub mod background;
pub mod background_objects;
pub mod background_textures;
pub mod decoration_textures;
pub mod decorations;
pub mod easing;
pub mod geometry;
pub mod minimap;
pub mod player;
pub mod shaders;
pub mod tile_textures;
pub mod ui;
pub mod ui_textures;
pub mod world;

// Re-export easing functions
pub use easing::*;

// Re-export geometry functions
pub use geometry::{draw_platform_edge, draw_block_decoration};

// Re-export decoration functions
pub use decorations::{draw_decorations, draw_decorations_with_textures};

// Re-export player rendering functions
pub use player::{draw_player, draw_tentacle};

// Re-export UI functions
pub use ui::draw_hud;
#[cfg(debug_assertions)]
pub use ui::draw_debug;

// Re-export minimap functions
pub use minimap::draw_minimap;

// Re-export autotile functions
pub use autotile::{draw_autotile, get_tile_neighbors, neighbors, tile_seed};

// Re-export world rendering functions
pub use world::{
    draw_biome_background, draw_biome_particles, draw_checkpoints, draw_crab,
    draw_crumbling_platform, draw_death_effect, draw_exit, draw_gem, draw_grapple_points,
    draw_moving_platform, draw_pufferfish, draw_tilemap, draw_tilemap_themed, draw_water_pools,
};

// Re-export background objects
pub use background_objects::{BiomeBackground, draw_biome_background_layers};

// Re-export background (parallax)
pub use background::ParallaxBackground;

// Re-export shader manager
pub use shaders::ShaderManager;

// Re-export background texture system
pub use background_textures::{
    BackgroundTextureManager, BiomeTexture, draw_textured_background,
};

// Re-export decoration texture system
pub use decoration_textures::{
    DecorationTextureManager, draw_decoration_texture, position_seed,
};

// Re-export tile texture system
pub use tile_textures::{
    TileTextureManager, TextureQuality, draw_tile_texture, draw_platform_texture, draw_spike_texture,
};

// Re-export UI texture system
pub use ui_textures::{UiTextureManager, AdditionalUiTextures};

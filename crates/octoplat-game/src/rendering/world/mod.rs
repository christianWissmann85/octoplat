//! World entity rendering
//!
//! Renders tiles, enemies, platforms, collectibles, and other world entities.

mod collectibles;
mod effects;
mod entities;
mod tilemap;

pub use collectibles::{draw_checkpoints, draw_exit, draw_gem, draw_grapple_points, draw_water_pools};
pub use effects::{draw_biome_background, draw_biome_particles, draw_death_effect};
pub use entities::{draw_crab, draw_crumbling_platform, draw_moving_platform, draw_pufferfish};
pub use tilemap::{draw_tilemap, draw_tilemap_themed};

//! Decoration rendering
//!
//! Drawing functions for level decorations (seaweed, coral, rocks, etc.)
//!
//! The `primitives` module provides reusable drawing functions for common
//! decoration patterns like swaying plants and pulsing glow effects.
//!
//! Static decorations can use FLUX-generated texture sprites when available,
//! falling back to procedural primitives if no texture is loaded.

mod abyss;
mod arctic_waters;
mod coral_reefs;
mod ocean_depths;
pub mod primitives;
mod shipwreck;
mod sunken_ruins;
mod tropical_shore;
mod volcanic_vents;

use std::sync::atomic::{AtomicI32, Ordering};

use macroquad::prelude::*;

use octoplat_core::level::{Decoration, DecorationType};
use octoplat_core::procgen::biome::theme::BiomeTheme;

use super::decoration_textures::{DecorationTextureManager, draw_decoration_texture, position_seed};

#[cfg(debug_assertions)]
static LAST_DEBUG_TIME: AtomicI32 = AtomicI32::new(-1);

/// Draw all decorations in the level (with texture support)
pub fn draw_decorations_with_textures(
    decorations: &[Decoration],
    theme: &BiomeTheme,
    time: f32,
    textures: &DecorationTextureManager,
) {
    // Debug output (only on first call each second to reduce spam)
    #[cfg(debug_assertions)]
    {
        let debug_time = time as i32;
        let last_time = LAST_DEBUG_TIME.load(Ordering::Relaxed);
        if debug_time != last_time && !decorations.is_empty() {
            LAST_DEBUG_TIME.store(debug_time, Ordering::Relaxed);
            eprintln!(
                "[Decorations] Drawing {} decorations (time: {:.1}, textures: {})",
                decorations.len(),
                time,
                textures.loaded_count()
            );
        }
    }

    for deco in decorations {
        draw_decoration_with_texture(deco, theme, time, textures);
    }
}

/// Draw all decorations in the level (legacy, no textures)
pub fn draw_decorations(decorations: &[Decoration], theme: &BiomeTheme, time: f32) {
    for deco in decorations {
        draw_decoration(deco, theme, time);
    }
}

/// Draw a single decoration, using texture if available
fn draw_decoration_with_texture(
    deco: &Decoration,
    theme: &BiomeTheme,
    time: f32,
    textures: &DecorationTextureManager,
) {
    // Generate a seed from position for consistent variant selection
    let seed = position_seed(deco.position.x, deco.position.y);

    // Try to use texture for static decorations
    if let Some(texture) = textures.get(deco.decoration_type, seed) {
        let pos = to_mq_vec2(deco.position);
        draw_decoration_texture(texture, pos, deco.scale);
        return;
    }

    // Fall back to procedural rendering
    draw_decoration(deco, theme, time);
}

/// Draw a single decoration
fn draw_decoration(deco: &Decoration, theme: &BiomeTheme, time: f32) {
    let anim_time = time + deco.phase * std::f32::consts::TAU;

    match deco.decoration_type {
        // Ocean Depths
        DecorationType::Seaweed => ocean_depths::draw_seaweed(deco, theme, anim_time),
        DecorationType::Kelp => ocean_depths::draw_kelp(deco, theme, anim_time),
        DecorationType::Bubbles => ocean_depths::draw_bubbles(deco, theme, anim_time),
        DecorationType::SmallRock => ocean_depths::draw_small_rock(deco, theme),

        // Coral Reefs
        DecorationType::CoralBranch => coral_reefs::draw_coral_branch(deco, theme, anim_time),
        DecorationType::Anemone => coral_reefs::draw_anemone(deco, theme, anim_time),
        DecorationType::Shell => coral_reefs::draw_shell(deco, theme),

        // Tropical Shore
        DecorationType::PalmFrond => tropical_shore::draw_palm_frond(deco, theme, anim_time),
        DecorationType::Coconut => tropical_shore::draw_coconut(deco, theme),
        DecorationType::TropicalFlower => tropical_shore::draw_tropical_flower(deco, theme, anim_time),
        DecorationType::Starfish => tropical_shore::draw_starfish(deco, theme),

        // Shipwreck
        DecorationType::WoodDebris => shipwreck::draw_wood_debris(deco, theme),
        DecorationType::Barrel => shipwreck::draw_barrel(deco, theme),
        DecorationType::Chain => shipwreck::draw_chain(deco, theme, anim_time),
        DecorationType::Anchor => shipwreck::draw_anchor(deco, theme),

        // Arctic Waters
        DecorationType::IceShard => arctic_waters::draw_ice_shard(deco, theme, anim_time),
        DecorationType::Snowflake => arctic_waters::draw_snowflake(deco, theme, anim_time),
        DecorationType::FrostedRock => arctic_waters::draw_frosted_rock(deco, theme),
        DecorationType::IceCrystal => arctic_waters::draw_ice_crystal(deco, theme, anim_time),

        // Volcanic Vents
        DecorationType::LavaRock => volcanic_vents::draw_lava_rock(deco, theme, anim_time),
        DecorationType::SteamVent => volcanic_vents::draw_steam_vent(deco, theme, anim_time),
        DecorationType::Ash => volcanic_vents::draw_ash(deco, theme, anim_time),

        // Sunken Ruins
        DecorationType::BrokenColumn => sunken_ruins::draw_broken_column(deco, theme),
        DecorationType::AncientTile => sunken_ruins::draw_ancient_tile(deco, theme),
        DecorationType::MysticOrb => sunken_ruins::draw_mystic_orb(deco, theme, anim_time),
        DecorationType::VineGrowth => sunken_ruins::draw_vine_growth(deco, theme, anim_time),

        // Abyss
        DecorationType::Crystal => abyss::draw_crystal(deco, theme, anim_time),
        DecorationType::BioGlow => abyss::draw_bio_glow(deco, theme, anim_time),
        DecorationType::Tendril => abyss::draw_tendril(deco, theme, anim_time),
    }
}

/// Convert core Vec2 to macroquad Vec2
pub(crate) fn to_mq_vec2(v: octoplat_core::Vec2) -> Vec2 {
    vec2(v.x, v.y)
}

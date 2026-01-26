//! Level decorations - visual embellishments for biome variety
//!
//! Provides procedural props like seaweed, coral, rocks, and bubbles that
//! add visual richness without affecting gameplay.

use crate::procgen::BiomeId;
use crate::{vec2, Vec2};

use crate::Rng;

/// Generate decorations for a tilemap string
///
/// This function analyzes the tilemap and places decorations according to
/// the biome's visual style. It can be used for any level, not just procgen.
///
/// # Arguments
/// * `tilemap` - The tilemap string (lines of characters)
/// * `biome` - The biome to use for decoration types and colors
/// * `seed` - Seed for reproducible placement
/// * `tile_size` - Size of each tile in pixels (usually 32.0)
///
/// # Returns
/// A vector of decorations placed throughout the level
pub fn generate_decorations_for_tilemap(
    tilemap: &str,
    biome: BiomeId,
    seed: u64,
    tile_size: f32,
) -> Vec<Decoration> {
    let mut rng = Rng::new(seed);
    generate_decorations_with_rng(tilemap, biome, tile_size, &mut || rng.next_float())
}

/// Generate decorations for a tilemap using an external RNG
///
/// This function analyzes the tilemap and places decorations according to
/// the biome's visual style. Unlike `generate_decorations_for_tilemap`, this
/// accepts an external RNG callback, useful when you need to share RNG state
/// with other generation steps.
///
/// # Arguments
/// * `tilemap` - The tilemap string (lines of characters)
/// * `biome` - The biome to use for decoration types and colors
/// * `tile_size` - Size of each tile in pixels (usually 32.0)
/// * `rng` - Mutable callback that returns random floats in range 0.0-1.0
///
/// # Returns
/// A vector of decorations placed throughout the level
pub fn generate_decorations_with_rng<F>(
    tilemap: &str,
    biome: BiomeId,
    tile_size: f32,
    rng: &mut F,
) -> Vec<Decoration>
where
    F: FnMut() -> f32,
{
    let lines: Vec<&str> = tilemap.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    if width == 0 || height == 0 {
        return Vec::new();
    }

    // Pre-collect characters into a grid for O(1) access
    // This fixes the O(n²) performance issue from using chars().nth(x)
    let char_grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut decorations = Vec::new();

    // Get biome-specific decoration types and config
    let decoration_types = DecorationType::for_biome(biome);
    let config = DecorationConfig::for_biome(biome);

    // Helper to check if a tile is solid (O(1) access)
    let is_solid = |x: usize, y: usize| -> bool {
        if y >= height {
            return false;
        }
        char_grid[y].get(x).map(|&ch| matches!(ch, '#' | '=' | '-')).unwrap_or(false)
    };

    // Helper to check if a tile is empty (not solid, not hazard) (O(1) access)
    let is_empty = |x: usize, y: usize| -> bool {
        if y >= height {
            return false;
        }
        char_grid[y].get(x).map(|&ch| !matches!(ch, '#' | '=' | '-' | '^')).unwrap_or(false)
    };

    // Collect surface decoration types
    let surface_types: Vec<_> = decoration_types
        .iter()
        .filter(|t| !t.is_floating())
        .copied()
        .collect();

    // Collect floating decoration types
    let floating_types: Vec<_> = decoration_types
        .iter()
        .filter(|t| t.is_floating())
        .copied()
        .collect();

    // Place surface decorations on edges of solid tiles
    for y in 1..height.saturating_sub(1) {
        for x in 1..width.saturating_sub(1) {
            if !is_solid(x, y) {
                continue;
            }

            // Check for top edge (solid with empty above)
            if is_empty(x, y.saturating_sub(1))
                && rng() < config.surface_density && !surface_types.is_empty() {
                    let deco_type = surface_types[(rng() * surface_types.len() as f32) as usize % surface_types.len()];
                    let world_x = x as f32 * tile_size + tile_size * 0.5;
                    let world_y = y as f32 * tile_size - tile_size * 0.1;
                    decorations.push(Decoration::new(
                        vec2(world_x, world_y),
                        deco_type,
                        (rng() * 4.0) as u8,
                        0.7 + rng() * 0.6,
                        rng(),
                    ));
                }

            // Check for left edge (solid with empty to the left)
            if is_empty(x.saturating_sub(1), y) && rng() < config.surface_density * 0.6
                && !surface_types.is_empty() {
                    let deco_type = surface_types[(rng() * surface_types.len() as f32) as usize % surface_types.len()];
                    let world_x = x as f32 * tile_size - tile_size * 0.1;
                    let world_y = y as f32 * tile_size + tile_size * 0.5;
                    decorations.push(Decoration::new(
                        vec2(world_x, world_y),
                        deco_type,
                        (rng() * 4.0) as u8,
                        0.6 + rng() * 0.5,
                        rng(),
                    ));
                }

            // Check for right edge (solid with empty to the right)
            if x + 1 < width && is_empty(x + 1, y) && rng() < config.surface_density * 0.6
                && !surface_types.is_empty() {
                    let deco_type = surface_types[(rng() * surface_types.len() as f32) as usize % surface_types.len()];
                    let world_x = (x + 1) as f32 * tile_size + tile_size * 0.1;
                    let world_y = y as f32 * tile_size + tile_size * 0.5;
                    decorations.push(Decoration::new(
                        vec2(world_x, world_y),
                        deco_type,
                        (rng() * 4.0) as u8,
                        0.6 + rng() * 0.5,
                        rng(),
                    ));
                }
        }
    }

    // Place floating decorations in empty spaces
    for y in 2..height.saturating_sub(2) {
        for x in 2..width.saturating_sub(2) {
            if !is_empty(x, y) {
                continue;
            }

            // Sparse floating decorations
            if rng() < config.floating_density && !floating_types.is_empty() {
                let deco_type = floating_types[(rng() * floating_types.len() as f32) as usize % floating_types.len()];
                let world_x = x as f32 * tile_size + rng() * tile_size;
                let world_y = y as f32 * tile_size + rng() * tile_size;
                decorations.push(Decoration::new(
                    vec2(world_x, world_y),
                    deco_type,
                    (rng() * 4.0) as u8,
                    0.5 + rng() * 0.8,
                    rng(),
                ));
            }
        }
    }

    decorations
}

/// Types of decorations available per biome
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecorationType {
    // Ocean Depths
    Seaweed,
    Kelp,
    Bubbles,
    SmallRock,

    // Coral Reefs
    CoralBranch,
    Anemone,
    Shell,

    // Tropical Shore
    PalmFrond,
    Coconut,
    TropicalFlower,
    Starfish,

    // Shipwreck
    WoodDebris,
    Barrel,
    Chain,
    Anchor,

    // Arctic Waters
    IceShard,
    Snowflake,
    FrostedRock,
    IceCrystal,

    // Volcanic Vents
    LavaRock,
    SteamVent,
    Ash,

    // Sunken Ruins
    BrokenColumn,
    AncientTile,
    MysticOrb,
    VineGrowth,

    // Abyss
    Crystal,
    BioGlow,
    Tendril,
}

impl DecorationType {
    /// Get the decoration types available for a biome
    pub fn for_biome(biome: BiomeId) -> &'static [DecorationType] {
        match biome {
            BiomeId::OceanDepths => &[
                DecorationType::Seaweed,
                DecorationType::Kelp,
                DecorationType::Bubbles,
                DecorationType::SmallRock,
            ],
            BiomeId::CoralReefs => &[
                DecorationType::CoralBranch,
                DecorationType::Anemone,
                DecorationType::Shell,
                DecorationType::Seaweed,
            ],
            BiomeId::TropicalShore => &[
                DecorationType::PalmFrond,
                DecorationType::Coconut,
                DecorationType::TropicalFlower,
                DecorationType::Starfish,
            ],
            BiomeId::Shipwreck => &[
                DecorationType::WoodDebris,
                DecorationType::Barrel,
                DecorationType::Chain,
                DecorationType::Anchor,
            ],
            BiomeId::ArcticWaters => &[
                DecorationType::IceShard,
                DecorationType::Snowflake,
                DecorationType::FrostedRock,
                DecorationType::IceCrystal,
            ],
            BiomeId::VolcanicVents => &[
                DecorationType::LavaRock,
                DecorationType::SteamVent,
                DecorationType::Ash,
                DecorationType::SmallRock,
            ],
            BiomeId::SunkenRuins => &[
                DecorationType::BrokenColumn,
                DecorationType::AncientTile,
                DecorationType::MysticOrb,
                DecorationType::VineGrowth,
            ],
            BiomeId::Abyss => &[
                DecorationType::Crystal,
                DecorationType::BioGlow,
                DecorationType::Tendril,
                DecorationType::Bubbles,
            ],
        }
    }

    /// Whether this decoration floats in empty space (vs. attaching to surfaces)
    pub fn is_floating(&self) -> bool {
        matches!(
            self,
            DecorationType::Bubbles
                | DecorationType::Snowflake
                | DecorationType::Ash
                | DecorationType::MysticOrb
                | DecorationType::BioGlow
        )
    }
}

/// A single decoration instance in the level
#[derive(Clone, Debug)]
pub struct Decoration {
    /// World position
    pub position: Vec2,
    /// Type of decoration
    pub decoration_type: DecorationType,
    /// Visual variation index (0-3)
    pub variant: u8,
    /// Size scale factor (0.5-1.5)
    pub scale: f32,
    /// Animation phase offset (0.0-1.0) for desynchronizing animations
    pub phase: f32,
}

impl Decoration {
    /// Create a new decoration
    pub fn new(
        position: Vec2,
        decoration_type: DecorationType,
        variant: u8,
        scale: f32,
        phase: f32,
    ) -> Self {
        Self {
            position,
            decoration_type,
            variant: variant % 4,
            scale: scale.clamp(0.5, 1.5),
            phase: phase % 1.0,
        }
    }
}

/// Configuration for decoration density in a biome
#[derive(Clone, Debug)]
pub struct DecorationConfig {
    /// Density multiplier for surfaces (floor/wall decorations)
    pub surface_density: f32,
    /// Density multiplier for floating decorations
    pub floating_density: f32,
}

impl DecorationConfig {
    /// Get decoration config for a biome
    pub fn for_biome(biome: BiomeId) -> Self {
        match biome {
            BiomeId::OceanDepths => Self {
                surface_density: 0.25,
                floating_density: 0.08,
            },
            BiomeId::CoralReefs => Self {
                surface_density: 0.35,
                floating_density: 0.10,
            },
            BiomeId::TropicalShore => Self {
                surface_density: 0.30,
                floating_density: 0.06,
            },
            BiomeId::Shipwreck => Self {
                surface_density: 0.30,
                floating_density: 0.05,
            },
            BiomeId::ArcticWaters => Self {
                surface_density: 0.25,
                floating_density: 0.12,
            },
            BiomeId::VolcanicVents => Self {
                surface_density: 0.20,
                floating_density: 0.15,
            },
            BiomeId::SunkenRuins => Self {
                surface_density: 0.35,
                floating_density: 0.18,
            },
            BiomeId::Abyss => Self {
                surface_density: 0.25,
                floating_density: 0.20,
            },
        }
    }

    /// Scale density by biome progress (0.0-1.0 within biome)
    #[allow(dead_code)]
    pub fn with_progress(&self, progress: f32) -> Self {
        // Intensity scales from 0.5 to 1.0 based on progress
        let intensity = progress * 0.5 + 0.5;
        Self {
            surface_density: self.surface_density * intensity,
            floating_density: self.floating_density * intensity,
        }
    }
}

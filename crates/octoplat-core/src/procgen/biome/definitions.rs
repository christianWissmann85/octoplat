//! Biome definitions and configurations
//!
//! Each biome defines the visual style, enemy types, hazards, and special rules
//! for a section of the roguelite run.

use super::theme::{BiomeTheme, GeometryStyle};
use crate::Color;

/// Unique identifier for each biome
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BiomeId {
    /// Starting biome - calm, tutorial-friendly deep ocean
    OceanDepths,
    /// Colorful coral formations - introduces verticality
    CoralReefs,
    /// Warm tropical shores with palm trees and warm colors
    TropicalShore,
    /// Enclosed spaces in a sunken ship - darkness mechanics
    Shipwreck,
    /// Icy arctic waters with ice platforms and northern lights
    ArcticWaters,
    /// High danger volcanic area - timing challenges
    VolcanicVents,
    /// Ancient sunken ruins with columns and mysterious glow
    SunkenRuins,
    /// Final biome - maximum challenge in the deep abyss
    Abyss,
}

impl BiomeId {
    /// Get the next biome in progression order
    pub fn next(&self) -> Option<BiomeId> {
        match self {
            BiomeId::OceanDepths => Some(BiomeId::CoralReefs),
            BiomeId::CoralReefs => Some(BiomeId::TropicalShore),
            BiomeId::TropicalShore => Some(BiomeId::Shipwreck),
            BiomeId::Shipwreck => Some(BiomeId::ArcticWaters),
            BiomeId::ArcticWaters => Some(BiomeId::VolcanicVents),
            BiomeId::VolcanicVents => Some(BiomeId::SunkenRuins),
            BiomeId::SunkenRuins => Some(BiomeId::Abyss),
            BiomeId::Abyss => None, // Final biome, loops
        }
    }

    /// Get the biome definition
    pub fn definition(&self) -> &'static Biome {
        match self {
            BiomeId::OceanDepths => &OCEAN_DEPTHS,
            BiomeId::CoralReefs => &CORAL_REEFS,
            BiomeId::TropicalShore => &TROPICAL_SHORE,
            BiomeId::Shipwreck => &SHIPWRECK,
            BiomeId::ArcticWaters => &ARCTIC_WATERS,
            BiomeId::VolcanicVents => &VOLCANIC_VENTS,
            BiomeId::SunkenRuins => &SUNKEN_RUINS,
            BiomeId::Abyss => &ABYSS,
        }
    }

    /// Get all biomes in order
    pub fn all() -> &'static [BiomeId] {
        &[
            BiomeId::OceanDepths,
            BiomeId::CoralReefs,
            BiomeId::TropicalShore,
            BiomeId::Shipwreck,
            BiomeId::ArcticWaters,
            BiomeId::VolcanicVents,
            BiomeId::SunkenRuins,
            BiomeId::Abyss,
        ]
    }

    /// Get the string identifier for this biome (for level files)
    pub fn as_str(&self) -> &'static str {
        match self {
            BiomeId::OceanDepths => "ocean_depths",
            BiomeId::CoralReefs => "coral_reefs",
            BiomeId::TropicalShore => "tropical_shore",
            BiomeId::Shipwreck => "shipwreck",
            BiomeId::ArcticWaters => "arctic_waters",
            BiomeId::VolcanicVents => "volcanic_vents",
            BiomeId::SunkenRuins => "sunken_ruins",
            BiomeId::Abyss => "abyss",
        }
    }

    /// Parse a biome from a string identifier
    pub fn parse(s: &str) -> Option<BiomeId> {
        match s.to_lowercase().trim() {
            "ocean_depths" | "oceandepths" | "ocean" | "ocean-depths" => Some(BiomeId::OceanDepths),
            "coral_reefs" | "coralreefs" | "coral" | "coral-reefs" => Some(BiomeId::CoralReefs),
            "tropical_shore" | "tropicalshore" | "tropical" | "tropical-shore" | "shore" => {
                Some(BiomeId::TropicalShore)
            }
            "shipwreck" | "ship" => Some(BiomeId::Shipwreck),
            "arctic_waters" | "arcticwaters" | "arctic" | "arctic-waters" | "ice" => {
                Some(BiomeId::ArcticWaters)
            }
            "volcanic_vents" | "volcanicvents" | "volcanic" | "vents" | "volcanic-vents" => {
                Some(BiomeId::VolcanicVents)
            }
            "sunken_ruins" | "sunkenruins" | "ruins" | "sunken-ruins" | "ancient" => {
                Some(BiomeId::SunkenRuins)
            }
            "abyss" | "the_abyss" | "the-abyss" => Some(BiomeId::Abyss),
            _ => None,
        }
    }

    /// Get the display name for this biome
    pub fn display_name(&self) -> &'static str {
        self.definition().name
    }
}

/// Enemy types available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Crab,
    Pufferfish,
    // Future enemies can be added here
}

/// Hazard types available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HazardType {
    Spike,
    BouncePad,
    // Future hazards can be added here
}

/// Enemy roster configuration for a biome
#[derive(Clone, Debug)]
pub struct EnemyRoster {
    /// Available enemy types and their spawn weights
    pub enemies: &'static [(EnemyType, f32)],
}

impl EnemyRoster {
    pub const fn new(enemies: &'static [(EnemyType, f32)]) -> Self {
        Self { enemies }
    }
}

/// Hazard roster configuration for a biome
#[derive(Clone, Debug)]
pub struct HazardRoster {
    /// Available hazard types and their spawn weights
    pub hazards: &'static [(HazardType, f32)],
}

impl HazardRoster {
    pub const fn new(hazards: &'static [(HazardType, f32)]) -> Self {
        Self { hazards }
    }
}

/// Complete biome definition
#[derive(Clone, Debug)]
pub struct Biome {
    /// Unique identifier
    pub id: BiomeId,
    /// Display name
    pub name: &'static str,
    /// Visual theme configuration
    pub theme: BiomeTheme,
    /// Difficulty modifier (1.0 = base, higher = harder)
    pub difficulty_modifier: f32,
    /// Available enemies in this biome
    pub enemy_roster: EnemyRoster,
    /// Available hazards in this biome
    pub hazard_roster: HazardRoster,
    /// Number of levels in this biome before progression
    pub levels_in_biome: u8,
}

// ============================================================================
// Biome Definitions
// ============================================================================

static OCEAN_DEPTHS: Biome = Biome {
    id: BiomeId::OceanDepths,
    name: "Ocean Depths",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.05, 0.15, 0.25, 1.0),
        bg_color_bottom: Color::new(0.02, 0.08, 0.15, 1.0),
        solid_color: Color::new(0.2, 0.35, 0.45, 1.0),
        platform_color: Color::new(0.3, 0.45, 0.55, 1.0),
        hazard_color: Color::new(0.8, 0.3, 0.3, 1.0),
        accent_color: Color::new(0.4, 0.7, 0.9, 1.0),
        particle_color: Color::new(0.6, 0.8, 1.0, 0.5),
        geometry_style: GeometryStyle::Standard,
        glow_color: None,
    },
    difficulty_modifier: 0.8,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.8), (EnemyType::Pufferfish, 0.2)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.6), (HazardType::BouncePad, 0.4)]),
    levels_in_biome: 4,
};

static CORAL_REEFS: Biome = Biome {
    id: BiomeId::CoralReefs,
    name: "Coral Reefs",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.1, 0.25, 0.35, 1.0),
        bg_color_bottom: Color::new(0.05, 0.15, 0.25, 1.0),
        solid_color: Color::new(0.7, 0.4, 0.5, 1.0),
        platform_color: Color::new(0.5, 0.7, 0.4, 1.0),
        hazard_color: Color::new(0.9, 0.4, 0.4, 1.0),
        accent_color: Color::new(1.0, 0.6, 0.8, 1.0),
        particle_color: Color::new(1.0, 0.8, 0.6, 0.6),
        geometry_style: GeometryStyle::Organic,
        glow_color: Some(Color::new(1.0, 0.6, 0.8, 0.4)),
    },
    difficulty_modifier: 1.0,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.5), (EnemyType::Pufferfish, 0.5)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.5), (HazardType::BouncePad, 0.5)]),
    levels_in_biome: 4,
};

static TROPICAL_SHORE: Biome = Biome {
    id: BiomeId::TropicalShore,
    name: "Tropical Shore",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.4, 0.7, 0.9, 1.0),    // Bright sky blue
        bg_color_bottom: Color::new(0.1, 0.4, 0.5, 1.0), // Deeper turquoise
        solid_color: Color::new(0.85, 0.75, 0.55, 1.0),  // Sandy tan
        platform_color: Color::new(0.6, 0.45, 0.3, 1.0), // Wood/bark brown
        hazard_color: Color::new(0.9, 0.5, 0.3, 1.0),    // Coral orange
        accent_color: Color::new(0.2, 0.8, 0.4, 1.0),    // Palm green
        particle_color: Color::new(1.0, 1.0, 0.8, 0.5),  // Sunlit particles
        geometry_style: GeometryStyle::Tropical,
        glow_color: Some(Color::new(1.0, 0.9, 0.6, 0.3)), // Warm sun glow
    },
    difficulty_modifier: 0.9,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.7), (EnemyType::Pufferfish, 0.3)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.4), (HazardType::BouncePad, 0.6)]),
    levels_in_biome: 4,
};

static SHIPWRECK: Biome = Biome {
    id: BiomeId::Shipwreck,
    name: "Shipwreck",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.08, 0.1, 0.12, 1.0),
        bg_color_bottom: Color::new(0.04, 0.05, 0.08, 1.0),
        solid_color: Color::new(0.35, 0.25, 0.2, 1.0),
        platform_color: Color::new(0.45, 0.35, 0.25, 1.0),
        hazard_color: Color::new(0.7, 0.5, 0.3, 1.0),
        accent_color: Color::new(0.8, 0.7, 0.5, 1.0),
        particle_color: Color::new(0.5, 0.4, 0.3, 0.4),
        geometry_style: GeometryStyle::Broken,
        glow_color: None,
    },
    difficulty_modifier: 1.2,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.6), (EnemyType::Pufferfish, 0.4)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.7), (HazardType::BouncePad, 0.3)]),
    levels_in_biome: 4,
};

static ARCTIC_WATERS: Biome = Biome {
    id: BiomeId::ArcticWaters,
    name: "Arctic Waters",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.1, 0.15, 0.25, 1.0),   // Dark arctic sky
        bg_color_bottom: Color::new(0.05, 0.1, 0.2, 1.0), // Deep cold blue
        solid_color: Color::new(0.7, 0.85, 0.95, 1.0),    // Ice blue-white
        platform_color: Color::new(0.5, 0.7, 0.85, 1.0),  // Lighter ice
        hazard_color: Color::new(0.3, 0.6, 0.9, 1.0),     // Cold blue hazards
        accent_color: Color::new(0.4, 0.9, 0.7, 1.0),     // Aurora green
        particle_color: Color::new(0.8, 0.9, 1.0, 0.6),   // Snow particles
        geometry_style: GeometryStyle::Icy,
        glow_color: Some(Color::new(0.3, 0.9, 0.6, 0.4)), // Northern lights glow
    },
    difficulty_modifier: 1.1,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.5), (EnemyType::Pufferfish, 0.5)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.6), (HazardType::BouncePad, 0.4)]),
    levels_in_biome: 4,
};

static VOLCANIC_VENTS: Biome = Biome {
    id: BiomeId::VolcanicVents,
    name: "Volcanic Vents",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.2, 0.08, 0.05, 1.0),
        bg_color_bottom: Color::new(0.1, 0.04, 0.02, 1.0),
        solid_color: Color::new(0.3, 0.2, 0.15, 1.0),
        platform_color: Color::new(0.5, 0.3, 0.2, 1.0),
        hazard_color: Color::new(1.0, 0.4, 0.1, 1.0),
        accent_color: Color::new(1.0, 0.6, 0.2, 1.0),
        particle_color: Color::new(1.0, 0.5, 0.2, 0.7),
        geometry_style: GeometryStyle::Jagged,
        glow_color: Some(Color::new(1.0, 0.4, 0.1, 0.5)),
    },
    difficulty_modifier: 1.4,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.4), (EnemyType::Pufferfish, 0.6)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.8), (HazardType::BouncePad, 0.2)]),
    levels_in_biome: 4,
};

static SUNKEN_RUINS: Biome = Biome {
    id: BiomeId::SunkenRuins,
    name: "Sunken Ruins",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.08, 0.1, 0.15, 1.0),   // Mysterious dark blue
        bg_color_bottom: Color::new(0.04, 0.06, 0.1, 1.0), // Deep shadow
        solid_color: Color::new(0.5, 0.5, 0.45, 1.0),     // Ancient stone gray
        platform_color: Color::new(0.6, 0.55, 0.5, 1.0),  // Weathered stone
        hazard_color: Color::new(0.6, 0.4, 0.7, 1.0),     // Mystic purple
        accent_color: Color::new(0.4, 0.8, 0.7, 1.0),     // Ethereal teal
        particle_color: Color::new(0.5, 0.7, 0.8, 0.5),   // Mystical motes
        geometry_style: GeometryStyle::Ancient,
        glow_color: Some(Color::new(0.4, 0.7, 0.9, 0.5)), // Mysterious glow
    },
    difficulty_modifier: 1.5,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.35), (EnemyType::Pufferfish, 0.65)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.85), (HazardType::BouncePad, 0.15)]),
    levels_in_biome: 4,
};

static ABYSS: Biome = Biome {
    id: BiomeId::Abyss,
    name: "The Abyss",
    theme: BiomeTheme {
        bg_color_top: Color::new(0.02, 0.02, 0.05, 1.0),
        bg_color_bottom: Color::new(0.0, 0.0, 0.02, 1.0),
        solid_color: Color::new(0.15, 0.1, 0.2, 1.0),
        platform_color: Color::new(0.25, 0.2, 0.3, 1.0),
        hazard_color: Color::new(0.6, 0.2, 0.8, 1.0),
        accent_color: Color::new(0.5, 0.3, 0.9, 1.0),
        particle_color: Color::new(0.4, 0.2, 0.6, 0.5),
        geometry_style: GeometryStyle::Crystalline,
        glow_color: Some(Color::new(0.5, 0.3, 0.9, 0.6)),
    },
    difficulty_modifier: 1.6,
    enemy_roster: EnemyRoster::new(&[(EnemyType::Crab, 0.3), (EnemyType::Pufferfish, 0.7)]),
    hazard_roster: HazardRoster::new(&[(HazardType::Spike, 0.9), (HazardType::BouncePad, 0.1)]),
    levels_in_biome: 5, // Final biome is longer
};

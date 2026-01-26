//! Biome system for themed procedural generation
//!
//! Biomes define themed "worlds" that players progress through during a roguelite run.
//! Each biome has unique visual themes, enemy rosters, hazard types, and special rules.

mod definitions;
pub mod theme;

pub use definitions::{Biome, BiomeId, EnemyType, HazardType};
pub use theme::BiomeTheme;

/// Biome progression manager for roguelite runs
#[derive(Clone, Debug)]
pub struct BiomeProgression {
    /// Current biome being played
    current_biome: BiomeId,
    /// Number of levels completed in current biome
    levels_in_current: u8,
    /// Total levels completed across all biomes
    total_levels: u32,
    /// If set, locks progression to this single biome (Biome Challenge mode)
    locked_biome: Option<BiomeId>,
}

impl BiomeProgression {
    pub fn new() -> Self {
        Self {
            current_biome: BiomeId::OceanDepths,
            levels_in_current: 0,
            total_levels: 0,
            locked_biome: None,
        }
    }

    /// Lock progression to a specific biome
    pub fn set_locked_biome(&mut self, biome: Option<BiomeId>) {
        self.locked_biome = biome;
        if let Some(b) = biome {
            self.current_biome = b;
            self.levels_in_current = 0;
        }
    }

    /// Get the current biome
    pub fn current(&self) -> &Biome {
        self.current_biome.definition()
    }

    /// Get the current biome ID
    pub fn current_id(&self) -> BiomeId {
        self.current_biome
    }

    /// Progress to the next level, potentially advancing to a new biome
    /// Returns true if we advanced to a new biome
    pub fn advance_level(&mut self) -> bool {
        self.levels_in_current += 1;
        self.total_levels += 1;

        // If locked to a biome, never advance - just reset counter for endless play
        if self.locked_biome.is_some() {
            let biome = self.current_biome.definition();
            if self.levels_in_current >= biome.levels_in_biome {
                self.levels_in_current = 0;
            }
            return false;
        }

        let biome = self.current_biome.definition();
        if self.levels_in_current >= biome.levels_in_biome {
            // Time to advance to next biome
            if let Some(next) = self.current_biome.next() {
                self.current_biome = next;
                self.levels_in_current = 0;
                return true;
            }
            // If no next biome (at Abyss), stay but reset counter for endless play
            self.levels_in_current = 0;
        }
        false
    }

    /// Full reset including clearing locked biome
    pub fn full_reset(&mut self) {
        self.current_biome = BiomeId::OceanDepths;
        self.levels_in_current = 0;
        self.total_levels = 0;
        self.locked_biome = None;
    }

    /// Get progress through current biome (0.0 to 1.0)
    ///
    /// # Safety
    ///
    /// This function includes a debug assertion that `levels_in_biome > 0`.
    /// In release builds, returns 0.0 if the biome has no levels defined.
    pub fn biome_progress(&self) -> f32 {
        let biome = self.current_biome.definition();
        debug_assert!(
            biome.levels_in_biome > 0,
            "Biome {:?} has levels_in_biome = 0, which would cause division by zero",
            self.current_biome
        );
        if biome.levels_in_biome == 0 {
            return 0.0;
        }
        self.levels_in_current as f32 / biome.levels_in_biome as f32
    }

    /// Get overall run progress (0.0 to 1.0, based on biome progression)
    pub fn run_progress(&self) -> f32 {
        // 8 biomes total, each gets ~0.125 weight (except Abyss caps at 0.875)
        let biome_weight = match self.current_biome {
            BiomeId::OceanDepths => 0.0,
            BiomeId::CoralReefs => 0.125,
            BiomeId::TropicalShore => 0.25,
            BiomeId::Shipwreck => 0.375,
            BiomeId::ArcticWaters => 0.5,
            BiomeId::VolcanicVents => 0.625,
            BiomeId::SunkenRuins => 0.75,
            BiomeId::Abyss => 0.875,
        };
        biome_weight + self.biome_progress() * 0.125
    }

    /// Check if current level should be a boss level (end of biome)
    ///
    /// Returns `true` if this is the last level in the current biome.
    /// Returns `false` if the biome has no levels defined (defensive fallback).
    pub fn is_boss_level(&self) -> bool {
        let biome = self.current_biome.definition();
        // Use saturating_sub to prevent underflow if levels_in_biome is 0
        biome.levels_in_biome > 0
            && self.levels_in_current == biome.levels_in_biome.saturating_sub(1)
    }

    /// Get total levels completed
    pub fn total_levels(&self) -> u32 {
        self.total_levels
    }
}

impl Default for BiomeProgression {
    fn default() -> Self {
        Self::new()
    }
}

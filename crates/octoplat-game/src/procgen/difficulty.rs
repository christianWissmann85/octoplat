//! Difficulty scaling parameters for procedural generation
//!
//! Controls how level elements scale based on progression and difficulty preset.

use octoplat_core::state::DifficultyPreset;

/// Difficulty parameters that scale based on progress through the run
#[derive(Clone, Debug)]
pub struct DifficultyParams {
    /// Progress through the run (0.0 = start, 1.0 = end)
    pub progress: f32,
    /// Chance for collectible slots to spawn gems (decreases slightly)
    pub collectible_chance: f32,
    /// Chance for enemy slots to spawn enemies (increases)
    pub enemy_chance: f32,
    /// Chance for pufferfish vs crab when spawning enemy (increases)
    pub pufferfish_chance: f32,
    /// Chance for optional hazards to spawn (increases)
    pub hazard_chance: f32,
    /// Chance for optional grapple points (decreases slightly for challenge)
    pub grapple_chance: f32,
    /// Allowed difficulty tiers at this point
    pub min_tier: u8,
    pub max_tier: u8,
}

impl DifficultyParams {
    /// Calculate difficulty parameters for a given progress point
    ///
    /// Progress: 0.0 = beginning of run, 1.0 = end of run
    /// Difficulty preset affects the scaling curves
    pub fn for_progress(progress: f32, preset: DifficultyPreset) -> Self {
        let progress = progress.clamp(0.0, 1.0);

        // Base values that scale with progress
        let (base_enemy, max_enemy) = match preset {
            DifficultyPreset::Casual => (0.2, 0.4),
            DifficultyPreset::Standard => (0.3, 0.6),
            DifficultyPreset::Challenge => (0.4, 0.8),
        };

        let (base_hazard, max_hazard) = match preset {
            DifficultyPreset::Casual => (0.2, 0.35),
            DifficultyPreset::Standard => (0.3, 0.5),
            DifficultyPreset::Challenge => (0.4, 0.7),
        };

        let (base_puffer, max_puffer) = match preset {
            DifficultyPreset::Casual => (0.0, 0.15),
            DifficultyPreset::Standard => (0.1, 0.35),
            DifficultyPreset::Challenge => (0.2, 0.5),
        };

        // Grapple points decrease slightly as difficulty increases (fewer safety nets)
        let (base_grapple, min_grapple) = match preset {
            DifficultyPreset::Casual => (0.85, 0.75),
            DifficultyPreset::Standard => (0.8, 0.6),
            DifficultyPreset::Challenge => (0.7, 0.5),
        };

        // Collectibles stay fairly consistent but decrease slightly
        let (base_collect, min_collect) = match preset {
            DifficultyPreset::Casual => (0.7, 0.6),
            DifficultyPreset::Standard => (0.65, 0.5),
            DifficultyPreset::Challenge => (0.6, 0.4),
        };

        // Tier progression based on progress
        // 0-20%:  Tier 1 only
        // 20-40%: Tier 1-2
        // 40-60%: Tier 2-3
        // 60-80%: Tier 2-4
        // 80-100%: Tier 3-5
        let (min_tier, max_tier) = if progress < 0.2 {
            (1, 1)
        } else if progress < 0.4 {
            (1, 2)
        } else if progress < 0.6 {
            (2, 3)
        } else if progress < 0.8 {
            (2, 4)
        } else {
            (3, 5)
        };

        // Clamp max_tier based on preset
        let max_tier = match preset {
            DifficultyPreset::Casual => max_tier.min(2),
            DifficultyPreset::Standard => max_tier.min(4),
            DifficultyPreset::Challenge => max_tier,
        };

        Self {
            progress,
            collectible_chance: lerp(base_collect, min_collect, progress),
            enemy_chance: lerp(base_enemy, max_enemy, progress),
            pufferfish_chance: lerp(base_puffer, max_puffer, progress),
            hazard_chance: lerp(base_hazard, max_hazard, progress),
            grapple_chance: lerp(base_grapple, min_grapple, progress),
            min_tier,
            max_tier,
        }
    }
}

/// Linear interpolation helper
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

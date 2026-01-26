//! RogueLite run state management
//!
//! Tracks all state for a single roguelite run.

use crate::procgen::{BiomeId, BiomeProgression};
use super::DifficultyPreset;

/// Manages a single roguelite run
#[derive(Clone, Debug)]
pub struct RogueliteRun {
    /// Whether roguelite mode is active
    pub active: bool,
    /// Number of levels completed in this run
    pub level_count: u32,
    /// Total gems collected across all levels in this run
    pub total_gems: u32,
    /// Starting seed for the run (None = random)
    pub start_seed: Option<u64>,
    /// Total time spent in this run
    pub run_time: f32,
    /// Deaths in this run
    pub run_deaths: u32,
    /// Biome progression tracker
    pub biome_progression: BiomeProgression,
    /// Difficulty preset for this run
    pub preset: DifficultyPreset,
}

impl RogueliteRun {
    pub fn new() -> Self {
        Self {
            active: false,
            level_count: 0,
            total_gems: 0,
            start_seed: None,
            run_time: 0.0,
            run_deaths: 0,
            biome_progression: BiomeProgression::new(),
            preset: DifficultyPreset::Standard,
        }
    }

    /// Start a new roguelite run locked to a specific biome
    pub fn start_biome_challenge(&mut self, biome: BiomeId, preset: DifficultyPreset, seed: Option<u64>) {
        self.active = true;
        self.level_count = 0;
        self.total_gems = 0;
        self.start_seed = seed;
        self.run_time = 0.0;
        self.run_deaths = 0;
        self.preset = preset;
        self.biome_progression.set_locked_biome(Some(biome));
    }

    /// Record a death in this run
    pub fn record_death(&mut self) {
        self.run_deaths += 1;
    }

    /// Update run time
    pub fn update_time(&mut self, dt: f32) {
        self.run_time += dt;
    }

    /// Capture the starting seed if not already set
    pub fn capture_seed(&mut self, seed: Option<u64>) {
        if self.start_seed.is_none() {
            self.start_seed = seed;
        }
    }
}

impl Default for RogueliteRun {
    fn default() -> Self {
        Self::new()
    }
}

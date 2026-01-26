//! Progression manager subsystem
//!
//! Coordinates save data, lives system, and roguelite run state.
//!
//! # Roguelite Run State
//!
//! The roguelite "active" state is tracked internally and should be accessed
//! via `is_in_roguelite_run()`. The state persists across `AppState` transitions
//! (e.g., Playing -> GameOver) so that run statistics can be recorded properly.
//!
//! - Use `start_run()` or `start_biome_challenge()` to begin a run
//! - Use `end_run()` when returning to menu or on generation failure
//! - Use `is_in_roguelite_run()` to check if currently in a roguelite context

use crate::roguelite::RogueliteRun;
use octoplat_core::procgen::BiomeId;
use octoplat_core::save::SaveManager;
use octoplat_core::state::{DifficultyPreset, LivesManager};

/// Manages game progression, saves, and run state.
///
/// This subsystem handles:
/// - Save data persistence (via SaveManager)
/// - Lives system and death tracking
/// - Roguelite run state and progression
pub struct ProgressionManager {
    /// Save data persistence
    pub save_manager: SaveManager,

    /// Lives system
    pub lives: LivesManager,

    /// Roguelite run state
    pub roguelite: RogueliteRun,
}

impl ProgressionManager {
    /// Create a new ProgressionManager with default starting lives
    pub fn new(starting_lives: u32) -> Self {
        Self {
            save_manager: SaveManager::new(),
            lives: LivesManager::new(starting_lives),
            roguelite: RogueliteRun::new(),
        }
    }

    /// Check if currently in a roguelite run context
    ///
    /// This returns true from the moment a roguelite run starts until `end_run()` is called.
    /// The state persists across `AppState` transitions (Playing -> GameOver, etc.)
    /// so that run statistics can be properly tracked and recorded.
    #[inline]
    pub fn is_in_roguelite_run(&self) -> bool {
        self.roguelite.active
    }

    /// Record a death and update lives
    ///
    /// Returns true if game over (no lives remaining)
    pub fn record_death(&mut self) -> bool {
        self.lives.session_deaths += 1;

        if self.is_in_roguelite_run() {
            self.roguelite.record_death();
        }

        // Decrement lives
        if self.lives.current > 0 && self.lives.current != u32::MAX {
            self.lives.current -= 1;
        }

        self.is_game_over()
    }

    /// Check if game is over (no lives remaining)
    pub fn is_game_over(&self) -> bool {
        self.lives.current == 0
    }

    /// Award an extra life
    ///
    /// Returns true if life was awarded (wasn't at max)
    pub fn award_extra_life(&mut self, max_lives: u32) -> bool {
        self.lives.award_life(max_lives)
    }

    /// Check if we should award a gem milestone life
    ///
    /// Returns true if a life should be awarded based on gems collected
    pub fn check_gem_milestone(&mut self, total_gems: u32, max_lives: u32) -> bool {
        if total_gems >= self.lives.next_life_gems {
            self.lives.next_life_gems += 50; // Next milestone
            self.award_extra_life(max_lives)
        } else {
            false
        }
    }

    /// Start a new roguelite run
    pub fn start_run(&mut self, starting_lives: u32, seed: Option<u64>) {
        self.lives = LivesManager::new(starting_lives);
        self.lives.start_session(starting_lives, 50, true);
        self.roguelite = RogueliteRun::new();
        self.roguelite.active = true;
        self.roguelite.start_seed = seed;
    }

    /// Start a biome challenge run
    pub fn start_biome_challenge(
        &mut self,
        biome: BiomeId,
        preset: DifficultyPreset,
        seed: Option<u64>,
        starting_lives: u32,
    ) {
        self.lives = LivesManager::new(starting_lives);
        self.lives.start_session(starting_lives, 50, true);
        self.roguelite.start_biome_challenge(biome, preset, seed);
    }

    /// End the current run
    pub fn end_run(&mut self) {
        self.roguelite.active = false;
    }

    /// Reset session deaths (for level restart)
    pub fn reset_session(&mut self) {
        self.lives.reset_session();
    }

    /// Update run time
    pub fn update_run_time(&mut self, dt: f32) {
        if self.is_in_roguelite_run() {
            self.roguelite.update_time(dt);
        }
    }

    /// Complete a level in roguelite mode
    pub fn complete_level(&mut self, gems_collected: u32) {
        if self.is_in_roguelite_run() {
            self.roguelite.level_count += 1;
            self.roguelite.total_gems += gems_collected;
        }
    }

    /// Get current run stats
    pub fn run_stats(&self) -> RunStats {
        RunStats {
            levels_completed: self.roguelite.level_count,
            total_gems: self.roguelite.total_gems,
            run_time: self.roguelite.run_time,
            run_deaths: self.roguelite.run_deaths,
            current_lives: self.lives.current,
        }
    }
}

/// Summary statistics for the current run
#[derive(Clone, Debug)]
pub struct RunStats {
    pub levels_completed: u32,
    pub total_gems: u32,
    pub run_time: f32,
    pub run_deaths: u32,
    pub current_lives: u32,
}

impl Default for ProgressionManager {
    fn default() -> Self {
        Self::new(5)
    }
}

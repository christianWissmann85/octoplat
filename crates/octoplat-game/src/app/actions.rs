//! Game actions for handler communication
//!
//! Actions that can be requested by state handlers to be executed by the main loop.
//!
//! ## Mutation Strategy
//!
//! This codebase uses a hybrid approach for state mutations:
//!
//! **Use Actions for:**
//! - Cross-system state changes (e.g., level completion triggers save, audio, UI updates)
//! - State transitions between app states
//! - Operations that need coordination between multiple subsystems
//!
//! **Direct mutation is acceptable for:**
//! - UI-local state (menu selection indices)
//! - Per-frame updates in hot loops (player position, timers)
//! - Single-subsystem state that doesn't affect others
//!
//! When in doubt, prefer actions for clarity and traceability.

use crate::app_state::AppState;
use crate::audio::{MusicTrack, SoundId};
use crate::procgen::{BiomeId, DifficultyPreset};
use octoplat_core::state::GameplayDifficulty;

/// Menu identifiers for menu state reset actions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MenuId {
    Main,
    Pause,
    GameOver,
    Settings,
    BiomeSelect,
    DifficultySelect,
    LevelComplete,
    Error,
    RogueLiteLeaderboard,
}

/// Actions that can be requested by handlers
#[derive(Clone, Debug)]
pub enum GameAction {
    // ========================================================================
    // State Transitions
    // ========================================================================

    /// Transition to a new app state with fade animation
    TransitionTo(AppState),

    /// Immediately set app state without transition
    SetStateDirect(AppState),

    /// Return to main menu (records roguelite run, clears state)
    ReturnToMenu,

    // ========================================================================
    // Menu State
    // ========================================================================

    /// Reset a menu's selection to the first item
    ResetMenuSelection(MenuId),

    /// Set the return state for settings menu
    SetSettingsReturnState(AppState),

    /// Start a menu slide transition
    StartMenuSlide(crate::ui::SlideDirection),

    // ========================================================================
    // Audio
    // ========================================================================

    /// Play a sound effect
    PlaySound(SoundId),

    /// Play a music track immediately
    PlayMusic(MusicTrack),

    /// Crossfade to a music track over duration
    CrossfadeMusic { track: MusicTrack, duration: f32 },

    /// Crossfade to biome music
    CrossfadeToBiomeMusic { biome: BiomeId, duration: f32 },

    /// Stop all music
    StopMusic,

    /// Pause music
    PauseMusic,

    /// Resume music
    ResumeMusic,

    /// Play ambient sounds for a biome
    PlayBiomeAmbient(BiomeId),

    /// Stop ambient sounds
    StopAmbient,

    /// Pause ambient sounds
    PauseAmbient,

    /// Resume ambient sounds
    ResumeAmbient,

    // ========================================================================
    // Gameplay - Player
    // ========================================================================

    /// Trigger player death (checks invincibility, plays effects)
    TriggerDeath,

    /// Take damage from a hazard or enemy
    /// Returns TriggerDeath if HP reaches 0
    TakeDamage {
        amount: u8,
        source_pos: macroquad::prelude::Vec2,
    },

    /// Respawn player at checkpoint
    Respawn,

    /// Game over (out of lives)
    GameOver,

    /// Award an extra life
    AwardExtraLife,

    // ========================================================================
    // Gameplay - Level
    // ========================================================================

    /// Restart the current level (resets player, lives, time)
    RestartLevel,

    /// Mark level as complete (triggers completion flow)
    MarkLevelComplete,

    /// Set the level text display timer
    SetLevelTextTimer(f32),

    /// Progress to next level
    NextLevel,

    // ========================================================================
    // Gameplay - UI
    // ========================================================================

    /// Toggle minimap visibility
    ToggleMinimap,

    /// Adjust minimap zoom scale by delta
    AdjustMinimapScale(f32),

    // ========================================================================
    // Roguelite Mode
    // ========================================================================

    /// Start a biome challenge roguelite run
    StartBiomeChallenge {
        biome: BiomeId,
        preset: DifficultyPreset,
        seed: Option<u64>,
    },

    /// Start linked segments mode (merged into roguelite)
    StartLinkedSegments {
        biome: BiomeId,
        preset: DifficultyPreset,
        seed: Option<u64>,
        segment_count: usize,
    },

    /// Complete current roguelite level and generate next
    CompleteRogueliteLevel,

    /// Set gameplay difficulty - applies HP, i-frames, enemy speed, and starting lives
    SetGameplayDifficulty(GameplayDifficulty),

    /// Exit roguelite mode and return to first level
    ExitRogueliteMode,

    // ========================================================================
    // Debug/Development
    // ========================================================================

    /// Start a procgen debug run
    StartProcgenRun {
        preset: DifficultyPreset,
        seed: Option<u64>,
    },
}

/// Batch of actions to be executed
#[derive(Clone, Debug, Default)]
pub struct GameActions(pub Vec<GameAction>);

impl GameActions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, action: GameAction) {
        self.0.push(action);
    }

    pub fn extend(&mut self, other: GameActions) {
        self.0.extend(other.0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for GameActions {
    type Item = GameAction;
    type IntoIter = std::vec::IntoIter<GameAction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<GameAction> for GameActions {
    fn from_iter<I: IntoIterator<Item = GameAction>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

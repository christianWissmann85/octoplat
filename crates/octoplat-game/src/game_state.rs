//! Game state that persists across level transitions
//!
//! Contains all game data needed for gameplay.
//!
//! GameState delegates to focused subsystems:
//! - StateController: app state machine and transitions
//! - EffectsController: audio, particles, shaders, feedback
//! - GameplayEngine: player, camera, level environment, death state
//! - ProgressionManager: saves, lives, roguelite run state
//! - UIState: menus, minimap visibility, seed input
//! - LevelState: level manager, backgrounds
//!
//! Access subsystem data directly (e.g., `game.gameplay.player`).
//! Convenience methods exist only for cross-subsystem coordination.

use macroquad::prelude::*;

use crate::app::SeedInputState;
use crate::effects::EffectsController;
use crate::gamepad::GamepadManager;
use crate::gameplay::GameplayEngine;
use crate::input::InputState;
use crate::level::LevelManager;
use crate::procgen::ProcgenManager;
use crate::progression::ProgressionManager;
use crate::rendering::{BiomeBackground, ParallaxBackground, BackgroundTextureManager, DecorationTextureManager, TileTextureManager, UiTextureManager};
use crate::state::StateController;
use crate::ui::GameMenus;

// ============================================================================
// UIState - Menu and UI-related state
// ============================================================================

use crate::procgen::{BiomeId, DifficultyPreset};

/// Cached HUD strings to avoid per-frame allocations
pub struct HudStringCache {
    // Roguelite HUD text
    pub roguelite_text: String,
    pub roguelite_key: (u32, u32), // (level, total_gems)

    // Biome progress text
    pub biome_text: String,
    pub biome_key: (BiomeId, u32, u32, bool), // (biome_id, biome_progress, run_progress, is_boss)

    // Seed display text
    pub seed_text: String,
    pub seed_key: (Option<u64>, DifficultyPreset), // (seed, preset)
}

impl Default for HudStringCache {
    fn default() -> Self {
        Self {
            roguelite_text: String::new(),
            roguelite_key: (0, 0),
            biome_text: String::new(),
            biome_key: (BiomeId::OceanDepths, 0, 0, false),
            seed_text: String::new(),
            seed_key: (None, DifficultyPreset::default()),
        }
    }
}

impl HudStringCache {
    pub fn new() -> Self {
        Self::default()
    }
}

/// UI state subsystem containing menu states and UI visibility
pub struct UIState {
    /// Menu states for all game menus
    pub menus: GameMenus,
    /// Whether the minimap is currently visible
    pub minimap_visible: bool,
    /// Seed input dialog state
    pub seed_input: SeedInputState,
    /// Cached HUD strings to avoid per-frame format! allocations
    pub hud_cache: HudStringCache,
}

impl UIState {
    pub fn new(minimap_default_visible: bool) -> Self {
        Self {
            menus: GameMenus::new(),
            minimap_visible: minimap_default_visible,
            seed_input: SeedInputState::new(),
            hud_cache: HudStringCache::new(),
        }
    }
}

// ============================================================================
// LevelState - Level and background state
// ============================================================================

/// Level state subsystem containing level manager and backgrounds
pub struct LevelState {
    /// Level manager for loading and managing levels
    pub manager: LevelManager,
    /// Standard parallax background (campaign mode)
    pub background: Option<ParallaxBackground>,
    /// Biome-specific background (roguelite mode)
    pub biome_background: Option<BiomeBackground>,
    /// Texture-based background manager for FLUX-generated backgrounds
    pub background_textures: BackgroundTextureManager,
    /// Texture-based decoration manager for FLUX-generated decorations
    pub decoration_textures: DecorationTextureManager,
    /// Texture-based tile overlay manager for FLUX-generated tile textures
    pub tile_textures: TileTextureManager,
    /// UI texture manager for loading/title screens
    pub ui_textures: UiTextureManager,
}

impl LevelState {
    pub fn new(tile_size: f32) -> Self {
        Self {
            manager: LevelManager::new(tile_size),
            background: ParallaxBackground::new().ok(),
            biome_background: None,
            background_textures: BackgroundTextureManager::new(),
            decoration_textures: DecorationTextureManager::new(),
            tile_textures: TileTextureManager::new(),
            ui_textures: UiTextureManager::new(),
        }
    }
}

/// Core game state struct - facade for subsystems
pub struct GameState {
    // ========================================================================
    // Subsystems
    // ========================================================================

    /// App state machine (app_state, transitions, title_time)
    pub state: StateController,

    /// Effects controller (audio, particles, shaders, feedback)
    pub fx: EffectsController,

    /// Gameplay engine (player, camera, level_env, death, config)
    pub gameplay: GameplayEngine,

    /// Progression manager (save_manager, lives, roguelite)
    pub progression: ProgressionManager,

    /// UI state (menus, minimap visibility, seed input)
    pub ui: UIState,

    /// Level state (level manager, backgrounds)
    pub level: LevelState,

    // ========================================================================
    // Remaining fields
    // ========================================================================

    /// Input state
    pub input: InputState,

    /// Gamepad manager
    pub gamepad: GamepadManager,

    /// Procedural generation
    pub procgen: ProcgenManager,
    pub procgen_seed: Option<u64>,
}

impl GameState {
    pub fn new() -> Self {
        let gameplay = GameplayEngine::new();
        let gamepad_deadzone = gameplay.config.gamepad_stick_deadzone;
        let minimap_default_visible = gameplay.config.minimap_default_visible;
        let starting_lives = 5;

        let mut state = Self {
            // Subsystems
            state: StateController::new(),
            fx: EffectsController::new(),
            gameplay,
            progression: ProgressionManager::new(starting_lives),
            ui: UIState::new(minimap_default_visible),
            level: LevelState::new(32.0),

            // Remaining fields
            input: InputState::default(),
            gamepad: GamepadManager::new(gamepad_deadzone),
            procgen: {
                let mut procgen = ProcgenManager::new();
                // Load archetype pool for roguelite mode
                if let Err(e) = procgen.load_archetype_pool("assets") {
                    #[cfg(debug_assertions)]
                    eprintln!("Failed to load archetype pool: {}", e);
                    let _ = e;
                }
                procgen
            },
            procgen_seed: None,
        };

        state.setup_level();
        state
    }

    // ========================================================================
    // Level management
    // ========================================================================

    /// Set up the current level (called on load and restart)
    pub fn setup_level(&mut self) {
        self.gameplay.setup_level(&self.level.manager);
    }

    /// Respawn player at checkpoint or level start
    pub fn respawn_player(&mut self) {
        self.gameplay.respawn_player(&self.level.manager);
    }

    // ========================================================================
    // Audio convenience methods
    // ========================================================================

    /// Play a sound effect
    pub fn play_sound(&self, id: crate::audio::SoundId) {
        self.fx.play_sound(id);
    }

    /// Play a sound effect with volume based on distance from player
    pub fn play_sound_at(&self, id: crate::audio::SoundId, position: Vec2) {
        self.fx.play_sound_at(id, position, self.gameplay.player.position);
    }

    // ========================================================================
    // Death handling
    // ========================================================================

    /// Trigger player death with effects
    pub fn trigger_death(&mut self) {
        if self.gameplay.trigger_death() {
            // Death was triggered - handle progression and effects
            self.progression.record_death();

            self.play_sound(crate::audio::SoundId::PlayerDeath);
            self.fx.spawn_death(self.gameplay.player.position);
            self.fx.spawn_hurt(self.gameplay.player.position);

            // Trigger chromatic aberration effect centered on screen
            let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
            self.fx.trigger_chromatic_hit(screen_center, 0.8);
        }
    }

    /// Award an extra life with effects
    pub fn award_extra_life(&mut self) {
        if self.progression.award_extra_life(self.gameplay.config.max_lives) {
            self.play_sound(crate::audio::SoundId::ExtraLife);
            self.fx.spawn_extra_life(self.gameplay.player.position);
        }
    }

}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

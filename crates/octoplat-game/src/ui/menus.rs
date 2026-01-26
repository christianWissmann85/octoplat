//! Game menus management
//!
//! Consolidates all menu states into a single manager.

use octoplat_core::state::{
    AppState, BiomeMenuItem, DifficultyMenuItem, ErrorMenuItem, GameOverMenuItem,
    LevelCompleteMenuItem, MainMenuItem, PauseMenuItem, SettingsMenuItem,
};
use super::MenuState;

/// Consolidated game menus
pub struct GameMenus {
    pub main: MenuState<MainMenuItem>,
    pub pause: MenuState<PauseMenuItem>,
    pub level_complete: MenuState<LevelCompleteMenuItem>,
    pub game_over: MenuState<GameOverMenuItem>,
    pub settings: MenuState<SettingsMenuItem>,
    pub settings_return_state: Option<AppState>,
    pub biome_select: MenuState<BiomeMenuItem>,
    pub difficulty_select: MenuState<DifficultyMenuItem>,
    pub error: MenuState<ErrorMenuItem>,
}

impl GameMenus {
    pub fn new() -> Self {
        Self {
            main: MenuState::from_array(MainMenuItem::ALL),
            pause: MenuState::from_array(PauseMenuItem::ALL),
            level_complete: MenuState::from_array(LevelCompleteMenuItem::ALL),
            game_over: MenuState::from_array(GameOverMenuItem::ALL),
            settings: MenuState::from_array(SettingsMenuItem::ALL),
            settings_return_state: None,
            biome_select: MenuState::from_array(BiomeMenuItem::ALL),
            difficulty_select: MenuState::from_array(DifficultyMenuItem::ALL),
            error: MenuState::from_array(ErrorMenuItem::ALL),
        }
    }
}

impl Default for GameMenus {
    fn default() -> Self {
        Self::new()
    }
}

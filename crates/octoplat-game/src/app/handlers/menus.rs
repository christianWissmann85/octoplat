//! Menu state handlers
//!
//! Handlers for all menu-related app states.

use macroquad::prelude::*;

use crate::app::{GameAction, GameActions, MenuId};
use crate::app_state::{
    AppState, BiomeMenuItem, ErrorMenuItem, GameOverMenuItem, LevelCompleteMenuItem, MainMenuItem, PauseMenuItem,
    PlayMode, SettingsMenuItem,
};
use crate::audio::SoundId;
use crate::game_state::GameState;
use crate::procgen::DifficultyPreset;
use crate::ui::{self, MenuAction};

/// Update Title screen state
pub fn title_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    game.state.title_time += dt;

    // Any key to continue to main menu (with transition)
    if (game.input.menu_confirm || game.input.jump_pressed)
        && game.state.transition.is_none() {
            actions.push(GameAction::PlaySound(SoundId::MenuSelect));
            actions.push(GameAction::TransitionTo(AppState::MainMenu));
        }

    actions
}

/// Render Title screen
pub fn title_render(game: &GameState) {
    ui::draw_title_screen(game.state.title_time, Some(&game.level.ui_textures));
}

/// Update MainMenu state
pub fn main_menu_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.main.selected;

    // Update menu
    if let MenuAction::Select(item) = game.ui.menus.main.update(&game.input, dt) {
        actions.push(GameAction::PlaySound(SoundId::MenuSelect));
        match item {
            MainMenuItem::RogueLite => {
                // Go to biome selection for RogueLite
                actions.push(GameAction::ResetMenuSelection(MenuId::BiomeSelect));
                actions.push(GameAction::SetStateDirect(AppState::BiomeSelect));
            }
            MainMenuItem::Settings => {
                actions.push(GameAction::SetSettingsReturnState(AppState::MainMenu));
                actions.push(GameAction::ResetMenuSelection(MenuId::Settings));
                actions.push(GameAction::SetStateDirect(AppState::Settings));
            }
            MainMenuItem::Quit => {
                std::process::exit(0);
            }
        }
    }

    // Play navigation sound if selection changed
    if game.ui.menus.main.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render MainMenu
pub fn main_menu_render(game: &GameState, time: f32) {
    ui::draw_main_menu(&game.ui.menus.main, time, Some(&game.level.ui_textures));
}

/// Update Paused state - now takes the preserved PlayMode
pub fn paused_update(game: &mut GameState, dt: f32, play_mode: PlayMode) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.pause.selected;

    // Update pause menu
    match game.ui.menus.pause.update(&game.input, dt) {
        MenuAction::Select(item) => {
            actions.push(GameAction::PlaySound(SoundId::MenuSelect));
            match item {
                PauseMenuItem::Resume => {
                    // Resume with the preserved play mode
                    actions.push(GameAction::SetStateDirect(AppState::Playing(play_mode.clone())));
                    actions.push(GameAction::ResumeMusic);
                    actions.push(GameAction::ResumeAmbient);
                }
                PauseMenuItem::Restart => {
                    actions.push(GameAction::RestartLevel);
                    // Resume with the preserved play mode
                    actions.push(GameAction::SetStateDirect(AppState::Playing(play_mode.clone())));
                    actions.push(GameAction::ResumeMusic);
                    actions.push(GameAction::ResumeAmbient);
                }
                PauseMenuItem::Settings => {
                    actions.push(GameAction::SetSettingsReturnState(AppState::Paused(play_mode.clone())));
                    actions.push(GameAction::ResetMenuSelection(MenuId::Settings));
                    actions.push(GameAction::SetStateDirect(AppState::Settings));
                }
                PauseMenuItem::QuitToMenu => {
                    actions.push(GameAction::ReturnToMenu);
                }
            }
        }
        MenuAction::Cancel => {
            actions.push(GameAction::PlaySound(SoundId::MenuBack));
            // Resume with the preserved play mode
            actions.push(GameAction::SetStateDirect(AppState::Playing(play_mode)));
            actions.push(GameAction::ResumeMusic);
            actions.push(GameAction::ResumeAmbient);
        }
        _ => {}
    }

    // Play navigation sound if selection changed
    if game.ui.menus.pause.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render Paused state (game is rendered underneath)
pub fn paused_render(game: &GameState, time: f32) {
    // Render game underneath
    super::playing::render(game, time);
    // Render pause overlay
    ui::draw_pause_menu(
        &game.ui.menus.pause,
        game.level.ui_textures.additional.pause_overlay.as_ref(),
    );
}

/// Update LevelComplete state
pub fn level_complete_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.level_complete.selected;

    // Update level complete menu
    if let MenuAction::Select(item) = game.ui.menus.level_complete.update(&game.input, dt) {
        actions.push(GameAction::PlaySound(SoundId::MenuSelect));
        match item {
            LevelCompleteMenuItem::NextLevel => {
                // In RogueLite mode, continue to next level
                if game.progression.is_in_roguelite_run() {
                    actions.push(GameAction::CompleteRogueliteLevel);
                }
            }
            LevelCompleteMenuItem::Replay => {
                actions.push(GameAction::RestartLevel);
                // Return to playing with current roguelite state
                let play_mode = PlayMode::RogueLite {
                    preset: game.progression.roguelite.preset,
                    seed: game.progression.roguelite.start_seed,
                    biome: game.progression.roguelite.biome_progression.current_id(),
                };
                actions.push(GameAction::SetStateDirect(AppState::Playing(play_mode)));
            }
            LevelCompleteMenuItem::Menu => {
                actions.push(GameAction::ReturnToMenu);
            }
        }
    }

    // Play navigation sound if selection changed
    if game.ui.menus.level_complete.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render LevelComplete state
pub fn level_complete_render(game: &GameState) {
    // Get best times/gems from save data
    let level_name = game.level.manager.current_level_name();
    let best_time = level_name.as_ref().and_then(|n| game.progression.save_manager.data.get_best_time(n));
    let best_gems = level_name.as_ref().and_then(|n| game.progression.save_manager.data.get_best_gems(n));

    // Render level complete screen
    ui::draw_level_complete(
        &game.ui.menus.level_complete,
        game.gameplay.level_env.gems_collected,
        game.gameplay.level_env.total_gems,
        game.gameplay.level_env.level_time,
        game.progression.lives.session_deaths,
        best_time,
        best_gems,
        game.level.ui_textures.additional.level_complete_banner.as_ref(),
    );
}

/// Update GameOver state
pub fn game_over_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.game_over.selected;

    // Update game over menu
    if let MenuAction::Select(item) = game.ui.menus.game_over.update(&game.input, dt) {
        actions.push(GameAction::PlaySound(SoundId::MenuSelect));
        match item {
            GameOverMenuItem::Retry => {
                // Start a new roguelite run with the same biome
                let biome = game.progression.roguelite.biome_progression.current_id();
                actions.push(GameAction::StartBiomeChallenge {
                    biome,
                    preset: game.progression.roguelite.preset,
                    seed: None,
                });
            }
            GameOverMenuItem::Menu => {
                actions.push(GameAction::ReturnToMenu);
            }
        }
    }

    // Play navigation sound if selection changed
    if game.ui.menus.game_over.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render GameOver state
pub fn game_over_render(game: &GameState) {
    ui::draw_roguelite_game_over(
        &game.ui.menus.game_over,
        game.progression.roguelite.level_count,
        game.progression.roguelite.total_gems,
        game.progression.roguelite.run_deaths,
        game.progression.roguelite.run_time,
        game.level.ui_textures.additional.game_over_background.as_ref(),
    );
}

/// Update Settings state
pub fn settings_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.settings.selected;

    // Handle escape to go back
    if game.input.menu_cancel {
        actions.push(GameAction::PlaySound(SoundId::MenuBack));
        // Save settings before leaving
        if let Err(e) = game.progression.save_manager.save() {
            #[cfg(debug_assertions)]
            eprintln!("Failed to save settings: {}", e);
            let _ = e;
        }
        let return_state = game.ui.menus.settings_return_state.take().unwrap_or(AppState::MainMenu);
        actions.push(GameAction::SetStateDirect(return_state));
        return actions;
    }

    // Get current setting values
    let sfx_vol = game.progression.save_manager.data.sfx_volume;
    let music_vol = game.progression.save_manager.data.music_volume;
    let shake_enabled = game.progression.save_manager.data.screen_shake_enabled;
    let minimap_size = game.progression.save_manager.data.minimap_size;
    let minimap_scale = game.progression.save_manager.data.minimap_scale;
    let minimap_opacity = game.progression.save_manager.data.minimap_opacity;

    // Handle left/right for sliders and toggles
    let selected_item = game.ui.menus.settings.items[game.ui.menus.settings.selected];
    if game.input.menu_left || game.input.menu_right {
        let delta = if game.input.menu_right { 0.1 } else { -0.1 };
        match selected_item {
            SettingsMenuItem::SfxVolume => {
                let new_vol = (sfx_vol + delta).clamp(0.0, 1.0);
                game.progression.save_manager.data_mut().sfx_volume = new_vol;
                if let Some(ref audio) = game.fx.audio {
                    audio.set_sfx_volume(new_vol);
                }
                actions.push(GameAction::PlaySound(SoundId::MenuMove));
            }
            SettingsMenuItem::MusicVolume => {
                let new_vol = (music_vol + delta).clamp(0.0, 1.0);
                game.progression.save_manager.data_mut().music_volume = new_vol;
                if let Some(ref audio) = game.fx.audio {
                    audio.set_music_volume(new_vol);
                }
            }
            SettingsMenuItem::ScreenShake => {
                game.progression.save_manager.data_mut().screen_shake_enabled = !shake_enabled;
                actions.push(GameAction::PlaySound(SoundId::MenuMove));
            }
            SettingsMenuItem::MinimapSize => {
                // Size: 100-250 pixels, step by 15
                let size_delta = if game.input.menu_right { 15.0 } else { -15.0 };
                let new_size = (minimap_size + size_delta).clamp(100.0, 250.0);
                game.progression.save_manager.data_mut().minimap_size = new_size;
                actions.push(GameAction::PlaySound(SoundId::MenuMove));
            }
            SettingsMenuItem::MinimapZoom => {
                // Scale/zoom: 1.0-6.0, step by 0.5
                let zoom_delta = if game.input.menu_right { 0.5 } else { -0.5 };
                let new_scale = (minimap_scale + zoom_delta).clamp(1.0, 6.0);
                game.progression.save_manager.data_mut().minimap_scale = new_scale;
                actions.push(GameAction::PlaySound(SoundId::MenuMove));
            }
            SettingsMenuItem::MinimapOpacity => {
                let new_opacity = (minimap_opacity + delta).clamp(0.2, 1.0);
                game.progression.save_manager.data_mut().minimap_opacity = new_opacity;
                actions.push(GameAction::PlaySound(SoundId::MenuMove));
            }
            _ => {}
        }
    }

    // Handle menu navigation
    if let MenuAction::Select(item) = game.ui.menus.settings.update(&game.input, dt) {
        actions.push(GameAction::PlaySound(SoundId::MenuSelect));
        match item {
            SettingsMenuItem::ScreenShake => {
                // Toggle screen shake
                game.progression.save_manager.data_mut().screen_shake_enabled = !shake_enabled;
            }
            SettingsMenuItem::Back => {
                // Save and go back
                if let Err(e) = game.progression.save_manager.save() {
                    #[cfg(debug_assertions)]
                    eprintln!("Failed to save settings: {}", e);
                    let _ = e;
                }
                let return_state = game.ui.menus.settings_return_state.take().unwrap_or(AppState::MainMenu);
                actions.push(GameAction::SetStateDirect(return_state));
            }
            _ => {}
        }
    }

    // Play navigation sound if selection changed
    if game.ui.menus.settings.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render Settings state
pub fn settings_render(game: &GameState) {
    ui::draw_settings(
        &game.ui.menus.settings,
        game.progression.save_manager.data.sfx_volume,
        game.progression.save_manager.data.music_volume,
        game.progression.save_manager.data.screen_shake_enabled,
        game.progression.save_manager.data.minimap_size,
        game.progression.save_manager.data.minimap_scale,
        game.progression.save_manager.data.minimap_opacity,
        Some(&game.level.ui_textures),
    );
}

/// Update RogueLiteLeaderboard state
pub fn roguelite_leaderboard_update(game: &mut GameState, _dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Handle input
    if game.input.menu_confirm {
        // Go to biome selection to start a new run
        actions.push(GameAction::PlaySound(SoundId::MenuSelect));
        game.ui.menus.biome_select.selected = 0;
        actions.push(GameAction::SetStateDirect(AppState::BiomeSelect));
    }

    if game.input.menu_cancel {
        actions.push(GameAction::PlaySound(SoundId::MenuBack));
        actions.push(GameAction::SetStateDirect(AppState::MainMenu));
    }

    actions
}

/// Render RogueLiteLeaderboard state
pub fn roguelite_leaderboard_render(game: &GameState) {
    ui::draw_roguelite_leaderboard(
        &game.progression.save_manager.data.endless_runs,
        game.progression.save_manager.data.endless_best_levels,
        game.progression.save_manager.data.endless_best_gems,
    );
}

/// Update BiomeSelect state
pub fn biome_select_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.biome_select.selected;

    // Update menu
    match game.ui.menus.biome_select.update(&game.input, dt) {
        MenuAction::Select(item) => {
            actions.push(GameAction::PlaySound(SoundId::MenuSelect));
            match item {
                BiomeMenuItem::Back => {
                    actions.push(GameAction::SetStateDirect(AppState::MainMenu));
                }
                biome_item => {
                    // Start biome challenge with the selected biome (now uses linked segments)
                    if let Some(biome_id) = biome_item.to_biome_id() {
                        actions.push(GameAction::StartBiomeChallenge {
                            biome: biome_id,
                            preset: DifficultyPreset::Standard,
                            seed: None,
                        });
                    }
                }
            }
        }
        MenuAction::Cancel => {
            actions.push(GameAction::PlaySound(SoundId::MenuBack));
            actions.push(GameAction::SetStateDirect(AppState::MainMenu));
        }
        _ => {}
    }

    // Play navigation sound if selection changed
    if game.ui.menus.biome_select.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render BiomeSelect state
pub fn biome_select_render(game: &GameState, time: f32) {
    ui::draw_biome_select(&game.ui.menus.biome_select, time, Some(&game.level.ui_textures));
}

/// Update Error state
pub fn error_update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Track selection for sound
    let prev_selection = game.ui.menus.error.selected;

    // Update error menu
    match game.ui.menus.error.update(&game.input, dt) {
        MenuAction::Select(item) => {
            actions.push(GameAction::PlaySound(SoundId::MenuSelect));
            match item {
                ErrorMenuItem::Retry => {
                    // Try starting roguelite again with the same biome
                    if game.progression.is_in_roguelite_run() {
                        let biome = game.progression.roguelite.biome_progression.current_id();
                        actions.push(GameAction::StartBiomeChallenge {
                            biome,
                            preset: game.progression.roguelite.preset,
                            seed: None,
                        });
                    } else {
                        // If no active roguelite, go back to biome select
                        game.ui.menus.biome_select.selected = 0;
                        actions.push(GameAction::SetStateDirect(AppState::BiomeSelect));
                    }
                }
                ErrorMenuItem::Menu => {
                    actions.push(GameAction::ReturnToMenu);
                }
            }
        }
        MenuAction::Cancel => {
            actions.push(GameAction::PlaySound(SoundId::MenuBack));
            actions.push(GameAction::ReturnToMenu);
        }
        _ => {}
    }

    // Play navigation sound if selection changed
    if game.ui.menus.error.selected != prev_selection {
        actions.push(GameAction::PlaySound(SoundId::MenuMove));
    }

    actions
}

/// Render Error state
pub fn error_render(game: &GameState, error_message: &str) {
    ui::draw_error_screen(&game.ui.menus.error, error_message);
}

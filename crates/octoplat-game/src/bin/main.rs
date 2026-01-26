//! Octoplat - Main Entry Point
//!
//! This is the game's main entry point. RogueLite-only experience with linked segments.

use macroquad::prelude::*;

use octoplat_game::app::{self, handlers, GameAction, GameActions, MenuId};
use octoplat_game::app_state::AppState;
use octoplat_game::audio::{AmbientManager, AudioManager, MusicManager, MusicTrack, SoundId};
use octoplat_game::game_state::GameState;
use octoplat_game::gameplay;
use octoplat_game::procgen;
use octoplat_game::rendering;
use octoplat_game::roguelite;
use octoplat_game::ui;
use octoplat_core::state::PlayMode;

fn window_conf() -> Conf {
    Conf {
        window_title: "Octoplat".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        icon: Some(miniquad::conf::Icon {
            small: *include_bytes!("../../../../assets/icons/icon_16x16.rgba"),
            medium: *include_bytes!("../../../../assets/icons/icon_32x32.rgba"),
            big: *include_bytes!("../../../../assets/icons/icon_64x64.rgba"),
        }),
        ..Default::default()
    }
}

/// Execute a batch of game actions
fn execute_actions(game: &mut GameState, actions: GameActions) {
    for action in actions {
        match action {
            GameAction::TransitionTo(target) => {
                game.state.transition = Some(ui::Transition::new(0.4));
                game.state.transition_target = Some(target);
            }
            GameAction::SetStateDirect(state) => {
                game.state.app_state = state;
            }
            GameAction::ReturnToMenu => {
                // Record roguelite run before leaving
                if game.progression.is_in_roguelite_run() {
                    roguelite::controller::record_run(&game.progression.roguelite, &mut game.progression.save_manager);
                }
                game.state.app_state = AppState::MainMenu;
                game.ui.menus.main.selected = 0;
                game.progression.end_run();
                // Clear biome background
                game.level.biome_background = None;
                // Crossfade back to title music and stop ambient sounds
                game.fx.crossfade_music(MusicTrack::Title, 1.0);
                game.fx.stop_ambient();
            }
            GameAction::ResetMenuSelection(menu_id) => {
                match menu_id {
                    MenuId::Main => game.ui.menus.main.selected = 0,
                    MenuId::Pause => game.ui.menus.pause.selected = 0,
                    MenuId::GameOver => game.ui.menus.game_over.selected = 0,
                    MenuId::Settings => game.ui.menus.settings.selected = 0,
                    MenuId::BiomeSelect => game.ui.menus.biome_select.selected = 0,
                    MenuId::LevelComplete => game.ui.menus.level_complete.selected = 0,
                    MenuId::Error => game.ui.menus.error.selected = 0,
                    MenuId::RogueLiteLeaderboard => {} // No selection state
                }
            }
            GameAction::SetSettingsReturnState(state) => {
                game.ui.menus.settings_return_state = Some(state);
            }
            GameAction::PlaySound(id) => {
                game.play_sound(id);
            }
            GameAction::PlayMusic(track) => {
                game.fx.play_music(track);
            }
            GameAction::CrossfadeMusic { track, duration } => {
                game.fx.crossfade_music(track, duration);
            }
            GameAction::CrossfadeToBiomeMusic { biome, duration } => {
                game.fx.crossfade_to_biome_music(biome, duration);
            }
            GameAction::StopMusic => {
                game.fx.stop_music();
            }
            GameAction::PauseMusic => {
                game.fx.pause_music();
            }
            GameAction::ResumeMusic => {
                game.fx.resume_music();
            }
            GameAction::PlayBiomeAmbient(biome) => {
                game.fx.play_biome_ambient(biome);
            }
            GameAction::StopAmbient => {
                game.fx.stop_ambient();
            }
            GameAction::PauseAmbient => {
                game.fx.pause_ambient();
            }
            GameAction::ResumeAmbient => {
                game.fx.resume_ambient();
            }
            GameAction::TriggerDeath => {
                game.trigger_death();
            }
            GameAction::Respawn => {
                game.respawn_player();
            }
            GameAction::GameOver => {
                game.gameplay.death.respawn();
                if game.progression.is_in_roguelite_run() {
                    roguelite::controller::record_run(&game.progression.roguelite, &mut game.progression.save_manager);
                }
                game.state.app_state = AppState::GameOver;
                game.ui.menus.game_over.selected = 0;
                // Play game over music
                game.fx.crossfade_music(MusicTrack::GameOver, 0.5);
            }
            GameAction::AwardExtraLife => {
                game.award_extra_life();
            }
            GameAction::RestartLevel => {
                game.setup_level();
                game.progression.lives.reset_session();
                game.gameplay.level_env.level_time = 0.0;
            }
            GameAction::MarkLevelComplete => {
                if !game.gameplay.level_env.level_complete {
                    game.gameplay.level_env.level_complete = true;
                    game.fx.feedback.reset_level_complete();
                    // Play level complete sound immediately
                    game.play_sound(SoundId::LevelComplete);
                }
            }
            GameAction::SetLevelTextTimer(duration) => {
                game.gameplay.level_env.show_level_text = duration;
            }
            GameAction::ToggleMinimap => {
                game.ui.minimap_visible = !game.ui.minimap_visible;
            }
            GameAction::AdjustMinimapScale(delta) => {
                let current = game.progression.save_manager.data.minimap_scale;
                let new_scale = (current + delta).clamp(1.0, 6.0);
                game.progression.save_manager.data_mut().minimap_scale = new_scale;
            }
            GameAction::NextLevel => {
                // Save progress before moving to next level
                save_level_progress(game);
                // In RogueLite mode, this is handled by CompleteRogueliteLevel
                // This action is kept for compatibility but shouldn't be called
            }
            GameAction::StartBiomeChallenge { biome, preset, seed } => {
                start_roguelite_mode(game, preset, seed, biome);
            }
            GameAction::CompleteRogueliteLevel => {
                roguelite::controller::complete_level(&mut game.progression.roguelite, game.gameplay.level_env.gems_collected);
                match roguelite::controller::generate_linked_level(
                    &game.progression.roguelite,
                    &mut game.procgen,
                    &mut game.level.manager,
                    &mut game.procgen_seed,
                ) {
                    Ok(level_data) => {
                        game.gameplay.level_env.decorations = level_data.decorations;
                        // Generate biome background for parallax layers
                        let biome = game.progression.roguelite.biome_progression.current_id();
                        let level_width = game.level.manager.tilemap()
                            .map(|tm| tm.bounds().w)
                            .unwrap_or(1000.0);
                        game.level.biome_background = Some(rendering::BiomeBackground::generate(
                            biome,
                            level_width,
                            level_data.seed,
                        ));
                    }
                    Err(e) => {
                        eprintln!("Failed to generate next level: {}", e);
                        game.ui.menus.error.selected = 0;
                        game.state.app_state = AppState::Error(format!("Failed to generate next level: {}", e));
                        return;
                    }
                }
                game.setup_level();
                game.gameplay.level_env.level_complete = false;
                game.gameplay.level_env.show_level_text = 3.0;
            }
            GameAction::ExitRogueliteMode => {
                roguelite::controller::record_run(&game.progression.roguelite, &mut game.progression.save_manager);
                game.progression.end_run();
                let _ = game.level.manager.load_first_level();
                game.setup_level();
                #[cfg(debug_assertions)]
                println!(
                    "Exited RogueLite mode. Final score: {} gems across {} levels",
                    game.progression.roguelite.total_gems, game.progression.roguelite.level_count
                );
            }
            GameAction::StartProcgenRun { preset, seed } => {
                // Default to OceanDepths for procgen debug shortcuts
                start_roguelite_mode(game, preset, seed, procgen::BiomeId::OceanDepths);
            }
            // LinkedSegments is now merged into RogueLite
            GameAction::StartLinkedSegments { biome, preset, seed, segment_count: _ } => {
                start_roguelite_mode(game, preset, seed, biome);
            }
        }
    }
}

/// Save current level progress
fn save_level_progress(game: &mut GameState) {
    if let Some(level_name) = game.level.manager.current_level_name() {
        game.progression.save_manager.data_mut().complete_level(
            &level_name,
            game.gameplay.level_env.level_time,
            game.gameplay.level_env.gems_collected,
        );
        game.progression.save_manager.data_mut().total_gems += game.gameplay.level_env.gems_collected;
        game.progression.save_manager.data_mut().total_deaths += game.progression.lives.session_deaths;
        game.progression.save_manager.data_mut().total_playtime += game.gameplay.level_env.level_time;

        if let Err(e) = game.progression.save_manager.save_if_dirty() {
            eprintln!("Failed to save progress: {}", e);
        }
    }
}

/// Start roguelite mode with a specific biome (now uses linked segments)
fn start_roguelite_mode(
    game: &mut GameState,
    preset: procgen::DifficultyPreset,
    seed: Option<u64>,
    biome: procgen::BiomeId,
) {
    roguelite::controller::start_biome_challenge(
        &mut game.progression.roguelite,
        &mut game.progression.lives,
        &game.gameplay.config,
        biome,
        preset,
        seed,
    );

    // Generate first level using linked segments
    match roguelite::controller::generate_linked_level(
        &game.progression.roguelite,
        &mut game.procgen,
        &mut game.level.manager,
        &mut game.procgen_seed,
    ) {
        Ok(level_data) => {
            game.gameplay.level_env.decorations = level_data.decorations;
            // Generate biome background for parallax layers
            let biome = game.progression.roguelite.biome_progression.current_id();
            let level_width = game.level.manager.tilemap()
                .map(|tm| tm.bounds().w)
                .unwrap_or(1000.0);
            game.level.biome_background = Some(rendering::BiomeBackground::generate(
                biome,
                level_width,
                level_data.seed,
            ));
            game.setup_level();
        }
        Err(e) => {
            eprintln!("ERROR: Failed to generate roguelite level: {}", e);
            // Show error screen instead of just returning to menu
            game.progression.end_run();
            game.ui.menus.error.selected = 0;
            game.state.app_state = AppState::Error(format!(
                "Failed to generate level: {}",
                e
            ));
            return;
        }
    }

    // Capture the starting seed if not specified
    game.progression.roguelite.capture_seed(game.procgen_seed);

    game.state.app_state = AppState::Playing(PlayMode::RogueLite { preset, seed, biome });

    // Start biome music with crossfade
    game.fx.crossfade_to_biome_music(biome, 1.0);

    // Start biome ambient sounds
    game.fx.play_biome_ambient(biome);
}

/// Update audio/visual feedback based on state changes
fn update_feedback(game: &mut GameState) {
    let result = gameplay::process_feedback(
        &mut game.gameplay.player,
        &mut game.fx.feedback,
        game.gameplay.level_env.gems_collected,
        &mut game.fx.effects,
        &game.gameplay.config,
        game.progression.save_manager.data_mut(),
    );

    // Play sounds from feedback
    for (sound_id, position) in result.sounds {
        if let Some(pos) = position {
            game.play_sound_at(sound_id, pos);
        } else {
            game.play_sound(sound_id);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::new();

    // ========================================================================
    // Loading screen with animated octopus
    // ========================================================================

    // Load UI textures FIRST so we can display them on the loading screen
    game.level.ui_textures.load_all().await;

    // Helper to render multiple animation frames (smoother loading)
    // Uses ui_textures if available for backgrounds/octopus
    async fn animate_loading(
        frames: u32,
        progress: f32,
        message: &str,
        ui_textures: Option<&crate::rendering::UiTextureManager>,
    ) {
        for _ in 0..frames {
            let time = get_time() as f32;
            ui::draw_loading_screen(progress, message, time, ui_textures);
            next_frame().await;
        }
    }

    // Show initial loading screen with a few frames of animation
    animate_loading(10, 0.0, "Initializing...", Some(&game.level.ui_textures)).await;

    // Initialize audio system with graceful failure handling
    // Set a custom panic hook to suppress ALSA errors on systems without audio (e.g., WSL)
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = panic_info.to_string();
        // Only suppress audio-related panics, let others through
        if msg.contains("PCM") || msg.contains("ALSA") || msg.contains("audio") {
            eprintln!("Note: Audio device not available - sound disabled");
        } else {
            // Use default behavior for other panics
            eprintln!("{}", panic_info);
        }
    }));

    animate_loading(8, 0.1, "Loading sound effects...", Some(&game.level.ui_textures)).await;
    game.fx.audio = Some(AudioManager::new().await);
    animate_loading(3, 0.25, "Loading sound effects...", Some(&game.level.ui_textures)).await;

    // Restore default panic hook
    std::panic::set_hook(default_hook);

    // Initialize music system - load tracks one at a time with progress updates
    animate_loading(8, 0.3, "Loading music...", Some(&game.level.ui_textures)).await;
    let mut music_manager = MusicManager::new();

    // Load music tracks with progress feedback
    let music_tracks = [
        (MusicTrack::Title, "Loading title music..."),
        (MusicTrack::GameOver, "Loading game over music..."),
        (MusicTrack::OceanDepths, "Loading Ocean Depths..."),
        (MusicTrack::CoralReefs, "Loading Coral Reefs..."),
        (MusicTrack::TropicalShore, "Loading Tropical Shore..."),
        (MusicTrack::Shipwreck, "Loading Shipwreck..."),
        (MusicTrack::ArcticWaters, "Loading Arctic Waters..."),
        (MusicTrack::VolcanicVents, "Loading Volcanic Vents..."),
        (MusicTrack::SunkenRuins, "Loading Sunken Ruins..."),
        (MusicTrack::Abyss, "Loading The Abyss..."),
    ];

    for (i, (track, msg)) in music_tracks.iter().enumerate() {
        let progress = 0.3 + (i as f32 / music_tracks.len() as f32) * 0.5;
        // Render animation frames before and after each track load
        animate_loading(8, progress, msg, Some(&game.level.ui_textures)).await;
        music_manager.load_track(*track).await;
        // Brief animation after load completes
        animate_loading(2, progress + 0.05, msg, Some(&game.level.ui_textures)).await;
    }

    music_manager.set_volume(game.progression.save_manager.data.music_volume);
    game.fx.set_music(music_manager);

    // Initialize ambient sound system
    animate_loading(8, 0.82, "Loading ambient sounds...", Some(&game.level.ui_textures)).await;
    let mut ambient_manager = AmbientManager::new();
    ambient_manager.load_all().await;
    ambient_manager.set_volume(0.4); // Slightly quieter than music
    game.fx.set_ambient(ambient_manager);
    animate_loading(3, 0.85, "Loading ambient sounds...", Some(&game.level.ui_textures)).await;

    // Load background textures for all biomes
    animate_loading(5, 0.85, "Loading backgrounds...", Some(&game.level.ui_textures)).await;
    game.level.background_textures.load_all_biomes().await;
    animate_loading(2, 0.88, "Loading backgrounds...", Some(&game.level.ui_textures)).await;

    // Load decoration textures
    animate_loading(3, 0.90, "Loading decorations...", Some(&game.level.ui_textures)).await;
    game.level.decoration_textures.load_all().await;
    animate_loading(2, 0.92, "Loading decorations...", Some(&game.level.ui_textures)).await;

    // Load tile textures
    animate_loading(3, 0.94, "Loading tile textures...", Some(&game.level.ui_textures)).await;
    game.level.tile_textures.load_all_biomes().await;
    animate_loading(2, 0.96, "Loading tile textures...", Some(&game.level.ui_textures)).await;

    // Apply saved audio settings
    animate_loading(3, 0.98, "Applying settings...", Some(&game.level.ui_textures)).await;
    if let Some(ref audio) = game.fx.audio {
        audio.set_sfx_volume(game.progression.save_manager.data.sfx_volume);
        audio.set_music_volume(game.progression.save_manager.data.music_volume);
    }

    // Final loading frame - show "Ready!" briefly
    animate_loading(15, 1.0, "Ready!", Some(&game.level.ui_textures)).await;

    // Start title music
    game.fx.play_music(MusicTrack::Title);

    loop {
        let dt = get_frame_time().min(0.05);
        let time = get_time() as f32;

        // Always update input (keyboard first, then merge gamepad)
        game.input.update(dt, game.gameplay.config.jump_buffer_time);

        // Poll gamepad and merge with keyboard input
        let gamepad_input = game.gamepad.poll();
        game.input.update_with_gamepad(&gamepad_input, dt, game.gameplay.config.jump_buffer_time);

        // Handle transitions using the state controller
        game.state.update_transition(dt);

        // Update effects (music crossfades, ambient sounds, particles)
        game.fx.update(dt);

        // State-specific update and render
        // TODO: Consider trait-based StateHandler dispatch if more states are added.
        // Current match statement is explicit and works well for ~10 states.
        // A trait-based system would add complexity for marginal compile-time safety benefit.
        match &game.state.app_state {
            AppState::Title => {
                let actions = handlers::menus::title_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::title_render(&game);
            }

            AppState::MainMenu => {
                let actions = handlers::menus::main_menu_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::main_menu_render(&game, time);
            }

            AppState::Playing(_mode) => {
                // Handle pause
                if game.input.pause_pressed && !game.ui.seed_input.active {
                    game.play_sound(SoundId::Pause);
                    // Preserve the current play mode when pausing
                    if let AppState::Playing(mode) = game.state.app_state.clone() {
                        game.state.app_state = AppState::Paused(mode);
                    }
                    game.ui.menus.pause.selected = 0;
                    // Pause music and ambient sounds while in pause menu
                    game.fx.pause_music();
                    game.fx.pause_ambient();
                } else {
                    // Normal gameplay
                    game.gameplay.level_env.update_time(dt);
                    game.progression.update_run_time(dt);

                    // Handle debug keybindings during gameplay
                    let keybind_actions = app::handle_gameplay_keybindings(
                        &mut game.ui.seed_input,
                        game.progression.is_in_roguelite_run(),
                        game.progression.roguelite.preset,
                        game.level.manager.current_level_path(),
                    );
                    execute_actions(&mut game, keybind_actions);

                    // Update game
                    let actions = handlers::playing::update(&mut game, dt);
                    execute_actions(&mut game, actions);

                    // Update audio and visual feedback based on state changes
                    update_feedback(&mut game);

                    // Check for level complete -> complete roguelite level
                    if game.gameplay.level_env.level_complete && game.gameplay.level_env.show_level_text <= 2.0
                        && game.progression.is_in_roguelite_run() {
                            let mut actions = GameActions::new();
                            actions.push(GameAction::CompleteRogueliteLevel);
                            execute_actions(&mut game, actions);
                        }

                    // Render game
                    handlers::playing::render(&game, time);
                }
            }

            AppState::Paused(ref mode) => {
                let mode = mode.clone();
                let actions = handlers::menus::paused_update(&mut game, dt, mode);
                execute_actions(&mut game, actions);
                handlers::menus::paused_render(&game, time);
            }

            AppState::LevelComplete => {
                let actions = handlers::menus::level_complete_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::level_complete_render(&game);
            }

            AppState::GameOver => {
                let actions = handlers::menus::game_over_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::game_over_render(&game);
            }

            AppState::Settings => {
                let actions = handlers::menus::settings_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::settings_render(&game);
            }

            AppState::RogueLiteLeaderboard => {
                let actions = handlers::menus::roguelite_leaderboard_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::roguelite_leaderboard_render(&game);
            }

            AppState::BiomeSelect => {
                let actions = handlers::menus::biome_select_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::biome_select_render(&game, time);
            }

            AppState::Error(ref message) => {
                let message = message.clone();
                let actions = handlers::menus::error_update(&mut game, dt);
                execute_actions(&mut game, actions);
                handlers::menus::error_render(&game, &message);
            }
        }

        // Draw transition overlay if active
        if let Some(ref transition) = game.state.transition {
            ui::draw_fade_overlay(transition.fade_alpha());
        }

        next_frame().await;
    }
}

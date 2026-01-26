//! Integration tests for game state types

use octoplat_core::state::{
    AppState, DeathState, DifficultyPreset, LivesManager,
    MainMenuItem, PauseMenuItem, PlayMode, PlayerState, RogueliteRun,
};
use octoplat_core::procgen::BiomeId;
use octoplat_core::vec2;

// =============================================================================
// AppState Tests
// =============================================================================

#[test]
fn test_app_state_initial() {
    let state = AppState::Title;
    assert!(matches!(state, AppState::Title));
}

#[test]
fn test_app_state_variants_exist() {
    // Verify all expected variants exist
    let _title = AppState::Title;
    let _main_menu = AppState::MainMenu;
    let _settings = AppState::Settings;
    let _biome_select = AppState::BiomeSelect;
    let _leaderboard = AppState::RogueLiteLeaderboard;
    let _level_complete = AppState::LevelComplete;
    let _game_over = AppState::GameOver;
}

#[test]
fn test_app_state_playing() {
    let play_mode = PlayMode::RogueLite {
        preset: DifficultyPreset::Standard,
        seed: Some(12345),
        biome: BiomeId::OceanDepths,
    };
    let _playing = AppState::Playing(play_mode.clone());
    let _paused = AppState::Paused(play_mode);
}

#[test]
fn test_app_state_error() {
    let _error = AppState::Error("Test error".to_string());
}

#[test]
fn test_play_mode_roguelite() {
    let mode = PlayMode::RogueLite {
        preset: DifficultyPreset::Standard,
        seed: Some(12345),
        biome: BiomeId::CoralReefs,
    };

    if let PlayMode::RogueLite { preset, seed, biome } = mode {
        assert_eq!(preset, DifficultyPreset::Standard);
        assert_eq!(seed, Some(12345));
        assert_eq!(biome, BiomeId::CoralReefs);
    }
}

// =============================================================================
// PlayerState Tests
// =============================================================================

#[test]
fn test_player_state_variants() {
    // Verify all expected variants exist
    let _idle = PlayerState::Idle;
    let _running = PlayerState::Running;
    let _jumping = PlayerState::Jumping;
    let _falling = PlayerState::Falling;
    let _wall_grip = PlayerState::WallGrip;
    let _swinging = PlayerState::Swinging;
    let _jet_boost = PlayerState::JetBoosting;
}

#[test]
fn test_player_state_equality() {
    assert_eq!(PlayerState::Idle, PlayerState::Idle);
    assert_ne!(PlayerState::Idle, PlayerState::Running);
}

#[test]
fn test_player_state_clone() {
    let state = PlayerState::Jumping;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

// =============================================================================
// DeathState Tests
// =============================================================================

#[test]
fn test_death_state_default() {
    let death = DeathState::default();
    assert!(!death.is_dead);
    assert_eq!(death.timer, 0.0);
}

#[test]
fn test_death_state_trigger() {
    let mut death = DeathState::default();
    death.trigger(vec2(100.0, 100.0), 0.5);
    assert!(death.is_dead);
    assert_eq!(death.timer, 0.5);
    assert!(death.position.is_some());
}

#[test]
fn test_death_state_update() {
    let mut death = DeathState::default();
    death.trigger(vec2(100.0, 100.0), 0.5);

    // Update with partial time
    let complete = death.update(0.2);
    assert!(!complete);
    assert!((death.timer - 0.3).abs() < 0.0001);

    // Update to completion
    let complete = death.update(0.5);
    assert!(complete);
}

#[test]
fn test_death_state_respawn() {
    let mut death = DeathState::default();
    death.trigger(vec2(100.0, 100.0), 0.5);
    death.respawn();
    assert!(!death.is_dead);
    assert!(death.position.is_none());
}

// =============================================================================
// LivesManager Tests
// =============================================================================

#[test]
fn test_lives_manager_creation() {
    let lives = LivesManager::new(5);
    assert_eq!(lives.current, 5);
}

#[test]
fn test_lives_manager_lose_life() {
    let mut lives = LivesManager::new(5);
    lives.current = lives.current.saturating_sub(1);
    assert_eq!(lives.current, 4);
}

#[test]
fn test_lives_manager_lose_all_lives() {
    let mut lives = LivesManager::new(2);
    lives.current = lives.current.saturating_sub(1);
    lives.current = lives.current.saturating_sub(1);
    assert_eq!(lives.current, 0);
    assert!(lives.is_game_over());
}

#[test]
fn test_lives_manager_cannot_go_negative() {
    let mut lives = LivesManager::new(1);
    lives.current = lives.current.saturating_sub(1);
    lives.current = lives.current.saturating_sub(1); // Should not go below 0
    assert_eq!(lives.current, 0);
}

#[test]
fn test_lives_manager_award_life() {
    let mut lives = LivesManager::new(3);
    let awarded = lives.award_life(5);
    assert!(awarded);
    assert_eq!(lives.current, 4);
}

#[test]
fn test_lives_manager_award_life_respects_max() {
    let mut lives = LivesManager::new(4);
    lives.award_life(5);
    lives.award_life(5);
    let awarded = lives.award_life(5); // Should not exceed max
    assert!(!awarded);
    assert_eq!(lives.current, 5);
}

#[test]
fn test_lives_manager_start_session() {
    let mut lives = LivesManager::new(5);
    lives.current = lives.current.saturating_sub(1);
    lives.current = lives.current.saturating_sub(1);
    lives.start_session(5, 50, false);
    assert_eq!(lives.current, 5);
}

#[test]
fn test_lives_manager_is_game_over() {
    let mut lives = LivesManager::new(1);
    assert!(!lives.is_game_over());
    lives.current = 0;
    assert!(lives.is_game_over());
}

// =============================================================================
// DifficultyPreset Tests
// =============================================================================

#[test]
fn test_difficulty_preset_variants() {
    let _easy = DifficultyPreset::Casual;
    let _normal = DifficultyPreset::Standard;
    let _hard = DifficultyPreset::Challenge;
}

#[test]
fn test_difficulty_preset_equality() {
    assert_eq!(DifficultyPreset::Standard, DifficultyPreset::Standard);
    assert_ne!(DifficultyPreset::Casual, DifficultyPreset::Challenge);
}

#[test]
fn test_difficulty_preset_clone() {
    let diff = DifficultyPreset::Challenge;
    let cloned = diff.clone();
    assert_eq!(diff, cloned);
}

// =============================================================================
// RogueliteRun Tests
// =============================================================================

#[test]
fn test_roguelite_run_creation() {
    let run = RogueliteRun::new();
    assert!(!run.active);
    assert_eq!(run.level_count, 0);
    assert_eq!(run.total_gems, 0);
    assert_eq!(run.run_deaths, 0);
}

#[test]
fn test_roguelite_run_start_biome_challenge() {
    let mut run = RogueliteRun::new();
    run.start_biome_challenge(BiomeId::OceanDepths, DifficultyPreset::Standard, Some(12345));
    assert!(run.active);
    assert_eq!(run.start_seed, Some(12345));
    assert_eq!(run.preset, DifficultyPreset::Standard);
}

#[test]
fn test_roguelite_run_record_death() {
    let mut run = RogueliteRun::new();
    run.record_death();
    assert_eq!(run.run_deaths, 1);
}

#[test]
fn test_roguelite_run_update_time() {
    let mut run = RogueliteRun::new();
    run.update_time(1.5);
    run.update_time(0.5);
    assert!((run.run_time - 2.0).abs() < 0.0001);
}

#[test]
fn test_roguelite_run_capture_seed() {
    let mut run = RogueliteRun::new();
    assert!(run.start_seed.is_none());
    run.capture_seed(Some(12345));
    assert_eq!(run.start_seed, Some(12345));
    // Second capture should not override
    run.capture_seed(Some(99999));
    assert_eq!(run.start_seed, Some(12345));
}

#[test]
fn test_roguelite_run_clone() {
    let run = RogueliteRun::new();
    let cloned = run.clone();
    assert_eq!(run.active, cloned.active);
    assert_eq!(run.level_count, cloned.level_count);
}

// =============================================================================
// Menu Item Tests
// =============================================================================

#[test]
fn test_main_menu_items() {
    let items = [
        MainMenuItem::RogueLite,
        MainMenuItem::Settings,
        MainMenuItem::Quit,
    ];

    for item in &items {
        let cloned = item.clone();
        assert_eq!(*item, cloned);
    }
}

#[test]
fn test_main_menu_item_labels() {
    // Verify labels are non-empty
    assert!(!MainMenuItem::RogueLite.label().is_empty());
    assert!(!MainMenuItem::Settings.label().is_empty());
    assert!(!MainMenuItem::Quit.label().is_empty());
}

#[test]
fn test_pause_menu_items() {
    let items = [
        PauseMenuItem::Resume,
        PauseMenuItem::Restart,
        PauseMenuItem::Settings,
        PauseMenuItem::QuitToMenu,
    ];

    for item in &items {
        let cloned = item.clone();
        assert_eq!(*item, cloned);
    }
}

#[test]
fn test_pause_menu_item_labels() {
    assert!(!PauseMenuItem::Resume.label().is_empty());
    assert!(!PauseMenuItem::Restart.label().is_empty());
    assert!(!PauseMenuItem::Settings.label().is_empty());
    assert!(!PauseMenuItem::QuitToMenu.label().is_empty());
}

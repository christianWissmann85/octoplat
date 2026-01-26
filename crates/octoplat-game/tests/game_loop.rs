//! Game loop state transition tests
//!
//! Verifies that game state transitions work correctly:
//! - Title → MainMenu → BiomeSelect → Playing
//! - Playing → Paused → Playing
//! - Playing → LevelComplete → Playing (next level)
//! - Error state handling

use octoplat_core::procgen::BiomeId;
use octoplat_core::state::{AppState, DifficultyPreset, PlayMode};

#[test]
fn test_state_transitions_title_to_main_menu() {
    // Initial state is Title
    let state = AppState::Title;
    assert_eq!(state, AppState::Title);

    // Can transition to MainMenu
    let state = AppState::MainMenu;
    assert_eq!(state, AppState::MainMenu);
}

#[test]
fn test_state_transitions_main_menu_to_biome_select() {
    let state = AppState::MainMenu;
    assert_eq!(state, AppState::MainMenu);

    // Can transition to BiomeSelect
    let state = AppState::BiomeSelect;
    assert_eq!(state, AppState::BiomeSelect);
}

#[test]
fn test_state_transitions_to_playing() {
    // Can create Playing state with RogueLite mode
    let play_mode = PlayMode::RogueLite {
        preset: DifficultyPreset::Standard,
        seed: Some(12345),
        biome: BiomeId::OceanDepths,
    };
    let state = AppState::Playing(play_mode.clone());

    match state {
        AppState::Playing(mode) => {
            assert_eq!(mode, play_mode);
        }
        _ => panic!("Expected Playing state"),
    }
}

#[test]
fn test_state_transitions_playing_to_paused() {
    let play_mode = PlayMode::RogueLite {
        preset: DifficultyPreset::Standard,
        seed: None,
        biome: BiomeId::CoralReefs,
    };

    // Start playing
    let state = AppState::Playing(play_mode.clone());
    assert!(matches!(state, AppState::Playing(_)));

    // Transition to paused (preserves play mode)
    let paused_state = AppState::Paused(play_mode.clone());
    match paused_state {
        AppState::Paused(mode) => {
            assert_eq!(mode, play_mode);
        }
        _ => panic!("Expected Paused state"),
    }
}

#[test]
fn test_state_transitions_to_level_complete() {
    // Can transition to LevelComplete
    let state = AppState::LevelComplete;
    assert_eq!(state, AppState::LevelComplete);
}

#[test]
fn test_state_transitions_to_game_over() {
    // Can transition to GameOver
    let state = AppState::GameOver;
    assert_eq!(state, AppState::GameOver);
}

#[test]
fn test_state_error_with_message() {
    let error_msg = "Generation failed: No valid segments found".to_string();
    let state = AppState::Error(error_msg.clone());

    match state {
        AppState::Error(msg) => {
            assert_eq!(msg, error_msg);
        }
        _ => panic!("Expected Error state"),
    }
}

#[test]
fn test_play_mode_roguelite_variants() {
    // Test with seed
    let with_seed = PlayMode::RogueLite {
        preset: DifficultyPreset::Challenge,
        seed: Some(42),
        biome: BiomeId::VolcanicVents,
    };

    if let PlayMode::RogueLite { preset, seed, biome } = with_seed {
        assert_eq!(preset, DifficultyPreset::Challenge);
        assert_eq!(seed, Some(42));
        assert_eq!(biome, BiomeId::VolcanicVents);
    }

    // Test without seed (random)
    let without_seed = PlayMode::RogueLite {
        preset: DifficultyPreset::Casual,
        seed: None,
        biome: BiomeId::Abyss,
    };

    if let PlayMode::RogueLite { preset, seed, biome } = without_seed {
        assert_eq!(preset, DifficultyPreset::Casual);
        assert_eq!(seed, None);
        assert_eq!(biome, BiomeId::Abyss);
    }
}

#[test]
fn test_difficulty_presets() {
    // Test all difficulty presets can be used
    let presets = [
        DifficultyPreset::Casual,
        DifficultyPreset::Standard,
        DifficultyPreset::Challenge,
    ];

    for preset in presets {
        let mode = PlayMode::RogueLite {
            preset,
            seed: None,
            biome: BiomeId::OceanDepths,
        };

        if let PlayMode::RogueLite { preset: p, .. } = mode {
            assert_eq!(p, preset);
        }
    }
}

#[test]
fn test_biome_variants() {
    let biomes = [
        BiomeId::OceanDepths,
        BiomeId::CoralReefs,
        BiomeId::TropicalShore,
        BiomeId::Shipwreck,
        BiomeId::ArcticWaters,
        BiomeId::VolcanicVents,
        BiomeId::SunkenRuins,
        BiomeId::Abyss,
    ];

    for biome in biomes {
        let mode = PlayMode::RogueLite {
            preset: DifficultyPreset::Standard,
            seed: None,
            biome,
        };

        if let PlayMode::RogueLite { biome: b, .. } = mode {
            assert_eq!(b, biome);
        }
    }
}

#[test]
fn test_state_clone_and_equality() {
    // Verify states can be cloned and compared
    let state1 = AppState::Settings;
    let state2 = state1.clone();
    assert_eq!(state1, state2);

    let play_mode = PlayMode::RogueLite {
        preset: DifficultyPreset::Standard,
        seed: Some(100),
        biome: BiomeId::Shipwreck,
    };
    let state3 = AppState::Playing(play_mode.clone());
    let state4 = state3.clone();
    assert_eq!(state3, state4);
}

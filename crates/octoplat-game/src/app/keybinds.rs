//! Gameplay keybindings
//!
//! Handles F-key shortcuts and seed input during gameplay.

use macroquad::prelude::*;

use super::actions::{GameAction, GameActions};
use crate::procgen::{BiomeId, DifficultyPreset};

/// State for seed input dialog
pub struct SeedInputState {
    pub active: bool,
    pub buffer: String,
}

impl SeedInputState {
    pub fn new() -> Self {
        Self {
            active: false,
            buffer: String::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            self.buffer.clear();
        }
    }

    pub fn close(&mut self) {
        self.active = false;
    }
}

impl Default for SeedInputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle F-key shortcuts during gameplay
///
/// Returns actions to execute. Also mutates seed_input state for the dialog.
pub fn handle_gameplay_keybindings(
    seed_input: &mut SeedInputState,
    roguelite_active: bool,
    current_preset: DifficultyPreset,
    _current_level_path: Option<std::path::PathBuf>,
) -> GameActions {
    let mut actions = GameActions::new();

    // F2 = Standard run, F3 = Casual run, F4 = Challenge run
    if is_key_pressed(KeyCode::F2) {
        actions.push(GameAction::StartProcgenRun {
            preset: DifficultyPreset::Standard,
            seed: None,
        });
    }
    if is_key_pressed(KeyCode::F3) {
        actions.push(GameAction::StartProcgenRun {
            preset: DifficultyPreset::Casual,
            seed: None,
        });
    }
    if is_key_pressed(KeyCode::F4) {
        actions.push(GameAction::StartProcgenRun {
            preset: DifficultyPreset::Challenge,
            seed: None,
        });
    }

    // F6 = Toggle seed input mode
    if is_key_pressed(KeyCode::F6) {
        seed_input.toggle();
    }

    // Handle seed input when in seed input mode
    if seed_input.active {
        for key in [
            KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
            KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
        ] {
            if is_key_pressed(key) {
                let digit = match key {
                    KeyCode::Key0 => '0',
                    KeyCode::Key1 => '1',
                    KeyCode::Key2 => '2',
                    KeyCode::Key3 => '3',
                    KeyCode::Key4 => '4',
                    KeyCode::Key5 => '5',
                    KeyCode::Key6 => '6',
                    KeyCode::Key7 => '7',
                    KeyCode::Key8 => '8',
                    KeyCode::Key9 => '9',
                    _ => continue,
                };
                if seed_input.buffer.len() < 10 {
                    seed_input.buffer.push(digit);
                }
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            seed_input.buffer.pop();
        }

        if is_key_pressed(KeyCode::Enter) {
            if let Ok(seed) = seed_input.buffer.parse::<u64>() {
                actions.push(GameAction::StartProcgenRun {
                    preset: current_preset,
                    seed: Some(seed),
                });
            }
            seed_input.close();
        }

        if is_key_pressed(KeyCode::Escape) {
            seed_input.close();
        }
    }

    // F7 = Start roguelite mode (Ocean Depths biome)
    if is_key_pressed(KeyCode::F7) && !roguelite_active {
        actions.push(GameAction::StartBiomeChallenge {
            biome: BiomeId::OceanDepths,
            preset: current_preset,
            seed: None,
        });
    }

    // F8 = Exit roguelite mode (return to menu)
    if is_key_pressed(KeyCode::F8) && roguelite_active {
        actions.push(GameAction::ExitRogueliteMode);
    }

    actions
}

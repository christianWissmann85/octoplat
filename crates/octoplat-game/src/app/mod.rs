//! App module
//!
//! Contains actions, keybindings, and state handlers.
//!
//! Note: This module remains in src/ because it orchestrates the full game
//! and depends on local modules not yet in octoplat_game (level, rendering).

pub mod actions;
pub mod handlers;
pub mod keybinds;

pub use actions::{GameAction, GameActions, MenuId};
pub use keybinds::{handle_gameplay_keybindings, SeedInputState};

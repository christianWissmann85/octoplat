//! UI module for menus, screens, and transitions
//!
//! Provides reusable menu components and screen rendering.

pub mod menu_state;
pub mod menus;
pub mod primitives;
pub mod screens;
pub mod transitions;

// Re-export commonly used types and functions
pub use menu_state::{MenuAction, MenuState};
pub use menus::GameMenus;
pub use screens::{
    draw_biome_select, draw_error_screen, draw_game_over, draw_level_complete,
    draw_loading_screen, draw_main_menu, draw_pause_menu, draw_roguelite_game_over,
    draw_roguelite_leaderboard, draw_settings, draw_title_screen,
};
pub use transitions::{
    draw_fade_overlay, Transition,
    DeathTransition, DeathTransitionPhase,
    LevelTransition, LevelTransitionDirection,
    MenuSlideTransition, SlideDirection,
    BiomeTransition,
};

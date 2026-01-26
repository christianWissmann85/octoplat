//! Octoplat Game - Full game runtime with macroquad rendering
//!
//! This crate contains the complete game runtime including:
//! - Audio system
//! - Input handling (keyboard and gamepad)
//! - Camera management
//! - Visual effects (particles, screen shake)
//! - UI system (menus, screens, transitions)
//! - Entity systems (player, hazards, platforms, collectibles)
//! - Gameplay systems (physics, feedback, collision)
//! - Level management (loading, transitions, checkpoints)
//! - Roguelite mode (the only game mode - linked segment procedural generation)
//! - Procedural generation

pub use octoplat_core;

// Asset embedding
pub mod assets;

// Foundation modules
pub mod audio;
pub mod camera;
pub mod collectibles;
pub mod collision;
pub mod config;
pub mod error;
pub mod gamepad;
pub mod input;
pub mod paths;

// State modules
pub mod app_state;
pub mod game_state;
pub mod state;

// Entity modules
pub mod hazards;
pub mod platforms;

// Player module
pub mod player;

// Effects module
pub mod effects;

// UI module
pub mod ui;

// Gameplay module
pub mod gameplay;

// Level module
pub mod level;

// Rendering utilities
pub mod rendering;

// App module (handlers, actions, keybinds)
pub mod app;

// Roguelite module
pub mod roguelite;

// Progression module (saves, lives, run state)
pub mod progression;

// Procgen additions (generator, hybrid)
pub mod procgen;

// Re-export key types
pub use audio::{AmbientManager, AmbientTrack, AudioManager, MusicManager, MusicTrack, SoundId};
pub use camera::GameCamera;
pub use collectibles::Gem;
pub use collision::{aabb_collision, check_ground, check_wall, CollisionResult, Hitbox};
pub use config::GameConfig;
pub use gamepad::{GamepadInput, GamepadManager};
pub use input::InputState;
pub use hazards::{Crab, Pufferfish, PufferfishPattern};
pub use platforms::{CrumblingPlatform, CrumblingState, MovingPlatform};
pub use player::{Player, PlayerState};
pub use effects::{BurstConfig, EffectsManager, ParticleSystem, ScreenShake};
pub use ui::{
    draw_biome_select, draw_game_over, draw_level_complete,
    draw_main_menu, draw_pause_menu, draw_roguelite_game_over, draw_roguelite_leaderboard,
    draw_settings, draw_title_screen, draw_fade_overlay,
    GameMenus, MenuAction, MenuState, Transition,
};
pub use gameplay::{
    check_breakable_blocks, check_enemy_collision, check_fall_death, check_hazard_collision,
    EnemyCollisionResult, EnemyType, process_feedback, FeedbackResult,
    apply_platform_movement, handle_platform_collisions,
    update_moving_platforms, update_crumbling_platforms,
};
pub use level::{LevelEnvironment, LevelManager};

/// Compatibility module for converting between core and macroquad types
pub mod compat {
    use macroquad::prelude as mq;
    use octoplat_core as core;

    // ========================================================================
    // Function-based conversions
    // ========================================================================

    /// Convert core Vec2 to macroquad Vec2
    pub fn vec2_to_mq(v: core::Vec2) -> mq::Vec2 {
        mq::Vec2::new(v.x, v.y)
    }

    /// Convert macroquad Vec2 to core Vec2
    pub fn vec2_from_mq(v: mq::Vec2) -> core::Vec2 {
        core::Vec2::new(v.x, v.y)
    }

    /// Convert core Rect to macroquad Rect
    pub fn rect_to_mq(r: core::Rect) -> mq::Rect {
        mq::Rect::new(r.x, r.y, r.w, r.h)
    }

    /// Convert macroquad Rect to core Rect
    pub fn rect_from_mq(r: mq::Rect) -> core::Rect {
        core::Rect::new(r.x, r.y, r.w, r.h)
    }

    /// Convert core Color to macroquad Color
    pub fn color_to_mq(c: core::Color) -> mq::Color {
        mq::Color::new(c.r, c.g, c.b, c.a)
    }

    /// Convert macroquad Color to core Color
    pub fn color_from_mq(c: mq::Color) -> core::Color {
        core::Color::new(c.r, c.g, c.b, c.a)
    }

    // ========================================================================
    // Trait-based conversions (for method call syntax)
    // ========================================================================

    /// Extension trait for converting octoplat_core::Color to macroquad::Color
    pub trait ToMqColor {
        fn to_mq_color(&self) -> mq::Color;
    }

    impl ToMqColor for core::Color {
        #[inline]
        fn to_mq_color(&self) -> mq::Color {
            mq::Color::new(self.r, self.g, self.b, self.a)
        }
    }

    /// Extension trait for converting octoplat_core::Vec2 to macroquad::Vec2
    pub trait ToMqVec2 {
        fn to_mq_vec2(&self) -> mq::Vec2;
    }

    impl ToMqVec2 for core::Vec2 {
        #[inline]
        fn to_mq_vec2(&self) -> mq::Vec2 {
            mq::Vec2::new(self.x, self.y)
        }
    }

    /// Extension trait for converting macroquad::Vec2 to octoplat_core::Vec2
    pub trait ToCoreVec2 {
        fn to_core_vec2(&self) -> core::Vec2;
    }

    impl ToCoreVec2 for mq::Vec2 {
        #[inline]
        fn to_core_vec2(&self) -> core::Vec2 {
            core::Vec2::new(self.x, self.y)
        }
    }

    /// Extension trait for converting octoplat_core::Rect to macroquad::Rect
    pub trait ToMqRect {
        fn to_mq_rect(&self) -> mq::Rect;
    }

    impl ToMqRect for core::Rect {
        #[inline]
        fn to_mq_rect(&self) -> mq::Rect {
            mq::Rect::new(self.x, self.y, self.w, self.h)
        }
    }

    /// Extension trait for converting macroquad::Rect to octoplat_core::Rect
    pub trait ToCoreRect {
        fn to_core_rect(&self) -> core::Rect;
    }

    impl ToCoreRect for mq::Rect {
        #[inline]
        fn to_core_rect(&self) -> core::Rect {
            core::Rect::new(self.x, self.y, self.w, self.h)
        }
    }
}

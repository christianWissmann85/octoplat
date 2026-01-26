//! Player module
//!
//! Contains the Player struct and all movement/ability logic.
//!
//! Submodules:
//! - `abilities`: Jet boost, ink cloud, and grapple abilities
//! - `collision`: Collision detection and resolution
//! - `movement`: Physics application and movement helpers
//! - `state`: PlayerState enum
//! - `state_machine`: State transitions and timer management
//! - `visual`: Visual effects (squash/stretch, flash, breathing)

mod abilities;
mod collision;
mod movement;
mod state;
mod state_machine;
mod visual;

pub use state::PlayerState;
pub use movement::move_toward;

use macroquad::prelude::*;
use std::collections::HashSet;

use crate::collision::{check_ground, check_wall, Hitbox};
use crate::compat::{vec2_from_mq, rect_to_mq};
use crate::config::GameConfig;
use crate::input::InputState;
use octoplat_core::level::TileMap;

/// Pre-allocated buffers for collision detection to avoid per-frame allocations
pub struct CollisionCache {
    pub nearby_tiles: Vec<Rect>,
    pub oneway_tiles: Vec<Rect>,
    pub bounce_tiles: Vec<Rect>,
}

impl CollisionCache {
    pub fn new() -> Self {
        // Pre-allocate with reasonable capacity for typical collision queries
        Self {
            nearby_tiles: Vec::with_capacity(32),
            oneway_tiles: Vec::with_capacity(16),
            bounce_tiles: Vec::with_capacity(8),
        }
    }

    /// Clear and refill buffers from level data
    pub fn refresh(
        &mut self,
        level: &TileMap,
        pos_core: octoplat_core::Vec2,
        destroyed_blocks: &HashSet<(usize, usize)>,
    ) {
        self.nearby_tiles.clear();
        self.oneway_tiles.clear();
        self.bounce_tiles.clear();

        self.nearby_tiles.extend(
            level.get_nearby_solid_rects_excluding(pos_core, 64.0, destroyed_blocks)
                .into_iter()
                .map(rect_to_mq)
        );
        self.oneway_tiles.extend(
            level.get_nearby_oneway_rects(pos_core, 64.0)
                .into_iter()
                .map(rect_to_mq)
        );
        self.bounce_tiles.extend(
            level.get_nearby_bounce_rects(pos_core, 64.0)
                .into_iter()
                .map(rect_to_mq)
        );
    }
}

impl Default for CollisionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// The Player struct for the octopus character
pub struct Player {
    // Position and physics
    pub position: Vec2,
    pub velocity: Vec2,
    pub hitbox: Hitbox,

    // State
    pub state: PlayerState,
    pub facing_right: bool,
    pub is_sprinting: bool,

    // Timers for game feel
    pub coyote_timer: f32,
    pub landing_recovery_timer: f32,

    // Wall mechanics
    pub wall_stamina: f32,
    pub wall_direction: i8,
    pub wall_jumps_remaining: u8,
    pub wall_jump_cooldown: f32,
    pub last_wall_jump_x: Option<f32>,
    pub same_wall_cooldown: f32,

    // Jet Boost
    pub jet_charges: u8,
    pub jet_timer: f32,
    pub jet_direction: Vec2,
    pub jet_regen_timer: f32,

    // Ink Cloud
    pub ink_charges: u8,
    pub ink_timer: f32,
    pub is_inked: bool,

    // Grapple/Swing
    pub grapple_point: Option<Vec2>,
    pub rope_length: f32,
    pub swing_angular_velocity: f32,

    // Visual effects
    pub visual_scale_y: f32,
    pub target_scale_y: f32,
    pub visual_rotation: f32,
    pub target_rotation: f32,
    pub breathing_phase: f32,
    pub hit_flash_timer: f32,
    pub invincibility_timer: f32,
}

impl Player {
    pub fn new(spawn_position: Vec2, config: &GameConfig) -> Self {
        Self {
            position: spawn_position,
            velocity: Vec2::ZERO,
            hitbox: Hitbox::new(config.player_hitbox.x, config.player_hitbox.y),
            state: PlayerState::Falling,
            facing_right: true,
            is_sprinting: false,
            coyote_timer: 0.0,
            landing_recovery_timer: 0.0,
            wall_stamina: config.wall_stamina_max,
            wall_direction: 0,
            wall_jumps_remaining: config.wall_jumps_max,
            wall_jump_cooldown: 0.0,
            last_wall_jump_x: None,
            same_wall_cooldown: 0.0,
            jet_charges: config.jet_max_charges,
            jet_timer: 0.0,
            jet_direction: Vec2::ZERO,
            jet_regen_timer: 0.0,
            ink_charges: config.ink_max_charges,
            ink_timer: 0.0,
            is_inked: false,
            grapple_point: None,
            rope_length: 0.0,
            swing_angular_velocity: 0.0,
            visual_scale_y: 1.0,
            target_scale_y: 1.0,
            visual_rotation: 0.0,
            target_rotation: 0.0,
            breathing_phase: 0.0,
            hit_flash_timer: 0.0,
            invincibility_timer: 0.0,
        }
    }

    /// Get the collision rectangle in world space
    pub fn collision_rect(&self) -> Rect {
        self.hitbox.to_rect(self.position)
    }

    /// Main update function - handles all player logic
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        input: &mut InputState,
        level: &TileMap,
        grapple_points: &[Vec2],
        config: &GameConfig,
        dt: f32,
        destroyed_blocks: &HashSet<(usize, usize)>,
        crumbling_platform_rects: &[Rect],
    ) {
        #[cfg(debug_assertions)]
        static FRAME_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        #[cfg(debug_assertions)]
        let frame = FRAME_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        #[cfg(debug_assertions)]
        let start_state = self.state;
        #[cfg(debug_assertions)]
        let start_pos = self.position;
        #[cfg(debug_assertions)]
        let start_vel = self.velocity;

        let pos_core = vec2_from_mq(self.position);

        // Collect nearby collision tiles (using into_iter avoids intermediate allocation)
        let nearby_tiles: Vec<Rect> = level.get_nearby_solid_rects_excluding(pos_core, 64.0, destroyed_blocks)
            .into_iter().map(rect_to_mq).collect();
        let oneway_tiles: Vec<Rect> = level.get_nearby_oneway_rects(pos_core, 64.0)
            .into_iter().map(rect_to_mq).collect();
        let bounce_tiles: Vec<Rect> = level.get_nearby_bounce_rects(pos_core, 64.0)
            .into_iter().map(rect_to_mq).collect();
        let player_rect = self.collision_rect();

        // === 1. Sense Environment ===
        // Include one-way platforms and crumbling platforms in ground check
        let on_ground = check_ground(player_rect, &nearby_tiles)
            || self.check_oneway_ground(&oneway_tiles)
            || check_ground(player_rect, crumbling_platform_rects);
        let wall_dir = check_wall(player_rect, &nearby_tiles);

        #[cfg(debug_assertions)]
        eprintln!(
            "\n[FRAME {}] START: state={:?}, pos=({:.2},{:.2}), vel=({:.2},{:.2}), on_ground={}, dt={:.4}",
            frame, start_state, start_pos.x, start_pos.y, start_vel.x, start_vel.y, on_ground, dt
        );

        // === 2. Update Timers ===
        self.update_timers(on_ground, config, dt);

        // === 3. Update Facing Direction ===
        if input.move_dir.x > config.input_deadzone {
            self.facing_right = true;
        } else if input.move_dir.x < -config.input_deadzone {
            self.facing_right = false;
        }

        // === 4. Global Ability Checks ===
        self.handle_ability_inputs(input, grapple_points, config);

        // === 5. State Transitions ===
        self.handle_state_transitions(input, on_ground, wall_dir, config, dt);

        // === 6. Apply Physics Based on State ===
        self.apply_state_physics(input, config, dt);

        #[cfg(debug_assertions)]
        eprintln!(
            "[FRAME {}] POST-PHYSICS: vel=({:.2},{:.2}), state={:?}",
            frame, self.velocity.x, self.velocity.y, self.state
        );

        // === 7. Variable Jump Height ===
        if self.state == PlayerState::Jumping && input.jump_released && self.velocity.y < 0.0 {
            self.velocity.y *= config.jump_cut_multiplier;
        }

        // === 8. Move and Resolve Collisions ===
        self.position.x += self.velocity.x * dt;
        self.resolve_horizontal_collision(&nearby_tiles, config);

        self.position.y += self.velocity.y * dt;
        self.resolve_vertical_collision(&nearby_tiles, config);
        self.resolve_oneway_collision(&oneway_tiles);

        // === 9. Check Bounce Pads ===
        self.check_bounce_pads(&bounce_tiles, config);

        // === 10. Debug Physics Sanity Checks ===
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.velocity.x.is_nan(), "velocity.x became NaN");
            debug_assert!(!self.velocity.y.is_nan(), "velocity.y became NaN");
            debug_assert!(!self.position.x.is_nan(), "position.x became NaN");
            debug_assert!(!self.position.y.is_nan(), "position.y became NaN");
            debug_assert!(
                !self.velocity.x.is_infinite(),
                "velocity.x became infinite: {}",
                self.velocity.x
            );
            debug_assert!(
                !self.velocity.y.is_infinite(),
                "velocity.y became infinite: {}",
                self.velocity.y
            );
            debug_assert!(
                self.velocity.x.abs() < 10000.0,
                "velocity.x exceeded sanity limit: {}",
                self.velocity.x
            );
            debug_assert!(
                self.velocity.y.abs() < 10000.0,
                "velocity.y exceeded sanity limit: {}",
                self.velocity.y
            );

            // End of frame summary - only log if state changed or significant movement
            let state_changed = self.state != start_state;
            let pos_changed = (self.position - start_pos).length() > 0.01;
            if state_changed || pos_changed {
                eprintln!(
                    "[FRAME {}] END: state={:?}, pos=({:.2},{:.2}), vel=({:.2},{:.2}){}",
                    frame, self.state, self.position.x, self.position.y, self.velocity.x, self.velocity.y,
                    if state_changed { " [STATE CHANGED]" } else { "" }
                );
            }
        }
    }
}

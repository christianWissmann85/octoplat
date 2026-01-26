//! Physics for dynamic platforms
//!
//! Handles moving platform interactions and collision resolution.

use macroquad::prelude::Rect;

use crate::config::GameConfig;
use crate::platforms::{CrumblingPlatform, MovingPlatform};
use crate::player::Player;

/// Apply velocity from moving platforms when player is standing on them
///
/// Accepts an iterator over platform references for HashMap compatibility.
pub fn apply_platform_movement<'a>(
    player: &mut Player,
    platforms: impl Iterator<Item = &'a MovingPlatform>,
    dt: f32,
) {
    let player_rect = player.collision_rect();
    let ground_check = Rect::new(
        player_rect.x + 2.0,
        player_rect.y + player_rect.h,
        player_rect.w - 4.0,
        4.0,
    );

    // Check moving platforms
    for platform in platforms {
        let platform_rect = platform.collision_rect();
        if ground_check.overlaps(&platform_rect) {
            // Player is standing on this platform, apply its velocity
            player.position.x += platform.velocity.x * dt;
        }
    }
}

/// Handle collisions with dynamic platforms (moving and crumbling)
///
/// Accepts iterators over platform references for HashMap compatibility.
pub fn handle_platform_collisions<'a, 'b>(
    player: &mut Player,
    moving_platforms: impl Iterator<Item = &'a MovingPlatform>,
    crumbling_platforms: impl Iterator<Item = &'b mut CrumblingPlatform>,
    config: &GameConfig,
) {
    let player_rect = player.collision_rect();

    // Ground check rect (just below player's feet)
    let ground_check = Rect::new(
        player_rect.x + 2.0,
        player_rect.y + player_rect.h - 2.0,
        player_rect.w - 4.0,
        6.0,
    );

    // Collision with moving platforms
    for platform in moving_platforms {
        let platform_rect = platform.collision_rect();

        if player_rect.overlaps(&platform_rect) {
            // Resolve vertical collision (landing on top)
            if player.velocity.y >= 0.0 {
                let player_bottom = player_rect.y + player_rect.h;
                let platform_top = platform_rect.y;

                if player_bottom > platform_top && player_bottom < platform_top + 16.0 {
                    player.position.y = platform_top - player_rect.h / 2.0;
                    player.velocity.y = 0.0;
                }
            }
        }
    }

    // Collision with crumbling platforms
    for platform in crumbling_platforms {
        if !platform.is_solid() {
            continue;
        }

        let platform_rect = platform.collision_rect();

        if player_rect.overlaps(&platform_rect) {
            // Resolve vertical collision
            if player.velocity.y >= 0.0 {
                let player_bottom = player_rect.y + player_rect.h;
                let platform_top = platform_rect.y;

                if player_bottom > platform_top && player_bottom < platform_top + 16.0 {
                    player.position.y = platform_top - player_rect.h / 2.0;
                    player.velocity.y = 0.0;
                }
            }
        }

        // Check if player is standing on crumbling platform (trigger it)
        if ground_check.overlaps(&platform_rect) {
            platform.trigger(config);
        }
    }
}

/// Update all moving platforms
///
/// Accepts an iterator over mutable platform references for HashMap compatibility.
pub fn update_moving_platforms<'a>(
    platforms: impl Iterator<Item = &'a mut MovingPlatform>,
    config: &GameConfig,
    dt: f32,
) {
    for platform in platforms {
        platform.update(config, dt);
    }
}

/// Update all crumbling platforms
///
/// Accepts an iterator over mutable platform references for HashMap compatibility.
pub fn update_crumbling_platforms<'a>(
    platforms: impl Iterator<Item = &'a mut CrumblingPlatform>,
    config: &GameConfig,
    dt: f32,
) {
    for platform in platforms {
        platform.update(config, dt);
    }
}

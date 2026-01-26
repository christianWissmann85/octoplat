//! Player collision resolution
//!
//! Handles collision detection and resolution against tiles, one-way platforms,
//! and bounce pads. Includes corner correction for smoother movement through gaps.

use macroquad::prelude::*;

use crate::collision::aabb_collision;
use crate::config::GameConfig;
use octoplat_core::constants::COLLISION;

use super::Player;
use super::PlayerState;

impl Player {
    /// Resolve horizontal collision with solid tiles
    pub(super) fn resolve_horizontal_collision(&mut self, tiles: &[Rect], config: &GameConfig) {
        let player_rect = self.collision_rect();

        for tile in tiles {
            let collision = aabb_collision(player_rect, *tile);
            if collision.collided && collision.normal.x.abs() > 0.5 {
                // Try corner correction if penetration is small enough
                if collision.penetration <= config.corner_correction_threshold {
                    if let Some(nudge) = self.try_corner_correction_vertical(tiles, config) {
                        self.position.y += nudge;
                        // Re-check collision against ALL tiles after nudge (not just current)
                        let new_rect = self.collision_rect();
                        let still_colliding = tiles.iter().any(|t| aabb_collision(new_rect, *t).collided);
                        if !still_colliding {
                            continue; // Nudge worked, skip normal pushback
                        }
                        // Nudge didn't fully resolve, revert and do normal pushback
                        self.position.y -= nudge;
                    }
                }

                self.position.x += collision.normal.x * collision.penetration;
                self.velocity.x = 0.0;
            }
        }
    }

    /// Resolve vertical collision with solid tiles
    pub(super) fn resolve_vertical_collision(&mut self, tiles: &[Rect], config: &GameConfig) {
        let player_rect = self.collision_rect();

        for tile in tiles {
            let collision = aabb_collision(player_rect, *tile);
            if collision.collided && collision.normal.y.abs() > 0.5 {
                // Try corner correction if penetration is small enough
                if collision.penetration <= config.corner_correction_threshold {
                    if let Some(nudge) = self.try_corner_correction_horizontal(tiles, config) {
                        self.position.x += nudge;
                        // Re-check collision against ALL tiles after nudge (not just current)
                        let new_rect = self.collision_rect();
                        let still_colliding = tiles.iter().any(|t| aabb_collision(new_rect, *t).collided);
                        if !still_colliding {
                            continue; // Nudge worked, skip normal pushback
                        }
                        // Nudge didn't fully resolve, revert and do normal pushback
                        self.position.x -= nudge;
                    }
                }

                self.position.y += collision.normal.y * collision.penetration;

                if collision.normal.y < 0.0 {
                    self.velocity.y = 0.0;
                } else {
                    self.velocity.y = self.velocity.y.max(0.0);
                }
            }
        }
    }

    /// Try to find a vertical nudge that would clear all collisions (for entering tight horizontal gaps)
    /// Only activates when player is moving horizontally
    fn try_corner_correction_vertical(&self, tiles: &[Rect], config: &GameConfig) -> Option<f32> {
        // Only apply when moving horizontally (trying to enter a gap)
        if self.velocity.x.abs() < COLLISION.corner_velocity_threshold {
            return None;
        }

        let threshold = config.corner_correction_threshold;
        let player_rect = self.collision_rect();

        // Try nudging up - must not cause ANY collision
        let nudged_up = Rect {
            x: player_rect.x,
            y: player_rect.y - threshold,
            w: player_rect.w,
            h: player_rect.h,
        };
        let collides_up = tiles.iter().any(|tile| aabb_collision(nudged_up, *tile).collided);

        if !collides_up {
            return Some(-threshold);
        }

        // Try nudging down - must not cause ANY collision
        let nudged_down = Rect {
            x: player_rect.x,
            y: player_rect.y + threshold,
            w: player_rect.w,
            h: player_rect.h,
        };
        let collides_down = tiles.iter().any(|tile| aabb_collision(nudged_down, *tile).collided);

        if !collides_down {
            return Some(threshold);
        }

        None
    }

    /// Try to find a horizontal nudge that would clear all collisions (for entering tight vertical gaps)
    /// Only activates when player is moving vertically (jumping/falling)
    fn try_corner_correction_horizontal(&self, tiles: &[Rect], config: &GameConfig) -> Option<f32> {
        // Only apply when moving vertically (jumping/falling into a gap)
        if self.velocity.y.abs() < COLLISION.corner_velocity_threshold {
            return None;
        }

        let threshold = config.corner_correction_threshold;
        let player_rect = self.collision_rect();

        // Try nudging left - must not cause ANY collision
        let nudged_left = Rect {
            x: player_rect.x - threshold,
            y: player_rect.y,
            w: player_rect.w,
            h: player_rect.h,
        };
        let collides_left = tiles.iter().any(|tile| aabb_collision(nudged_left, *tile).collided);

        if !collides_left {
            return Some(-threshold);
        }

        // Try nudging right - must not cause ANY collision
        let nudged_right = Rect {
            x: player_rect.x + threshold,
            y: player_rect.y,
            w: player_rect.w,
            h: player_rect.h,
        };
        let collides_right = tiles.iter().any(|tile| aabb_collision(nudged_right, *tile).collided);

        if !collides_right {
            return Some(threshold);
        }

        None
    }

    /// Check if standing on a one-way platform (for ground detection)
    pub(super) fn check_oneway_ground(&self, oneway_tiles: &[Rect]) -> bool {
        if self.velocity.y < 0.0 {
            return false;
        }

        let player_rect = self.collision_rect();
        let ground_check = Rect {
            x: player_rect.x + 2.0,
            y: player_rect.y + player_rect.h,
            w: player_rect.w - 4.0,
            h: 2.0,
        };

        oneway_tiles.iter().any(|tile| ground_check.overlaps(tile))
    }

    /// Check if landing on bounce pads and apply bounce
    pub(super) fn check_bounce_pads(&mut self, bounce_tiles: &[Rect], config: &GameConfig) {
        let player_rect = self.collision_rect();
        let player_bottom = player_rect.y + player_rect.h;

        for tile in bounce_tiles {
            let tile_top = tile.y;

            if player_bottom >= tile_top && player_bottom <= tile_top + COLLISION.bounce_pad_collision_window {
                let player_right = player_rect.x + player_rect.w;
                let tile_right = tile.x + tile.w;

                if player_rect.x < tile_right && player_right > tile.x {
                    self.velocity.y = -config.bounce_velocity;
                    self.state = PlayerState::Jumping;
                }
            }
        }
    }

    /// Resolve collision with one-way platforms (only from above when falling)
    pub(super) fn resolve_oneway_collision(&mut self, oneway_tiles: &[Rect]) {
        if self.velocity.y < 0.0 {
            return;
        }

        let player_rect = self.collision_rect();
        let player_bottom = player_rect.y + player_rect.h;

        for tile in oneway_tiles {
            let tile_top = tile.y;

            if player_bottom >= tile_top && player_bottom <= tile_top + COLLISION.one_way_platform_window {
                let player_right = player_rect.x + player_rect.w;
                let tile_right = tile.x + tile.w;

                if player_rect.x < tile_right && player_right > tile.x {
                    self.position.y = tile_top - player_rect.h / 2.0;
                    self.velocity.y = 0.0;
                }
            }
        }
    }
}

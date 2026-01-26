//! Collision detection for gameplay
//!
//! Handles collision checks between player and hazards, enemies, and breakable blocks.

use std::collections::HashSet;
use macroquad::prelude::{Rect, Vec2};

use octoplat_core::level::TileMap;

use crate::compat::{vec2_from_mq, rect_to_mq};
use crate::config::GameConfig;
use crate::hazards::{Crab, Pufferfish};
use crate::player::Player;

/// Result of hazard collision check
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HazardCollisionResult {
    /// Damage amount (0 = no collision)
    pub damage: u8,
    /// Position of the hazard that was hit (for knockback/effects)
    pub position: Option<Vec2>,
}

impl HazardCollisionResult {
    pub fn none() -> Self {
        Self { damage: 0, position: None }
    }

    pub fn hit(damage: u8, position: Vec2) -> Self {
        Self { damage, position: Some(position) }
    }
}

/// Check if player is touching any hazard tiles
///
/// Returns the damage amount if hit (spike_damage from config), or 0 if no collision.
pub fn check_hazard_collision(player: &Player, tilemap: &TileMap, config: &GameConfig) -> HazardCollisionResult {
    let pos_core = vec2_from_mq(player.position);
    let hazard_rects: Vec<Rect> = tilemap.get_nearby_hazard_rects(pos_core, 64.0)
        .into_iter().map(rect_to_mq).collect();
    let player_rect = player.collision_rect();

    for hazard in hazard_rects {
        if player_rect.overlaps(&hazard) {
            let hazard_center = Vec2::new(
                hazard.x + hazard.w / 2.0,
                hazard.y + hazard.h / 2.0,
            );
            return HazardCollisionResult::hit(config.spike_damage, hazard_center);
        }
    }
    HazardCollisionResult::none()
}

/// Type of enemy that was killed or that hit the player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Crab,
    Pufferfish,
}

/// Result of an enemy collision check
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyCollisionResult {
    /// No collision occurred
    None,
    /// Player was hit by enemy - includes damage, position, and enemy type
    PlayerHit {
        damage: u8,
        position: Vec2,
        enemy_type: EnemyType,
    },
    /// Enemy was killed by jet boost - includes position and type for effects
    EnemyKilled {
        position: Vec2,
        enemy_type: EnemyType,
    },
}

/// Check enemy collisions, handling jet boost kills
///
/// Returns the collision result. If EnemyKilled, the enemy is marked as dead
/// and the player is bounced. The caller must handle the PlayerHit case.
/// Player is invincible during jet boost (any direction kills enemies).
///
/// Accepts iterators over mutable references for HashMap compatibility.
pub fn check_enemy_collision<'a>(
    player: &mut Player,
    crabs: impl Iterator<Item = &'a mut Crab>,
    pufferfish: impl Iterator<Item = &'a mut Pufferfish>,
    config: &GameConfig,
) -> EnemyCollisionResult {
    use crate::player::PlayerState;

    let player_rect = player.collision_rect();
    let is_attacking = player.state == PlayerState::JetBoosting;

    // Check crab collisions
    for crab in crabs {
        if !crab.alive {
            continue;
        }

        if player_rect.overlaps(&crab.collision_rect()) {
            if is_attacking {
                // Jet boost kills the crab, bounce player
                let crab_pos = crab.position;
                crab.alive = false;
                player.velocity.y = -config.bounce_velocity * 0.5;
                player.state = PlayerState::Jumping;
                player.jet_timer = 0.0; // End jet on kill
                return EnemyCollisionResult::EnemyKilled {
                    position: crab_pos,
                    enemy_type: EnemyType::Crab,
                };
            } else if !player.is_inked && !player.is_invincible() {
                // Player takes damage (unless invincible)
                return EnemyCollisionResult::PlayerHit {
                    damage: config.crab_damage,
                    position: crab.position,
                    enemy_type: EnemyType::Crab,
                };
            }
        }
    }

    // Check pufferfish collisions
    for puffer in pufferfish {
        if !puffer.alive {
            continue;
        }

        if player_rect.overlaps(&puffer.collision_rect()) {
            if is_attacking {
                // Jet boost kills the pufferfish, bounce player
                let puffer_pos = puffer.position;
                puffer.alive = false;
                player.velocity.y = -config.bounce_velocity * 0.5;
                player.state = PlayerState::Jumping;
                player.jet_timer = 0.0; // End jet on kill
                return EnemyCollisionResult::EnemyKilled {
                    position: puffer_pos,
                    enemy_type: EnemyType::Pufferfish,
                };
            } else if !player.is_inked && !player.is_invincible() {
                // Player takes damage (unless invincible)
                return EnemyCollisionResult::PlayerHit {
                    damage: config.pufferfish_damage,
                    position: puffer.position,
                    enemy_type: EnemyType::Pufferfish,
                };
            }
        }
    }

    EnemyCollisionResult::None
}

/// Check if downward jet player breaks any breakable blocks
///
/// Returns true if a block was broken, in which case the player is bounced.
pub fn check_breakable_blocks(
    player: &mut Player,
    tilemap: &TileMap,
    destroyed_blocks: &mut HashSet<(usize, usize)>,
    config: &GameConfig,
) -> bool {
    use crate::player::PlayerState;

    // Only break blocks when jet boosting downward with downward velocity
    if !player.is_jet_downward() || player.velocity.y <= 0.0 {
        return false;
    }

    let pos_core = vec2_from_mq(player.position);
    let breakable_tiles: Vec<(usize, usize, Rect)> = tilemap
        .get_nearby_breakable_tiles(pos_core, 64.0)
        .into_iter()
        .map(|(x, y, r)| (x, y, rect_to_mq(r)))
        .collect();
    let player_rect = player.collision_rect();

    // Project where the player will be - check current position plus some lookahead
    // based on jet velocity to catch the collision before it happens
    let lookahead = player.velocity.y * 0.02; // ~1 frame of movement
    let jet_check = Rect {
        x: player_rect.x + 2.0,
        y: player_rect.y + player_rect.h / 2.0,
        w: player_rect.w - 4.0,
        h: player_rect.h / 2.0 + lookahead.max(8.0),
    };

    for (x, y, rect) in breakable_tiles {
        // Skip already destroyed blocks
        if destroyed_blocks.contains(&(x, y)) {
            continue;
        }

        // Check if player's jet hitbox will hit the breakable block
        if jet_check.overlaps(&rect) {
            // Destroy the block
            destroyed_blocks.insert((x, y));

            // Bounce player up (cancel jet)
            player.velocity.y = -config.bounce_velocity * 0.6;
            player.state = PlayerState::Jumping;
            player.jet_timer = 0.0; // End jet on break
            return true; // Only break one block at a time
        }
    }

    false
}

/// Check if player has fallen below the level bounds
pub fn check_fall_death(player: &Player, level_bounds: Rect) -> bool {
    player.position.y > level_bounds.h + 100.0
}

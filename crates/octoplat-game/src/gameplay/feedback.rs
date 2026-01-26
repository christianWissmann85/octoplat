//! Audio and visual feedback processing
//!
//! Detects state changes and triggers appropriate audio and visual effects.

use macroquad::prelude::*;
use octoplat_core::physics::FeedbackTracker;
use octoplat_core::save::SaveData;
use octoplat_core::state::PlayerState;

use crate::audio::SoundId;
use crate::config::GameConfig;
use crate::effects::EffectsManager;
use crate::player::Player;

/// Result of feedback processing - sounds to play and any statistics updates
pub struct FeedbackResult {
    /// Sounds to play (with optional position for spatial audio)
    pub sounds: Vec<(SoundId, Option<Vec2>)>,
}

impl FeedbackResult {
    pub fn new() -> Self {
        Self { sounds: Vec::new() }
    }

    pub fn add_sound(&mut self, id: SoundId) {
        self.sounds.push((id, None));
    }

    pub fn add_sound_at(&mut self, id: SoundId, pos: Vec2) {
        self.sounds.push((id, Some(pos)));
    }
}

impl Default for FeedbackResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Process player state changes and trigger feedback effects
///
/// Returns sounds that should be played. Updates effects and save_data statistics.
pub fn process_feedback(
    player: &mut Player,
    tracker: &mut FeedbackTracker,
    gems_collected: u32,
    effects: &mut EffectsManager,
    config: &GameConfig,
    save_data: &mut SaveData,
) -> FeedbackResult {
    let mut result = FeedbackResult::new();

    let current_state = player.state;
    let player_pos = player.position;
    let player_feet = vec2(player_pos.x, player_pos.y + 16.0);

    // Detect state transitions
    if current_state != tracker.prev_player_state {
        match current_state {
            PlayerState::Jumping => {
                // Check if it was a wall jump
                if tracker.prev_player_state == PlayerState::WallGrip {
                    result.add_sound(SoundId::WallJump);
                    effects.spawn_wall_jump(player_pos, player.wall_direction);
                } else {
                    result.add_sound(SoundId::Jump);
                    effects.spawn_jump(player_feet);
                }
                // Stretch on jump
                player.trigger_stretch();
                // Track jump statistic
                save_data.total_jumps += 1;
            }
            PlayerState::Idle | PlayerState::Running => {
                // Landed from air
                if matches!(
                    tracker.prev_player_state,
                    PlayerState::Falling | PlayerState::Jumping | PlayerState::JetBoosting
                ) {
                    result.add_sound(SoundId::Land);
                    // Land intensity based on fall velocity
                    let intensity = (tracker.prev_velocity_y / 500.0).clamp(0.3, 1.5);
                    effects.spawn_land(player_feet, intensity);
                    // Squash on landing with overshoot animation based on intensity
                    player.trigger_squash(intensity);
                    // Hard landing recovery (brief movement slowdown)
                    if tracker.prev_velocity_y > config.hard_landing_threshold {
                        player.trigger_landing_recovery(config.landing_recovery_time);
                    }
                }
            }
            PlayerState::JetBoosting => {
                // Use dive sound for downward jets, regular jet sound otherwise
                if player.is_jet_downward() {
                    result.add_sound(SoundId::Dive);
                } else {
                    result.add_sound(SoundId::JetBoost);
                }
                effects.spawn_jet_boost(player_pos, player.jet_direction);
            }
            PlayerState::Swinging => {
                // Play grapple shoot sound at player position
                result.add_sound(SoundId::GrappleShoot);
                // Play attach sound at grapple point (spatial audio)
                if let Some(grapple_pos) = player.grapple_point {
                    result.add_sound_at(SoundId::GrappleAttach, grapple_pos);
                    effects.spawn_grapple_attach(grapple_pos);
                }
                // Track grapple statistic
                save_data.total_grapples += 1;
            }
            _ => {}
        }

        // Downward jet impact detection (JetBoosting downward -> something else)
        if tracker.prev_player_state == PlayerState::JetBoosting
            && tracker.prev_velocity_y > 100.0  // Was moving down fast
            && matches!(current_state, PlayerState::Jumping | PlayerState::Idle | PlayerState::Running)
        {
            effects.spawn_dive_impact(player_feet);
        }
    }

    // Jet boost trail (continuous while boosting)
    if current_state == PlayerState::JetBoosting
        && rand::gen_range(0.0, 1.0) < 0.5 {
            effects.spawn_jet_boost(player_pos, player.jet_direction);
        }

    // Check for gem collection
    if gems_collected > tracker.prev_gems_collected {
        result.add_sound(SoundId::GemCollect);
        // Gems handle their own position, so we use player pos as approximation
        effects.spawn_gem_collect(player_pos);
    }

    // Check for bounce pad (sudden upward velocity while not jumping normally)
    let bounce_threshold = -config.bounce_velocity * 0.9;
    if player.velocity.y < bounce_threshold && tracker.prev_velocity_y >= 0.0 {
        result.add_sound(SoundId::BouncePad);
        effects.spawn_bounce(player_feet);
    }

    // Detect ink activation
    if player.is_inked && !tracker.prev_is_inked {
        effects.spawn_ink_cloud(player_pos);
        result.add_sound(SoundId::InkShoot);
    }

    // Update tracking
    tracker.update(
        current_state,
        gems_collected,
        player.velocity.y,
        player.is_inked,
    );

    result
}

//! Feedback tracking for audio and visual effects
//!
//! Tracks state changes to trigger appropriate feedback effects.

use crate::Vec2;
use crate::state::PlayerState;

/// Tracks previous state for detecting changes that trigger feedback
#[derive(Clone, Debug)]
pub struct FeedbackTracker {
    /// Previous player state for transition detection
    pub prev_player_state: PlayerState,
    /// Previous gem count for collection detection
    pub prev_gems_collected: u32,
    /// Previous checkpoint for activation detection
    pub prev_checkpoint: Option<Vec2>,
    /// Previous vertical velocity for landing/bounce detection
    pub prev_velocity_y: f32,
    /// Previous ink state for ink activation detection
    pub prev_is_inked: bool,
    /// Whether level complete sound has been played
    pub played_level_complete_sound: bool,
}

impl FeedbackTracker {
    pub fn new() -> Self {
        Self {
            prev_player_state: PlayerState::Falling,
            prev_gems_collected: 0,
            prev_checkpoint: None,
            prev_velocity_y: 0.0,
            prev_is_inked: false,
            played_level_complete_sound: false,
        }
    }

    /// Update tracking state after processing feedback
    pub fn update(&mut self, state: PlayerState, gems: u32, vel_y: f32, inked: bool) {
        self.prev_player_state = state;
        self.prev_gems_collected = gems;
        self.prev_velocity_y = vel_y;
        self.prev_is_inked = inked;
    }

    /// Set the current checkpoint position
    pub fn set_checkpoint(&mut self, pos: Option<Vec2>) {
        self.prev_checkpoint = pos;
    }

    /// Mark that the level complete sound has been played
    pub fn mark_level_complete_played(&mut self) {
        self.played_level_complete_sound = true;
    }

    /// Reset level complete sound flag (for new levels)
    pub fn reset_level_complete(&mut self) {
        self.played_level_complete_sound = false;
    }
}

impl Default for FeedbackTracker {
    fn default() -> Self {
        Self::new()
    }
}

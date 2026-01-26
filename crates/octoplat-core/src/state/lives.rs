//! Lives management system
//!
//! Tracks player lives and session death counts.

/// Manages player lives and session statistics
#[derive(Clone, Debug)]
pub struct LivesManager {
    /// Current number of lives
    pub current: u32,
    /// Deaths in current session/level
    pub session_deaths: u32,
    /// Gem milestone for next extra life (roguelite mode)
    pub next_life_gems: u32,
}

impl LivesManager {
    pub fn new(starting: u32) -> Self {
        Self {
            current: starting,
            session_deaths: 0,
            next_life_gems: 50,
        }
    }

    /// Start a new session with specified starting lives
    ///
    /// If `roguelite` is true, sets the gem milestone for earning extra lives.
    pub fn start_session(&mut self, starting_lives: u32, gem_milestone: u32, roguelite: bool) {
        self.current = starting_lives;
        self.session_deaths = 0;
        if roguelite {
            self.next_life_gems = gem_milestone;
        }
    }

    /// Award an extra life, returns true if actually awarded (not at max)
    pub fn award_life(&mut self, max: u32) -> bool {
        if self.current < max {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Check gem milestone and award life if reached
    /// Returns true if a life was awarded
    pub fn check_gem_milestone(&mut self, total_gems: u32, increment: u32, max: u32) -> bool {
        if total_gems >= self.next_life_gems {
            self.next_life_gems += increment;
            self.award_life(max)
        } else {
            false
        }
    }

    /// Set infinite lives (for debug/testing)
    pub fn set_infinite(&mut self) {
        self.current = u32::MAX;
    }

    /// Check if out of lives (game over state)
    pub fn is_game_over(&self) -> bool {
        self.current == 0
    }

    /// Reset session deaths counter
    pub fn reset_session(&mut self) {
        self.session_deaths = 0;
    }
}

impl Default for LivesManager {
    fn default() -> Self {
        Self::new(5)
    }
}

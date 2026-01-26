//! Application state machine controller
//!
//! Manages app state transitions and screen transition animations.

use crate::app_state::AppState;
use crate::ui::Transition;

/// Lifecycle event emitted during state transitions
#[derive(Debug, Clone)]
pub enum StateLifecycleEvent {
    /// State is being exited
    Exit(AppState),
    /// State is being entered
    Enter(AppState),
}

/// Controls the application state machine and screen transitions.
///
/// This subsystem handles:
/// - Current app state tracking (Title, MainMenu, Playing, etc.)
/// - Screen transition animations (fade in/out)
/// - Title screen timing
/// - Lifecycle event tracking for state transitions
pub struct StateController {
    /// Current application state
    pub app_state: AppState,

    /// Active screen transition (fade effect)
    pub transition: Option<Transition>,

    /// Target state for transition completion
    pub transition_target: Option<AppState>,

    /// Time spent on title screen (for animation timing)
    pub title_time: f32,

    /// Pending lifecycle events to be processed by the game loop
    pending_events: Vec<StateLifecycleEvent>,
}

impl StateController {
    /// Create a new StateController starting at the title screen
    pub fn new() -> Self {
        Self {
            app_state: AppState::Title,
            transition: None,
            transition_target: None,
            title_time: 0.0,
            pending_events: Vec::new(),
        }
    }

    /// Start a screen transition to a new state
    ///
    /// The transition will fade out, switch states at the midpoint,
    /// then fade back in.
    pub fn start_transition(&mut self, target: AppState, duration: f32) {
        self.transition = Some(Transition::new(duration));
        self.transition_target = Some(target);
    }

    /// Set the app state directly without transition
    ///
    /// Emits lifecycle events for the transition.
    pub fn set_state(&mut self, state: AppState) {
        self.emit_exit_event();
        self.app_state = state;
        self.emit_enter_event();
    }

    /// Transition to a new state with lifecycle hooks
    ///
    /// This is the preferred method for state transitions as it properly
    /// emits lifecycle events that the game loop can respond to.
    pub fn transition_to(&mut self, target: AppState) {
        self.emit_exit_event();
        self.app_state = target;
        self.emit_enter_event();
    }

    /// Update the transition animation
    ///
    /// Returns true if a transition is complete this frame.
    pub fn update_transition(&mut self, dt: f32) -> bool {
        if let Some(ref mut transition) = self.transition {
            if transition.update(dt) {
                // Transition complete
                self.transition = None;
                return true;
            } else if transition.should_switch() {
                // At midpoint, switch state and emit lifecycle events
                if let Some(target) = self.transition_target.take() {
                    self.emit_exit_event();
                    self.app_state = target;
                    self.emit_enter_event();
                }
            }
        }
        false
    }

    /// Check if we're in the middle of a transition
    pub fn is_transitioning(&self) -> bool {
        self.transition.is_some()
    }

    /// Get the current fade alpha for the transition overlay
    pub fn fade_alpha(&self) -> Option<f32> {
        self.transition.as_ref().map(|t| t.fade_alpha())
    }

    /// Update title screen time
    pub fn update_title_time(&mut self, dt: f32) {
        self.title_time += dt;
    }

    // ========================================================================
    // Lifecycle Event Management
    // ========================================================================

    /// Emit an exit event for the current state
    fn emit_exit_event(&mut self) {
        self.pending_events
            .push(StateLifecycleEvent::Exit(self.app_state.clone()));
    }

    /// Emit an enter event for the current state
    fn emit_enter_event(&mut self) {
        self.pending_events
            .push(StateLifecycleEvent::Enter(self.app_state.clone()));
    }

    /// Take all pending lifecycle events
    ///
    /// Call this from the game loop to process state transition events.
    /// Events are cleared after retrieval.
    pub fn take_lifecycle_events(&mut self) -> Vec<StateLifecycleEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Check if there are pending lifecycle events
    pub fn has_pending_events(&self) -> bool {
        !self.pending_events.is_empty()
    }
}

impl Default for StateController {
    fn default() -> Self {
        Self::new()
    }
}

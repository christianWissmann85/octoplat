//! Gamepad input handling
//!
//! On native platforms, uses the `gamepads` crate for controller support.
//! On web (WASM), gamepad support is disabled due to wasm-bindgen conflicts with macroquad.

use macroquad::prelude::*;

/// Processed gamepad input for the current frame
#[derive(Default, Clone)]
pub struct GamepadInput {
    // Movement (after deadzone applied)
    pub move_dir: Vec2,

    // Actions (current frame state)
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub jump_released: bool,
    pub grapple_pressed: bool,
    pub grapple_held: bool,
    pub sprint_held: bool,
    pub jet_boost_pressed: bool,
    pub ink_pressed: bool,
    pub dive_pressed: bool,

    // Menu navigation
    pub menu_up: bool,
    pub menu_down: bool,
    pub menu_left: bool,
    pub menu_right: bool,
    pub menu_confirm: bool,
    pub menu_cancel: bool,
    pub pause_pressed: bool,

    // UI toggles
    pub minimap_toggle_pressed: bool,
    pub minimap_zoom_in_pressed: bool,
    pub minimap_zoom_out_pressed: bool,

    // Whether a gamepad is connected and providing input
    pub active: bool,
}

// ============================================================================
// Native implementation (with gamepads crate)
// ============================================================================
#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use gamepads::{Button, Gamepads};

    /// Manages gamepad state and input processing
    pub struct GamepadManager {
        gamepads: Gamepads,
        deadzone: f32,

        // Previous button states for edge detection
        prev_jump: bool,
        prev_grapple: bool,
        prev_jet: bool,
        prev_ink: bool,
        prev_dive: bool,
        prev_start: bool,
        prev_dpad_up: bool,
        prev_dpad_down: bool,
        prev_dpad_left: bool,
        prev_dpad_right: bool,
        prev_back: bool,
    }

    impl GamepadManager {
        pub fn new(deadzone: f32) -> Self {
            Self {
                gamepads: Gamepads::new(),
                deadzone,
                prev_jump: false,
                prev_grapple: false,
                prev_jet: false,
                prev_ink: false,
                prev_dive: false,
                prev_start: false,
                prev_dpad_up: false,
                prev_dpad_down: false,
                prev_dpad_left: false,
                prev_dpad_right: false,
                prev_back: false,
            }
        }

        /// Poll gamepads and get processed input
        pub fn poll(&mut self) -> GamepadInput {
            self.gamepads.poll();

            let mut input = GamepadInput::default();

            // Find first connected gamepad
            let Some(gamepad) = self.gamepads.all().next() else {
                // No gamepad connected - reset previous states
                self.reset_prev_states();
                return input;
            };

            input.active = true;

            // Left stick movement with circular deadzone
            let (stick_x, stick_y) = gamepad.left_stick();
            let stick = vec2(stick_x, stick_y);
            let stick_len = stick.length();

            if stick_len > self.deadzone {
                // Remap from deadzone->1 to 0->1
                let normalized_len = (stick_len - self.deadzone) / (1.0 - self.deadzone);
                input.move_dir = stick.normalize() * normalized_len.min(1.0);
            }

            // Button mapping:
            // A = Jump (ActionDown)
            // B = Slide (ActionRight)
            // X = Ink (ActionLeft)
            // RT = Grapple/Wall Grab (FrontRightLower)
            // LB = Sprint (FrontLeftUpper)
            // RB = Jet Boost (FrontRightUpper)
            // Start = Pause (RightCenterCluster)
            // D-pad for menu navigation

            let jump = gamepad.is_currently_pressed(Button::ActionDown);
            let grapple = gamepad.is_currently_pressed(Button::FrontRightLower);
            let jet = gamepad.is_currently_pressed(Button::FrontRightUpper);
            let ink = gamepad.is_currently_pressed(Button::ActionLeft);
            let sprint = gamepad.is_currently_pressed(Button::FrontLeftUpper);
            let start = gamepad.is_currently_pressed(Button::RightCenterCluster);
            let back = gamepad.is_currently_pressed(Button::LeftCenterCluster);

            // D-pad
            let dpad_up = gamepad.is_currently_pressed(Button::DPadUp);
            let dpad_down = gamepad.is_currently_pressed(Button::DPadDown);
            let dpad_left = gamepad.is_currently_pressed(Button::DPadLeft);
            let dpad_right = gamepad.is_currently_pressed(Button::DPadRight);

            // Edge detection for actions
            input.jump_pressed = jump && !self.prev_jump;
            input.jump_held = jump;
            input.jump_released = !jump && self.prev_jump;

            input.grapple_pressed = grapple && !self.prev_grapple;
            input.grapple_held = grapple;

            input.jet_boost_pressed = jet && !self.prev_jet;
            input.ink_pressed = ink && !self.prev_ink;
            input.sprint_held = sprint;

            // Dive on pressing down on stick or D-pad down
            let stick_down = input.move_dir.y > 0.5;
            let dive_input = stick_down || dpad_down;
            input.dive_pressed = dive_input && !self.prev_dive;

            // Menu navigation (edge-detected)
            input.menu_up = dpad_up && !self.prev_dpad_up;
            input.menu_down = dpad_down && !self.prev_dpad_down;
            input.menu_left = dpad_left && !self.prev_dpad_left;
            input.menu_right = dpad_right && !self.prev_dpad_right;
            input.menu_confirm = input.jump_pressed;
            input.menu_cancel = start && !self.prev_start;
            input.pause_pressed = start && !self.prev_start;

            // UI toggles (edge-detected)
            input.minimap_toggle_pressed = back && !self.prev_back;
            // D-pad left/right also used for minimap zoom during gameplay
            input.minimap_zoom_in_pressed = dpad_right && !self.prev_dpad_right;
            input.minimap_zoom_out_pressed = dpad_left && !self.prev_dpad_left;

            // Update previous states
            self.prev_jump = jump;
            self.prev_grapple = grapple;
            self.prev_jet = jet;
            self.prev_ink = ink;
            self.prev_dive = dive_input;
            self.prev_start = start;
            self.prev_dpad_up = dpad_up;
            self.prev_dpad_down = dpad_down;
            self.prev_dpad_left = dpad_left;
            self.prev_dpad_right = dpad_right;
            self.prev_back = back;

            input
        }

        fn reset_prev_states(&mut self) {
            self.prev_jump = false;
            self.prev_grapple = false;
            self.prev_jet = false;
            self.prev_ink = false;
            self.prev_dive = false;
            self.prev_start = false;
            self.prev_dpad_up = false;
            self.prev_dpad_down = false;
            self.prev_dpad_left = false;
            self.prev_dpad_right = false;
            self.prev_back = false;
        }
    }
}

// ============================================================================
// Web (WASM) stub implementation - no gamepad support
// ============================================================================
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;

    /// Stub gamepad manager for web builds (no controller support)
    pub struct GamepadManager {
        _deadzone: f32,
    }

    impl GamepadManager {
        pub fn new(deadzone: f32) -> Self {
            Self { _deadzone: deadzone }
        }

        /// Always returns empty input on web (no gamepad support)
        pub fn poll(&mut self) -> GamepadInput {
            GamepadInput::default()
        }
    }
}

// Re-export the appropriate implementation
#[cfg(not(target_arch = "wasm32"))]
pub use native::GamepadManager;

#[cfg(target_arch = "wasm32")]
pub use web::GamepadManager;

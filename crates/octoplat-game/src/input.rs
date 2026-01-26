use macroquad::prelude::*;

use crate::gamepad::GamepadInput;

/// Processed input with buffering support
#[derive(Default, Clone)]
pub struct InputState {
    // Directional (normalized)
    pub move_dir: Vec2,

    // Actions (current frame)
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub jump_released: bool,
    pub sprint_held: bool,       // Shift key
    pub dive_pressed: bool,      // Down while in air
    pub jet_boost_pressed: bool, // E key
    pub ink_pressed: bool,       // Q key
    pub grapple_pressed: bool,   // F key or right mouse
    pub grapple_held: bool,      // Holding grapple

    // Menu navigation (edge-detected)
    pub menu_up: bool,
    pub menu_down: bool,
    pub menu_left: bool,
    pub menu_right: bool,
    pub menu_confirm: bool,  // Space or Enter
    pub menu_cancel: bool,   // Escape
    pub menu_delete: bool,   // Delete or Backspace
    pub pause_pressed: bool, // Escape during gameplay

    // UI toggles
    pub minimap_toggle_pressed: bool, // M key
    pub minimap_zoom_in_pressed: bool,  // + or = key
    pub minimap_zoom_out_pressed: bool, // - key

    // Buffered actions
    pub jump_buffer_active: bool,
    jump_buffer_time: f32,
    pub grapple_buffer_active: bool,
    grapple_buffer_time: f32,
    pub dive_buffer_active: bool,
    dive_buffer_time: f32,

    // Raw tracking for edge detection
    prev_jump: bool,
    prev_down: bool,
    prev_up: bool,
    prev_left: bool,
    prev_right: bool,
    prev_enter: bool,
    prev_escape: bool,
    prev_delete: bool,
    prev_minimap: bool,
    prev_zoom_in: bool,
    prev_zoom_out: bool,

    // Gamepad tracking
    pub using_gamepad: bool,
}

impl InputState {
    pub fn update(&mut self, dt: f32, buffer_window: f32) {
        // Read raw input
        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let jump_key = is_key_down(KeyCode::Space);
        let sprint_key = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let jet_key = is_key_pressed(KeyCode::E);
        let ink_key = is_key_pressed(KeyCode::Q);
        let grapple_key = is_key_down(KeyCode::F) || is_mouse_button_down(MouseButton::Right);
        let enter_key = is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter);
        let escape_key = is_key_down(KeyCode::Escape);
        let delete_key = is_key_down(KeyCode::Delete) || is_key_down(KeyCode::Backspace);
        let minimap_key = is_key_down(KeyCode::M);
        let zoom_in_key = is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd);
        let zoom_out_key = is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract);

        // Build movement vector (8-directional)
        let mut dir = Vec2::ZERO;
        if left {
            dir.x -= 1.0;
        }
        if right {
            dir.x += 1.0;
        }
        if up {
            dir.y -= 1.0;
        }
        if down {
            dir.y += 1.0;
        }

        // Normalize for consistent diagonal speed
        self.move_dir = if dir.length_squared() > 0.0 {
            dir.normalize()
        } else {
            Vec2::ZERO
        };

        // Jump edge detection
        self.jump_pressed = jump_key && !self.prev_jump;
        self.jump_held = jump_key;
        self.jump_released = !jump_key && self.prev_jump;
        self.prev_jump = jump_key;

        // Jump buffering - start/refresh buffer on press
        if self.jump_pressed {
            self.jump_buffer_time = buffer_window;
        }

        // Tick down buffer
        self.jump_buffer_time = (self.jump_buffer_time - dt).max(0.0);
        self.jump_buffer_active = self.jump_buffer_time > 0.0;

        // Sprint (held)
        self.sprint_held = sprint_key;

        // Dive (down pressed while in air - edge detection + buffer)
        self.dive_pressed = down && !self.prev_down;
        if self.dive_pressed {
            self.dive_buffer_time = buffer_window;
        }
        self.dive_buffer_time = (self.dive_buffer_time - dt).max(0.0);
        self.dive_buffer_active = self.dive_buffer_time > 0.0;

        // Jet boost and ink
        self.jet_boost_pressed = jet_key;
        self.ink_pressed = ink_key;

        // Grapple (toggle: press to attach, press again to release) + buffer
        let grapple_just_pressed = grapple_key && !self.grapple_held;
        self.grapple_pressed = grapple_just_pressed;
        self.grapple_held = grapple_key;
        if grapple_just_pressed {
            self.grapple_buffer_time = buffer_window;
        }
        self.grapple_buffer_time = (self.grapple_buffer_time - dt).max(0.0);
        self.grapple_buffer_active = self.grapple_buffer_time > 0.0;

        // Menu navigation (edge-detected for single presses)
        self.menu_up = up && !self.prev_up;
        self.menu_down = down && !self.prev_down;
        self.menu_left = left && !self.prev_left;
        self.menu_right = right && !self.prev_right;
        self.menu_confirm = (enter_key && !self.prev_enter) || self.jump_pressed;
        self.menu_cancel = escape_key && !self.prev_escape;
        self.menu_delete = delete_key && !self.prev_delete;
        self.pause_pressed = escape_key && !self.prev_escape;

        // UI toggles (edge-detected)
        self.minimap_toggle_pressed = minimap_key && !self.prev_minimap;
        self.minimap_zoom_in_pressed = zoom_in_key && !self.prev_zoom_in;
        self.minimap_zoom_out_pressed = zoom_out_key && !self.prev_zoom_out;

        // Update previous state for edge detection
        self.prev_down = down;
        self.prev_up = up;
        self.prev_left = left;
        self.prev_right = right;
        self.prev_enter = enter_key;
        self.prev_escape = escape_key;
        self.prev_delete = delete_key;
        self.prev_minimap = minimap_key;
        self.prev_zoom_in = zoom_in_key;
        self.prev_zoom_out = zoom_out_key;
    }

    /// Consume the jump buffer (call when jump executes)
    pub fn consume_jump_buffer(&mut self) {
        self.jump_buffer_time = 0.0;
        self.jump_buffer_active = false;
    }

    /// Consume the grapple buffer (call when grapple executes)
    pub fn consume_grapple_buffer(&mut self) {
        self.grapple_buffer_time = 0.0;
        self.grapple_buffer_active = false;
    }

    /// Consume the dive buffer (call when dive executes)
    pub fn consume_dive_buffer(&mut self) {
        self.dive_buffer_time = 0.0;
        self.dive_buffer_active = false;
    }

    /// Merge gamepad input with keyboard input
    /// Gamepad analog stick takes priority if it has input, otherwise keyboard
    /// Actions use OR logic - either keyboard or gamepad triggers action
    pub fn update_with_gamepad(&mut self, gamepad: &GamepadInput, dt: f32, buffer_window: f32) {
        if !gamepad.active {
            self.using_gamepad = false;
            return;
        }

        // Movement: gamepad takes priority if there's significant stick input
        if gamepad.move_dir.length_squared() > 0.01 {
            self.move_dir = gamepad.move_dir;
            self.using_gamepad = true;
        } else if self.move_dir.length_squared() > 0.01 {
            // Keyboard has input, use that
            self.using_gamepad = false;
        }

        // Actions: OR logic (either keyboard or gamepad triggers)
        self.jump_pressed = self.jump_pressed || gamepad.jump_pressed;
        self.jump_held = self.jump_held || gamepad.jump_held;
        self.jump_released = self.jump_released || gamepad.jump_released;
        self.sprint_held = self.sprint_held || gamepad.sprint_held;
        self.dive_pressed = self.dive_pressed || gamepad.dive_pressed;
        self.jet_boost_pressed = self.jet_boost_pressed || gamepad.jet_boost_pressed;
        self.ink_pressed = self.ink_pressed || gamepad.ink_pressed;
        self.grapple_pressed = self.grapple_pressed || gamepad.grapple_pressed;
        self.grapple_held = self.grapple_held || gamepad.grapple_held;

        // Menu navigation: OR logic
        self.menu_up = self.menu_up || gamepad.menu_up;
        self.menu_down = self.menu_down || gamepad.menu_down;
        self.menu_left = self.menu_left || gamepad.menu_left;
        self.menu_right = self.menu_right || gamepad.menu_right;
        self.menu_confirm = self.menu_confirm || gamepad.menu_confirm;
        self.menu_cancel = self.menu_cancel || gamepad.menu_cancel;
        self.pause_pressed = self.pause_pressed || gamepad.pause_pressed;

        // UI toggles: OR logic
        self.minimap_toggle_pressed = self.minimap_toggle_pressed || gamepad.minimap_toggle_pressed;
        self.minimap_zoom_in_pressed = self.minimap_zoom_in_pressed || gamepad.minimap_zoom_in_pressed;
        self.minimap_zoom_out_pressed = self.minimap_zoom_out_pressed || gamepad.minimap_zoom_out_pressed;

        // Refresh buffers if gamepad triggered the action
        if gamepad.jump_pressed {
            self.jump_buffer_time = buffer_window;
        }
        if gamepad.grapple_pressed {
            self.grapple_buffer_time = buffer_window;
        }
        if gamepad.dive_pressed {
            self.dive_buffer_time = buffer_window;
        }

        // Update buffer states
        self.jump_buffer_time = (self.jump_buffer_time - dt).max(0.0);
        self.jump_buffer_active = self.jump_buffer_time > 0.0;
        self.grapple_buffer_time = (self.grapple_buffer_time - dt).max(0.0);
        self.grapple_buffer_active = self.grapple_buffer_time > 0.0;
        self.dive_buffer_time = (self.dive_buffer_time - dt).max(0.0);
        self.dive_buffer_active = self.dive_buffer_time > 0.0;
    }
}

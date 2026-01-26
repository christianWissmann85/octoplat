//! Generic menu state management

use crate::input::InputState;

/// Menu state with selection tracking
pub struct MenuState<T: Clone + Copy + PartialEq> {
    /// Available menu items
    pub items: Vec<T>,
    /// Currently selected index
    pub selected: usize,
    /// Animation timer for selection pulse
    pub pulse_time: f32,
    /// Time since last selection change (for animation)
    pub selection_time: f32,
}

impl<T: Clone + Copy + PartialEq> MenuState<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            selected: 0,
            pulse_time: 0.0,
            selection_time: 0.0,
        }
    }

    pub fn from_array<const N: usize>(items: [T; N]) -> Self {
        Self::new(items.to_vec())
    }

    /// Update menu state based on input, returns selected item if confirmed
    pub fn update(&mut self, input: &InputState, dt: f32) -> MenuAction<T> {
        self.pulse_time += dt;
        self.selection_time += dt;

        // Navigation
        if input.menu_up {
            self.move_selection(-1);
        }
        if input.menu_down {
            self.move_selection(1);
        }

        // Confirm selection
        if input.menu_confirm {
            return MenuAction::Select(self.items[self.selected]);
        }

        // Cancel/back
        if input.menu_cancel {
            return MenuAction::Cancel;
        }

        MenuAction::None
    }

    fn move_selection(&mut self, delta: i32) {
        let len = self.items.len() as i32;
        let new_index = (self.selected as i32 + delta).rem_euclid(len);
        if new_index as usize != self.selected {
            self.selected = new_index as usize;
            self.selection_time = 0.0;
        }
    }

    pub fn selected_item(&self) -> T {
        self.items[self.selected]
    }

    /// Get the pulse alpha for selection highlight (0.0 to 1.0)
    pub fn pulse_alpha(&self) -> f32 {
        // Smooth pulse using sine wave
        (self.pulse_time * 3.0).sin() * 0.15 + 0.85
    }

    /// Get selection animation progress (0.0 to 1.0, quick ease-out)
    pub fn selection_anim(&self) -> f32 {
        let t = (self.selection_time * 8.0).min(1.0);
        // Ease-out cubic
        1.0 - (1.0 - t).powi(3)
    }
}

/// Result of menu update
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuAction<T> {
    None,
    Select(T),
    Cancel,
}

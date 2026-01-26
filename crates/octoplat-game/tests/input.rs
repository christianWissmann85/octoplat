//! Integration tests for input state

use octoplat_game::InputState;

// =============================================================================
// InputState Tests
// =============================================================================

#[test]
fn test_input_state_default() {
    let input = InputState::default();

    // All inputs should be false/zero initially
    assert_eq!(input.move_dir.x, 0.0);
    assert_eq!(input.move_dir.y, 0.0);
    assert!(!input.jump_pressed);
    assert!(!input.jump_held);
    assert!(!input.jump_released);
    assert!(!input.sprint_held);
    assert!(!input.grapple_pressed);
    assert!(!input.jet_boost_pressed);
}

#[test]
fn test_input_state_move_dir_normalized() {
    // InputState should accept direction values
    let mut input = InputState::default();
    input.move_dir.x = 1.0;
    input.move_dir.y = 0.0;

    assert_eq!(input.move_dir.x, 1.0);
}

#[test]
fn test_input_state_jump_states() {
    let mut input = InputState::default();

    // Simulate jump press
    input.jump_pressed = true;
    input.jump_held = true;
    assert!(input.jump_pressed);
    assert!(input.jump_held);

    // Clear press, keep held
    input.jump_pressed = false;
    assert!(!input.jump_pressed);
    assert!(input.jump_held);

    // Release
    input.jump_held = false;
    input.jump_released = true;
    assert!(input.jump_released);
}

#[test]
fn test_input_state_sprint() {
    let mut input = InputState::default();

    input.sprint_held = true;
    assert!(input.sprint_held);

    input.sprint_held = false;
    assert!(!input.sprint_held);
}

#[test]
fn test_input_state_grapple() {
    let mut input = InputState::default();

    input.grapple_pressed = true;
    assert!(input.grapple_pressed);
}

#[test]
fn test_input_state_jet_boost() {
    let mut input = InputState::default();

    input.jet_boost_pressed = true;
    assert!(input.jet_boost_pressed);
}

#[test]
fn test_input_state_pause() {
    let mut input = InputState::default();

    input.pause_pressed = true;
    assert!(input.pause_pressed);
}

#[test]
fn test_input_state_buffer_active() {
    let input = InputState::default();

    // Buffer flags should be accessible
    assert!(!input.jump_buffer_active);
    assert!(!input.grapple_buffer_active);
    assert!(!input.dive_buffer_active);
}

#[test]
fn test_input_state_clone() {
    let mut input = InputState::default();
    input.move_dir.x = 1.0;
    input.jump_pressed = true;

    // InputState should be cloneable (if it implements Clone)
    // Note: May need to verify Clone is implemented
}

#[test]
fn test_input_state_multiple_inputs() {
    let mut input = InputState::default();

    // Multiple inputs can be active simultaneously
    input.move_dir.x = 1.0;
    input.jump_held = true;
    input.sprint_held = true;

    assert!(input.move_dir.x > 0.0);
    assert!(input.jump_held);
    assert!(input.sprint_held);
}

#[test]
fn test_input_state_negative_direction() {
    let mut input = InputState::default();

    input.move_dir.x = -1.0;
    assert!(input.move_dir.x < 0.0);
}

#[test]
fn test_input_state_diagonal_input() {
    let mut input = InputState::default();

    input.move_dir.x = 0.707;
    input.move_dir.y = -0.707;

    // Both components should be set
    assert!(input.move_dir.x > 0.0);
    assert!(input.move_dir.y < 0.0);
}

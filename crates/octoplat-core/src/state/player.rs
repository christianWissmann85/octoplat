//! Player movement states

/// Player movement states - mutually exclusive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    Jumping,
    Falling,
    WallGrip,
    JetBoosting,
    Swinging,
}

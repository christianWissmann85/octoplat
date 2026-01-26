//! Gameplay module
//!
//! Contains physics, collision detection, feedback systems, and the gameplay engine.

mod engine;
pub mod collision;
pub mod feedback;
pub mod physics;

pub use engine::GameplayEngine;
pub use collision::{
    check_breakable_blocks, check_enemy_collision, check_fall_death, check_hazard_collision,
    EnemyCollisionResult, EnemyType,
};
pub use feedback::{process_feedback, FeedbackResult};
pub use physics::*;

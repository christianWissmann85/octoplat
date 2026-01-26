//! Physics and collision detection
//!
//! Contains collision primitives and feedback tracking.

mod collision;
mod feedback;

pub use collision::{aabb_collision, check_ground, check_wall, CollisionResult, Hitbox};
pub use feedback::FeedbackTracker;

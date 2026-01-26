//! Centralized game constants for physics, collision, and procgen.
//!
//! These constants ensure consistent behavior across all systems and make
//! it easy to tune values without hunting through multiple files.

/// Procgen constants for level generation algorithms
pub struct ProcgenConstants {
    /// How many rows to search downward from spawn to find valid ground (tiles)
    pub spawn_search_height: i32,
    /// Minimum column offset when searching for corridor exit points
    pub corridor_column_search_min: usize,
    /// Maximum column offset when searching for corridor exit points
    pub corridor_column_search_max: usize,
    /// Rate at which difficulty scales with level progression (asymptotic curve)
    pub difficulty_scale_rate: f32,
}

impl Default for ProcgenConstants {
    fn default() -> Self {
        Self {
            spawn_search_height: 10,
            corridor_column_search_min: 3,
            corridor_column_search_max: 12,
            difficulty_scale_rate: 0.05,
        }
    }
}

impl ProcgenConstants {
    /// Global instance of procgen constants
    pub const fn default_const() -> Self {
        Self {
            spawn_search_height: 10,
            corridor_column_search_min: 3,
            corridor_column_search_max: 12,
            difficulty_scale_rate: 0.05,
        }
    }
}

/// Collision detection constants
pub struct CollisionConstants {
    /// Minimum velocity magnitude required to trigger corner correction (pixels/sec)
    /// Below this threshold, player is considered stationary for corner correction purposes
    pub corner_velocity_threshold: f32,
    /// Vertical window below platform top for one-way platform collision (pixels)
    /// Player within this window from top is considered landing on the platform
    pub one_way_platform_window: f32,
    /// Vertical window for bounce pad collision detection (pixels)
    pub bounce_pad_collision_window: f32,
}

impl Default for CollisionConstants {
    fn default() -> Self {
        Self {
            corner_velocity_threshold: 10.0,
            one_way_platform_window: 12.0,
            bounce_pad_collision_window: 4.0,
        }
    }
}

impl CollisionConstants {
    /// Global instance of collision constants
    pub const fn default_const() -> Self {
        Self {
            corner_velocity_threshold: 10.0,
            one_way_platform_window: 12.0,
            bounce_pad_collision_window: 4.0,
        }
    }
}

/// Global procgen constants instance
pub static PROCGEN: ProcgenConstants = ProcgenConstants::default_const();

/// Global collision constants instance
pub static COLLISION: CollisionConstants = CollisionConstants::default_const();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procgen_defaults() {
        let constants = ProcgenConstants::default();
        assert_eq!(constants.spawn_search_height, 10);
        assert_eq!(constants.corridor_column_search_min, 3);
        assert_eq!(constants.corridor_column_search_max, 12);
        assert!((constants.difficulty_scale_rate - 0.05).abs() < f32::EPSILON);
    }

    #[test]
    fn test_collision_defaults() {
        let constants = CollisionConstants::default();
        assert!((constants.corner_velocity_threshold - 10.0).abs() < f32::EPSILON);
        assert!((constants.one_way_platform_window - 12.0).abs() < f32::EPSILON);
        assert!((constants.bounce_pad_collision_window - 4.0).abs() < f32::EPSILON);
    }
}

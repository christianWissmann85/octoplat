use crate::Vec2;

/// Types of markers that can be placed in a level
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkerType {
    PlayerSpawn,
    Gem,
    GrapplePoint,
    Checkpoint,
    LevelExit,
    WaterPool,
    // Enemies
    Crab,
    PufferfishStationary,
    PufferfishHorizontal,
    PufferfishVertical,
    // Dynamic platforms
    MovingPlatformHorizontalStart,
    MovingPlatformHorizontalEnd,
    MovingPlatformVerticalStart,
    MovingPlatformVerticalEnd,
    CrumblingPlatform,
}

/// A marker found during level parsing
#[derive(Clone, Debug)]
pub struct LevelMarker {
    pub position: Vec2,
    pub marker_type: MarkerType,
}

impl LevelMarker {
    pub fn new(position: Vec2, marker_type: MarkerType) -> Self {
        Self {
            position,
            marker_type,
        }
    }
}

use macroquad::prelude::*;

/// Axis-Aligned Bounding Box for collision
#[derive(Clone, Copy, Debug)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2,
}

impl Hitbox {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            offset: Vec2::ZERO,
        }
    }

    /// Convert to Rect at a given position (center-based)
    pub fn to_rect(&self, position: Vec2) -> Rect {
        Rect {
            x: position.x + self.offset.x - self.width / 2.0,
            y: position.y + self.offset.y - self.height / 2.0,
            w: self.width,
            h: self.height,
        }
    }
}

/// Result of a collision check with penetration info
#[derive(Clone, Copy, Debug, Default)]
pub struct CollisionResult {
    pub collided: bool,
    pub normal: Vec2,
    pub penetration: f32,
}

/// Check AABB vs AABB collision with penetration vector
pub fn aabb_collision(a: Rect, b: Rect) -> CollisionResult {
    let a_center = vec2(a.x + a.w / 2.0, a.y + a.h / 2.0);
    let b_center = vec2(b.x + b.w / 2.0, b.y + b.h / 2.0);

    let overlap_x = (a.w / 2.0 + b.w / 2.0) - (a_center.x - b_center.x).abs();
    let overlap_y = (a.h / 2.0 + b.h / 2.0) - (a_center.y - b_center.y).abs();

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return CollisionResult::default();
    }

    // Push out along smallest overlap axis
    let (normal, penetration) = if overlap_x < overlap_y {
        let dir = if a_center.x < b_center.x { -1.0 } else { 1.0 };
        (vec2(dir, 0.0), overlap_x)
    } else {
        let dir = if a_center.y < b_center.y { -1.0 } else { 1.0 };
        (vec2(0.0, dir), overlap_y)
    };

    CollisionResult {
        collided: true,
        normal,
        penetration,
    }
}

/// Check if player is touching ground (small raycast down)
pub fn check_ground(player_rect: Rect, tiles: &[Rect]) -> bool {
    let ground_check = Rect {
        x: player_rect.x + 2.0,
        y: player_rect.y + player_rect.h,
        w: player_rect.w - 4.0,
        h: 2.0,
    };

    let result = tiles.iter().any(|tile| ground_check.overlaps(tile));

    #[cfg(debug_assertions)]
    {
        // Only log when ground state might be interesting (near tiles)
        if !tiles.is_empty() {
            let nearest_tile = tiles.iter()
                .min_by(|a, b| {
                    let dist_a = (a.y - (player_rect.y + player_rect.h)).abs();
                    let dist_b = (b.y - (player_rect.y + player_rect.h)).abs();
                    dist_a.partial_cmp(&dist_b).unwrap()
                });
            if let Some(tile) = nearest_tile {
                let gap = tile.y - (player_rect.y + player_rect.h);
                // Only log if we're close to ground (within 10 pixels)
                if gap.abs() < 10.0 {
                    eprintln!(
                        "[GROUND] check={}, player_bottom={:.2}, nearest_tile_top={:.2}, gap={:.2}, check_rect=({:.2},{:.2},{:.2},{:.2})",
                        result, player_rect.y + player_rect.h, tile.y, gap,
                        ground_check.x, ground_check.y, ground_check.w, ground_check.h
                    );
                }
            }
        }
    }

    result
}

/// Check for wall contact (left or right)
/// Returns: -1 for left wall, 1 for right wall, 0 for no wall
pub fn check_wall(player_rect: Rect, tiles: &[Rect]) -> i8 {
    let left_check = Rect {
        x: player_rect.x - 2.0,
        y: player_rect.y + 4.0,
        w: 2.0,
        h: player_rect.h - 8.0,
    };

    let right_check = Rect {
        x: player_rect.x + player_rect.w,
        y: player_rect.y + 4.0,
        w: 2.0,
        h: player_rect.h - 8.0,
    };

    let left_hit = tiles.iter().any(|tile| left_check.overlaps(tile));
    let right_hit = tiles.iter().any(|tile| right_check.overlaps(tile));

    match (left_hit, right_hit) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    }
}

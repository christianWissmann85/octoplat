use macroquad::prelude::*;

pub struct Gem {
    pub position: Vec2,
    pub collected: bool,
    pub hitbox_radius: f32,
    pub bob_offset: f32,
}

impl Gem {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            collected: false,
            hitbox_radius: 16.0,
            bob_offset: rand::gen_range(0.0, std::f32::consts::TAU),
        }
    }

    /// Check collision with player rect
    pub fn check_collection(&mut self, player_rect: Rect) -> bool {
        if self.collected {
            return false;
        }

        // Simple circle-rect collision
        let closest_x = self
            .position
            .x
            .clamp(player_rect.x, player_rect.x + player_rect.w);
        let closest_y = self
            .position
            .y
            .clamp(player_rect.y, player_rect.y + player_rect.h);

        let distance = vec2(closest_x - self.position.x, closest_y - self.position.y);

        if distance.length() < self.hitbox_radius {
            self.collected = true;
            true
        } else {
            false
        }
    }

    /// Get render position with bobbing animation
    pub fn render_position(&self, time: f32) -> Vec2 {
        let bob = (time * 2.0 + self.bob_offset).sin() * 4.0;
        vec2(self.position.x, self.position.y + bob)
    }

}

pub mod vec2;
mod rect;
mod color;

pub use vec2::Vec2;
pub use rect::Rect;
pub use color::Color;

// Re-export convenience function at types level
pub use vec2::vec2;

//! Shader system for visual effects
//!
//! Provides unified shader management for glow, chromatic aberration, etc.

mod manager;
mod glow;
mod chromatic;

pub use manager::ShaderManager;
pub use chromatic::ChromaticAberration;

// GlowEffect and bloom functions for visual effects
#[allow(unused_imports)]
pub use glow::GlowEffect;
pub use glow::{draw_bloom, draw_bloom_pulsing};

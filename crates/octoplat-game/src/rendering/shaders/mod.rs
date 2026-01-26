//! Shader system for visual effects
//!
//! Provides unified shader management for glow, chromatic aberration, etc.

mod manager;
mod glow;
mod chromatic;

pub use manager::ShaderManager;
pub use chromatic::ChromaticAberration;

// GlowEffect available for future centralized glow management
#[allow(unused_imports)]
pub use glow::GlowEffect;

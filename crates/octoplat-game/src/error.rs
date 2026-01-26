//! Game-level error types wrapping all subsystem errors.
//!
//! Provides a unified error type for the game runtime, wrapping core errors
//! and game-specific errors like procgen, audio, and shader failures.

use std::fmt;

use octoplat_core::error::OctoplatError;

use crate::procgen::ProcgenError;

/// Game-level error type wrapping all subsystem errors
#[derive(Debug, Clone)]
pub enum GameError {
    /// Core library error (level loading, parsing, validation)
    Core(OctoplatError),

    /// Procedural generation error
    Procgen(ProcgenError),

    /// Audio system error
    Audio(String),

    /// Shader compilation/loading error
    Shader(String),

    /// Asset loading error
    Asset {
        asset_type: &'static str,
        path: String,
        reason: String,
    },
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(e) => write!(f, "{}", e),
            Self::Procgen(e) => write!(f, "Procgen error: {}", e),
            Self::Audio(msg) => write!(f, "Audio error: {}", msg),
            Self::Shader(msg) => write!(f, "Shader error: {}", msg),
            Self::Asset { asset_type, path, reason } => {
                write!(f, "Failed to load {} '{}': {}", asset_type, path, reason)
            }
        }
    }
}

impl std::error::Error for GameError {}

impl From<OctoplatError> for GameError {
    fn from(e: OctoplatError) -> Self {
        GameError::Core(e)
    }
}

impl From<ProcgenError> for GameError {
    fn from(e: ProcgenError) -> Self {
        GameError::Procgen(e)
    }
}

/// For backwards compatibility with AppState::Error(String)
impl From<GameError> for String {
    fn from(e: GameError) -> Self {
        e.to_string()
    }
}

/// Convenience type alias
pub type Result<T> = std::result::Result<T, GameError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_error_conversion() {
        let core_err = OctoplatError::EmptyTilemap;
        let game_err: GameError = core_err.into();
        assert!(game_err.to_string().contains("empty"));
    }

    #[test]
    fn test_procgen_error_conversion() {
        let procgen_err = ProcgenError::PoolNotLoaded;
        let game_err: GameError = procgen_err.into();
        assert!(game_err.to_string().contains("Procgen"));
    }

    #[test]
    fn test_game_error_to_string() {
        let err = GameError::Audio("failed to initialize".to_string());
        let s: String = err.into();
        assert!(s.contains("Audio"));
    }
}

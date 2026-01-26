//! Error types for octoplat-core operations.
//!
//! Provides structured error types for level loading, parsing, validation,
//! and save/load operations. These replace ad-hoc `Result<T, String>` returns.

use std::fmt;
use std::path::PathBuf;

/// Core error type for octoplat operations
#[derive(Debug, Clone)]
pub enum OctoplatError {
    /// File I/O errors
    Io {
        path: PathBuf,
        operation: &'static str,
        message: String,
    },

    /// Level parsing errors
    Parse {
        file: String,
        line: Option<usize>,
        reason: String,
    },

    /// Level validation errors
    Validation {
        level_id: String,
        issues: Vec<String>,
    },

    /// Level not found
    LevelNotFound {
        id: String,
        searched: Vec<PathBuf>,
    },

    /// Level file too large
    FileTooLarge {
        path: String,
        size: usize,
        max_size: usize,
    },

    /// Tilemap dimension exceeded
    TilemapTooLarge {
        width: usize,
        height: usize,
        max_dimension: usize,
    },

    /// Empty tilemap
    EmptyTilemap,

    /// Save/load errors
    Save {
        reason: String,
    },

    /// Serialization error
    Serialization {
        reason: String,
    },
}

impl fmt::Display for OctoplatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, operation, message } => {
                write!(f, "Failed to {} '{}': {}", operation, path.display(), message)
            }
            Self::Parse { file, line, reason } => {
                if let Some(line_num) = line {
                    write!(f, "Parse error in '{}' at line {}: {}", file, line_num, reason)
                } else {
                    write!(f, "Parse error in '{}': {}", file, reason)
                }
            }
            Self::Validation { level_id, issues } => {
                write!(f, "Level '{}' validation failed: {}", level_id, issues.join(", "))
            }
            Self::LevelNotFound { id, searched } => {
                let paths: Vec<_> = searched.iter().map(|p| p.display().to_string()).collect();
                write!(f, "Level '{}' not found in: {}", id, paths.join(", "))
            }
            Self::FileTooLarge { path, size, max_size } => {
                write!(
                    f,
                    "Level file '{}' too large: {} bytes (max {} bytes)",
                    path, size, max_size
                )
            }
            Self::TilemapTooLarge { width, height, max_dimension } => {
                write!(
                    f,
                    "Tilemap too large: {}x{} (max {}x{})",
                    width, height, max_dimension, max_dimension
                )
            }
            Self::EmptyTilemap => {
                write!(f, "Tilemap is empty - no valid map data found")
            }
            Self::Save { reason } => write!(f, "Save error: {}", reason),
            Self::Serialization { reason } => write!(f, "Serialization error: {}", reason),
        }
    }
}

impl std::error::Error for OctoplatError {}

/// For backwards compatibility with AppState::Error(String) and other String error consumers
impl From<OctoplatError> for String {
    fn from(e: OctoplatError) -> Self {
        e.to_string()
    }
}

/// Convenience type alias
pub type Result<T> = std::result::Result<T, OctoplatError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_display() {
        let err = OctoplatError::Io {
            path: PathBuf::from("/path/to/file.lvl"),
            operation: "read",
            message: "file not found".to_string(),
        };
        assert!(err.to_string().contains("read"));
        assert!(err.to_string().contains("/path/to/file.lvl"));
    }

    #[test]
    fn test_parse_error_with_line() {
        let err = OctoplatError::Parse {
            file: "test.lvl".to_string(),
            line: Some(42),
            reason: "unexpected character".to_string(),
        };
        assert!(err.to_string().contains("line 42"));
    }

    #[test]
    fn test_parse_error_without_line() {
        let err = OctoplatError::Parse {
            file: "test.lvl".to_string(),
            line: None,
            reason: "malformed header".to_string(),
        };
        assert!(!err.to_string().contains("line"));
    }

    #[test]
    fn test_error_to_string_conversion() {
        let err = OctoplatError::EmptyTilemap;
        let s: String = err.into();
        assert!(s.contains("empty"));
    }
}

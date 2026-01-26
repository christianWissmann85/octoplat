#![allow(dead_code)]

//! Cross-platform path handling for game data directories
//!
//! Provides consistent paths for:
//! - User-created levels (saved to user data directory)
//! - Bundled levels (read from assets)
//! - Save data

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Application name used for data directories
const APP_NAME: &str = "octoplat";

/// Get the user data directory for the application
/// - Windows: %APPDATA%/octoplat
/// - macOS: ~/Library/Application Support/octoplat
/// - Linux: $XDG_DATA_HOME/octoplat or ~/.local/share/octoplat
pub fn user_data_dir() -> PathBuf {
    let base = if cfg!(target_os = "windows") {
        // Windows: use APPDATA
        env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Fallback to USERPROFILE/AppData/Roaming
                env::var("USERPROFILE")
                    .map(|p| PathBuf::from(p).join("AppData").join("Roaming"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            })
    } else if cfg!(target_os = "macos") {
        // macOS: use ~/Library/Application Support
        env::var("HOME")
            .map(|p| PathBuf::from(p).join("Library").join("Application Support"))
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        // Linux/Unix: use XDG_DATA_HOME or ~/.local/share
        env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                env::var("HOME")
                    .map(|p| PathBuf::from(p).join(".local").join("share"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            })
    };

    base.join(APP_NAME)
}

/// Get the directory for user-created levels
pub fn user_levels_dir() -> PathBuf {
    user_data_dir().join("levels")
}

/// Get the directory for save data
pub fn save_data_dir() -> PathBuf {
    user_data_dir()
}

/// Get the path for the main save file
pub fn save_file_path() -> PathBuf {
    save_data_dir().join("save.json")
}

/// Get the bundled levels directory (relative to executable or CWD)
/// This is for read-only access to shipped levels
pub fn bundled_levels_dir() -> PathBuf {
    // Try relative to executable first
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let assets_path = exe_dir.join("assets").join("levels");
            if assets_path.exists() {
                return assets_path;
            }
        }
    }

    // Fall back to relative path (for development)
    PathBuf::from("assets").join("levels")
}

/// Get the full path for a user-created level
pub fn user_level_path(level_name: &str) -> PathBuf {
    let sanitized = sanitize_filename(level_name);
    user_levels_dir().join(format!("{}.txt", sanitized))
}

/// Get the full path for a bundled level
pub fn bundled_level_path(level_id: &str) -> PathBuf {
    bundled_levels_dir().join(format!("{}.txt", level_id))
}

/// Sanitize a level name for use as a filename
/// - Converts to lowercase
/// - Replaces spaces with underscores
/// - Removes or replaces invalid characters
pub fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' | '-' => '_',
            c if c.is_alphanumeric() || c == '_' => c,
            _ => '_',
        })
        .collect::<String>()
        // Remove consecutive underscores
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| {
            format!(
                "Failed to create directory '{}': {}",
                path.display(),
                e
            )
        })?;
    }
    Ok(())
}

/// Ensure the user levels directory exists
pub fn ensure_user_levels_dir() -> Result<PathBuf, String> {
    let dir = user_levels_dir();
    ensure_dir_exists(&dir)?;
    Ok(dir)
}

/// Ensure the save data directory exists
pub fn ensure_save_dir() -> Result<PathBuf, String> {
    let dir = save_data_dir();
    ensure_dir_exists(&dir)?;
    Ok(dir)
}

/// List all user-created levels
pub fn list_user_levels() -> Vec<String> {
    let dir = user_levels_dir();
    if !dir.exists() {
        return Vec::new();
    }

    fs::read_dir(&dir)
        .map(|entries| {
            entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.extension()?.to_str()? == "txt" {
                        path.file_stem()?.to_str().map(String::from)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Check if a level exists in user levels
pub fn user_level_exists(level_name: &str) -> bool {
    user_level_path(level_name).exists()
}

/// Check if a level exists in bundled levels
pub fn bundled_level_exists(level_id: &str) -> bool {
    bundled_level_path(level_id).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("My Level"), "my_level");
        assert_eq!(sanitize_filename("Test-Level-1"), "test_level_1");
        assert_eq!(sanitize_filename("Level!!!"), "level");
        assert_eq!(sanitize_filename("  spaces  "), "spaces");
        assert_eq!(sanitize_filename("UPPER_case"), "upper_case");
    }
}

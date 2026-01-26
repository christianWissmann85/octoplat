#![allow(dead_code)]

//! Save system for persistent game data
//!
//! Handles saving and loading player progress, settings, and statistics.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use crate::paths;

/// Persistent save data
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SaveData {
    // Level completion tracking
    pub levels_completed: HashSet<String>,
    pub best_times: HashMap<String, f32>,
    pub best_gems: HashMap<String, u32>,

    // Lifetime statistics
    pub total_deaths: u32,
    pub total_gems: u32,
    pub total_playtime: f32,
    pub total_jumps: u32,
    pub total_dives: u32,
    pub total_grapples: u32,

    // Endless mode records
    pub endless_best_levels: u32,
    pub endless_best_gems: u32,
    pub endless_runs: Vec<EndlessRun>,

    // Settings (persisted)
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub screen_shake_enabled: bool,

    // Minimap settings
    #[serde(default = "default_minimap_scale")]
    pub minimap_scale: f32,
    #[serde(default = "default_minimap_opacity")]
    pub minimap_opacity: f32,
    #[serde(default = "default_minimap_size")]
    pub minimap_size: f32,
}

fn default_minimap_scale() -> f32 {
    3.0
}

fn default_minimap_opacity() -> f32 {
    0.7
}

fn default_minimap_size() -> f32 {
    150.0
}

/// Record of an endless mode run
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EndlessRun {
    pub seed: u64,
    pub levels_completed: u32,
    pub gems_collected: u32,
    pub deaths: u32,
    pub time: f32,
    pub timestamp: u64,
}

impl SaveData {
    /// Create new save data with default settings
    pub fn new() -> Self {
        Self {
            sfx_volume: 0.7,
            music_volume: 0.5,
            screen_shake_enabled: true,
            minimap_scale: default_minimap_scale(),
            minimap_opacity: default_minimap_opacity(),
            minimap_size: default_minimap_size(),
            endless_runs: Vec::new(),
            ..Default::default()
        }
    }

    /// Record completing a level
    pub fn complete_level(&mut self, level_name: &str, time: f32, gems: u32) {
        self.levels_completed.insert(level_name.to_string());

        // Update best time
        let best_time = self.best_times.entry(level_name.to_string()).or_insert(f32::MAX);
        if time < *best_time {
            *best_time = time;
        }

        // Update best gems
        let best_gems = self.best_gems.entry(level_name.to_string()).or_insert(0);
        if gems > *best_gems {
            *best_gems = gems;
        }
    }

    /// Record an endless run
    pub fn record_endless_run(&mut self, run: EndlessRun) {
        // Update bests
        if run.levels_completed > self.endless_best_levels {
            self.endless_best_levels = run.levels_completed;
        }
        if run.gems_collected > self.endless_best_gems {
            self.endless_best_gems = run.gems_collected;
        }

        // Add to history, keep top 10
        self.endless_runs.push(run);
        self.endless_runs.sort_by(|a, b| {
            b.levels_completed.cmp(&a.levels_completed)
                .then(b.gems_collected.cmp(&a.gems_collected))
        });
        self.endless_runs.truncate(10);
    }

    /// Get the best time for a level (if any)
    pub fn get_best_time(&self, level_name: &str) -> Option<f32> {
        self.best_times.get(level_name).copied()
    }

    /// Get the best gem count for a level (if any)
    pub fn get_best_gems(&self, level_name: &str) -> Option<u32> {
        self.best_gems.get(level_name).copied()
    }
}

/// Save manager handles file I/O
pub struct SaveManager {
    save_path: PathBuf,
    pub data: SaveData,
    dirty: bool,
}

impl SaveManager {
    /// Create a new save manager, loading existing data if available
    pub fn new() -> Self {
        let save_path = Self::get_save_path();
        let data = Self::load_from_path(&save_path).unwrap_or_default();

        Self {
            save_path,
            data,
            dirty: false,
        }
    }

    /// Get the platform-appropriate save path
    fn get_save_path() -> PathBuf {
        // Ensure save directory exists
        let _ = paths::ensure_save_dir();
        paths::save_file_path()
    }

    /// Load save data from a path
    fn load_from_path(path: &PathBuf) -> Option<SaveData> {
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Save to disk if data has changed
    pub fn save_if_dirty(&mut self) -> Result<(), String> {
        if self.dirty {
            self.save()?;
            self.dirty = false;
        }
        Ok(())
    }

    /// Force save to disk using atomic write (write to temp, then rename)
    ///
    /// This prevents save file corruption if the write is interrupted (e.g., power loss,
    /// crash, or disk full). The rename operation is atomic on most filesystems.
    pub fn save(&self) -> Result<(), String> {
        use std::io::Write;

        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;

        // Ensure parent directory exists
        if let Some(parent) = self.save_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create save directory: {}", e))?;
        }

        // Create a temporary file in the same directory (ensures same filesystem for atomic rename)
        let temp_path = self.save_path.with_extension("tmp");

        // Write to temporary file
        let mut file = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp save file: {}", e))?;

        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write temp save file: {}", e))?;

        // Sync to disk to ensure data is written before rename
        file.sync_all()
            .map_err(|e| format!("Failed to sync save file: {}", e))?;

        // Atomic rename (replaces target if it exists)
        fs::rename(&temp_path, &self.save_path)
            .map_err(|e| format!("Failed to finalize save file: {}", e))?;

        Ok(())
    }

    /// Get mutable access to save data and mark as dirty
    pub fn data_mut(&mut self) -> &mut SaveData {
        self.dirty = true;
        &mut self.data
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

//! Level manager for loading and managing game levels

use macroquad::prelude::*;
use std::path::{Path, PathBuf};

use octoplat_core::level::{LevelData, TileMap};
use octoplat_core::paths;
use crate::compat::vec2_to_mq;

/// Manages level loading, transitions, and checkpoints
pub struct LevelManager {
    /// Currently loaded level
    current_level: Option<LevelData>,
    /// Current level identifier (filename without extension)
    current_level_id: String,
    /// List of available level IDs in order
    level_order: Vec<String>,
    /// Current checkpoint position (respawn point)
    checkpoint: Option<Vec2>,
    /// Tile size for parsing
    tile_size: f32,
}

impl LevelManager {
    pub fn new(tile_size: f32) -> Self {
        Self {
            current_level: None,
            current_level_id: String::new(),
            level_order: Vec::new(),
            checkpoint: None,
            tile_size,
        }
    }

    /// Set the level order (list of level IDs)
    pub fn set_level_order(&mut self, order: Vec<String>) {
        self.level_order = order;
    }

    /// Load a level from string content
    pub fn load_from_string(&mut self, level_id: &str, content: &str) -> Result<(), String> {
        let level_data = LevelData::parse(content, self.tile_size)?;
        self.current_level = Some(level_data);
        self.current_level_id = level_id.to_string();
        self.checkpoint = None; // Reset checkpoint on new level
        Ok(())
    }

    /// Load a level by ID (tries user levels, then bundled levels)
    pub fn load_level(&mut self, level_id: &str) -> Result<(), String> {
        // 1. Try user levels directory first (highest priority for modding)
        let user_path = paths::user_level_path(level_id);
        if let Ok(content) = std::fs::read_to_string(&user_path) {
            #[cfg(debug_assertions)]
            println!("Loaded level from user directory: {}", user_path.display());
            return self.load_from_string(level_id, &content);
        }

        // 2. Try bundled levels directory (next to executable or in CWD)
        let bundled_path = paths::bundled_level_path(level_id);
        if let Ok(content) = std::fs::read_to_string(&bundled_path) {
            #[cfg(debug_assertions)]
            println!("Loaded level from bundled directory: {}", bundled_path.display());
            return self.load_from_string(level_id, &content);
        }

        Err(format!(
            "Level '{}' not found (checked user dir and bundled)",
            level_id
        ))
    }

    /// Load the first level in the order
    pub fn load_first_level(&mut self) -> Result<(), String> {
        if let Some(first) = self.level_order.first().cloned() {
            self.load_level(&first)
        } else {
            Err("No levels defined".to_string())
        }
    }

    /// Advance to the next level
    pub fn load_next_level(&mut self) -> Result<bool, String> {
        // First check if current level specifies a next level
        let explicit_next = self
            .current_level
            .as_ref()
            .and_then(|l| l.next_level.clone());

        if let Some(next_id) = explicit_next {
            self.load_level(&next_id)?;
            return Ok(true);
        }

        // Otherwise use level order
        if let Some(current_idx) = self
            .level_order
            .iter()
            .position(|id| id == &self.current_level_id)
        {
            if current_idx + 1 < self.level_order.len() {
                let next_id = self.level_order[current_idx + 1].clone();
                self.load_level(&next_id)?;
                return Ok(true);
            }
        }

        // No more levels
        Ok(false)
    }

    /// Get the current tilemap
    pub fn tilemap(&self) -> Option<&TileMap> {
        self.current_level.as_ref().map(|l| &l.tilemap)
    }

    /// Get the current level name
    pub fn level_name(&self) -> &str {
        self.current_level
            .as_ref()
            .map(|l| l.name.as_str())
            .unwrap_or("No Level")
    }

    /// Get spawn position (checkpoint if set, otherwise level spawn)
    pub fn get_spawn_position(&self) -> Vec2 {
        if let Some(checkpoint) = self.checkpoint {
            return checkpoint;
        }

        self.tilemap()
            .map(|tm| vec2_to_mq(tm.get_spawn_position()))
            .unwrap_or(vec2(100.0, 100.0))
    }

    /// Set checkpoint at position
    pub fn set_checkpoint(&mut self, position: Vec2) {
        self.checkpoint = Some(position);
    }

    /// Get the file path of the current level
    /// Returns the path where this level exists (user dir, bundled, or None if embedded-only)
    pub fn current_level_path(&self) -> Option<PathBuf> {
        if self.current_level_id.is_empty() {
            return None;
        }

        // Check user levels first
        let user_path = paths::user_level_path(&self.current_level_id);
        if user_path.exists() {
            return Some(user_path);
        }

        // Check bundled levels
        let bundled_path = paths::bundled_level_path(&self.current_level_id);
        if bundled_path.exists() {
            return Some(bundled_path);
        }

        // Level is embedded-only, return bundled path anyway for reference
        Some(bundled_path)
    }

    /// Get the current level name/ID
    pub fn current_level_name(&self) -> Option<String> {
        if !self.current_level_id.is_empty() {
            Some(self.current_level_id.clone())
        } else {
            None
        }
    }

    /// Load a level from an arbitrary file path
    pub fn load_level_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path = path.as_ref();

        // Read from filesystem
        let content = std::fs::read_to_string(path).map_err(|e| {
            format!("Failed to read '{}': {}", path.display(), e)
        })?;

        let level_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        self.load_from_string(level_id, &content)
    }

    /// Load a procedurally generated level
    pub fn load_generated(&mut self, name: &str, map_data: &str, seed: u64) -> Result<(), String> {
        let content = format!("name: {}\n\n---\n{}", name, map_data);
        self.load_from_string(&format!("procgen_{}", seed), &content)
    }

    /// Clear checkpoint (player will respawn at level start)
    #[allow(dead_code)]
    pub fn clear_checkpoint(&mut self) {
        self.checkpoint = None;
    }

    /// Get the current checkpoint position
    pub fn checkpoint(&self) -> Option<Vec2> {
        self.checkpoint
    }

    /// Get reference to the current level data
    pub fn current_level(&self) -> Option<&LevelData> {
        self.current_level.as_ref()
    }

    /// Get the current level ID
    pub fn current_level_id(&self) -> &str {
        &self.current_level_id
    }

    /// Get the level order
    pub fn level_order(&self) -> &[String] {
        &self.level_order
    }

}

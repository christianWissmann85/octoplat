use std::fmt;

use super::tilemap::TileMap;
use crate::error::OctoplatError;
use crate::procgen::{BiomeId, LevelArchetype};

/// Maximum allowed level file size (1MB) to prevent OOM on malformed files
const MAX_LEVEL_SIZE: usize = 1_000_000;

/// Maximum allowed tilemap dimension (500x500 tiles) for sanity checking
const MAX_TILEMAP_DIMENSION: usize = 500;

/// Metadata for a level
#[derive(Clone, Debug)]
pub struct LevelData {
    pub name: String,
    pub next_level: Option<String>,
    pub biome: Option<BiomeId>,
    pub archetype: Option<LevelArchetype>,
    pub difficulty_tier: Option<u8>,
    pub tilemap: TileMap,
}

impl LevelData {
    /// Parse a level file with header format:
    /// ```text
    /// name: Level Name
    /// next: next_level_id
    /// biome: ocean_depths
    /// archetype: gauntlet
    /// difficulty: 2
    ///
    /// ---
    /// <map data>
    /// ```
    pub fn parse(content: &str, tile_size: f32) -> Result<Self, OctoplatError> {
        // Validate file size to prevent OOM attacks
        if content.len() > MAX_LEVEL_SIZE {
            return Err(OctoplatError::FileTooLarge {
                path: "<content>".to_string(),
                size: content.len(),
                max_size: MAX_LEVEL_SIZE,
            });
        }

        let mut name = String::from("Unnamed Level");
        let mut next_level: Option<String> = None;
        let mut biome: Option<BiomeId> = None;
        let mut archetype: Option<LevelArchetype> = None;
        let mut difficulty_tier: Option<u8> = None;
        let mut map_data = String::new();
        let mut in_map_section = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for section separator
            if trimmed == "---" {
                in_map_section = true;
                continue;
            }

            if in_map_section {
                // Collect map data (preserve original line, not trimmed)
                map_data.push_str(line);
                map_data.push('\n');
            } else {
                // Parse header fields
                if let Some(value) = trimmed.strip_prefix("name:") {
                    name = value.trim().to_string();
                } else if let Some(value) = trimmed.strip_prefix("next:") {
                    let next = value.trim().to_string();
                    if !next.is_empty() {
                        next_level = Some(next);
                    }
                } else if let Some(value) = trimmed.strip_prefix("biome:") {
                    biome = BiomeId::parse(value.trim());
                } else if let Some(value) = trimmed.strip_prefix("archetype:") {
                    archetype = LevelArchetype::parse(value.trim());
                } else if let Some(value) = trimmed.strip_prefix("difficulty:") {
                    difficulty_tier = value.trim().parse().ok();
                }
                // Ignore other header lines (comments, empty lines, etc.)
            }
        }

        // If no separator found, treat entire content as map data (backwards compatible)
        if !in_map_section {
            map_data = content.to_string();
        }

        let tilemap = TileMap::from_string(&map_data, tile_size);

        // Validate tilemap dimensions
        if tilemap.width > MAX_TILEMAP_DIMENSION || tilemap.height > MAX_TILEMAP_DIMENSION {
            return Err(OctoplatError::TilemapTooLarge {
                width: tilemap.width,
                height: tilemap.height,
                max_dimension: MAX_TILEMAP_DIMENSION,
            });
        }

        // Warn if tilemap is empty (likely malformed)
        if tilemap.width == 0 || tilemap.height == 0 {
            return Err(OctoplatError::EmptyTilemap);
        }

        Ok(Self {
            name,
            next_level,
            biome,
            archetype,
            difficulty_tier,
            tilemap,
        })
    }

    /// Create from raw map string (no header)
    #[allow(dead_code)] // Useful for tests and procedural level generation
    pub fn from_map_string(name: &str, map_data: &str, tile_size: f32) -> Self {
        Self {
            name: name.to_string(),
            next_level: None,
            biome: None,
            archetype: None,
            difficulty_tier: None,
            tilemap: TileMap::from_string(map_data, tile_size),
        }
    }

}

/// Serialize the level data to a string that can be parsed back.
///
/// The output format is:
/// ```text
/// name: Level Name
/// next: next_level_id
/// biome: ocean_depths
/// archetype: gauntlet
/// difficulty: 2
/// ---
/// <tilemap data>
/// ```
impl fmt::Display for LevelData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write header fields
        writeln!(f, "name: {}", self.name)?;

        if let Some(ref next) = self.next_level {
            writeln!(f, "next: {}", next)?;
        }

        if let Some(biome) = self.biome {
            writeln!(f, "biome: {}", biome.as_str())?;
        }

        if let Some(archetype) = self.archetype {
            writeln!(f, "archetype: {}", archetype.as_str())?;
        }

        if let Some(tier) = self.difficulty_tier {
            writeln!(f, "difficulty: {}", tier)?;
        }

        // Write separator
        writeln!(f, "---")?;

        // Write tilemap data
        write!(f, "{}", self.tilemap.to_level_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_header() {
        let content = r#"
name: Test Level
next: level_02

---
###
#P#
###
"#;
        let level = LevelData::parse(content, 32.0).unwrap();
        assert_eq!(level.name, "Test Level");
        assert_eq!(level.next_level, Some("level_02".to_string()));
        assert_eq!(level.tilemap.width, 3);
        assert_eq!(level.tilemap.height, 3);
    }

    #[test]
    fn test_parse_without_header() {
        let content = r#"
###
#P#
###
"#;
        let level = LevelData::parse(content, 32.0).unwrap();
        assert_eq!(level.name, "Unnamed Level");
        assert_eq!(level.next_level, None);
    }

    #[test]
    fn test_reject_oversized_file() {
        // Create content larger than MAX_LEVEL_SIZE
        let large_content = "x".repeat(MAX_LEVEL_SIZE + 1);
        let result = LevelData::parse(&large_content, 32.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OctoplatError::FileTooLarge { .. }));
    }

    #[test]
    fn test_reject_empty_tilemap() {
        let content = r#"
name: Empty Level
---
"#;
        let result = LevelData::parse(content, 32.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OctoplatError::EmptyTilemap));
    }
}

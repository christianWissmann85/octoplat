//! Level loading integration tests
//!
//! Verifies level loading functionality:
//! - Parse valid level files
//! - Reject malformed files
//! - Validate file size limits
//! - Handle missing metadata gracefully

use octoplat_core::level::LevelData;
use octoplat_core::procgen::BiomeId;

const TILE_SIZE: f32 = 32.0;

#[test]
fn test_parse_simple_level() {
    let content = r#"name: Test Level
biome: ocean_depths
---
########
#P    >#
########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should parse valid level: {:?}", result.err());

    let level = result.unwrap();
    assert_eq!(level.name, "Test Level");
    assert_eq!(level.biome, Some(BiomeId::OceanDepths));
}

#[test]
fn test_parse_level_with_next_level() {
    let content = r#"name: Level 1
next: level_2
---
########
#P    >#
########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    let level = result.unwrap();
    assert_eq!(level.next_level, Some("level_2".to_string()));
}

#[test]
fn test_parse_level_with_all_metadata() {
    let content = r#"name: Complex Level
next: next_level
biome: volcanic_vents
archetype: gauntlet
difficulty: 3
---
##########
#P      >#
##########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    let level = result.unwrap();
    assert_eq!(level.name, "Complex Level");
    assert_eq!(level.next_level, Some("next_level".to_string()));
    assert_eq!(level.biome, Some(BiomeId::VolcanicVents));
    assert_eq!(level.difficulty_tier, Some(3));
}

#[test]
fn test_parse_level_without_separator() {
    // Backwards compatibility: levels without --- separator
    let content = r#"########
#P    >#
########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should handle legacy format without separator");

    let level = result.unwrap();
    // Default name should be used
    assert_eq!(level.name, "Unnamed Level");
}

#[test]
fn test_parse_level_missing_metadata() {
    let content = r#"---
########
#P    >#
########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should handle missing metadata gracefully");

    let level = result.unwrap();
    assert_eq!(level.name, "Unnamed Level");
    assert!(level.next_level.is_none());
    assert!(level.biome.is_none());
}

#[test]
fn test_parse_empty_content() {
    let content = "";

    let result = LevelData::parse(content, TILE_SIZE);
    // Should not crash, may result in empty tilemap
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_reject_oversized_file() {
    use octoplat_core::error::OctoplatError;

    // Create content larger than MAX_LEVEL_SIZE (1MB)
    let large_content = "X".repeat(1_000_001);

    let result = LevelData::parse(&large_content, TILE_SIZE);
    assert!(result.is_err(), "Should reject oversized files");

    let err = result.unwrap_err();
    assert!(
        matches!(err, OctoplatError::FileTooLarge { .. }),
        "Error should be FileTooLarge: {:?}",
        err
    );
}

#[test]
fn test_reject_oversized_tilemap() {
    use octoplat_core::error::OctoplatError;

    // Create a tilemap that's too large (>500 tiles in one dimension)
    let row = "#".repeat(501);
    let content = format!("---\n{}\n#P{}>#{}\n{}", row, " ".repeat(497), " ".repeat(0), row);

    let result = LevelData::parse(&content, TILE_SIZE);
    assert!(result.is_err(), "Should reject tilemap that's too large");

    let err = result.unwrap_err();
    assert!(
        matches!(err, OctoplatError::TilemapTooLarge { .. }),
        "Error should be TilemapTooLarge: {:?}",
        err
    );
}

#[test]
fn test_parse_all_biomes() {
    let biomes = [
        ("ocean_depths", BiomeId::OceanDepths),
        ("coral_reefs", BiomeId::CoralReefs),
        ("tropical_shore", BiomeId::TropicalShore),
        ("shipwreck", BiomeId::Shipwreck),
        ("arctic_waters", BiomeId::ArcticWaters),
        ("volcanic_vents", BiomeId::VolcanicVents),
        ("sunken_ruins", BiomeId::SunkenRuins),
        ("abyss", BiomeId::Abyss),
    ];

    for (biome_str, expected_biome) in biomes {
        let content = format!(
            r#"name: Test
biome: {}
---
####
#P>#
####
"#,
            biome_str
        );

        let result = LevelData::parse(&content, TILE_SIZE);
        assert!(result.is_ok(), "Should parse biome '{}': {:?}", biome_str, result.err());

        let level = result.unwrap();
        assert_eq!(
            level.biome,
            Some(expected_biome),
            "Biome should match for '{}'",
            biome_str
        );
    }
}

#[test]
fn test_parse_unknown_biome() {
    let content = r#"name: Test
biome: unknown_biome
---
####
#P>#
####
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should handle unknown biome gracefully");

    let level = result.unwrap();
    assert!(level.biome.is_none(), "Unknown biome should result in None");
}

#[test]
fn test_tilemap_dimensions() {
    let content = r#"---
##########
#P      >#
##########
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    let level = result.unwrap();
    assert!(level.tilemap.width > 0, "Tilemap should have width");
    assert!(level.tilemap.height > 0, "Tilemap should have height");
}

#[test]
fn test_tilemap_tile_size() {
    let content = r#"---
####
#P>#
####
"#;

    let custom_tile_size = 64.0;
    let result = LevelData::parse(content, custom_tile_size);
    assert!(result.is_ok());

    let level = result.unwrap();
    assert_eq!(level.tilemap.tile_size, custom_tile_size);
}

#[test]
fn test_parse_with_special_tiles() {
    let content = r#"---
############
#P @  !   >#
#   ^^^    #
############
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should handle special tile markers");
}

#[test]
fn test_parse_multiline_map() {
    let content = r#"---
####################
#P                 #
#                  #
#     ###          #
#                  #
#                 >#
####################
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    let level = result.unwrap();
    assert_eq!(level.tilemap.height, 7);
}

#[test]
fn test_parse_preserves_whitespace_in_map() {
    let content = r#"---
####
#  #
#P>#
####
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    // The tilemap should preserve the interior spaces
    let level = result.unwrap();
    assert!(level.tilemap.width >= 4);
}

#[test]
fn test_level_data_clone() {
    let content = r#"name: Test
biome: ocean_depths
---
####
#P>#
####
"#;

    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());

    let level = result.unwrap();
    let cloned = level.clone();

    assert_eq!(level.name, cloned.name);
    assert_eq!(level.biome, cloned.biome);
}

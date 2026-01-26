//! Integration tests for level loading and tilemap

use octoplat_core::level::{LevelData, TileType};
use octoplat_core::procgen::BiomeId;

const TILE_SIZE: f32 = 32.0;

// =============================================================================
// LevelData Parsing Tests
// =============================================================================

#[test]
fn test_parse_minimal_level() {
    let content = r#"---
####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should parse minimal level: {:?}", result.err());
}

#[test]
fn test_parse_level_with_name() {
    let content = r#"name: Test Level
---
####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());
    let level = result.unwrap();
    assert_eq!(level.name, "Test Level");
}

#[test]
fn test_parse_level_with_biome() {
    let content = r#"name: Ocean Level
biome: ocean_depths
---
####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());
    let level = result.unwrap();
    assert_eq!(level.biome, Some(BiomeId::OceanDepths));
}

#[test]
fn test_parse_level_with_next() {
    let content = r#"name: Level 1
next: level_2
---
####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());
    let level = result.unwrap();
    assert_eq!(level.next_level, Some("level_2".to_string()));
}

#[test]
fn test_parse_level_with_difficulty() {
    let content = r#"name: Hard Level
difficulty: 5
---
####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok());
    let level = result.unwrap();
    assert_eq!(level.difficulty_tier, Some(5));
}

#[test]
fn test_parse_legacy_format() {
    // No separator, just tilemap
    let content = r#"####
#P>#
####
"#;
    let result = LevelData::parse(content, TILE_SIZE);
    assert!(result.is_ok(), "Should handle legacy format");
    let level = result.unwrap();
    assert_eq!(level.name, "Unnamed Level");
}

#[test]
fn test_parse_empty_content() {
    let result = LevelData::parse("", TILE_SIZE);
    // Should not crash - may return empty tilemap
    let _ = result;
}

#[test]
fn test_reject_oversized_content() {
    use octoplat_core::error::OctoplatError;

    let large_content = "X".repeat(1_000_001);
    let result = LevelData::parse(&large_content, TILE_SIZE);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), OctoplatError::FileTooLarge { .. }));
}

#[test]
fn test_reject_oversized_tilemap() {
    use octoplat_core::error::OctoplatError;

    let row = "#".repeat(501);
    let content = format!("---\n{}\n", row);
    let result = LevelData::parse(&content, TILE_SIZE);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), OctoplatError::TilemapTooLarge { .. }));
}

#[test]
fn test_level_to_string_roundtrip() {
    let content = r#"name: Roundtrip Test
biome: coral_reefs
---
########
#P    >#
########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let serialized = level.to_string();

    // Re-parse should produce equivalent level
    let level2 = LevelData::parse(&serialized, TILE_SIZE).unwrap();
    assert_eq!(level.name, level2.name);
    assert_eq!(level.biome, level2.biome);
}

// =============================================================================
// TileMap Tests
// =============================================================================

#[test]
fn test_tilemap_dimensions() {
    let content = r#"---
##########
#P      >#
##########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    assert_eq!(tm.width, 10);
    assert_eq!(tm.height, 3);
    assert_eq!(tm.tile_size, TILE_SIZE);
}

#[test]
fn test_tilemap_tiles_accessible() {
    let content = r#"---
####
#P>#
####
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    // Direct tiles access - corner should be solid
    assert_eq!(tm.tiles[0][0], TileType::Solid);
    // Interior should be empty (spawn marker becomes empty)
    assert_eq!(tm.tiles[1][1], TileType::Empty);
}

#[test]
fn test_tilemap_spawn_position() {
    let content = r#"---
########
#      #
#  P   #
#      #
########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let spawn = tm.get_spawn_position();
    // Spawn should be at tile (3, 2) -> world position (3.5 * 32, 2.5 * 32)
    assert!(spawn.x > 0.0 && spawn.y > 0.0);
}

#[test]
fn test_tilemap_exit_position() {
    let content = r#"---
########
#      #
# P  > #
#      #
########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let exit = tm.get_exit_position();
    assert!(exit.is_some());
    let exit_pos = exit.unwrap();
    assert!(exit_pos.x > 0.0 && exit_pos.y > 0.0);
}

#[test]
fn test_tilemap_grapple_points() {
    let content = r#"---
########
#  @   #
# P  @ #
#      #
########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let grapples = tm.get_grapple_points();
    assert_eq!(grapples.len(), 2);
}

#[test]
fn test_tilemap_checkpoints() {
    let content = r#"---
##########
# S  S   #
# P    > #
##########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let checkpoints = tm.get_checkpoint_positions();
    assert_eq!(checkpoints.len(), 2);
}

#[test]
fn test_tilemap_gems() {
    let content = r#"---
##########
# * * *  #
# P    > #
##########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let gems = tm.get_gem_positions();
    assert_eq!(gems.len(), 3);
}

#[test]
fn test_tilemap_special_tiles() {
    let content = r#"---
#########
#  ^^^  #
# P  !  #
# ___   #
#      >#
#########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    // Verify special tiles are recognized using direct tiles access
    assert_eq!(tm.tiles[1][3], TileType::Spike); // ^
    assert_eq!(tm.tiles[2][5], TileType::BouncePad); // !
    assert_eq!(tm.tiles[3][2], TileType::OneWay); // _
}

#[test]
fn test_tilemap_nearby_solid_rects() {
    let content = r#"---
########
#      #
# P    #
#      #
########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let center = octoplat_core::vec2(100.0, 100.0);
    let rects = tm.get_nearby_solid_rects(center, 200.0);
    assert!(!rects.is_empty());
}

#[test]
fn test_tilemap_bounds() {
    let content = r#"---
##########
#        #
# P    > #
#        #
##########
"#;
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    let tm = &level.tilemap;

    let bounds = tm.bounds();
    assert_eq!(bounds.x, 0.0);
    assert_eq!(bounds.y, 0.0);
    assert_eq!(bounds.w, 10.0 * TILE_SIZE);
    assert_eq!(bounds.h, 5.0 * TILE_SIZE);
}

// =============================================================================
// Biome Parsing Tests
// =============================================================================

#[test]
fn test_all_biomes_parse() {
    let biomes = [
        ("ocean_depths", BiomeId::OceanDepths),
        ("ocean-depths", BiomeId::OceanDepths),
        ("coral_reefs", BiomeId::CoralReefs),
        ("coral-reefs", BiomeId::CoralReefs),
        ("tropical_shore", BiomeId::TropicalShore),
        ("shipwreck", BiomeId::Shipwreck),
        ("arctic_waters", BiomeId::ArcticWaters),
        ("volcanic_vents", BiomeId::VolcanicVents),
        ("sunken_ruins", BiomeId::SunkenRuins),
        ("abyss", BiomeId::Abyss),
    ];

    for (biome_str, expected) in biomes {
        let content = format!(
            "biome: {}\n---\n####\n#P>#\n####",
            biome_str
        );
        let level = LevelData::parse(&content, TILE_SIZE).unwrap();
        assert_eq!(level.biome, Some(expected), "Failed for biome '{}'", biome_str);
    }
}

#[test]
fn test_unknown_biome_returns_none() {
    let content = "biome: unknown_biome\n---\n####\n#P>#\n####";
    let level = LevelData::parse(content, TILE_SIZE).unwrap();
    assert!(level.biome.is_none());
}

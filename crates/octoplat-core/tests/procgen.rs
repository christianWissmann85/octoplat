//! Integration tests for procedural generation and validation

use octoplat_core::procgen::{
    ArchetypePool, ArchetypeSequencer, BiomeId, BiomeTheme,
    LevelArchetype, LevelValidator, MechanicsRequired, MechanicsUsed, MoveType,
    TilePos, ValidationResult,
};

// =============================================================================
// TilePos Tests
// =============================================================================

#[test]
fn test_tile_pos_creation() {
    let pos = TilePos::new(10, 20);
    assert_eq!(pos.x, 10);
    assert_eq!(pos.y, 20);
}

#[test]
fn test_tile_pos_distance() {
    let a = TilePos::new(0, 0);
    let b = TilePos::new(3, 4);
    let dist = a.distance_to(b);
    assert!((dist - 5.0).abs() < 0.0001);
}

#[test]
fn test_tile_pos_manhattan_distance() {
    let a = TilePos::new(0, 0);
    let b = TilePos::new(3, 4);
    assert_eq!(a.manhattan_distance(b), 7);
}

#[test]
fn test_tile_pos_equality() {
    let a = TilePos::new(5, 10);
    let b = TilePos::new(5, 10);
    let c = TilePos::new(5, 11);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_tile_pos_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(TilePos::new(1, 2));
    set.insert(TilePos::new(3, 4));
    set.insert(TilePos::new(1, 2)); // Duplicate

    assert_eq!(set.len(), 2);
}

// =============================================================================
// BiomeId Tests
// =============================================================================

#[test]
fn test_biome_id_all_variants() {
    let biomes = [
        BiomeId::OceanDepths,
        BiomeId::CoralReefs,
        BiomeId::TropicalShore,
        BiomeId::Shipwreck,
        BiomeId::ArcticWaters,
        BiomeId::VolcanicVents,
        BiomeId::SunkenRuins,
        BiomeId::Abyss,
    ];

    assert_eq!(biomes.len(), 8);
}

#[test]
fn test_biome_id_equality() {
    assert_eq!(BiomeId::OceanDepths, BiomeId::OceanDepths);
    assert_ne!(BiomeId::OceanDepths, BiomeId::Abyss);
}

#[test]
fn test_biome_id_clone() {
    let biome = BiomeId::CoralReefs;
    let cloned = biome.clone();
    assert_eq!(biome, cloned);
}

#[test]
fn test_biome_id_parse() {
    assert_eq!(BiomeId::parse("ocean_depths"), Some(BiomeId::OceanDepths));
    assert_eq!(BiomeId::parse("coral_reefs"), Some(BiomeId::CoralReefs));
    assert_eq!(BiomeId::parse("abyss"), Some(BiomeId::Abyss));
    assert_eq!(BiomeId::parse("unknown"), None);
}

// =============================================================================
// BiomeTheme Tests
// =============================================================================

#[test]
fn test_biome_theme_default() {
    let theme = BiomeTheme::default();
    // Theme should have valid colors
    assert!(theme.solid_color.r >= 0.0 && theme.solid_color.r <= 1.0);
    assert!(theme.bg_color_top.r >= 0.0 && theme.bg_color_top.r <= 1.0);
}

#[test]
fn test_biome_theme_bg_color_at() {
    let theme = BiomeTheme::default();

    // Test gradient at various positions
    let top = theme.bg_color_at(0.0);
    let middle = theme.bg_color_at(0.5);
    let bottom = theme.bg_color_at(1.0);

    // Colors should be in valid range
    assert!(top.r >= 0.0 && top.r <= 1.0);
    assert!(middle.r >= 0.0 && middle.r <= 1.0);
    assert!(bottom.r >= 0.0 && bottom.r <= 1.0);
}

#[test]
fn test_biome_theme_border_colors() {
    let theme = BiomeTheme::default();

    let border = theme.solid_border_color();
    let highlight = theme.solid_highlight_color();

    // Border should be darker
    assert!(border.r <= theme.solid_color.r);

    // Highlight should be lighter (or clamped to 1.0)
    assert!(highlight.r >= theme.solid_color.r || highlight.r == 1.0);
}

// =============================================================================
// LevelArchetype Tests
// =============================================================================

#[test]
fn test_level_archetype_variants() {
    let archetypes = [
        LevelArchetype::TheAscent,
        LevelArchetype::TheGauntlet,
        LevelArchetype::TheMaze,
        LevelArchetype::TheArena,
        LevelArchetype::TheCrossing,
        LevelArchetype::TheDepths,
    ];

    assert_eq!(archetypes.len(), 6);
}

#[test]
fn test_level_archetype_parse() {
    assert_eq!(LevelArchetype::parse("ascent"), Some(LevelArchetype::TheAscent));
    assert_eq!(LevelArchetype::parse("gauntlet"), Some(LevelArchetype::TheGauntlet));
    assert_eq!(LevelArchetype::parse("maze"), Some(LevelArchetype::TheMaze));
    assert_eq!(LevelArchetype::parse("unknown"), None);
}

#[test]
fn test_level_archetype_as_str() {
    assert_eq!(LevelArchetype::TheAscent.as_str(), "ascent");
    assert_eq!(LevelArchetype::TheGauntlet.as_str(), "gauntlet");
    assert_eq!(LevelArchetype::TheMaze.as_str(), "maze");
}

// =============================================================================
// LevelValidator Tests
// =============================================================================

fn make_tiles(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

#[test]
fn test_validator_simple_path() {
    let tiles = make_tiles(
        r#"
############
#P        >#
############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    assert!(result.is_completable, "Simple path should be completable");
}

#[test]
fn test_validator_no_spawn() {
    let tiles = make_tiles(
        r#"
######
#    >#
######
"#,
    );
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable);
    assert!(result.issues.iter().any(|i| i.to_lowercase().contains("spawn")));
}

#[test]
fn test_validator_no_exit() {
    let tiles = make_tiles(
        r#"
######
#P   #
######
"#,
    );
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable);
    assert!(result.issues.iter().any(|i| i.to_lowercase().contains("exit")));
}

#[test]
fn test_validator_blocked_path() {
    let tiles = make_tiles(
        r#"
##########
#P   # > #
##########
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable);
}

#[test]
fn test_validator_empty_level() {
    let tiles: Vec<Vec<char>> = Vec::new();
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable);
}

#[test]
fn test_validator_with_jump() {
    let tiles = make_tiles(
        r#"
############
#P         #
###     ####
#         >#
############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    assert!(result.is_completable);
}

#[test]
fn test_validator_with_grapple_point() {
    let tiles = make_tiles(
        r#"
##############
#P    @      #
#            #
#        #####
#           >#
##############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    // Should not crash - result validity depends on mechanics
    let _ = result;
}

#[test]
fn test_validator_with_bounce_pad() {
    let tiles = make_tiles(
        r#"
##############
#P           #
#!       #####
#           >#
##############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    // Should not crash
    let _ = result;
}

#[test]
fn test_validator_with_hazards() {
    let tiles = make_tiles(
        r#"
##############
#P          >#
#   ^^^ ######
##############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    // Should handle hazards
    let _ = result;
}

// =============================================================================
// ValidationResult Tests
// =============================================================================

#[test]
fn test_validation_result_failed() {
    let result = ValidationResult::failed("Test failure");
    assert!(!result.is_completable);
    assert!(!result.is_interesting);
    assert_eq!(result.path_length, 0);
    assert!(result.issues.contains(&"Test failure".to_string()));
}

#[test]
fn test_validation_result_interest_score_range() {
    let tiles = make_tiles(
        r#"
##############
#P          >#
##############
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);

    if result.is_completable {
        assert!(result.interest_score >= 0.0 && result.interest_score <= 1.0);
    }
}

// =============================================================================
// MechanicsUsed Tests
// =============================================================================

#[test]
fn test_mechanics_used_default() {
    let mechanics = MechanicsUsed::default();
    assert!(!mechanics.walking);
    assert!(!mechanics.jumping);
    assert!(!mechanics.wall_jumping);
    assert_eq!(mechanics.count(), 0);
}

#[test]
fn test_mechanics_used_count() {
    let mut mechanics = MechanicsUsed::default();
    mechanics.walking = true;
    mechanics.jumping = true;
    assert_eq!(mechanics.count(), 2);
}

// =============================================================================
// MechanicsRequired Tests
// =============================================================================

#[test]
fn test_mechanics_required_none() {
    let req = MechanicsRequired::none();
    assert!(!req.grapple);
    assert!(!req.wall_jump);
    assert!(!req.bounce);
    assert!(!req.has_advanced());
}

#[test]
fn test_mechanics_required_has_advanced() {
    let req = MechanicsRequired::new(true, false, false);
    assert!(req.has_advanced());

    let req2 = MechanicsRequired::new(false, true, false);
    assert!(req2.has_advanced());

    let req3 = MechanicsRequired::new(false, false, true);
    assert!(req3.has_advanced());
}

// =============================================================================
// MoveType Tests
// =============================================================================

#[test]
fn test_move_type_variants() {
    let types = [
        MoveType::Walk,
        MoveType::Jump,
        MoveType::WallJump,
        MoveType::Grapple,
        MoveType::Bounce,
        MoveType::Fall,
        MoveType::Dive,
        MoveType::JetBoost,
    ];

    assert_eq!(types.len(), 8);
}

#[test]
fn test_move_type_equality() {
    assert_eq!(MoveType::Walk, MoveType::Walk);
    assert_ne!(MoveType::Walk, MoveType::Jump);
}

// =============================================================================
// ArchetypePool Tests
// =============================================================================

#[test]
fn test_archetype_pool_creation() {
    let pool = ArchetypePool::new();
    assert!(pool.is_empty());
}

#[test]
fn test_archetype_pool_is_empty() {
    let pool = ArchetypePool::new();
    assert!(pool.is_empty());
}

// =============================================================================
// ArchetypeSequencer Tests
// =============================================================================

#[test]
fn test_archetype_sequencer_creation() {
    let sequencer = ArchetypeSequencer::new(12345);
    // Sequencer should be created without panic
    let _ = sequencer;
}

#[test]
fn test_archetype_sequencer_select() {
    let mut sequencer = ArchetypeSequencer::new(12345);
    let available = vec![
        LevelArchetype::TheAscent,
        LevelArchetype::TheGauntlet,
        LevelArchetype::TheMaze,
    ];

    let selected = sequencer.select_archetype(&available, 0, false);
    assert!(selected.is_some());
    assert!(available.contains(&selected.unwrap()));
}

#[test]
fn test_archetype_sequencer_boss_selection() {
    let mut sequencer = ArchetypeSequencer::new(12345);
    let available = vec![
        LevelArchetype::TheAscent,
        LevelArchetype::TheGauntlet,
        LevelArchetype::TheMaze,
        LevelArchetype::TheArena,
    ];

    // Boss levels might prefer certain archetypes
    let selected = sequencer.select_archetype(&available, 3, true);
    assert!(selected.is_some());
}

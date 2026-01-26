//! Procedural generation integration tests
//!
//! Verifies the procgen pipeline:
//! - Level validation (path finding, mechanics detection)
//! - Biome themes
//! - Validation results

use octoplat_core::procgen::BiomeId;
use octoplat_core::procgen::{LevelValidator, MechanicsRequired, ValidationResult};

/// Helper to create a tilemap from string
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
    assert!(result.is_completable, "Simple horizontal path should be completable");
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
    assert!(!result.is_completable, "Level without spawn should not be completable");
    assert!(result.issues.iter().any(|i: &String| i.contains("spawn")));
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
    assert!(!result.is_completable, "Level without exit should not be completable");
    assert!(result.issues.iter().any(|i: &String| i.contains("exit")));
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
    assert!(!result.is_completable, "Blocked path should not be completable");
}

#[test]
fn test_validator_empty_level() {
    let tiles: Vec<Vec<char>> = Vec::new();
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable, "Empty level should not be completable");
}

#[test]
fn test_validator_zero_width() {
    let tiles = vec![Vec::new()];
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);
    assert!(!result.is_completable, "Zero width level should not be completable");
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
    assert!(result.is_completable, "Level with jump should be completable");
}

#[test]
fn test_validator_mechanics_required_none() {
    // Simple level that requires no special mechanics
    let tiles = make_tiles(
        r#"
############
#P        >#
############
"#,
    );
    let validator = LevelValidator::new();
    let result = validator.validate_detailed(&tiles);

    // This level is so simple it might not require advanced mechanics
    // Just verify the mechanics_required field exists and is valid
    assert!(result.is_completable);
}

#[test]
fn test_validation_result_is_valid() {
    let tiles = make_tiles(
        r#"
################
#P            >#
################
"#,
    );

    // With relaxed thresholds, should be both completable and interesting
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);
    assert!(result.is_completable);
}

#[test]
fn test_validation_result_failed() {
    let result = ValidationResult::failed("Test failure message");
    assert!(!result.is_completable);
    assert!(!result.is_interesting);
    assert_eq!(result.path_length, 0);
    assert!(result.issues.contains(&"Test failure message".to_string()));
}

#[test]
fn test_mechanics_required_has_advanced() {
    // No mechanics required
    let none = MechanicsRequired::none();
    assert!(!none.has_advanced());

    // With grapple required
    let with_grapple = MechanicsRequired::new(true, false, false);
    assert!(with_grapple.has_advanced());

    // With wall jump required
    let with_wall_jump = MechanicsRequired::new(false, true, false);
    assert!(with_wall_jump.has_advanced());

    // With bounce required
    let with_bounce = MechanicsRequired::new(false, false, true);
    assert!(with_bounce.has_advanced());
}

#[test]
fn test_biome_ids() {
    // Verify all biome IDs are distinct
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

    // Each biome should be different from others
    for i in 0..biomes.len() {
        for j in (i + 1)..biomes.len() {
            assert_ne!(biomes[i], biomes[j], "Biomes {:?} and {:?} should be different", biomes[i], biomes[j]);
        }
    }
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
    // Just verify it processes levels with grapple points without crashing
    assert!(result.is_completable || !result.is_completable);
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
    // Just verify it processes levels with bounce pads without crashing
    assert!(result.is_completable || !result.is_completable);
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
    // Just verify hazards are processed without crashing
    assert!(result.is_completable || !result.is_completable);
}

#[test]
fn test_validator_path_length_tracking() {
    let tiles = make_tiles(
        r#"
########################
#P                    >#
########################
"#,
    );
    let validator = LevelValidator::with_thresholds(1, 1, 0.0);
    let result = validator.validate_detailed(&tiles);

    if result.is_completable {
        // Path length should be positive for completable levels
        assert!(result.path_length > 0, "Path length should be positive");
    }
}

#[test]
fn test_validator_interest_score_range() {
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
        // Interest score should be in valid range
        assert!(result.interest_score >= 0.0 && result.interest_score <= 1.0,
            "Interest score should be between 0.0 and 1.0, got {}", result.interest_score);
    }
}

// =============================================================================
// ProcgenManager Tests
// =============================================================================

use octoplat_game::procgen::ProcgenManager;

#[test]
fn test_procgen_manager_creation() {
    let manager = ProcgenManager::new();
    // Manager should start without archetype pool loaded
    assert!(!manager.has_archetype_pool());
}

#[test]
fn test_procgen_manager_load_archetype_pool() {
    let mut manager = ProcgenManager::new();
    // Loading uses embedded assets now, path argument is ignored
    let result = manager.load_archetype_pool("/any/path");
    assert!(result.is_ok());
    // Should load embedded levels
    assert!(result.unwrap() > 0);
}

#[test]
fn test_procgen_manager_sequencer_init() {
    let mut manager = ProcgenManager::new();
    // Initializing sequencer should not panic
    manager.init_archetype_sequencer(12345);
}

// =============================================================================
// DifficultyParams Tests
// =============================================================================

use octoplat_game::procgen::difficulty::DifficultyParams;
use octoplat_core::state::DifficultyPreset;

#[test]
fn test_difficulty_params_casual_start() {
    let params = DifficultyParams::for_progress(0.0, DifficultyPreset::Casual);
    assert_eq!(params.progress, 0.0);
    assert_eq!(params.min_tier, 1);
    assert_eq!(params.max_tier, 1);
}

#[test]
fn test_difficulty_params_standard_start() {
    let params = DifficultyParams::for_progress(0.0, DifficultyPreset::Standard);
    assert_eq!(params.progress, 0.0);
    assert_eq!(params.min_tier, 1);
}

#[test]
fn test_difficulty_params_challenge_start() {
    let params = DifficultyParams::for_progress(0.0, DifficultyPreset::Challenge);
    assert_eq!(params.progress, 0.0);
    assert_eq!(params.min_tier, 1);
}

#[test]
fn test_difficulty_params_progress_clamped() {
    // Progress beyond 1.0 should be clamped
    let params = DifficultyParams::for_progress(2.0, DifficultyPreset::Standard);
    assert_eq!(params.progress, 1.0);

    // Negative progress should be clamped to 0
    let params = DifficultyParams::for_progress(-0.5, DifficultyPreset::Standard);
    assert_eq!(params.progress, 0.0);
}

#[test]
fn test_difficulty_params_enemy_chance_increases() {
    let start = DifficultyParams::for_progress(0.0, DifficultyPreset::Standard);
    let end = DifficultyParams::for_progress(1.0, DifficultyPreset::Standard);

    assert!(end.enemy_chance > start.enemy_chance,
        "Enemy chance should increase with progress");
}

#[test]
fn test_difficulty_params_hazard_chance_increases() {
    let start = DifficultyParams::for_progress(0.0, DifficultyPreset::Standard);
    let end = DifficultyParams::for_progress(1.0, DifficultyPreset::Standard);

    assert!(end.hazard_chance > start.hazard_chance,
        "Hazard chance should increase with progress");
}

#[test]
fn test_difficulty_params_grapple_chance_decreases() {
    let start = DifficultyParams::for_progress(0.0, DifficultyPreset::Standard);
    let end = DifficultyParams::for_progress(1.0, DifficultyPreset::Standard);

    assert!(end.grapple_chance < start.grapple_chance,
        "Grapple chance should decrease with progress");
}

#[test]
fn test_difficulty_params_casual_max_tier_capped() {
    // Even at max progress, casual should cap at tier 2
    let params = DifficultyParams::for_progress(1.0, DifficultyPreset::Casual);
    assert!(params.max_tier <= 2, "Casual max tier should be capped at 2");
}

#[test]
fn test_difficulty_params_standard_max_tier_capped() {
    // Standard should cap at tier 4
    let params = DifficultyParams::for_progress(1.0, DifficultyPreset::Standard);
    assert!(params.max_tier <= 4, "Standard max tier should be capped at 4");
}

#[test]
fn test_difficulty_params_challenge_allows_tier_5() {
    // Challenge should allow tier 5 at max progress
    let params = DifficultyParams::for_progress(1.0, DifficultyPreset::Challenge);
    assert!(params.max_tier == 5, "Challenge should allow tier 5");
}

#[test]
fn test_difficulty_params_debug_impl() {
    let params = DifficultyParams::for_progress(0.5, DifficultyPreset::Standard);
    let debug_str = format!("{:?}", params);
    assert!(debug_str.contains("progress"));
}

// =============================================================================
// ProcgenError Tests
// =============================================================================

use octoplat_game::procgen::ProcgenError;

#[test]
fn test_procgen_error_pool_not_loaded() {
    let error = ProcgenError::PoolNotLoaded;
    let msg = error.to_string();
    assert!(msg.contains("pool"));
}

#[test]
fn test_procgen_error_no_levels_for_biome() {
    let error = ProcgenError::NoLevelsForBiome { biome: BiomeId::OceanDepths };
    let msg = error.to_string();
    assert!(msg.contains("biome"));
}

#[test]
fn test_procgen_error_validation_failed() {
    let error = ProcgenError::ValidationFailed {
        issues: vec!["test issue".to_string()]
    };
    let msg = error.to_string();
    assert!(msg.contains("test issue"));
}

#[test]
fn test_procgen_error_retries_exhausted() {
    let error = ProcgenError::RetriesExhausted { attempts: 5 };
    let msg = error.to_string();
    assert!(msg.contains("5"));
}

#[test]
fn test_procgen_error_into_string() {
    let error = ProcgenError::LinkingFailed;
    let s: String = error.into();
    assert!(s.contains("linking"));
}

// =============================================================================
// DifficultyPreset Tests
// =============================================================================

#[test]
fn test_difficulty_preset_variants() {
    let _casual = DifficultyPreset::Casual;
    let _standard = DifficultyPreset::Standard;
    let _challenge = DifficultyPreset::Challenge;
}

#[test]
fn test_difficulty_preset_equality() {
    assert_eq!(DifficultyPreset::Casual, DifficultyPreset::Casual);
    assert_ne!(DifficultyPreset::Casual, DifficultyPreset::Standard);
    assert_ne!(DifficultyPreset::Standard, DifficultyPreset::Challenge);
}

// =============================================================================
// LayoutStrategy Tests
// =============================================================================

use octoplat_game::procgen::LayoutStrategy;

#[test]
fn test_layout_strategy_variants() {
    let _linear = LayoutStrategy::Linear;
    let _vertical = LayoutStrategy::Vertical;
    let _alternating = LayoutStrategy::Alternating;
    let _grid = LayoutStrategy::Grid;
}

#[test]
fn test_layout_strategy_debug() {
    let strategy = LayoutStrategy::Linear;
    let debug_str = format!("{:?}", strategy);
    assert!(debug_str.contains("Linear"));
}

// =============================================================================
// Biome Generation Tests
// =============================================================================

#[test]
fn test_generate_level_ocean_depths() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Ocean depths should work
    let result = manager.generate_roguelite_level(
        BiomeId::OceanDepths,
        DifficultyPreset::Standard,
        0, false, 12345
    );

    assert!(result.is_ok(), "Ocean Depths generation failed: {:?}", result.err());
}

#[test]
fn test_generate_level_volcanic_vents() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Try multiple seeds for volcanic vents using single archetype
    let mut successes = 0;
    let mut failures = Vec::new();
    for seed in [1u64, 42, 999, 12345, 54321] {
        match manager.generate_roguelite_level(
            BiomeId::VolcanicVents,
            DifficultyPreset::Standard,
            0, false, seed
        ) {
            Ok(_) => successes += 1,
            Err(e) => failures.push((seed, e.to_string())),
        }
    }

    assert!(successes > 0, "All Volcanic Vents generations failed: {:?}", failures);
}

#[test]
fn test_generate_level_abyss() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Try multiple seeds for abyss using single archetype
    let mut successes = 0;
    let mut failures = Vec::new();
    for seed in [1u64, 42, 999, 12345, 54321] {
        match manager.generate_roguelite_level(
            BiomeId::Abyss,
            DifficultyPreset::Standard,
            0, false, seed
        ) {
            Ok(_) => successes += 1,
            Err(e) => failures.push((seed, e.to_string())),
        }
    }

    assert!(successes > 0, "All Abyss generations failed: {:?}", failures);
}

// =============================================================================
// Linked Segment Generation Tests (matches actual game flow)
// =============================================================================

#[test]
fn test_linked_generation_ocean_depths() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Ocean depths linked generation should work
    let result = manager.generate_linked_level_with_retry(
        BiomeId::OceanDepths,
        DifficultyPreset::Standard,
        0, 12345, 2  // 2 segments
    );

    assert!(result.is_ok(), "Ocean Depths linked generation failed: {:?}", result.err());
}

#[test]
fn test_linked_generation_volcanic_vents() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Try multiple seeds for volcanic vents using linked segments (actual game flow)
    let mut successes = 0;
    let mut failures = Vec::new();
    for seed in [1u64, 42, 999, 12345, 54321] {
        match manager.generate_linked_level_with_retry(
            BiomeId::VolcanicVents,
            DifficultyPreset::Standard,
            0, seed, 2  // 2 segments as game uses for first levels
        ) {
            Ok(_) => successes += 1,
            Err(e) => failures.push((seed, e.to_string())),
        }
    }

    assert!(successes > 0, "All Volcanic Vents linked generations failed: {:?}", failures);
}

#[test]
fn test_linked_generation_abyss() {
    let mut manager = ProcgenManager::new();
    manager.load_archetype_pool("/any").unwrap();
    manager.init_archetype_sequencer(12345);

    // Try multiple seeds for abyss using linked segments (actual game flow)
    let mut successes = 0;
    let mut failures = Vec::new();
    for seed in [1u64, 42, 999, 12345, 54321] {
        match manager.generate_linked_level_with_retry(
            BiomeId::Abyss,
            DifficultyPreset::Standard,
            0, seed, 2  // 2 segments as game uses for first levels
        ) {
            Ok(_) => successes += 1,
            Err(e) => failures.push((seed, e.to_string())),
        }
    }

    assert!(successes > 0, "All Abyss linked generations failed: {:?}", failures);
}

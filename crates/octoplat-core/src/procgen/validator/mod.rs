//! Level validation for checking reachability and interestingness
//!
//! This module implements a sophisticated validation system that verifies procedurally
//! generated levels are both completable and engaging to play.

mod analysis;
mod pathfinding;
mod scoring;
mod types;

use analysis::GeometryConstraints;

pub use types::{
    MechanicsRequired, MechanicsUsed, MoveType, MovementCaps, TilePos, ValidationResult,
};

/// Level validator that checks reachability and interestingness
pub struct LevelValidator {
    caps: MovementCaps,
    constraints: GeometryConstraints,
    min_path_length: usize,
    min_mechanics_available: usize,
    min_interest_score: f32,
}

impl LevelValidator {
    pub fn new() -> Self {
        Self {
            caps: MovementCaps::default(),
            constraints: GeometryConstraints::default(),
            min_path_length: 5,
            min_mechanics_available: 2,
            min_interest_score: 0.3,
        }
    }

    #[allow(dead_code)]
    pub fn with_thresholds(
        min_path_length: usize,
        min_mechanics: usize,
        min_interest: f32,
    ) -> Self {
        Self {
            caps: MovementCaps::default(),
            constraints: GeometryConstraints::default(),
            min_path_length,
            min_mechanics_available: min_mechanics,
            min_interest_score: min_interest,
        }
    }

    pub fn validate_detailed(&self, tiles: &[Vec<char>]) -> ValidationResult {
        let height = tiles.len();
        if height == 0 {
            return ValidationResult::failed("Empty level");
        }
        let width = tiles[0].len();
        if width == 0 {
            return ValidationResult::failed("Zero width level");
        }

        let spawn = analysis::find_marker(tiles, 'P');
        let exit = analysis::find_marker(tiles, '>');

        let (spawn, exit) = match (spawn, exit) {
            (Some(s), Some(e)) => (s, e),
            (None, _) => return ValidationResult::failed("No spawn point (P) found"),
            (_, None) => return ValidationResult::failed("No exit point (>) found"),
        };

        // Note: We no longer enforce strict left/right placement for spawn/exit.
        // Non-linear layouts (Vertical, Alternating, Grid) place segments in ways
        // that don't guarantee spawn-left/exit-right positioning.
        // The flood-fill and BFS checks below are sufficient to ensure level validity.

        // Quick flood-fill connectivity check
        if !pathfinding::are_connected_flood_fill(tiles, spawn, exit) {
            return ValidationResult::failed("Spawn and exit are in disconnected regions (no valid path)");
        }

        let grapple_points = analysis::find_all_markers(tiles, '@');
        let bounce_pads = analysis::find_all_markers(tiles, '!');
        let hazards = analysis::find_hazards(tiles);

        let bfs_result = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &bounce_pads,
            &hazards,
            false,
            false,
            false,
            false,
        );

        match bfs_result {
            None => ValidationResult::failed("No path from spawn to exit"),
            Some((path_length, mechanics_used)) => {
                let mechanics_required = self.determine_required_mechanics(tiles);

                let mut result = ValidationResult {
                    is_completable: true,
                    is_interesting: true,
                    path_length,
                    issues: Vec::new(),
                    mechanics_used: mechanics_used.clone(),
                    mechanics_required: mechanics_required.clone(),
                    interest_score: 0.0,
                };

                result.interest_score = scoring::calculate_interest_score(
                    tiles,
                    &grapple_points,
                    &bounce_pads,
                    &hazards,
                    path_length,
                    &mechanics_used,
                );

                let bottlenecks = analysis::find_passage_bottlenecks(tiles, &self.constraints);
                if !bottlenecks.is_empty() {
                    result.issues.push(format!(
                        "Found {} passage bottleneck(s) that may be impassable",
                        bottlenecks.len()
                    ));
                }

                if path_length < self.min_path_length {
                    result.issues.push(format!(
                        "Path too short ({} steps, minimum {})",
                        path_length, self.min_path_length
                    ));
                    result.is_interesting = false;
                }

                let available_mechanics =
                    scoring::count_available_mechanics(tiles, &grapple_points, &bounce_pads);
                if available_mechanics < self.min_mechanics_available {
                    result.issues.push(format!(
                        "Too few mechanics available ({}, minimum {})",
                        available_mechanics, self.min_mechanics_available
                    ));
                    result.is_interesting = false;
                }

                if result.interest_score < self.min_interest_score {
                    result.issues.push(format!(
                        "Interest score too low ({:.2}, minimum {:.2})",
                        result.interest_score, self.min_interest_score
                    ));
                    result.is_interesting = false;
                }

                result
            }
        }
    }

    pub fn determine_required_mechanics(&self, tiles: &[Vec<char>]) -> MechanicsRequired {
        let height = tiles.len();
        if height == 0 {
            return MechanicsRequired::none();
        }
        let width = tiles.first().map(|r| r.len()).unwrap_or(0);
        if width == 0 {
            return MechanicsRequired::none();
        }

        let spawn = match analysis::find_marker(tiles, 'P') {
            Some(s) => s,
            None => return MechanicsRequired::none(),
        };
        let exit = match analysis::find_marker(tiles, '>') {
            Some(e) => e,
            None => return MechanicsRequired::none(),
        };

        let grapple_points = analysis::find_all_markers(tiles, '@');
        let bounce_pads = analysis::find_all_markers(tiles, '!');
        let hazards = analysis::find_hazards(tiles);

        let baseline = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &bounce_pads,
            &hazards,
            false,
            false,
            false,
            false,
        );
        if baseline.is_none() {
            return MechanicsRequired::none();
        }

        let mut required = MechanicsRequired::none();

        // Test without grapple
        let without_grapple = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &[],
            &bounce_pads,
            &hazards,
            false,
            false,
            false,
            false,
        );
        required.grapple = without_grapple.is_none();

        // Test without wall jump
        let without_wall_jump = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &bounce_pads,
            &hazards,
            true,
            false,
            false,
            false,
        );
        required.wall_jump = without_wall_jump.is_none();

        // Test without bounce
        let without_bounce = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &[],
            &hazards,
            false,
            false,
            false,
            false,
        );
        required.bounce = without_bounce.is_none();

        // Test without dive
        let without_dive = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &bounce_pads,
            &hazards,
            false,
            true,
            false,
            false,
        );
        required.dive = without_dive.is_none();

        // Test without jet boost
        let without_jet = pathfinding::bfs_with_mechanics_disabled(
            &self.caps,
            tiles,
            spawn,
            exit,
            &grapple_points,
            &bounce_pads,
            &hazards,
            false,
            false,
            true,
            false,
        );
        required.jet_boost = without_jet.is_none();

        required
    }
}

impl Default for LevelValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tiles(s: &str) -> Vec<Vec<char>> {
        s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect()
    }

    #[test]
    fn test_simple_path() {
        let tiles = make_tiles(
            r#"
########
#P    >#
########
"#,
        );
        let validator = LevelValidator::with_thresholds(1, 1, 0.0);
        let result = validator.validate_detailed(&tiles);
        assert!(result.is_completable);
    }

    #[test]
    fn test_no_spawn() {
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
        assert!(result.issues.iter().any(|i| i.contains("spawn")));
    }

    #[test]
    fn test_no_exit() {
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
        assert!(result.issues.iter().any(|i| i.contains("exit")));
    }
}

//! Level visual polish - unified setup for themes, backgrounds, and decorations
//!
//! Provides a single function to set up all visual polish for roguelite levels.

use std::path::Path;

use octoplat_core::procgen::BiomeId;
use octoplat_core::procgen::biome::BiomeTheme;
use octoplat_core::DEFAULT_TILE_SIZE;

use crate::level::{Decoration, LevelData, generate_decorations_for_tilemap};
use crate::rendering::BiomeBackground;

/// Visual polish components for a level
pub struct LevelVisuals {
    /// Biome theme for colored rendering
    pub theme: BiomeTheme,
    /// Parallax background layers
    pub background: BiomeBackground,
    /// Decorations (seaweed, coral, rocks, etc.)
    pub decorations: Vec<Decoration>,
    /// The biome used for generation
    pub biome: BiomeId,
    /// The seed used for generation
    pub seed: u64,
}

/// Set up all visual polish for a level from its content
///
/// This function:
/// - Parses the level to determine the biome (or uses the override)
/// - Computes a deterministic seed from the level content
/// - Generates theme, background, and decorations
///
/// # Arguments
/// * `level_content` - The raw level file content (with or without header)
/// * `biome_override` - Optional biome to use instead of parsing from level
/// * `level_width` - Width of the level in pixels (for background generation)
///
/// # Returns
/// A `LevelVisuals` struct with all visual components ready to use
pub fn setup_level_visuals(
    level_content: &str,
    biome_override: Option<BiomeId>,
    level_width: f32,
) -> LevelVisuals {
    // Determine biome: use override, or parse from level, or default
    let biome = biome_override.unwrap_or_else(|| {
        LevelData::parse(level_content, DEFAULT_TILE_SIZE)
            .ok()
            .and_then(|data| data.biome)
            .unwrap_or(BiomeId::OceanDepths)
    });

    // Compute deterministic seed from content
    let seed = compute_content_seed(level_content);

    // Get theme from biome definition
    let theme = biome.definition().theme;

    // Generate parallax background
    let background = BiomeBackground::generate(biome, level_width, seed);

    // Extract tilemap portion for decoration generation
    let tilemap_data = extract_tilemap(level_content);

    // Generate decorations
    let decorations = generate_decorations_for_tilemap(
        tilemap_data,
        biome,
        seed,
        DEFAULT_TILE_SIZE,
    );

    #[cfg(debug_assertions)]
    eprintln!(
        "[Decorations] Generated {} decorations for biome {:?} (seed: {})",
        decorations.len(),
        biome,
        seed
    );
    #[cfg(debug_assertions)]
    if !decorations.is_empty() {
        eprintln!(
            "[Decorations] Types: {:?}",
            decorations.iter().take(5).map(|d| format!("{:?}", d.decoration_type)).collect::<Vec<_>>()
        );
    }

    LevelVisuals {
        theme,
        background,
        decorations,
        biome,
        seed,
    }
}

/// Compute a deterministic seed from level content
///
/// Uses a simple hash function to ensure the same level content
/// always produces the same decorations and background.
fn compute_content_seed(content: &str) -> u64 {
    content.bytes().fold(0u64, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(b as u64)
    })
}

/// Extract the tilemap portion from level content
///
/// Handles both levels with metadata headers (separated by ---) and
/// raw tilemap content without headers. Leading empty lines are trimmed.
fn extract_tilemap(content: &str) -> &str {
    let tilemap = if let Some(separator_pos) = content.find("\n---\n") {
        &content[separator_pos + 5..]
    } else if content.starts_with("---") {
        &content[4..]
    } else {
        // No header, treat entire content as tilemap
        content
    };
    // Trim leading empty lines so the first line determines width correctly
    tilemap.trim_start_matches('\n')
}

/// Read level content from a path
///
/// Returns the level content as a String, or None if the file cannot be read.
pub fn read_level_content<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_content_seed_deterministic() {
        let content = "name: Test\nbiome: ocean-depths\n---\n####\n#  #\n####";
        let seed1 = compute_content_seed(content);
        let seed2 = compute_content_seed(content);
        assert_eq!(seed1, seed2);
    }

    #[test]
    fn test_compute_content_seed_different_content() {
        let content1 = "####\n#  #\n####";
        let content2 = "####\n#  #\n###";
        let seed1 = compute_content_seed(content1);
        let seed2 = compute_content_seed(content2);
        assert_ne!(seed1, seed2);
    }

    #[test]
    fn test_extract_tilemap_with_header() {
        let content = "name: Test\nbiome: ocean-depths\n---\n####\n#  #";
        let tilemap = extract_tilemap(content);
        assert_eq!(tilemap, "####\n#  #");
    }

    #[test]
    fn test_extract_tilemap_no_header() {
        let content = "####\n#  #\n####";
        let tilemap = extract_tilemap(content);
        assert_eq!(tilemap, content);
    }

    #[test]
    fn test_extract_tilemap_with_leading_newlines() {
        // Simulates a level file with blank line after ---
        let content = "name: Test\n---\n\n####\n#  #";
        let tilemap = extract_tilemap(content);
        // Should strip the leading newline
        assert_eq!(tilemap, "####\n#  #");
    }

    #[test]
    fn test_setup_level_visuals_generates_decorations() {
        // Simulate actual level file format (like level_01.txt)
        let content = r#"name: First Strokes
biome: ocean_depths
next: level_02
---

##################################################
#                                                #
# P                                              #
# #####                                          #
#         *                                      #
#       #####                                    #
#                 *                              #
#              ######                            #
#                                                #
#                       *                        #
#                     #####                      #
#   *                                            #
#  ###                         *              >  #
#                            #####       ########
#                                                #
##################################################"#;

        let level_width = 50.0 * 32.0; // 50 tiles * 32 pixels
        let visuals = setup_level_visuals(content, None, level_width);

        // Verify biome was detected
        assert_eq!(visuals.biome, BiomeId::OceanDepths);

        // Verify decorations were generated (should have some given the level has solid tiles)
        eprintln!("Test generated {} decorations", visuals.decorations.len());
        assert!(!visuals.decorations.is_empty(),
            "Expected decorations to be generated for this level, but got 0");
    }
}

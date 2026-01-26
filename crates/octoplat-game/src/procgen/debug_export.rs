//! Debug export functionality for procedural generation
//!
//! Exports generated levels to text files for debugging and analysis.

use std::fs;

use octoplat_core::procgen::BiomeId;
use octoplat_core::state::DifficultyPreset;
use super::segment_linker::LayoutStrategy;

/// Debug directory for exported levels
const DEBUG_EXPORT_DIR: &str = "debug_levels";

/// Export a generated level to a text file for debugging
#[allow(clippy::too_many_arguments)]
pub fn export_debug_level(
    tilemap: &str,
    biome: BiomeId,
    preset: DifficultyPreset,
    layout: LayoutStrategy,
    segment_names: &[String],
    seed: u64,
    level_index: u32,
    width: usize,
    height: usize,
) {
    // Create debug directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(DEBUG_EXPORT_DIR) {
        #[cfg(debug_assertions)]
        eprintln!("[Debug Export] Failed to create directory: {}", e);
        let _ = e;
        return;
    }

    let biome_def = biome.definition();
    let preset_name = match preset {
        DifficultyPreset::Casual => "casual",
        DifficultyPreset::Standard => "standard",
        DifficultyPreset::Challenge => "challenge",
    };
    let layout_name = match layout {
        LayoutStrategy::Linear => "linear",
        LayoutStrategy::Vertical => "vertical",
        LayoutStrategy::Alternating => "alternating",
        LayoutStrategy::Grid => "grid",
    };

    // Generate filename with timestamp for uniqueness
    let filename = format!(
        "{}/level_{:04}_{}_{}_{}_seed{}.txt",
        DEBUG_EXPORT_DIR,
        level_index,
        biome_def.name.to_lowercase().replace(" ", "_"),
        preset_name,
        layout_name,
        seed % 100000
    );

    // Build content with metadata header
    let mut content = String::new();
    content.push_str("# Debug Level Export\n");
    content.push_str("# ===================\n");
    content.push_str(&format!("# Biome: {} ({:?})\n", biome_def.name, biome_def.id));
    content.push_str(&format!("# Difficulty: {:?}\n", preset));
    content.push_str(&format!("# Layout: {:?}\n", layout));
    content.push_str(&format!("# Level Index: {}\n", level_index));
    content.push_str(&format!("# Seed: {}\n", seed));
    content.push_str(&format!("# Dimensions: {}x{}\n", width, height));
    content.push_str(&format!("# Segments ({}): {}\n", segment_names.len(), segment_names.join(" -> ")));
    content.push_str("#\n");
    content.push_str("# Legend:\n");
    content.push_str("#   # = solid wall\n");
    content.push_str("#   P = player spawn\n");
    content.push_str("#   > = exit\n");
    content.push_str("#   $ = gem\n");
    content.push_str("#   ~ = hazard\n");
    content.push_str("#   ? = grapple point\n");
    content.push_str("#   %% = checkpoint\n");
    content.push_str("#   (space) = empty/air\n");
    content.push_str("#\n");
    content.push_str("# Tilemap:\n");
    content.push_str("# --------\n\n");
    content.push_str(tilemap);

    // Write to file
    match fs::write(&filename, content) {
        Ok(_) => {
            #[cfg(debug_assertions)]
            println!("[Debug Export] Level saved to: {}", filename);
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("[Debug Export] Failed to write file: {}", e);
            let _ = e;
        }
    }
}

/// Export all individual segments before linking for comparison
pub fn export_debug_segments(
    segments: &[(String, String)],  // (name, tilemap) pairs
    biome: BiomeId,
    seed: u64,
) {
    // Create debug directory if it doesn't exist
    let segments_dir = format!("{}/segments", DEBUG_EXPORT_DIR);
    if let Err(e) = fs::create_dir_all(&segments_dir) {
        #[cfg(debug_assertions)]
        eprintln!("[Debug Export] Failed to create segments directory: {}", e);
        let _ = e;
        return;
    }

    let biome_def = biome.definition();

    for (idx, (name, tilemap)) in segments.iter().enumerate() {
        let filename = format!(
            "{}/seg_{:02}_{}_{}_{}.txt",
            segments_dir,
            idx,
            biome_def.name.to_lowercase().replace(" ", "_"),
            name.replace(" ", "_").replace("/", "_"),
            seed % 10000
        );

        let mut content = String::new();
        content.push_str(&format!("# Segment: {}\n", name));
        content.push_str(&format!("# Index: {}\n", idx));
        content.push_str(&format!("# Biome: {}\n", biome_def.name));
        content.push_str("#\n\n");
        content.push_str(tilemap);

        if let Err(e) = fs::write(&filename, content) {
            #[cfg(debug_assertions)]
            eprintln!("[Debug Export] Failed to write segment file: {}", e);
            let _ = e;
        }
    }

    #[cfg(debug_assertions)]
    println!("[Debug Export] {} segments saved to: {}/", segments.len(), segments_dir);
}

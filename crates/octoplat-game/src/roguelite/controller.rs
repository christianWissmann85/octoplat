//! RogueLite mode controller
//!
//! Handles level generation, progression, and run recording.

use crate::config::GameConfig;
use crate::level::{Decoration, LevelManager};
use crate::procgen::{BiomeId, DifficultyPreset, ProcgenManager};
use octoplat_core::save::{EndlessRun, SaveManager};
use octoplat_core::state::LivesManager;

use super::RogueliteRun;

/// Result of level generation including visual data
pub struct GeneratedLevelData {
    pub seed: u64,
    pub decorations: Vec<Decoration>,
}

/// Start a new roguelite run locked to a specific biome
pub fn start_biome_challenge(
    run: &mut RogueliteRun,
    lives: &mut LivesManager,
    config: &GameConfig,
    biome: BiomeId,
    preset: DifficultyPreset,
    seed: Option<u64>,
) {
    run.start_biome_challenge(biome, preset, seed);
    lives.start_session(config.starting_lives, config.endless_gem_milestone, true);
}

/// Generate next level for roguelite mode using linked segments
///
/// Always uses linked segment generation for variety and consistent experience.
///
/// Returns the generated level data including seed and decorations.
pub fn generate_linked_level(
    run: &RogueliteRun,
    procgen: &mut ProcgenManager,
    level_manager: &mut LevelManager,
    procgen_seed: &mut Option<u64>,
) -> Result<GeneratedLevelData, String> {
    // Get current biome from progression
    let current_biome = run.biome_progression.current_id();

    // Determine seed: use specified seed for first level, generate for subsequent
    let seed = if run.level_count == 0 {
        run.start_seed.unwrap_or_else(|| {
            use macroquad::prelude::get_time;
            (get_time() * 1000000.0) as u64
        })
    } else {
        use macroquad::prelude::get_time;
        (get_time() * 1000000.0) as u64
    };

    // Determine segment count based on difficulty and progression
    let segment_count = match run.preset {
        DifficultyPreset::Casual => {
            if run.level_count < 5 { 2 } else { 3 }
        }
        DifficultyPreset::Standard => {
            if run.level_count < 5 { 2 }
            else if run.level_count < 10 { 3 }
            else { 4 }
        }
        DifficultyPreset::Challenge => {
            if run.level_count < 3 { 3 }
            else if run.level_count < 8 { 4 }
            else { 5 }
        }
    };

    // Generate linked level
    let generated = procgen.generate_linked_level_with_retry(
        current_biome,
        run.preset,
        run.level_count,
        seed,
        segment_count,
    )?;

    let biome_def = current_biome.definition();
    #[cfg(debug_assertions)]
    println!(
        "RogueLite level {} generated in {} ({} segments, seed: {}, {} decorations)",
        run.level_count + 1,
        biome_def.name,
        segment_count,
        generated.seed,
        generated.decorations.len(),
    );
    let _ = biome_def;
    *procgen_seed = Some(generated.seed);
    level_manager.load_generated(&generated.name, &generated.map_data, generated.seed)?;
    Ok(GeneratedLevelData {
        seed: generated.seed,
        decorations: generated.decorations,
    })
}

/// Generate next level for roguelite mode (legacy - uses single archetype)
///
/// Kept for backward compatibility but generate_linked_level is preferred.
#[deprecated(since = "0.2.0", note = "Use generate_linked_level instead")]
pub fn generate_level(
    run: &RogueliteRun,
    procgen: &mut ProcgenManager,
    level_manager: &mut LevelManager,
    procgen_seed: &mut Option<u64>,
) -> Result<GeneratedLevelData, String> {
    // Delegate to linked level generation for consistency
    generate_linked_level(run, procgen, level_manager, procgen_seed)
}

/// Complete current level and advance progression
///
/// Returns true if biome advanced.
pub fn complete_level(run: &mut RogueliteRun, gems_collected: u32) -> bool {
    run.total_gems += gems_collected;
    run.level_count += 1;

    // Advance biome progression
    let advanced_biome = run.biome_progression.advance_level();
    let current_biome = run.biome_progression.current();

    #[cfg(debug_assertions)]
    if advanced_biome {
        println!(
            "RogueLite level {} complete! Entering biome: {} | Total gems: {}",
            run.level_count, current_biome.name, run.total_gems
        );
    } else {
        println!(
            "RogueLite level {} complete! Biome: {} | Total gems: {}",
            run.level_count, current_biome.name, run.total_gems
        );
    }

    advanced_biome
}

/// Record the run to save data
pub fn record_run(run: &RogueliteRun, save_manager: &mut SaveManager) {
    if run.level_count > 0 || run.total_gems > 0 {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let endless_run = EndlessRun {
            seed: run.start_seed.unwrap_or(0),
            levels_completed: run.level_count,
            gems_collected: run.total_gems,
            deaths: run.run_deaths,
            time: run.run_time,
            timestamp,
        };

        save_manager.data_mut().record_endless_run(endless_run);
        if let Err(e) = save_manager.save() {
            #[cfg(debug_assertions)]
            eprintln!("Failed to save roguelite run: {}", e);
            let _ = e;
        }

        #[cfg(debug_assertions)]
        println!(
            "RogueLite run recorded: {} levels, {} gems, {} deaths, {:.1}s",
            run.level_count, run.total_gems, run.run_deaths, run.run_time
        );
    }
}

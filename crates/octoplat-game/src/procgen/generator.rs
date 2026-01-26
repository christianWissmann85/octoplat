//! Procedural generation manager
//!
//! Main orchestration for level generation using archetype pools and segment linking.

use octoplat_core::procgen::{
    LevelValidator,
    BiomeId,
    ArchetypePool, ArchetypeSequencer, LevelArchetype, PooledLevel,
};
use octoplat_core::Rng;
use octoplat_core::level::{LevelData, generate_decorations_for_tilemap};
use octoplat_core::DEFAULT_TILE_SIZE;
use octoplat_core::state::DifficultyPreset;
use super::segment_linker::{SegmentLinker, SegmentLinkerConfig, select_segments, select_layout_strategy};
use super::debug_export::{export_debug_level, export_debug_segments};
use super::difficulty::DifficultyParams;
use super::ProcgenError;
use crate::level::Decoration;
use rust_embed::RustEmbed;

/// Embedded roguelite level assets
#[derive(RustEmbed)]
#[folder = "../../assets/roguelite"]
struct RogueliteAssets;

/// Result of level generation
pub struct GeneratedLevel {
    /// Full tilemap string (compatible with TileMap::from_string)
    pub map_data: String,
    /// Level metadata
    pub name: String,
    /// Seed used for generation
    pub seed: u64,
    /// Visual decorations for the level
    pub decorations: Vec<Decoration>,
}

/// Maximum retry attempts for generating a valid level
/// Generation is fast, so we can afford many retries to find a valid level
const MAX_GENERATION_RETRIES: u32 = 50;

/// Main procedural generation manager
pub struct ProcgenManager {
    /// Level validator for checking reachability
    validator: LevelValidator,
    /// Pool of handcrafted levels for archetype-based generation
    archetype_pool: Option<ArchetypePool>,
    /// Archetype sequencer for pacing level archetypes
    archetype_sequencer: Option<ArchetypeSequencer>,
}

impl ProcgenManager {
    pub fn new() -> Self {
        Self {
            validator: LevelValidator::new(),
            archetype_pool: None,
            archetype_sequencer: None,
        }
    }

    // ========================================================================
    // Archetype-based generation (handcrafted level pools)
    // ========================================================================

    /// Load handcrafted levels from embedded roguelite assets
    ///
    /// Loads segments from `assets/roguelite/ocean_depths/<archetype>/` and registers
    /// them for ALL biomes. Visual theming comes from the biome theme, not the segments.
    /// Returns the number of levels loaded.
    pub fn load_archetype_pool(&mut self, _assets_path: &str) -> Result<usize, ProcgenError> {
        let mut pool = ArchetypePool::new();

        // All biomes use the same segment pool (visual theming comes from BiomeTheme)
        let all_biomes = BiomeId::all();

        // Iterate through all embedded files
        for file_path in RogueliteAssets::iter() {
            // Only process .txt files from ocean_depths (our canonical segment source)
            if !file_path.ends_with(".txt") || !file_path.starts_with("ocean_depths/") {
                continue;
            }

            // Parse the path: ocean_depths/<archetype>/<filename>.txt
            let path_parts: Vec<&str> = file_path.split('/').collect();
            if path_parts.len() != 3 {
                continue;
            }

            let archetype_str = path_parts[1];
            let filename = path_parts[2];

            // Parse archetype from path
            let archetype = match LevelArchetype::parse(archetype_str) {
                Some(a) => a,
                None => continue,
            };

            // Get the embedded file content
            if let Some(file) = RogueliteAssets::get(&file_path) {
                if let Ok(content) = std::str::from_utf8(file.data.as_ref()) {
                    if let Ok(level_data) = LevelData::parse(content, 32.0) {
                        let base_id = filename
                            .strip_suffix(".txt")
                            .unwrap_or("unknown")
                            .to_string();

                        // Register this segment for ALL biomes
                        for &biome in all_biomes {
                            let pooled = PooledLevel {
                                content: content.to_string(),
                                archetype: level_data.archetype.unwrap_or(archetype),
                                biome,
                                difficulty_tier: level_data.difficulty_tier.unwrap_or(2),
                                id: format!("{}_{}", biome.as_str(), base_id),
                                name: level_data.name.clone(),
                            };

                            pool.add_level(pooled);
                        }
                    }
                }
            }
        }

        let count = pool.level_count();
        #[cfg(debug_assertions)]
        println!("Loaded {} levels into archetype pool ({} segments x {} biomes)",
                 count, count / all_biomes.len().max(1), all_biomes.len());
        self.archetype_pool = Some(pool);
        Ok(count)
    }

    /// Initialize the archetype sequencer for a new run
    pub fn init_archetype_sequencer(&mut self, seed: u64) {
        self.archetype_sequencer = Some(ArchetypeSequencer::new(seed));
        if let Some(pool) = &mut self.archetype_pool {
            pool.clear_recently_used();
        }
    }

    /// Check if the archetype pool has levels available
    pub fn has_archetype_pool(&self) -> bool {
        self.archetype_pool.as_ref().map(|p| !p.is_empty()).unwrap_or(false)
    }

    /// Generate a level from the handcrafted archetype pool
    ///
    /// Falls back to hybrid generation if pool doesn't have a suitable level.
    pub fn generate_archetype_level(
        &mut self,
        biome: BiomeId,
        preset: DifficultyPreset,
        level_index: u32,
        is_boss_level: bool,
        seed: u64,
    ) -> Result<GeneratedLevel, ProcgenError> {
        // Ensure sequencer is initialized
        if self.archetype_sequencer.is_none() {
            self.init_archetype_sequencer(seed);
        }

        let pool = match &self.archetype_pool {
            Some(p) if !p.is_empty() => p,
            _ => return Err(ProcgenError::PoolNotLoaded),
        };

        // Get difficulty params for tier range
        let progress = (level_index as f32) / 20.0; // Assume ~20 levels per run
        let difficulty = DifficultyParams::for_progress(progress.min(1.0), preset);

        // Get available archetypes for this biome
        let available = pool.available_archetypes(biome);
        if available.is_empty() {
            return Err(ProcgenError::NoLevelsForBiome { biome });
        }

        // Select archetype using sequencer
        let sequencer = self.archetype_sequencer.as_mut()
            .ok_or(ProcgenError::SequencerNotInitialized)?;
        let archetype = sequencer
            .select_archetype(&available, level_index, is_boss_level)
            .ok_or(ProcgenError::ArchetypeSelectionFailed)?;

        // Get candidate levels from pool
        let candidates = pool.get_levels(biome, archetype, difficulty.min_tier, difficulty.max_tier);

        // If no exact match, try any level in biome
        let candidates = if candidates.is_empty() {
            pool.get_any_level_for_biome(biome, difficulty.min_tier, difficulty.max_tier)
        } else {
            candidates
        };

        if candidates.is_empty() {
            return Err(ProcgenError::NoMatchingLevels {
                biome,
                archetype: Some(archetype),
                min_tier: difficulty.min_tier,
                max_tier: difficulty.max_tier,
            });
        }

        // Select a random level from candidates
        let mut rng = Rng::new(seed.wrapping_add(level_index as u64));
        let level_idx = rng.range_usize(0, candidates.len());
        let selected = candidates[level_idx];

        // Mark as used (need mutable access) - clone values before mutable borrow
        let selected_id = selected.id.clone();
        let selected_content = selected.content.clone();
        #[cfg(debug_assertions)]
        let selected_name = selected.name.clone();
        #[cfg(debug_assertions)]
        let selected_archetype = selected.archetype;

        if let Some(pool) = &mut self.archetype_pool {
            pool.mark_used(&selected_id);
        }

        // Apply difficulty scaling to the level content
        let scaled_content = self.apply_difficulty_scaling(&selected_content, &difficulty, seed);

        let biome_def = biome.definition();
        let preset_name = match preset {
            DifficultyPreset::Casual => "Casual",
            DifficultyPreset::Standard => "Standard",
            DifficultyPreset::Challenge => "Challenge",
        };

        #[cfg(debug_assertions)]
        println!(
            "Archetype level selected: {} ({:?}) in {} (tier range {}-{})",
            selected_name, selected_archetype, biome_def.name, difficulty.min_tier, difficulty.max_tier
        );

        // Generate decorations for the archetype level
        // Extract tilemap portion (after the --- separator)
        let tilemap_data = if let Some(separator_pos) = scaled_content.find("\n---\n") {
            &scaled_content[separator_pos + 5..]
        } else if scaled_content.starts_with("---") {
            &scaled_content[4..]
        } else {
            // No header, treat entire content as tilemap
            &scaled_content
        };
        // Trim leading empty lines so the first line determines width correctly
        let tilemap_data = tilemap_data.trim_start_matches('\n');
        let decorations = generate_decorations_for_tilemap(tilemap_data, biome, seed, DEFAULT_TILE_SIZE);

        #[cfg(debug_assertions)]
        eprintln!(
            "[Decorations] Archetype generator: {} decorations for {:?}",
            decorations.len(),
            biome
        );

        Ok(GeneratedLevel {
            map_data: scaled_content,
            name: format!("{} {} #{}", biome_def.name, preset_name, seed % 10000),
            seed,
            decorations,
        })
    }

    /// Apply difficulty scaling to level content
    ///
    /// Replaces variable markers based on difficulty params:
    /// - '?' = collectible slot (gem based on difficulty)
    /// - '%' = enemy slot (enemy type/spawn based on difficulty)
    /// - '$' = hazard slot (spike based on difficulty)
    /// - '~' = grapple slot (grapple point based on difficulty)
    fn apply_difficulty_scaling(&mut self, content: &str, difficulty: &DifficultyParams, seed: u64) -> String {
        let mut rng = Rng::new(seed);
        let mut result = String::with_capacity(content.len());

        // Find the map data section (after ---)
        let parts: Vec<&str> = content.splitn(2, "---").collect();
        if parts.len() == 2 {
            // Keep header as-is
            result.push_str(parts[0]);
            result.push_str("---");

            // Process map data
            for ch in parts[1].chars() {
                let replacement = match ch {
                    // Collectible slot: gem or empty
                    '?' => {
                        if rng.next_float() < difficulty.collectible_chance {
                            '*' // Gem
                        } else {
                            ' ' // Empty
                        }
                    }
                    // Enemy slot: enemy based on difficulty or empty
                    '%' => {
                        if rng.next_float() < difficulty.enemy_chance {
                            if rng.next_float() < difficulty.pufferfish_chance {
                                'O' // Pufferfish (harder)
                            } else {
                                'C' // Crab (easier)
                            }
                        } else {
                            ' ' // No enemy
                        }
                    }
                    // Hazard slot: spike or empty
                    '$' => {
                        if rng.next_float() < difficulty.hazard_chance {
                            '^' // Spike
                        } else {
                            ' ' // Empty
                        }
                    }
                    // Grapple slot: grapple point or empty
                    '~' => {
                        if rng.next_float() < difficulty.grapple_chance {
                            '@' // Grapple point
                        } else {
                            ' ' // Empty
                        }
                    }
                    // Keep other characters unchanged
                    other => other,
                };
                result.push(replacement);
            }
        } else {
            // No header section, process entire content
            for ch in content.chars() {
                let replacement = match ch {
                    '?' => {
                        if rng.next_float() < difficulty.collectible_chance {
                            '*'
                        } else {
                            ' '
                        }
                    }
                    '%' => {
                        if rng.next_float() < difficulty.enemy_chance {
                            if rng.next_float() < difficulty.pufferfish_chance {
                                'O'
                            } else {
                                'C'
                            }
                        } else {
                            ' '
                        }
                    }
                    '$' => {
                        if rng.next_float() < difficulty.hazard_chance {
                            '^'
                        } else {
                            ' '
                        }
                    }
                    '~' => {
                        if rng.next_float() < difficulty.grapple_chance {
                            '@'
                        } else {
                            ' '
                        }
                    }
                    other => other,
                };
                result.push(replacement);
            }
        }

        result
    }

    /// Generate a linked level by combining multiple archetype segments
    ///
    /// This creates larger, varied levels by stitching together hand-crafted
    /// archetype levels as segments connected by corridors.
    pub fn generate_linked_level(
        &mut self,
        biome: BiomeId,
        preset: DifficultyPreset,
        level_index: u32,
        seed: u64,
        segment_count: usize,
    ) -> Result<GeneratedLevel, ProcgenError> {
        let pool = match &self.archetype_pool {
            Some(p) if !p.is_empty() => p,
            _ => return Err(ProcgenError::PoolNotLoaded),
        };

        // Get difficulty params for tier range
        let progress = (level_index as f32) / 20.0;
        let difficulty = DifficultyParams::for_progress(progress.min(1.0), preset);

        // Get all levels for this biome
        let all_levels: Vec<&PooledLevel> = pool.get_all_for_biome(biome);
        if all_levels.is_empty() {
            return Err(ProcgenError::NoLevelsForBiome { biome });
        }

        // Select segments with variety and difficulty progression
        let segments = select_segments(
            &all_levels,
            biome,
            segment_count,
            difficulty.min_tier,
            difficulty.max_tier,
            seed,
        );

        if segments.is_empty() {
            return Err(ProcgenError::SegmentSelectionFailed {
                biome,
                min_tier: difficulty.min_tier,
                max_tier: difficulty.max_tier,
            });
        }

        // Select layout strategy based on progression and difficulty
        let layout = select_layout_strategy(level_index, preset, seed);

        // Configure and run segment linker
        let config = SegmentLinkerConfig {
            seed,
            biome,
            preset,
            segment_count,
            corridor_width: 6,
            corridor_height: 5,
            layout,
        };

        let mut linker = SegmentLinker::new(config);
        let result = linker.link(&segments);

        if !result.success {
            return Err(ProcgenError::LinkingFailed);
        }

        // Debug export: save individual segments before linking
        let segment_data: Vec<(String, String)> = segments
            .iter()
            .map(|s| (s.name.clone(), s.content.clone()))
            .collect();
        export_debug_segments(&segment_data, biome, seed);

        // Debug export: save the linked level
        export_debug_level(
            &result.tilemap,
            biome,
            preset,
            layout,
            &result.segment_names,
            seed,
            level_index,
            result.width,
            result.height,
        );

        // Apply difficulty scaling to the combined level
        let scaled_content = self.apply_difficulty_scaling(&result.tilemap, &difficulty, seed);

        // Validate the linked level
        let tiles: Vec<Vec<char>> = scaled_content
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let validation = self.validator.validate_detailed(&tiles);

        if !validation.is_completable {
            return Err(ProcgenError::ValidationFailed {
                issues: validation.issues,
            });
        }

        let biome_def = biome.definition();
        let preset_name = match preset {
            DifficultyPreset::Casual => "Casual",
            DifficultyPreset::Standard => "Standard",
            DifficultyPreset::Challenge => "Challenge",
        };

        #[cfg(debug_assertions)]
        println!(
            "Linked level generated: {} segments ({}) in {} ({}x{}) using {:?} layout",
            result.segment_names.len(),
            result.segment_names.join(" → "),
            biome_def.name,
            result.width,
            result.height,
            result.layout
        );

        // Generate decorations
        let decorations = generate_decorations_for_tilemap(&scaled_content, biome, seed, DEFAULT_TILE_SIZE);

        #[cfg(debug_assertions)]
        eprintln!(
            "[Decorations] Segment linker: {} decorations for {:?}",
            decorations.len(),
            biome
        );

        Ok(GeneratedLevel {
            map_data: scaled_content,
            name: format!("{} {} #{}", biome_def.name, preset_name, seed % 10000),
            seed,
            decorations,
        })
    }

    /// Generate a linked level with retry logic
    pub fn generate_linked_level_with_retry(
        &mut self,
        biome: BiomeId,
        preset: DifficultyPreset,
        level_index: u32,
        seed: u64,
        segment_count: usize,
    ) -> Result<GeneratedLevel, ProcgenError> {
        #[cfg(debug_assertions)]
        eprintln!(
            "[Procgen] Starting generation: biome={:?}, preset={:?}, level={}, segments={}, seed={}",
            biome, preset, level_index, segment_count, seed
        );

        for attempt in 0..MAX_GENERATION_RETRIES {
            let try_seed = seed.wrapping_add(attempt as u64 * 12345);

            match self.generate_linked_level(biome, preset, level_index, try_seed, segment_count) {
                Ok(level) => {
                    // Only log if it took multiple attempts
                    if attempt > 0 {
                        #[cfg(debug_assertions)]
                        println!("[Procgen] Success after {} attempts", attempt + 1);
                    }
                    return Ok(level);
                }
                Err(_e) => {
                    // Log sparingly: every 10 attempts or on specific milestones
                    #[cfg(debug_assertions)]
                    if attempt == 9 || attempt == 24 || attempt == 49 {
                        eprintln!("[Procgen] {} attempts failed so far, continuing...", attempt + 1);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        eprintln!(
            "[Procgen] All {} attempts exhausted for biome={:?}, preset={:?}",
            MAX_GENERATION_RETRIES, biome, preset
        );

        Err(ProcgenError::RetriesExhausted {
            attempts: MAX_GENERATION_RETRIES,
        })
    }

    /// Generate a roguelite level using linked segments for large, varied levels
    pub fn generate_roguelite_level(
        &mut self,
        biome: BiomeId,
        preset: DifficultyPreset,
        level_index: u32,
        _is_boss_level: bool,
        seed: u64,
    ) -> Result<GeneratedLevel, ProcgenError> {
        // Always use linked segments for multi-segment levels (6-24 segments)
        // Scale segment count with level progression for longer, more interesting levels
        let segment_count = match preset {
            DifficultyPreset::Casual => 6 + (level_index as usize / 3).min(6),      // 6-12
            DifficultyPreset::Standard => 10 + (level_index as usize / 2).min(10),  // 10-20
            DifficultyPreset::Challenge => 14 + (level_index as usize).min(10),     // 14-24
        };

        self.generate_linked_level_with_retry(biome, preset, level_index, seed, segment_count)
    }
}

impl Default for ProcgenManager {
    fn default() -> Self {
        Self::new()
    }
}

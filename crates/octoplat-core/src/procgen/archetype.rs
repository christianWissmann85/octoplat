//! Level archetype system for roguelite mode
//!
//! Archetypes define the fundamental gameplay structure of a level,
//! allowing curated level pools organized by gameplay pattern.

use std::collections::{HashMap, VecDeque};

use super::BiomeId;

/// Level archetypes that define fundamental gameplay patterns
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LevelArchetype {
    /// Vertical climbing challenge - ascend through platforms
    TheAscent,
    /// Horizontal sprint challenge - move quickly through hazards
    TheGauntlet,
    /// Controlled descent - navigate downward carefully
    TheDepths,
    /// Branching exploration - multiple paths to exit
    TheMaze,
    /// Combat-focused arena - enemy encounters
    TheArena,
    /// Grapple/swing heavy - traversal focused
    TheCrossing,
}

impl LevelArchetype {
    /// Get all archetypes
    pub fn all() -> &'static [LevelArchetype] {
        &[
            LevelArchetype::TheAscent,
            LevelArchetype::TheGauntlet,
            LevelArchetype::TheDepths,
            LevelArchetype::TheMaze,
            LevelArchetype::TheArena,
            LevelArchetype::TheCrossing,
        ]
    }

    /// Get archetypes suitable for the first level of a run
    pub fn starting_archetypes() -> &'static [LevelArchetype] {
        &[
            LevelArchetype::TheGauntlet,
            LevelArchetype::TheMaze,
            LevelArchetype::TheAscent,
        ]
    }

    /// Get the string identifier for this archetype (for level files)
    pub fn as_str(&self) -> &'static str {
        match self {
            LevelArchetype::TheAscent => "ascent",
            LevelArchetype::TheGauntlet => "gauntlet",
            LevelArchetype::TheDepths => "depths",
            LevelArchetype::TheMaze => "maze",
            LevelArchetype::TheArena => "arena",
            LevelArchetype::TheCrossing => "crossing",
        }
    }

    /// Parse an archetype from a string identifier
    pub fn parse(s: &str) -> Option<LevelArchetype> {
        match s.to_lowercase().trim() {
            "ascent" | "the_ascent" | "theascent" => Some(LevelArchetype::TheAscent),
            "gauntlet" | "the_gauntlet" | "thegauntlet" => Some(LevelArchetype::TheGauntlet),
            "depths" | "the_depths" | "thedepths" => Some(LevelArchetype::TheDepths),
            "maze" | "the_maze" | "themaze" => Some(LevelArchetype::TheMaze),
            "arena" | "the_arena" | "thearena" => Some(LevelArchetype::TheArena),
            "crossing" | "the_crossing" | "thecrossing" => Some(LevelArchetype::TheCrossing),
            _ => None,
        }
    }

    /// Get the display name for this archetype
    pub fn display_name(&self) -> &'static str {
        match self {
            LevelArchetype::TheAscent => "The Ascent",
            LevelArchetype::TheGauntlet => "The Gauntlet",
            LevelArchetype::TheDepths => "The Depths",
            LevelArchetype::TheMaze => "The Maze",
            LevelArchetype::TheArena => "The Arena",
            LevelArchetype::TheCrossing => "The Crossing",
        }
    }

    /// Check if this archetype should avoid following the given archetype (for pacing)
    pub fn should_avoid_after(&self, previous: LevelArchetype) -> bool {
        // Never repeat same archetype
        if *self == previous {
            return true;
        }

        // TheDepths should not follow TheAscent (jarring vertical reversal)
        if *self == LevelArchetype::TheDepths && previous == LevelArchetype::TheAscent {
            return true;
        }

        // TheAscent should not follow TheDepths (same reason)
        if *self == LevelArchetype::TheAscent && previous == LevelArchetype::TheDepths {
            return true;
        }

        false
    }
}

/// A level loaded from the handcrafted pool
#[derive(Clone, Debug)]
pub struct PooledLevel {
    /// Raw file content
    pub content: String,
    /// Level archetype
    pub archetype: LevelArchetype,
    /// Biome this level belongs to
    pub biome: BiomeId,
    /// Difficulty tier (1-5)
    pub difficulty_tier: u8,
    /// Unique identifier (filename without extension)
    pub id: String,
    /// Level name from header
    pub name: String,
}

/// Pool of handcrafted levels organized by biome and archetype
#[derive(Clone, Debug, Default)]
pub struct ArchetypePool {
    /// Levels indexed by (biome, archetype)
    levels: HashMap<(BiomeId, LevelArchetype), Vec<PooledLevel>>,
    /// Recently used level IDs to avoid repetition
    recently_used: VecDeque<String>,
    /// Maximum size of recently_used queue
    max_recent: usize,
}

impl ArchetypePool {
    /// Create a new empty pool
    pub fn new() -> Self {
        Self {
            levels: HashMap::new(),
            recently_used: VecDeque::new(),
            max_recent: 10,
        }
    }

    /// Add a level to the pool
    pub fn add_level(&mut self, level: PooledLevel) {
        let key = (level.biome, level.archetype);
        self.levels.entry(key).or_default().push(level);
    }

    /// Get the number of levels in the pool
    pub fn level_count(&self) -> usize {
        self.levels.values().map(|v| v.len()).sum()
    }

    /// Check if the pool has any levels
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    /// Get levels matching criteria
    pub fn get_levels(
        &self,
        biome: BiomeId,
        archetype: LevelArchetype,
        min_tier: u8,
        max_tier: u8,
    ) -> Vec<&PooledLevel> {
        let key = (biome, archetype);
        self.levels
            .get(&key)
            .map(|levels| {
                levels
                    .iter()
                    .filter(|l| {
                        l.difficulty_tier >= min_tier
                            && l.difficulty_tier <= max_tier
                            && !self.recently_used.contains(&l.id)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get any level for a biome (fallback when archetype not available)
    pub fn get_any_level_for_biome(
        &self,
        biome: BiomeId,
        min_tier: u8,
        max_tier: u8,
    ) -> Vec<&PooledLevel> {
        let mut result = Vec::new();
        for archetype in LevelArchetype::all() {
            result.extend(self.get_levels(biome, *archetype, min_tier, max_tier));
        }
        result
    }

    /// Get all levels for a biome (regardless of tier or recently used)
    ///
    /// Used by the segment linker to select from the full pool.
    pub fn get_all_for_biome(&self, biome: BiomeId) -> Vec<&PooledLevel> {
        let mut result = Vec::new();
        for archetype in LevelArchetype::all() {
            let key = (biome, *archetype);
            if let Some(levels) = self.levels.get(&key) {
                result.extend(levels.iter());
            }
        }
        result
    }

    /// Mark a level as recently used
    pub fn mark_used(&mut self, level_id: &str) {
        // Remove if already in queue
        self.recently_used.retain(|id| id != level_id);

        // Add to front
        self.recently_used.push_front(level_id.to_string());

        // Trim to max size
        while self.recently_used.len() > self.max_recent {
            self.recently_used.pop_back();
        }
    }

    /// Clear the recently used queue (for new runs)
    pub fn clear_recently_used(&mut self) {
        self.recently_used.clear();
    }

    /// Get available archetypes for a biome
    pub fn available_archetypes(&self, biome: BiomeId) -> Vec<LevelArchetype> {
        LevelArchetype::all()
            .iter()
            .filter(|arch| {
                let key = (biome, **arch);
                self.levels.get(&key).map(|v| !v.is_empty()).unwrap_or(false)
            })
            .copied()
            .collect()
    }
}

// Re-export Rng from the crate root for backwards compatibility
pub use crate::Rng as SimpleRng;

/// Archetype sequencer that manages level flow and pacing
#[derive(Clone, Debug)]
pub struct ArchetypeSequencer {
    /// History of archetypes used in the current run
    history: Vec<LevelArchetype>,
    /// Random number generator
    rng: SimpleRng,
}

impl ArchetypeSequencer {
    /// Create a new sequencer with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            history: Vec::new(),
            rng: SimpleRng::new(seed),
        }
    }

    /// Reset for a new run
    pub fn reset(&mut self, seed: u64) {
        self.history.clear();
        self.rng = SimpleRng::new(seed);
    }

    /// Select the next archetype based on sequencing rules
    ///
    /// Arguments:
    /// - `available`: Archetypes that have levels in the pool
    /// - `level_index`: Current level number in the run (0-indexed)
    /// - `is_boss_level`: Whether this is a biome boss level
    pub fn select_archetype(
        &mut self,
        available: &[LevelArchetype],
        level_index: u32,
        is_boss_level: bool,
    ) -> Option<LevelArchetype> {
        if available.is_empty() {
            return None;
        }

        // Boss levels prefer TheArena
        if is_boss_level && available.contains(&LevelArchetype::TheArena) {
            let choice = LevelArchetype::TheArena;
            self.history.push(choice);
            return Some(choice);
        }

        // First level uses starting archetypes
        if level_index == 0 {
            let starting: Vec<_> = LevelArchetype::starting_archetypes()
                .iter()
                .filter(|a| available.contains(a))
                .copied()
                .collect();

            if !starting.is_empty() {
                let choice = *self.rng.choose(&starting)?;
                self.history.push(choice);
                return Some(choice);
            }
        }

        // Build weighted selection based on history
        let mut weighted: Vec<(LevelArchetype, f32)> = Vec::new();
        let last_archetype = self.history.last().copied();

        for arch in available {
            // Skip if should avoid after previous
            if let Some(prev) = last_archetype {
                if arch.should_avoid_after(prev) {
                    continue;
                }
            }

            // Calculate weight based on recency
            let recency_count = self.history.iter().rev().take(5).filter(|a| *a == arch).count();
            let weight = match recency_count {
                0 => 3.0, // Not used recently - high weight
                1 => 1.5, // Used once - medium weight
                2 => 0.5, // Used twice - low weight
                _ => 0.1, // Used more - very low weight
            };

            weighted.push((*arch, weight));
        }

        // If all filtered out, fall back to any available
        if weighted.is_empty() {
            let choice = *self.rng.choose(available)?;
            self.history.push(choice);
            return Some(choice);
        }

        let choice = *self.rng.weighted_choose(&weighted)?;
        self.history.push(choice);
        Some(choice)
    }

    /// Get the history of archetypes used
    pub fn history(&self) -> &[LevelArchetype] {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_parse() {
        assert_eq!(LevelArchetype::parse("ascent"), Some(LevelArchetype::TheAscent));
        assert_eq!(LevelArchetype::parse("THE_GAUNTLET"), Some(LevelArchetype::TheGauntlet));
        assert_eq!(LevelArchetype::parse("maze"), Some(LevelArchetype::TheMaze));
        assert_eq!(LevelArchetype::parse("invalid"), None);
    }

    #[test]
    fn test_archetype_sequencing() {
        let mut sequencer = ArchetypeSequencer::new(12345);
        let available = LevelArchetype::all().to_vec();

        // First level should be from starting archetypes
        let first = sequencer.select_archetype(&available, 0, false).unwrap();
        assert!(LevelArchetype::starting_archetypes().contains(&first));

        // Boss level should prefer arena
        let boss = sequencer.select_archetype(&available, 3, true).unwrap();
        assert_eq!(boss, LevelArchetype::TheArena);
    }

    #[test]
    fn test_archetype_pool() {
        let mut pool = ArchetypePool::new();

        let level = PooledLevel {
            content: "test".to_string(),
            archetype: LevelArchetype::TheGauntlet,
            biome: BiomeId::OceanDepths,
            difficulty_tier: 2,
            id: "test_level_01".to_string(),
            name: "Test Level".to_string(),
        };

        pool.add_level(level);
        assert_eq!(pool.level_count(), 1);

        let levels = pool.get_levels(BiomeId::OceanDepths, LevelArchetype::TheGauntlet, 1, 3);
        assert_eq!(levels.len(), 1);

        // Mark as used
        pool.mark_used("test_level_01");
        let levels_after = pool.get_levels(BiomeId::OceanDepths, LevelArchetype::TheGauntlet, 1, 3);
        assert_eq!(levels_after.len(), 0); // Excluded due to recently used
    }

    #[test]
    fn test_should_avoid_after() {
        assert!(LevelArchetype::TheDepths.should_avoid_after(LevelArchetype::TheAscent));
        assert!(LevelArchetype::TheAscent.should_avoid_after(LevelArchetype::TheDepths));
        assert!(LevelArchetype::TheGauntlet.should_avoid_after(LevelArchetype::TheGauntlet));
        assert!(!LevelArchetype::TheGauntlet.should_avoid_after(LevelArchetype::TheMaze));
    }
}

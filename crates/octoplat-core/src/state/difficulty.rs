//! Difficulty presets for procedural generation and gameplay

/// Difficulty preset for procedural runs (affects level generation)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DifficultyPreset {
    /// Relaxed gameplay, fewer enemies/hazards, more grapple points
    Casual,
    /// Balanced experience with gradual ramp
    #[default]
    Standard,
    /// Intense run with high enemy/hazard density
    Challenge,
}

impl DifficultyPreset {
    /// Get display name for the preset
    pub fn name(&self) -> &'static str {
        match self {
            DifficultyPreset::Casual => "Casual",
            DifficultyPreset::Standard => "Standard",
            DifficultyPreset::Challenge => "Challenge",
        }
    }
}

/// Gameplay difficulty tier (affects HP, i-frames, enemy speed, starting lives)
///
/// This is separate from DifficultyPreset which affects procedural generation.
/// GameplayDifficulty affects the combat/survival mechanics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GameplayDifficulty {
    /// Drifting - easiest mode with high HP and slow enemies
    Drifting,
    /// Treading Water - balanced/default experience
    #[default]
    TreadingWater,
    /// OctoHard - challenging with lower HP
    OctoHard,
    /// The Kraken - hardest mode, one-hit death
    TheKraken,
}

impl GameplayDifficulty {
    /// All difficulty variants in order
    pub const ALL: [GameplayDifficulty; 4] = [
        GameplayDifficulty::Drifting,
        GameplayDifficulty::TreadingWater,
        GameplayDifficulty::OctoHard,
        GameplayDifficulty::TheKraken,
    ];

    /// Display name for the difficulty
    pub fn name(&self) -> &'static str {
        match self {
            GameplayDifficulty::Drifting => "Drifting",
            GameplayDifficulty::TreadingWater => "Treading Water",
            GameplayDifficulty::OctoHard => "OctoHard",
            GameplayDifficulty::TheKraken => "The Kraken",
        }
    }

    /// Description of the difficulty
    pub fn description(&self) -> &'static str {
        match self {
            GameplayDifficulty::Drifting => "Relaxed - high HP, long i-frames, slow enemies",
            GameplayDifficulty::TreadingWater => "Balanced - standard HP and enemy speed",
            GameplayDifficulty::OctoHard => "Challenging - low HP, short i-frames",
            GameplayDifficulty::TheKraken => "Brutal - one hit death, fast enemies",
        }
    }

    /// Maximum HP for this difficulty
    pub fn max_hp(&self) -> u8 {
        match self {
            GameplayDifficulty::Drifting => 5,
            GameplayDifficulty::TreadingWater => 3,
            GameplayDifficulty::OctoHard => 2,
            GameplayDifficulty::TheKraken => 1,
        }
    }

    /// Invincibility frame duration in seconds
    pub fn invincibility_duration(&self) -> f32 {
        match self {
            GameplayDifficulty::Drifting => 2.0,
            GameplayDifficulty::TreadingWater => 1.0,
            GameplayDifficulty::OctoHard => 0.5,
            GameplayDifficulty::TheKraken => 0.3,
        }
    }

    /// Enemy speed multiplier (1.0 = normal)
    pub fn enemy_speed_multiplier(&self) -> f32 {
        match self {
            GameplayDifficulty::Drifting => 0.7,
            GameplayDifficulty::TreadingWater => 1.0,
            GameplayDifficulty::OctoHard => 1.0,
            GameplayDifficulty::TheKraken => 1.2,
        }
    }

    /// Starting lives for this difficulty
    pub fn starting_lives(&self) -> u32 {
        match self {
            GameplayDifficulty::Drifting => 7,
            GameplayDifficulty::TreadingWater => 5,
            GameplayDifficulty::OctoHard => 4,
            GameplayDifficulty::TheKraken => 3,
        }
    }
}

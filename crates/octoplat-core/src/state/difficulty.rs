//! Difficulty presets for procedural generation

/// Difficulty preset for procedural runs
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

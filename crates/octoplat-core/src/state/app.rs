//! Application state machine for game flow control
//!
//! Manages transitions between menus, gameplay, and other screens.

use crate::procgen::BiomeId;
use super::{DifficultyPreset, GameplayDifficulty};

/// Top-level application state
#[derive(Clone, Debug, PartialEq, Default)]
pub enum AppState {
    /// Title screen with "Press Start"
    #[default]
    Title,
    /// Main menu with game mode selection
    MainMenu,
    /// Active gameplay
    Playing(PlayMode),
    /// Game is paused (overlay on gameplay)
    Paused(PlayMode),
    /// Level completion screen with stats
    LevelComplete,
    /// Player died, showing retry options
    GameOver,
    /// Settings/options menu
    Settings,
    /// RogueLite mode leaderboard
    RogueLiteLeaderboard,
    /// Biome selection for RogueLite mode
    BiomeSelect,
    /// Difficulty selection for RogueLite mode (after biome selection)
    DifficultySelect { biome: BiomeId },
    /// Error screen with message (for generation failures, etc.)
    Error(String),
}

/// Game mode being played - now unified as RogueLite with linked segments
#[derive(Clone, Debug, PartialEq)]
pub enum PlayMode {
    /// RogueLite mode - linked segments procedural generation locked to a specific biome
    RogueLite {
        preset: DifficultyPreset,
        seed: Option<u64>,
        /// The biome this run is locked to
        biome: BiomeId,
    },
}

/// Menu selection for main menu
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainMenuItem {
    RogueLite,
    Settings,
    Quit,
}

impl MainMenuItem {
    pub const ALL: [MainMenuItem; 3] = [
        MainMenuItem::RogueLite,
        MainMenuItem::Settings,
        MainMenuItem::Quit,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            MainMenuItem::RogueLite => "RogueLite",
            MainMenuItem::Settings => "Settings",
            MainMenuItem::Quit => "Quit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MainMenuItem::RogueLite => "Endless procedural levels with linked segments",
            MainMenuItem::Settings => "Audio and display options",
            MainMenuItem::Quit => "Exit to desktop",
        }
    }
}

/// Menu selection for pause menu
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PauseMenuItem {
    Resume,
    Restart,
    Settings,
    QuitToMenu,
}

impl PauseMenuItem {
    pub const ALL: [PauseMenuItem; 4] = [
        PauseMenuItem::Resume,
        PauseMenuItem::Restart,
        PauseMenuItem::Settings,
        PauseMenuItem::QuitToMenu,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            PauseMenuItem::Resume => "Resume",
            PauseMenuItem::Restart => "Restart Level",
            PauseMenuItem::Settings => "Settings",
            PauseMenuItem::QuitToMenu => "Quit to Menu",
        }
    }
}

/// Menu selection for level complete screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LevelCompleteMenuItem {
    NextLevel,
    Replay,
    Menu,
}

impl LevelCompleteMenuItem {
    pub const ALL: [LevelCompleteMenuItem; 3] = [
        LevelCompleteMenuItem::NextLevel,
        LevelCompleteMenuItem::Replay,
        LevelCompleteMenuItem::Menu,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            LevelCompleteMenuItem::NextLevel => "Next Level",
            LevelCompleteMenuItem::Replay => "Replay",
            LevelCompleteMenuItem::Menu => "Main Menu",
        }
    }
}

/// Menu selection for game over screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameOverMenuItem {
    Retry,
    Menu,
}

impl GameOverMenuItem {
    pub const ALL: [GameOverMenuItem; 2] = [
        GameOverMenuItem::Retry,
        GameOverMenuItem::Menu,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            GameOverMenuItem::Retry => "Try Again",
            GameOverMenuItem::Menu => "Main Menu",
        }
    }
}

/// Menu selection for settings screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsMenuItem {
    SfxVolume,
    MusicVolume,
    ScreenShake,
    MinimapSize,
    MinimapZoom,
    MinimapOpacity,
    Back,
}

impl SettingsMenuItem {
    pub const ALL: [SettingsMenuItem; 7] = [
        SettingsMenuItem::SfxVolume,
        SettingsMenuItem::MusicVolume,
        SettingsMenuItem::ScreenShake,
        SettingsMenuItem::MinimapSize,
        SettingsMenuItem::MinimapZoom,
        SettingsMenuItem::MinimapOpacity,
        SettingsMenuItem::Back,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SettingsMenuItem::SfxVolume => "SFX Volume",
            SettingsMenuItem::MusicVolume => "Music Volume",
            SettingsMenuItem::ScreenShake => "Screen Shake",
            SettingsMenuItem::MinimapSize => "Minimap Size",
            SettingsMenuItem::MinimapZoom => "Minimap Zoom",
            SettingsMenuItem::MinimapOpacity => "Minimap Opacity",
            SettingsMenuItem::Back => "Back",
        }
    }
}

/// Menu selection for error screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorMenuItem {
    Retry,
    Menu,
}

impl ErrorMenuItem {
    pub const ALL: [ErrorMenuItem; 2] = [
        ErrorMenuItem::Retry,
        ErrorMenuItem::Menu,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            ErrorMenuItem::Retry => "Try Again",
            ErrorMenuItem::Menu => "Main Menu",
        }
    }
}

/// Menu selection for biome selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BiomeMenuItem {
    OceanDepths,
    CoralReefs,
    TropicalShore,
    Shipwreck,
    ArcticWaters,
    VolcanicVents,
    SunkenRuins,
    Abyss,
    Back,
}

impl BiomeMenuItem {
    pub const ALL: [BiomeMenuItem; 9] = [
        BiomeMenuItem::OceanDepths,
        BiomeMenuItem::CoralReefs,
        BiomeMenuItem::TropicalShore,
        BiomeMenuItem::Shipwreck,
        BiomeMenuItem::ArcticWaters,
        BiomeMenuItem::VolcanicVents,
        BiomeMenuItem::SunkenRuins,
        BiomeMenuItem::Abyss,
        BiomeMenuItem::Back,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            BiomeMenuItem::OceanDepths => "Ocean Depths",
            BiomeMenuItem::CoralReefs => "Coral Reefs",
            BiomeMenuItem::TropicalShore => "Tropical Shore",
            BiomeMenuItem::Shipwreck => "Shipwreck",
            BiomeMenuItem::ArcticWaters => "Arctic Waters",
            BiomeMenuItem::VolcanicVents => "Volcanic Vents",
            BiomeMenuItem::SunkenRuins => "Sunken Ruins",
            BiomeMenuItem::Abyss => "The Abyss",
            BiomeMenuItem::Back => "Back",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BiomeMenuItem::OceanDepths => "Calm deep waters - beginner friendly",
            BiomeMenuItem::CoralReefs => "Colorful reefs with vertical challenges",
            BiomeMenuItem::TropicalShore => "Warm tropical waters with palm trees",
            BiomeMenuItem::Shipwreck => "Dark enclosed spaces in sunken ships",
            BiomeMenuItem::ArcticWaters => "Icy platforms under northern lights",
            BiomeMenuItem::VolcanicVents => "Dangerous volcanic area with timing challenges",
            BiomeMenuItem::SunkenRuins => "Ancient columns with mysterious glow",
            BiomeMenuItem::Abyss => "Maximum challenge in the deepest depths",
            BiomeMenuItem::Back => "Return to mode selection",
        }
    }

    /// Convert menu item to BiomeId (if applicable)
    pub fn to_biome_id(&self) -> Option<BiomeId> {
        match self {
            BiomeMenuItem::OceanDepths => Some(BiomeId::OceanDepths),
            BiomeMenuItem::CoralReefs => Some(BiomeId::CoralReefs),
            BiomeMenuItem::TropicalShore => Some(BiomeId::TropicalShore),
            BiomeMenuItem::Shipwreck => Some(BiomeId::Shipwreck),
            BiomeMenuItem::ArcticWaters => Some(BiomeId::ArcticWaters),
            BiomeMenuItem::VolcanicVents => Some(BiomeId::VolcanicVents),
            BiomeMenuItem::SunkenRuins => Some(BiomeId::SunkenRuins),
            BiomeMenuItem::Abyss => Some(BiomeId::Abyss),
            BiomeMenuItem::Back => None,
        }
    }
}

/// Menu selection for difficulty selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DifficultyMenuItem {
    Drifting,
    TreadingWater,
    OctoHard,
    TheKraken,
    Back,
}

impl DifficultyMenuItem {
    pub const ALL: [DifficultyMenuItem; 5] = [
        DifficultyMenuItem::Drifting,
        DifficultyMenuItem::TreadingWater,
        DifficultyMenuItem::OctoHard,
        DifficultyMenuItem::TheKraken,
        DifficultyMenuItem::Back,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            DifficultyMenuItem::Drifting => "Drifting",
            DifficultyMenuItem::TreadingWater => "Treading Water",
            DifficultyMenuItem::OctoHard => "OctoHard",
            DifficultyMenuItem::TheKraken => "The Kraken",
            DifficultyMenuItem::Back => "Back",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DifficultyMenuItem::Drifting => "5 HP | 2.0s i-frames | 0.7x enemy speed | 7 lives",
            DifficultyMenuItem::TreadingWater => "3 HP | 1.0s i-frames | 1.0x enemy speed | 5 lives",
            DifficultyMenuItem::OctoHard => "2 HP | 0.5s i-frames | 1.0x enemy speed | 4 lives",
            DifficultyMenuItem::TheKraken => "1 HP | 0.3s i-frames | 1.2x enemy speed | 3 lives",
            DifficultyMenuItem::Back => "Return to biome selection",
        }
    }

    /// Convert menu item to GameplayDifficulty (if applicable)
    pub fn to_gameplay_difficulty(&self) -> Option<GameplayDifficulty> {
        match self {
            DifficultyMenuItem::Drifting => Some(GameplayDifficulty::Drifting),
            DifficultyMenuItem::TreadingWater => Some(GameplayDifficulty::TreadingWater),
            DifficultyMenuItem::OctoHard => Some(GameplayDifficulty::OctoHard),
            DifficultyMenuItem::TheKraken => Some(GameplayDifficulty::TheKraken),
            DifficultyMenuItem::Back => None,
        }
    }
}

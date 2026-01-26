//! Audio system with file-based sound effects and music playback
//!
//! Loads sound effects from embedded assets with support for multiple variants
//! per sound type, randomly selecting variants for variety.

pub mod music;
pub mod ambient;

pub use music::{MusicManager, MusicTrack};
pub use ambient::{AmbientManager, AmbientTrack};

use crate::assets::AudioAssets;
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::rand::gen_range;
use std::cell::Cell;
use std::collections::HashMap;

/// Sound effect identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundId {
    // Player actions
    Jump,
    Land,
    WallJump,
    Dive,
    JetBoost,

    // Grapple
    GrappleShoot,
    GrappleAttach,

    // Ink ability
    InkShoot,

    // World interactions
    GemCollect,
    Checkpoint,
    LevelComplete,
    BouncePad,
    ExtraLife,

    // Damage
    PlayerHurt,
    PlayerDeath,

    // UI
    MenuMove,
    MenuSelect,
    MenuBack,
    Pause,
}

impl SoundId {
    /// Get the asset path patterns for this sound
    /// Returns (base_path, extension, variant_count)
    fn asset_info(&self) -> (&'static str, &'static str, usize) {
        match self {
            // Player sounds
            SoundId::Jump => ("sfx/organized/player/jump_", "wav", 3),
            SoundId::Land => ("sfx/organized/player/land_", "ogg", 3),
            SoundId::WallJump => ("sfx/organized/player/wall_jump_", "ogg", 2),
            SoundId::Dive => ("sfx/organized/player/dive_", "ogg", 3),
            SoundId::JetBoost => ("sfx/organized/abilities/jet_boost_sfx_movement_portal", "wav", 2),

            // Abilities
            SoundId::GrappleShoot => ("sfx/organized/abilities/grapple_shoot_sfx_wpn_laser", "wav", 3),
            SoundId::GrappleAttach => ("sfx/organized/abilities/grapple_attach_", "ogg", 2),
            SoundId::InkShoot => ("sfx/organized/abilities/ink_shoot_", "ogg", 3),

            // Collectibles
            SoundId::GemCollect => ("sfx/organized/collectibles/gem_", "wav", 3),
            SoundId::Checkpoint => ("sfx/organized/collectibles/fanfare", "wav", 3),
            SoundId::LevelComplete => ("sfx/organized/collectibles/level_complete_", "ogg", 1),
            SoundId::BouncePad => ("sfx/organized/collectibles/bounce_", "ogg", 2),
            SoundId::ExtraLife => ("sfx/organized/collectibles/fanfare", "wav", 3), // Reuse fanfare

            // Damage
            SoundId::PlayerHurt => ("sfx/organized/damage/hurt_", "wav", 3),
            SoundId::PlayerDeath => ("sfx/organized/damage/death_sfx_exp_short_", "wav", 3),

            // UI
            SoundId::MenuMove => ("sfx/organized/ui/menu_move_", "ogg", 3),
            SoundId::MenuSelect => ("sfx/organized/ui/menu_select_", "ogg", 3),
            SoundId::MenuBack => ("sfx/organized/ui/menu_back_", "ogg", 2),
            SoundId::Pause => ("sfx/organized/ui/pause_", "ogg", 2),
        }
    }

    /// Generate variant paths for this sound
    fn variant_paths(&self) -> Vec<String> {
        let (base, ext, count) = self.asset_info();

        // Handle different naming conventions
        match self {
            SoundId::JetBoost => {
                vec![
                    format!("{}1.{}", base, ext),
                    format!("{}3.{}", base, ext),
                ]
            }
            SoundId::GrappleShoot => {
                vec![
                    format!("{}3.{}", base, ext),
                    format!("{}6.{}", base, ext),
                    format!("{}8.{}", base, ext),
                ]
            }
            SoundId::Checkpoint | SoundId::ExtraLife => {
                (1..=count).map(|i| format!("{}{}.{}", base, i, ext)).collect()
            }
            SoundId::PlayerDeath => {
                vec![
                    "sfx/organized/damage/death_sfx_exp_short_hard4.wav".to_string(),
                    "sfx/organized/damage/death_sfx_exp_short_hard8.wav".to_string(),
                    "sfx/organized/damage/death_sfx_exp_short_soft7.wav".to_string(),
                ]
            }
            _ => {
                // Standard 01, 02, 03 naming
                (1..=count).map(|i| format!("{}{:02}.{}", base, i, ext)).collect()
            }
        }
    }
}

/// Audio manager that holds and plays sounds
pub struct AudioManager {
    /// Sound variants for each sound ID
    sounds: HashMap<SoundId, Vec<Sound>>,
    sfx_volume: Cell<f32>,
    music_volume: Cell<f32>,
}

impl AudioManager {
    /// Create a new audio manager and load all sounds
    pub async fn new() -> Self {
        let mut manager = Self {
            sounds: HashMap::new(),
            sfx_volume: Cell::new(0.7),
            music_volume: Cell::new(0.5),
        };

        // Load all sounds from embedded assets
        manager.load_sounds().await;

        manager
    }

    /// Load all sounds from embedded assets
    async fn load_sounds(&mut self) {
        let sound_ids = [
            SoundId::Jump,
            SoundId::Land,
            SoundId::WallJump,
            SoundId::Dive,
            SoundId::JetBoost,
            SoundId::GrappleShoot,
            SoundId::GrappleAttach,
            SoundId::InkShoot,
            SoundId::GemCollect,
            SoundId::Checkpoint,
            SoundId::LevelComplete,
            SoundId::BouncePad,
            SoundId::ExtraLife,
            SoundId::PlayerHurt,
            SoundId::PlayerDeath,
            SoundId::MenuMove,
            SoundId::MenuSelect,
            SoundId::MenuBack,
            SoundId::Pause,
        ];

        for id in sound_ids {
            self.load_sound_variants(id).await;
        }
    }

    /// Load all variants of a sound
    async fn load_sound_variants(&mut self, id: SoundId) {
        let paths = id.variant_paths();
        let mut variants = Vec::new();

        for path in &paths {
            if let Some(bytes) = AudioAssets::get_audio(path) {
                match load_sound_from_bytes(&bytes).await {
                    Ok(sound) => {
                        variants.push(sound);
                    }
                    Err(e) => {
                        eprintln!("Failed to decode sound {:?} from {}: {}", id, path, e);
                    }
                }
            }
        }

        if variants.is_empty() {
            eprintln!("Warning: No variants loaded for sound {:?}", id);
        }

        self.sounds.insert(id, variants);
    }

    /// Play a sound effect (randomly selects from variants)
    pub fn play(&self, id: SoundId) {
        if let Some(variants) = self.sounds.get(&id) {
            if !variants.is_empty() {
                let index = if variants.len() > 1 {
                    gen_range(0, variants.len())
                } else {
                    0
                };
                play_sound(
                    &variants[index],
                    PlaySoundParams {
                        looped: false,
                        volume: self.sfx_volume.get(),
                    },
                );
            }
        }
    }

    /// Play a sound with custom volume multiplier
    pub fn play_with_volume(&self, id: SoundId, volume_mult: f32) {
        if let Some(variants) = self.sounds.get(&id) {
            if !variants.is_empty() {
                let index = if variants.len() > 1 {
                    gen_range(0, variants.len())
                } else {
                    0
                };
                play_sound(
                    &variants[index],
                    PlaySoundParams {
                        looped: false,
                        volume: self.sfx_volume.get() * volume_mult,
                    },
                );
            }
        }
    }

    /// Set SFX volume (0.0 to 1.0)
    pub fn set_sfx_volume(&self, volume: f32) {
        self.sfx_volume.set(volume.clamp(0.0, 1.0));
    }

    /// Get current SFX volume
    pub fn sfx_volume(&self) -> f32 {
        self.sfx_volume.get()
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_music_volume(&self, volume: f32) {
        self.music_volume.set(volume.clamp(0.0, 1.0));
    }

    /// Get current music volume
    pub fn music_volume(&self) -> f32 {
        self.music_volume.get()
    }

    /// Get count of loaded sound types
    pub fn loaded_sound_count(&self) -> usize {
        self.sounds.iter().filter(|(_, v)| !v.is_empty()).count()
    }

    /// Get total variant count across all sounds
    pub fn total_variant_count(&self) -> usize {
        self.sounds.values().map(|v| v.len()).sum()
    }
}

//! Ambient sound system for atmospheric underwater soundscapes
//!
//! Manages looping ambient sounds with support for layered soundscapes
//! that can vary by biome.

use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, set_sound_volume, PlaySoundParams, Sound};
use macroquad::rand::gen_range;
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;
use crate::assets::AudioAssets;

/// Identifies different ambient sound layers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AmbientTrack {
    /// Deep underwater atmosphere
    UnderwaterLoop,
    /// Continuous bubble stream
    BubblesLoop,
    /// Rain on water surface (for shallow biomes)
    RainLoop,
}

impl AmbientTrack {
    /// Get the asset paths for this ambient track (supports variants)
    pub fn asset_paths(&self) -> Vec<&'static str> {
        match self {
            AmbientTrack::UnderwaterLoop => vec![
                "sfx/organized/ambient/underwater_loop_01.ogg",
                "sfx/organized/ambient/underwater_loop_02.ogg",
            ],
            AmbientTrack::BubblesLoop => vec![
                "sfx/organized/ambient/bubbles_loop_01.ogg",
                "sfx/organized/ambient/bubbles_loop_02.ogg",
            ],
            AmbientTrack::RainLoop => vec![
                "sfx/organized/ambient/rain_loop.ogg",
            ],
        }
    }

    /// All ambient tracks that should be preloaded
    pub const ALL: [AmbientTrack; 3] = [
        AmbientTrack::UnderwaterLoop,
        AmbientTrack::BubblesLoop,
        AmbientTrack::RainLoop,
    ];
}

/// One-shot ambient sound effects (not looped)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AmbientOneShot {
    /// Single bubble pop
    Bubble,
}

impl AmbientOneShot {
    /// Get asset paths for this one-shot sound
    pub fn asset_paths(&self) -> Vec<&'static str> {
        match self {
            AmbientOneShot::Bubble => vec![
                "sfx/organized/ambient/bubble_01.ogg",
                "sfx/organized/ambient/bubble_02.ogg",
                "sfx/organized/ambient/bubble_03.ogg",
            ],
        }
    }

    /// All one-shot sounds to preload
    pub const ALL: [AmbientOneShot; 1] = [
        AmbientOneShot::Bubble,
    ];
}

/// Configuration for a biome's ambient soundscape
#[derive(Clone)]
pub struct BiomeSoundscape {
    /// Ambient layers to play with their relative volumes
    pub layers: Vec<(AmbientTrack, f32)>,
}

impl BiomeSoundscape {
    /// Get the soundscape for a biome
    pub fn for_biome(biome: BiomeId) -> Self {
        match biome {
            BiomeId::OceanDepths => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 1.0),
                    (AmbientTrack::BubblesLoop, 0.3),
                ],
            },
            BiomeId::CoralReefs => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 0.8),
                    (AmbientTrack::BubblesLoop, 0.5),
                ],
            },
            BiomeId::TropicalShore => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 0.5),
                    (AmbientTrack::RainLoop, 0.4),
                ],
            },
            BiomeId::Shipwreck => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 1.0),
                    (AmbientTrack::BubblesLoop, 0.2),
                ],
            },
            BiomeId::ArcticWaters => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 0.9),
                ],
            },
            BiomeId::VolcanicVents => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 0.7),
                    (AmbientTrack::BubblesLoop, 0.8),
                ],
            },
            BiomeId::SunkenRuins => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 1.0),
                    (AmbientTrack::BubblesLoop, 0.4),
                ],
            },
            BiomeId::Abyss => Self {
                layers: vec![
                    (AmbientTrack::UnderwaterLoop, 1.2),
                ],
            },
        }
    }
}

/// State for a currently playing ambient layer
struct PlayingLayer {
    track: AmbientTrack,
    /// Volume multiplier for this layer (relative to master volume)
    layer_volume: f32,
}

/// Manages ambient soundscapes with layered looping sounds
pub struct AmbientManager {
    /// Loaded ambient loop sounds (track -> variants)
    loops: HashMap<AmbientTrack, Vec<Sound>>,
    /// Loaded one-shot sounds (id -> variants)
    one_shots: HashMap<AmbientOneShot, Vec<Sound>>,
    /// Currently playing ambient layers
    playing: Vec<PlayingLayer>,
    /// Current biome soundscape
    current_biome: Option<BiomeId>,
    /// Master volume for ambient sounds
    volume: f32,
    /// Whether ambient sounds are enabled
    enabled: bool,
    /// Time accumulator for random bubble sounds
    bubble_timer: f32,
    /// Next bubble time
    next_bubble_time: f32,
}

impl AmbientManager {
    /// Create a new ambient manager
    pub fn new() -> Self {
        Self {
            loops: HashMap::new(),
            one_shots: HashMap::new(),
            playing: Vec::new(),
            current_biome: None,
            volume: 0.4,
            enabled: true,
            bubble_timer: 0.0,
            next_bubble_time: gen_range(2.0, 5.0),
        }
    }

    /// Load all ambient sounds
    pub async fn load_all(&mut self) {
        // Load looping tracks
        for track in AmbientTrack::ALL {
            self.load_loop(track).await;
        }

        // Load one-shot sounds
        for sound in AmbientOneShot::ALL {
            self.load_one_shot(sound).await;
        }
    }

    /// Load a looping ambient track
    async fn load_loop(&mut self, track: AmbientTrack) {
        let paths = track.asset_paths();
        let mut variants = Vec::new();

        for path in paths {
            if let Some(bytes) = AudioAssets::get_audio(path) {
                match load_sound_from_bytes(&bytes).await {
                    Ok(sound) => {
                        variants.push(sound);
                    }
                    Err(e) => {
                        eprintln!("Failed to decode ambient loop {:?} from {}: {}", track, path, e);
                    }
                }
            }
        }

        if variants.is_empty() {
            eprintln!("Warning: No variants loaded for ambient track {:?}", track);
        } else {
            #[cfg(debug_assertions)]
            println!("Loaded ambient {:?}: {} variant(s)", track, variants.len());
        }

        self.loops.insert(track, variants);
    }

    /// Load a one-shot ambient sound
    async fn load_one_shot(&mut self, sound_id: AmbientOneShot) {
        let paths = sound_id.asset_paths();
        let mut variants = Vec::new();

        for path in paths {
            if let Some(bytes) = AudioAssets::get_audio(path) {
                match load_sound_from_bytes(&bytes).await {
                    Ok(sound) => {
                        variants.push(sound);
                    }
                    Err(e) => {
                        eprintln!("Failed to decode ambient one-shot {:?} from {}: {}", sound_id, path, e);
                    }
                }
            }
        }

        if !variants.is_empty() {
            #[cfg(debug_assertions)]
            println!("Loaded ambient one-shot {:?}: {} variant(s)", sound_id, variants.len());
        }

        self.one_shots.insert(sound_id, variants);
    }

    /// Start playing ambient sounds for a biome
    pub fn play_biome(&mut self, biome: BiomeId) {
        if !self.enabled {
            return;
        }

        // Don't restart if already playing this biome
        if self.current_biome == Some(biome) {
            return;
        }

        // Stop current soundscape
        self.stop();

        // Get soundscape for biome
        let soundscape = BiomeSoundscape::for_biome(biome);

        // Start each layer
        for (track, layer_volume) in soundscape.layers {
            if let Some(variants) = self.loops.get(&track) {
                if !variants.is_empty() {
                    // Pick a random variant
                    let idx = if variants.len() > 1 {
                        gen_range(0, variants.len())
                    } else {
                        0
                    };

                    let actual_volume = self.volume * layer_volume;
                    play_sound(
                        &variants[idx],
                        PlaySoundParams {
                            looped: true,
                            volume: actual_volume,
                        },
                    );

                    self.playing.push(PlayingLayer {
                        track,
                        layer_volume,
                    });
                }
            }
        }

        self.current_biome = Some(biome);
    }

    /// Play a random one-shot ambient sound
    pub fn play_one_shot(&self, sound_id: AmbientOneShot) {
        if !self.enabled {
            return;
        }

        if let Some(variants) = self.one_shots.get(&sound_id) {
            if !variants.is_empty() {
                let idx = if variants.len() > 1 {
                    gen_range(0, variants.len())
                } else {
                    0
                };

                play_sound(
                    &variants[idx],
                    PlaySoundParams {
                        looped: false,
                        volume: self.volume * 0.5, // One-shots a bit quieter
                    },
                );
            }
        }
    }

    /// Update ambient sounds (call each frame)
    pub fn update(&mut self, dt: f32) {
        if !self.enabled || self.current_biome.is_none() {
            return;
        }

        // Random bubble sounds
        self.bubble_timer += dt;
        if self.bubble_timer >= self.next_bubble_time {
            self.bubble_timer = 0.0;
            self.next_bubble_time = gen_range(3.0, 8.0);
            self.play_one_shot(AmbientOneShot::Bubble);
        }
    }

    /// Stop all ambient sounds
    pub fn stop(&mut self) {
        for layer in &self.playing {
            if let Some(variants) = self.loops.get(&layer.track) {
                for sound in variants {
                    stop_sound(sound);
                }
            }
        }
        self.playing.clear();
        self.current_biome = None;
    }

    /// Pause ambient sounds
    pub fn pause(&mut self) {
        for layer in &self.playing {
            if let Some(variants) = self.loops.get(&layer.track) {
                for sound in variants {
                    set_sound_volume(sound, 0.0);
                }
            }
        }
    }

    /// Resume ambient sounds
    pub fn resume(&mut self) {
        for layer in &self.playing {
            if let Some(variants) = self.loops.get(&layer.track) {
                for sound in variants {
                    set_sound_volume(sound, self.volume * layer.layer_volume);
                }
            }
        }
    }

    /// Set master volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);

        // Update playing sounds
        for layer in &self.playing {
            if let Some(variants) = self.loops.get(&layer.track) {
                for sound in variants {
                    set_sound_volume(sound, self.volume * layer.layer_volume);
                }
            }
        }
    }

    /// Get current volume
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Enable or disable ambient sounds
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.stop();
        }
    }

    /// Check if ambient sounds are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get current biome
    pub fn current_biome(&self) -> Option<BiomeId> {
        self.current_biome
    }
}

impl Default for AmbientManager {
    fn default() -> Self {
        Self::new()
    }
}

//! Music system with file-based loading and biome-specific tracks
//!
//! Handles background music playback with crossfading between tracks.

use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, set_sound_volume, PlaySoundParams, Sound};
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;
use crate::assets::AudioAssets;

/// Identifies different music tracks in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicTrack {
    // Menu tracks
    Title,
    GameOver,

    // Biome tracks
    OceanDepths,
    CoralReefs,
    TropicalShore,
    Shipwreck,
    ArcticWaters,
    VolcanicVents,
    SunkenRuins,
    Abyss,
}

impl MusicTrack {
    /// Get the embedded asset path for this track (relative to assets/audio/)
    pub fn asset_path(&self) -> &'static str {
        match self {
            MusicTrack::Title => "music/menu/title.ogg",
            MusicTrack::GameOver => "music/menu/game_over.ogg",
            MusicTrack::OceanDepths => "music/biomes/ocean_depths.ogg",
            MusicTrack::CoralReefs => "music/biomes/coral_reefs.ogg",
            MusicTrack::TropicalShore => "music/biomes/tropical_shore.ogg",
            MusicTrack::Shipwreck => "music/biomes/shipwreck.ogg",
            MusicTrack::ArcticWaters => "music/biomes/arctic_waters.ogg",
            MusicTrack::VolcanicVents => "music/biomes/volcanic_vents.ogg",
            MusicTrack::SunkenRuins => "music/biomes/sunken_ruins.ogg",
            MusicTrack::Abyss => "music/biomes/abyss.ogg",
        }
    }

    /// Convert from BiomeId to MusicTrack
    pub fn from_biome(biome: BiomeId) -> Self {
        match biome {
            BiomeId::OceanDepths => MusicTrack::OceanDepths,
            BiomeId::CoralReefs => MusicTrack::CoralReefs,
            BiomeId::TropicalShore => MusicTrack::TropicalShore,
            BiomeId::Shipwreck => MusicTrack::Shipwreck,
            BiomeId::ArcticWaters => MusicTrack::ArcticWaters,
            BiomeId::VolcanicVents => MusicTrack::VolcanicVents,
            BiomeId::SunkenRuins => MusicTrack::SunkenRuins,
            BiomeId::Abyss => MusicTrack::Abyss,
        }
    }

    /// All tracks that should be preloaded
    pub const ALL: [MusicTrack; 10] = [
        MusicTrack::Title,
        MusicTrack::GameOver,
        MusicTrack::OceanDepths,
        MusicTrack::CoralReefs,
        MusicTrack::TropicalShore,
        MusicTrack::Shipwreck,
        MusicTrack::ArcticWaters,
        MusicTrack::VolcanicVents,
        MusicTrack::SunkenRuins,
        MusicTrack::Abyss,
    ];
}

/// State of a crossfade transition
struct CrossfadeState {
    from_track: MusicTrack,
    to_track: MusicTrack,
    progress: f32,
    duration: f32,
}

/// Music manager handles loading and playing background music
pub struct MusicManager {
    tracks: HashMap<MusicTrack, Sound>,
    current_track: Option<MusicTrack>,
    crossfade: Option<CrossfadeState>,
    volume: f32,
    enabled: bool,
}

impl MusicManager {
    /// Create a new music manager (tracks not loaded yet)
    pub fn new() -> Self {
        Self {
            tracks: HashMap::new(),
            current_track: None,
            crossfade: None,
            volume: 0.5,
            enabled: true,
        }
    }

    /// Load all music tracks asynchronously
    pub async fn load_all_tracks(&mut self) {
        for track in MusicTrack::ALL {
            self.load_track(track).await;
        }
    }

    /// Load a single track from embedded assets
    pub async fn load_track(&mut self, track: MusicTrack) {
        let path = track.asset_path();
        match AudioAssets::get_audio(path) {
            Some(bytes) => {
                match load_sound_from_bytes(&bytes).await {
                    Ok(sound) => {
                        self.tracks.insert(track, sound);
                    }
                    Err(e) => {
                        eprintln!("Failed to decode music {:?}: {}", track, e);
                    }
                }
            }
            None => {
                eprintln!("Failed to find embedded music {:?} at path: {}", track, path);
            }
        }
    }

    /// Play a track immediately (stops current track)
    pub fn play(&mut self, track: MusicTrack) {
        if !self.enabled {
            return;
        }

        // Stop current track if playing
        if let Some(current) = self.current_track {
            if let Some(sound) = self.tracks.get(&current) {
                stop_sound(sound);
            }
        }

        // Cancel any ongoing crossfade
        self.crossfade = None;

        // Play new track
        if let Some(sound) = self.tracks.get(&track) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: self.volume,
                },
            );
            self.current_track = Some(track);
        }
    }

    /// Play a track for a specific biome
    pub fn play_biome(&mut self, biome: BiomeId) {
        let track = MusicTrack::from_biome(biome);

        // Don't restart if already playing this track
        if self.current_track == Some(track) {
            return;
        }

        self.play(track);
    }

    /// Crossfade to a new track over the specified duration
    pub fn crossfade_to(&mut self, track: MusicTrack, duration: f32) {
        if !self.enabled {
            return;
        }

        // Don't crossfade if already playing this track
        if self.current_track == Some(track) {
            return;
        }

        // If there's no current track, just play directly
        let Some(current) = self.current_track else {
            self.play(track);
            return;
        };

        // Start the new track at volume 0
        if let Some(sound) = self.tracks.get(&track) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: 0.0,
                },
            );
        } else {
            return;
        }

        // Set up crossfade state
        self.crossfade = Some(CrossfadeState {
            from_track: current,
            to_track: track,
            progress: 0.0,
            duration,
        });
    }

    /// Crossfade to a biome's track
    pub fn crossfade_to_biome(&mut self, biome: BiomeId, duration: f32) {
        let track = MusicTrack::from_biome(biome);
        self.crossfade_to(track, duration);
    }

    /// Update crossfade progress (call each frame with delta time)
    pub fn update(&mut self, dt: f32) {
        let Some(ref mut crossfade) = self.crossfade else {
            return;
        };

        crossfade.progress += dt;
        let t = (crossfade.progress / crossfade.duration).min(1.0);

        // Update volumes with smooth easing
        let ease_t = t * t * (3.0 - 2.0 * t); // smoothstep

        // Fade out old track
        if let Some(sound) = self.tracks.get(&crossfade.from_track) {
            set_sound_volume(sound, self.volume * (1.0 - ease_t));
        }

        // Fade in new track
        if let Some(sound) = self.tracks.get(&crossfade.to_track) {
            set_sound_volume(sound, self.volume * ease_t);
        }

        // Complete crossfade
        if t >= 1.0 {
            // Stop old track
            if let Some(sound) = self.tracks.get(&crossfade.from_track) {
                stop_sound(sound);
            }

            // Update current track
            self.current_track = Some(crossfade.to_track);
            self.crossfade = None;
        }
    }

    /// Stop all music
    pub fn stop(&mut self) {
        if let Some(current) = self.current_track {
            if let Some(sound) = self.tracks.get(&current) {
                stop_sound(sound);
            }
        }

        // Also stop crossfade target if any
        if let Some(ref crossfade) = self.crossfade {
            if let Some(sound) = self.tracks.get(&crossfade.to_track) {
                stop_sound(sound);
            }
        }

        self.current_track = None;
        self.crossfade = None;
    }

    /// Pause current music (keeps track position)
    pub fn pause(&mut self) {
        // macroquad doesn't have pause, so we just reduce volume to 0
        if let Some(current) = self.current_track {
            if let Some(sound) = self.tracks.get(&current) {
                set_sound_volume(sound, 0.0);
            }
        }
    }

    /// Resume paused music
    pub fn resume(&mut self) {
        if let Some(current) = self.current_track {
            if let Some(sound) = self.tracks.get(&current) {
                set_sound_volume(sound, self.volume);
            }
        }
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);

        // Update current track volume
        if let Some(current) = self.current_track {
            if let Some(sound) = self.tracks.get(&current) {
                // Don't update if crossfading (crossfade handles volumes)
                if self.crossfade.is_none() {
                    set_sound_volume(sound, self.volume);
                }
            }
        }
    }

    /// Get current volume
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Enable or disable music
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.stop();
        }
    }

    /// Check if music is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get currently playing track
    pub fn current_track(&self) -> Option<MusicTrack> {
        self.current_track
    }

    /// Check if a specific track is loaded
    pub fn is_track_loaded(&self, track: MusicTrack) -> bool {
        self.tracks.contains_key(&track)
    }

    /// Get count of loaded tracks
    pub fn loaded_track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl Default for MusicManager {
    fn default() -> Self {
        Self::new()
    }
}

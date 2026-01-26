//! Effects controller subsystem
//!
//! Coordinates audio, visual effects, shaders, and feedback tracking.

use macroquad::prelude::*;

use crate::audio::{AmbientManager, AudioManager, MusicManager, MusicTrack, SoundId};
use crate::rendering::ShaderManager;
use octoplat_core::physics::FeedbackTracker;
use octoplat_core::procgen::BiomeId;

use super::EffectsManager;

/// Coordinates all audio and visual effects systems.
///
/// This subsystem handles:
/// - Audio playback (via AudioManager)
/// - Music playback (via MusicManager)
/// - Ambient sounds (via AmbientManager)
/// - Visual particle effects (via EffectsManager)
/// - Shader effects (via ShaderManager)
/// - Audio/visual feedback state (via FeedbackTracker)
pub struct EffectsController {
    /// Audio system (optional - may be unavailable on some platforms)
    pub audio: Option<AudioManager>,

    /// Music system (optional - loaded asynchronously)
    pub music: Option<MusicManager>,

    /// Ambient sound system (optional - loaded asynchronously)
    pub ambient: Option<AmbientManager>,

    /// Visual effects (particles, screen shake)
    pub effects: EffectsManager,

    /// Shader effects (chromatic aberration, etc.)
    pub shaders: ShaderManager,

    /// Tracks state for audio/visual feedback decisions
    pub feedback: FeedbackTracker,
}

impl EffectsController {
    /// Create a new EffectsController with no audio (audio must be initialized async)
    pub fn new() -> Self {
        Self {
            audio: None,
            music: None,
            ambient: None,
            effects: EffectsManager::new(),
            shaders: ShaderManager::new(),
            feedback: FeedbackTracker::new(),
        }
    }

    /// Set the audio manager (called after async initialization)
    pub fn set_audio(&mut self, audio: AudioManager) {
        self.audio = Some(audio);
    }

    /// Set the music manager (called after async initialization)
    pub fn set_music(&mut self, music: MusicManager) {
        self.music = Some(music);
    }

    /// Set the ambient sound manager (called after async initialization)
    pub fn set_ambient(&mut self, ambient: AmbientManager) {
        self.ambient = Some(ambient);
    }

    /// Update all effect systems
    pub fn update(&mut self, dt: f32) {
        self.effects.update(dt);
        self.shaders.update_chromatic(dt);

        // Update music crossfades
        if let Some(ref mut music) = self.music {
            music.update(dt);
        }

        // Update ambient sounds (random bubbles, etc.)
        if let Some(ref mut ambient) = self.ambient {
            ambient.update(dt);
        }
    }

    /// Draw particle effects (call after world rendering, before UI)
    pub fn draw(&self) {
        self.effects.draw();
    }

    /// Apply shader post-effects (call after resetting to default camera)
    pub fn apply_shaders(&self) {
        self.shaders.apply_chromatic();
    }

    // ========================================================================
    // Audio methods
    // ========================================================================

    /// Play a sound effect
    pub fn play_sound(&self, id: SoundId) {
        if let Some(ref audio) = self.audio {
            audio.play(id);
        }
    }

    /// Play a sound effect with distance-based volume
    pub fn play_sound_at(&self, id: SoundId, position: Vec2, listener_pos: Vec2) {
        if let Some(ref audio) = self.audio {
            let distance = (position - listener_pos).length();
            // Full volume within 200 units, fade to 0 at 600 units
            let volume = ((600.0 - distance) / 400.0).clamp(0.0, 1.0);
            if volume > 0.01 {
                audio.play_with_volume(id, volume);
            }
        }
    }

    /// Set SFX volume
    pub fn set_sfx_volume(&self, volume: f32) {
        if let Some(ref audio) = self.audio {
            audio.set_sfx_volume(volume);
        }
    }

    /// Set music volume (deprecated - use set_music_track_volume)
    pub fn set_music_volume(&self, volume: f32) {
        if let Some(ref audio) = self.audio {
            audio.set_music_volume(volume);
        }
    }

    // ========================================================================
    // Music methods
    // ========================================================================

    /// Play a specific music track
    pub fn play_music(&mut self, track: MusicTrack) {
        if let Some(ref mut music) = self.music {
            music.play(track);
        }
    }

    /// Play music for a specific biome
    pub fn play_biome_music(&mut self, biome: BiomeId) {
        if let Some(ref mut music) = self.music {
            music.play_biome(biome);
        }
    }

    /// Crossfade to a new music track
    pub fn crossfade_music(&mut self, track: MusicTrack, duration: f32) {
        if let Some(ref mut music) = self.music {
            music.crossfade_to(track, duration);
        }
    }

    /// Crossfade to biome music
    pub fn crossfade_to_biome_music(&mut self, biome: BiomeId, duration: f32) {
        if let Some(ref mut music) = self.music {
            music.crossfade_to_biome(biome, duration);
        }
    }

    /// Stop all music
    pub fn stop_music(&mut self) {
        if let Some(ref mut music) = self.music {
            music.stop();
        }
    }

    /// Pause music
    pub fn pause_music(&mut self) {
        if let Some(ref mut music) = self.music {
            music.pause();
        }
    }

    /// Resume music
    pub fn resume_music(&mut self) {
        if let Some(ref mut music) = self.music {
            music.resume();
        }
    }

    /// Set music track volume
    pub fn set_music_track_volume(&mut self, volume: f32) {
        if let Some(ref mut music) = self.music {
            music.set_volume(volume);
        }
    }

    /// Enable or disable music
    pub fn set_music_enabled(&mut self, enabled: bool) {
        if let Some(ref mut music) = self.music {
            music.set_enabled(enabled);
        }
    }

    // ========================================================================
    // Ambient sound methods
    // ========================================================================

    /// Play ambient sounds for a biome
    pub fn play_biome_ambient(&mut self, biome: BiomeId) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.play_biome(biome);
        }
    }

    /// Stop all ambient sounds
    pub fn stop_ambient(&mut self) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.stop();
        }
    }

    /// Pause ambient sounds
    pub fn pause_ambient(&mut self) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.pause();
        }
    }

    /// Resume ambient sounds
    pub fn resume_ambient(&mut self) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.resume();
        }
    }

    /// Set ambient sound volume
    pub fn set_ambient_volume(&mut self, volume: f32) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.set_volume(volume);
        }
    }

    /// Enable or disable ambient sounds
    pub fn set_ambient_enabled(&mut self, enabled: bool) {
        if let Some(ref mut ambient) = self.ambient {
            ambient.set_enabled(enabled);
        }
    }

    // ========================================================================
    // Shader effect methods
    // ========================================================================

    /// Trigger chromatic aberration hit effect
    pub fn trigger_chromatic_hit(&mut self, center: Vec2, intensity: f32) {
        self.shaders.trigger_chromatic_hit(center, intensity);
    }

    // ========================================================================
    // Combined effect methods (audio + visuals)
    // ========================================================================

    /// Spawn death effects (particles, shake, no audio - caller handles sound)
    pub fn spawn_death(&mut self, position: Vec2) {
        self.effects.spawn_death(position);
    }

    /// Spawn hurt effects (particles, shake)
    pub fn spawn_hurt(&mut self, position: Vec2) {
        self.effects.spawn_hurt(position);
    }

    /// Spawn checkpoint activation effects
    pub fn spawn_checkpoint(&mut self, position: Vec2) {
        self.effects.spawn_checkpoint(position);
    }

    /// Spawn extra life effects
    pub fn spawn_extra_life(&mut self, position: Vec2) {
        self.effects.spawn_extra_life(position);
    }

    /// Spawn gem collection effects
    pub fn spawn_gem_collect(&mut self, position: Vec2) {
        self.effects.spawn_gem_collect(position);
    }

    /// Spawn jump dust
    pub fn spawn_jump(&mut self, position: Vec2) {
        self.effects.spawn_jump(position);
    }

    /// Spawn landing impact
    pub fn spawn_land(&mut self, position: Vec2, intensity: f32) {
        self.effects.spawn_land(position, intensity);
    }

    /// Spawn wall jump burst
    pub fn spawn_wall_jump(&mut self, position: Vec2, wall_dir: i8) {
        self.effects.spawn_wall_jump(position, wall_dir);
    }

    /// Spawn grapple attach splash
    pub fn spawn_grapple_attach(&mut self, position: Vec2) {
        self.effects.spawn_grapple_attach(position);
    }

    /// Spawn bounce pad effect
    pub fn spawn_bounce(&mut self, position: Vec2) {
        self.effects.spawn_bounce(position);
    }

    /// Spawn dive impact
    pub fn spawn_dive_impact(&mut self, position: Vec2) {
        self.effects.spawn_dive_impact(position);
    }

    /// Spawn jet boost trail
    pub fn spawn_jet_boost(&mut self, position: Vec2, direction: Vec2) {
        self.effects.spawn_jet_boost(position, direction);
    }

    /// Spawn ink cloud effect
    pub fn spawn_ink_cloud(&mut self, position: Vec2) {
        self.effects.spawn_ink_cloud(position);
    }

    // ========================================================================
    // Feedback tracking
    // ========================================================================

    /// Set checkpoint for feedback tracking
    pub fn set_checkpoint(&mut self, checkpoint: Option<octoplat_core::Vec2>) {
        self.feedback.set_checkpoint(checkpoint);
    }

    /// Reset level complete feedback state
    pub fn reset_level_complete(&mut self) {
        self.feedback.reset_level_complete();
    }

    /// Mark level complete sound as played
    pub fn mark_level_complete_played(&mut self) {
        self.feedback.mark_level_complete_played();
    }

    /// Check if level complete sound was played
    pub fn played_level_complete_sound(&self) -> bool {
        self.feedback.played_level_complete_sound
    }

    /// Get previous checkpoint for comparison
    pub fn prev_checkpoint(&self) -> Option<octoplat_core::Vec2> {
        self.feedback.prev_checkpoint
    }
}

impl Default for EffectsController {
    fn default() -> Self {
        Self::new()
    }
}

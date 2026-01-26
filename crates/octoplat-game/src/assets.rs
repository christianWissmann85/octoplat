//! Embedded assets for the game
//!
//! All game assets are embedded into the binary at compile time
//! to create a self-contained executable.

use rust_embed::RustEmbed;
use std::borrow::Cow;

/// Embedded audio assets (music and sound effects)
#[derive(RustEmbed)]
#[folder = "../../assets/audio/"]
pub struct AudioAssets;

impl AudioAssets {
    /// Get an embedded audio file by path
    /// Path should be relative to assets/audio/ (e.g., "music/menu/title.ogg")
    /// Returns the audio data as owned bytes
    pub fn get_audio(path: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(path).map(|file| file.data)
    }
}

/// Embedded background assets (parallax layer images)
#[derive(RustEmbed)]
#[folder = "../../assets/backgrounds/"]
pub struct BackgroundAssets;

impl BackgroundAssets {
    /// Get an embedded background image by path
    /// Path should be relative to assets/backgrounds/ (e.g., "ocean_depths/far.png")
    /// Returns the image data as owned bytes
    pub fn get_image(path: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(path).map(|file| file.data)
    }

    /// Check if a background image exists
    pub fn has_image(path: &str) -> bool {
        Self::get(path).is_some()
    }
}

/// Embedded decoration assets (sprites for level decorations)
#[derive(RustEmbed)]
#[folder = "../../assets/decorations/"]
pub struct DecorationAssets;

impl DecorationAssets {
    /// Get an embedded decoration image by path
    /// Path should be relative to assets/decorations/ (e.g., "ocean_depths/small_rock.png")
    /// Returns the image data as owned bytes
    pub fn get_image(path: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(path).map(|file| file.data)
    }

    /// Check if a decoration image exists
    pub fn has_image(path: &str) -> bool {
        Self::get(path).is_some()
    }
}

/// Embedded tile texture assets (tileable surface textures per biome)
#[derive(RustEmbed)]
#[folder = "../../assets/textures/tiles/"]
pub struct TileTextureAssets;

impl TileTextureAssets {
    /// Get an embedded tile texture by path
    /// Path should be relative to assets/textures/tiles/ (e.g., "ocean_depths.png")
    /// Returns the image data as owned bytes
    pub fn get_texture(path: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(path).map(|file| file.data)
    }

    /// Check if a tile texture exists
    pub fn has_texture(path: &str) -> bool {
        Self::get(path).is_some()
    }
}

/// Embedded UI assets (loading screen, title screen, menus)
#[derive(RustEmbed)]
#[folder = "../../assets/ui/"]
pub struct UiAssets;

impl UiAssets {
    /// Get an embedded UI image by path
    /// Path should be relative to assets/ui/ (e.g., "loading/background.png")
    /// Returns the image data as owned bytes
    pub fn get_image(path: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(path).map(|file| file.data)
    }

    /// Check if a UI image exists
    pub fn has_image(path: &str) -> bool {
        Self::get(path).is_some()
    }
}

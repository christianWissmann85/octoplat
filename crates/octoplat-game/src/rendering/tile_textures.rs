//! Tile texture overlay system
//!
//! Loads and renders tileable surface textures for solid blocks and platforms.
//! Textures are blended with procedural biome colors using multiply blending.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;

use crate::assets::TileTextureAssets;

/// Texture quality setting
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextureQuality {
    /// No tile textures (pure procedural)
    Off,
    /// Standard quality textures
    #[default]
    Low,
    /// High quality with linear filtering
    High,
}

impl TextureQuality {
    /// Cycle to next quality setting
    pub fn next(self) -> Self {
        match self {
            TextureQuality::Off => TextureQuality::Low,
            TextureQuality::Low => TextureQuality::High,
            TextureQuality::High => TextureQuality::Off,
        }
    }

    /// Get display label
    pub fn label(self) -> &'static str {
        match self {
            TextureQuality::Off => "Off",
            TextureQuality::Low => "Low",
            TextureQuality::High => "High",
        }
    }
}

/// Manager for loading and caching tile textures
pub struct TileTextureManager {
    /// Cached textures by biome
    textures: HashMap<BiomeId, Texture2D>,
    /// Quality setting
    quality: TextureQuality,
}

impl TileTextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            quality: TextureQuality::default(),
        }
    }

    /// Load texture for a specific biome
    pub async fn load_biome(&mut self, biome: BiomeId) {
        // Skip if already loaded
        if self.textures.contains_key(&biome) {
            return;
        }

        let filename = biome_filename(biome);

        if let Some(bytes) = TileTextureAssets::get_texture(filename) {
            let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            // Use linear filtering for high quality, nearest for low
            let filter = if self.quality == TextureQuality::High {
                FilterMode::Linear
            } else {
                FilterMode::Nearest
            };
            texture.set_filter(filter);
            self.textures.insert(biome, texture);
        }
    }

    /// Load textures for all biomes
    pub async fn load_all_biomes(&mut self) {
        for biome in BiomeId::all() {
            self.load_biome(*biome).await;
        }
    }

    /// Get the texture for a biome (if loaded and enabled)
    pub fn get(&self, biome: BiomeId) -> Option<&Texture2D> {
        if self.quality == TextureQuality::Off {
            return None;
        }
        self.textures.get(&biome)
    }

    /// Check if a biome has a texture loaded
    pub fn has_biome(&self, biome: BiomeId) -> bool {
        self.textures.contains_key(&biome)
    }

    /// Get current quality setting
    pub fn quality(&self) -> TextureQuality {
        self.quality
    }

    /// Set quality setting
    pub fn set_quality(&mut self, quality: TextureQuality) {
        self.quality = quality;
        // Update filter mode on existing textures
        let filter = if quality == TextureQuality::High {
            FilterMode::Linear
        } else {
            FilterMode::Nearest
        };
        for texture in self.textures.values() {
            texture.set_filter(filter);
        }
    }

    /// Get number of loaded textures
    pub fn loaded_count(&self) -> usize {
        self.textures.len()
    }
}

impl Default for TileTextureManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the filename for a biome's tile texture
fn biome_filename(biome: BiomeId) -> &'static str {
    match biome {
        BiomeId::OceanDepths => "ocean_depths.png",
        BiomeId::CoralReefs => "coral_reefs.png",
        BiomeId::TropicalShore => "tropical_shore.png",
        BiomeId::Shipwreck => "shipwreck.png",
        BiomeId::ArcticWaters => "arctic_waters.png",
        BiomeId::VolcanicVents => "volcanic_vents.png",
        BiomeId::SunkenRuins => "sunken_ruins.png",
        BiomeId::Abyss => "abyss.png",
    }
}

/// Draw a tile texture overlay with multiply blending
///
/// Uses world-space UV coordinates for seamless tiling across adjacent tiles.
/// The texture is tinted with the base color using multiply blending.
pub fn draw_tile_texture(
    texture: &Texture2D,
    px: f32,
    py: f32,
    size: f32,
    base_color: Color,
    opacity: f32,
) {
    let tex_size = texture.width();

    // World-space UV for seamless tiling
    // Modulo ensures coordinates wrap within texture bounds
    let u = (px % tex_size) / tex_size;
    let v = (py % tex_size) / tex_size;

    // Calculate source rectangle (handles wrapping at texture edges)
    let source_x = u * tex_size;
    let source_y = v * tex_size;

    // Tint color for multiply blend effect
    // We use the base color RGB and reduce alpha for subtle overlay
    let tint = Color::new(base_color.r, base_color.g, base_color.b, opacity);

    draw_texture_ex(
        texture,
        px,
        py,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            source: Some(Rect::new(source_x, source_y, size, size)),
            ..Default::default()
        },
    );
}

/// Draw a tile texture for platforms (thinner, more subtle)
pub fn draw_platform_texture(
    texture: &Texture2D,
    px: f32,
    py: f32,
    width: f32,
    height: f32,
    base_color: Color,
    opacity: f32,
) {
    let tex_size = texture.width();

    // World-space UV
    let u = (px % tex_size) / tex_size;
    let v = (py % tex_size) / tex_size;

    let source_x = u * tex_size;
    let source_y = v * tex_size;

    let tint = Color::new(base_color.r, base_color.g, base_color.b, opacity);

    draw_texture_ex(
        texture,
        px,
        py,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            source: Some(Rect::new(source_x, source_y, width, height)),
            ..Default::default()
        },
    );
}

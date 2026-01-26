//! Tile texture overlay system
//!
//! Loads and renders tileable surface textures for solid blocks and platforms.
//! Textures are blended with procedural biome colors using multiply blending.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;

use crate::assets::{HazardTextureAssets, TileTextureAssets};

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
    /// Spike hazard texture (shared across all biomes, tinted per biome)
    spike_texture: Option<Texture2D>,
    /// Quality setting
    quality: TextureQuality,
}

impl TileTextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            spike_texture: None,
            quality: TextureQuality::default(),
        }
    }

    /// Load the spike hazard texture
    pub async fn load_spike_texture(&mut self) {
        if self.spike_texture.is_some() {
            return;
        }

        if let Some(bytes) = HazardTextureAssets::get_texture("spike.png") {
            let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            let filter = if self.quality == TextureQuality::High {
                FilterMode::Linear
            } else {
                FilterMode::Nearest
            };
            texture.set_filter(filter);
            self.spike_texture = Some(texture);
        }
    }

    /// Get the spike texture (if loaded and enabled)
    pub fn get_spike(&self) -> Option<&Texture2D> {
        if self.quality == TextureQuality::Off {
            return None;
        }
        self.spike_texture.as_ref()
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

    /// Load textures for all biomes and hazards
    pub async fn load_all_biomes(&mut self) {
        for biome in BiomeId::all() {
            self.load_biome(*biome).await;
        }
        // Also load hazard textures
        self.load_spike_texture().await;
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
        // Update spike texture filter too
        if let Some(spike) = &self.spike_texture {
            spike.set_filter(filter);
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

/// Draw a spike hazard texture, scaled and tinted with biome hazard color
///
/// The spike texture is centered in the tile and scaled to fit.
/// Color tinting allows the same texture to match different biomes.
pub fn draw_spike_texture(
    texture: &Texture2D,
    px: f32,
    py: f32,
    tile_size: f32,
    hazard_color: Color,
) {
    // The spike texture is a sprite (not tileable), so we draw it centered
    let tex_width = texture.width();
    let tex_height = texture.height();

    // Scale texture to fit within tile, with small padding
    let padding = 2.0;
    let available_size = tile_size - padding * 2.0;
    let scale = available_size / tex_width.max(tex_height);

    let dest_width = tex_width * scale;
    let dest_height = tex_height * scale;

    // Center the spike in the tile
    let offset_x = (tile_size - dest_width) / 2.0;
    let offset_y = (tile_size - dest_height) / 2.0;

    // Tint with hazard color (additive blend with white base)
    // We blend the hazard color with white to preserve texture detail
    let tint = Color::new(
        (hazard_color.r + 0.5).min(1.0),
        (hazard_color.g + 0.5).min(1.0),
        (hazard_color.b + 0.5).min(1.0),
        1.0,
    );

    draw_texture_ex(
        texture,
        px + offset_x,
        py + offset_y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(dest_width, dest_height)),
            ..Default::default()
        },
    );
}

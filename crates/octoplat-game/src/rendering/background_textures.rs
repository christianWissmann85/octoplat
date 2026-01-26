//! Texture-based parallax background system
//!
//! Loads and renders PNG background images with parallax scrolling.
//! Each biome has a single background image that scrolls slower than the camera.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;

use crate::assets::BackgroundAssets;

/// Parallax depth for background scrolling (0.0 = static, 1.0 = moves with camera)
/// Lower values make the background scroll slower, creating depth illusion
pub const BACKGROUND_DEPTH: f32 = 0.3;

/// A single textured background for a biome
#[derive(Clone)]
pub struct BiomeTexture {
    /// The loaded texture
    pub texture: Texture2D,
    /// Parallax depth factor
    pub depth: f32,
    /// Whether to tile horizontally
    pub tile_x: bool,
}

impl BiomeTexture {
    /// Create a new biome texture
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            depth: BACKGROUND_DEPTH,
            tile_x: true,
        }
    }
}

/// Manager for loading and caching background textures
pub struct BackgroundTextureManager {
    /// Cached textures by biome
    textures: HashMap<BiomeId, BiomeTexture>,
}

impl BackgroundTextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load texture for a specific biome
    pub async fn load_biome(&mut self, biome: BiomeId) {
        // Skip if already loaded
        if self.textures.contains_key(&biome) {
            return;
        }

        let biome_dir = biome_directory(biome);
        let path = format!("{}/background.png", biome_dir);

        if let Some(bytes) = BackgroundAssets::get_image(&path) {
            let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            self.textures.insert(biome, BiomeTexture::new(texture));
        }
    }

    /// Load textures for all biomes
    pub async fn load_all_biomes(&mut self) {
        for biome in BiomeId::all() {
            self.load_biome(*biome).await;
        }
    }

    /// Get the texture for a biome (if loaded)
    pub fn get(&self, biome: BiomeId) -> Option<&BiomeTexture> {
        self.textures.get(&biome)
    }

    /// Check if a biome has a texture loaded
    pub fn has_biome(&self, biome: BiomeId) -> bool {
        self.textures.contains_key(&biome)
    }
}

impl Default for BackgroundTextureManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the directory name for a biome
fn biome_directory(biome: BiomeId) -> &'static str {
    match biome {
        BiomeId::OceanDepths => "ocean_depths",
        BiomeId::CoralReefs => "coral_reefs",
        BiomeId::TropicalShore => "tropical_shore",
        BiomeId::Shipwreck => "shipwreck",
        BiomeId::ArcticWaters => "arctic_waters",
        BiomeId::VolcanicVents => "volcanic_vents",
        BiomeId::SunkenRuins => "sunken_ruins",
        BiomeId::Abyss => "abyss",
    }
}

/// Draw textured background with parallax scrolling
pub fn draw_textured_background(
    texture: &BiomeTexture,
    camera_pos: Vec2,
    screen_size: Vec2,
) {
    let tex_width = texture.texture.width();
    let tex_height = texture.texture.height();

    // Calculate parallax offset (background moves slower than camera)
    let parallax_x = camera_pos.x * texture.depth;

    // Scale to fill screen height while maintaining aspect ratio
    let scale = screen_size.y / tex_height;
    let scaled_width = tex_width * scale;
    let scaled_height = screen_size.y;

    if texture.tile_x {
        // Tile horizontally to fill screen width
        let start_x = -(parallax_x * scale % scaled_width) - scaled_width;
        let end_x = screen_size.x + scaled_width;

        let mut x = start_x;
        while x < end_x {
            draw_texture_ex(
                &texture.texture,
                x,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(scaled_width, scaled_height)),
                    ..Default::default()
                },
            );
            x += scaled_width;
        }
    } else {
        // Single centered texture
        let x = (screen_size.x - scaled_width) / 2.0 - parallax_x * scale;
        draw_texture_ex(
            &texture.texture,
            x,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scaled_width, scaled_height)),
                ..Default::default()
            },
        );
    }
}


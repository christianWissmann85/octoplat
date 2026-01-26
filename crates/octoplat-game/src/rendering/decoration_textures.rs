//! Texture-based decoration rendering
//!
//! Loads and renders PNG decoration sprites instead of procedural primitives.
//! Supports multiple texture variants per decoration type for visual variety.

use macroquad::prelude::*;
use octoplat_core::level::DecorationType;

use crate::assets::DecorationAssets;

/// Manager for loading and caching decoration textures
///
/// Each decoration type can have multiple texture variants (e.g., small_rock.png,
/// small_rock_2.png, small_rock_3.png) which are randomly selected based on
/// decoration position for visual variety.
pub struct DecorationTextureManager {
    // Static decoration textures (multiple variants per type)
    small_rock: Vec<Texture2D>,
    shell: Vec<Texture2D>,
    coconut: Vec<Texture2D>,
    starfish: Vec<Texture2D>,
    wood_debris: Vec<Texture2D>,
    barrel: Vec<Texture2D>,
    anchor: Vec<Texture2D>,
    frosted_rock: Vec<Texture2D>,
    broken_column: Vec<Texture2D>,
    ancient_tile: Vec<Texture2D>,
}

impl DecorationTextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            small_rock: Vec::new(),
            shell: Vec::new(),
            coconut: Vec::new(),
            starfish: Vec::new(),
            wood_debris: Vec::new(),
            barrel: Vec::new(),
            anchor: Vec::new(),
            frosted_rock: Vec::new(),
            broken_column: Vec::new(),
            ancient_tile: Vec::new(),
        }
    }

    /// Load all available decoration textures including variants
    pub async fn load_all(&mut self) {
        self.small_rock = load_decoration_variants("ocean_depths", "small_rock").await;
        self.shell = load_decoration_variants("coral_reefs", "shell").await;
        self.coconut = load_decoration_variants("tropical_shore", "coconut").await;
        self.starfish = load_decoration_variants("tropical_shore", "starfish").await;
        self.wood_debris = load_decoration_variants("shipwreck", "wood_debris").await;
        self.barrel = load_decoration_variants("shipwreck", "barrel").await;
        self.anchor = load_decoration_variants("shipwreck", "anchor").await;
        self.frosted_rock = load_decoration_variants("arctic_waters", "frosted_rock").await;
        self.broken_column = load_decoration_variants("sunken_ruins", "broken_column").await;
        self.ancient_tile = load_decoration_variants("sunken_ruins", "ancient_tile").await;
    }

    /// Get a texture variant for a decoration type
    ///
    /// Uses a seed value (typically derived from position) to deterministically
    /// select a variant, ensuring the same decoration always gets the same variant.
    pub fn get(&self, deco_type: DecorationType, seed: u32) -> Option<&Texture2D> {
        let variants = match deco_type {
            DecorationType::SmallRock => &self.small_rock,
            DecorationType::Shell => &self.shell,
            DecorationType::Coconut => &self.coconut,
            DecorationType::Starfish => &self.starfish,
            DecorationType::WoodDebris => &self.wood_debris,
            DecorationType::Barrel => &self.barrel,
            DecorationType::Anchor => &self.anchor,
            DecorationType::FrostedRock => &self.frosted_rock,
            DecorationType::BrokenColumn => &self.broken_column,
            DecorationType::AncientTile => &self.ancient_tile,
            _ => return None, // Animated decorations don't have textures yet
        };

        if variants.is_empty() {
            return None;
        }

        // Select variant based on seed
        let index = (seed as usize) % variants.len();
        Some(&variants[index])
    }

    /// Check if textures are available for a decoration type
    pub fn has_texture(&self, deco_type: DecorationType) -> bool {
        match deco_type {
            DecorationType::SmallRock => !self.small_rock.is_empty(),
            DecorationType::Shell => !self.shell.is_empty(),
            DecorationType::Coconut => !self.coconut.is_empty(),
            DecorationType::Starfish => !self.starfish.is_empty(),
            DecorationType::WoodDebris => !self.wood_debris.is_empty(),
            DecorationType::Barrel => !self.barrel.is_empty(),
            DecorationType::Anchor => !self.anchor.is_empty(),
            DecorationType::FrostedRock => !self.frosted_rock.is_empty(),
            DecorationType::BrokenColumn => !self.broken_column.is_empty(),
            DecorationType::AncientTile => !self.ancient_tile.is_empty(),
            _ => false,
        }
    }

    /// Get the total number of loaded textures (including all variants)
    pub fn loaded_count(&self) -> usize {
        self.small_rock.len()
            + self.shell.len()
            + self.coconut.len()
            + self.starfish.len()
            + self.wood_debris.len()
            + self.barrel.len()
            + self.anchor.len()
            + self.frosted_rock.len()
            + self.broken_column.len()
            + self.ancient_tile.len()
    }

    /// Get the number of decoration types that have at least one texture
    pub fn types_with_textures(&self) -> usize {
        let mut count = 0;
        if !self.small_rock.is_empty() { count += 1; }
        if !self.shell.is_empty() { count += 1; }
        if !self.coconut.is_empty() { count += 1; }
        if !self.starfish.is_empty() { count += 1; }
        if !self.wood_debris.is_empty() { count += 1; }
        if !self.barrel.is_empty() { count += 1; }
        if !self.anchor.is_empty() { count += 1; }
        if !self.frosted_rock.is_empty() { count += 1; }
        if !self.broken_column.is_empty() { count += 1; }
        if !self.ancient_tile.is_empty() { count += 1; }
        count
    }
}

impl Default for DecorationTextureManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Load all variants of a decoration texture
///
/// Looks for: {subdir}/{name}.png, {subdir}/{name}_2.png, {subdir}/{name}_3.png, etc.
async fn load_decoration_variants(subdir: &str, name: &str) -> Vec<Texture2D> {
    let mut variants = Vec::new();

    // Try to load base texture (name.png)
    let base_path = format!("{}/{}.png", subdir, name);
    if let Some(texture) = load_decoration(&base_path).await {
        variants.push(texture);
    }

    // Try to load numbered variants (_2, _3, _4, ...)
    for i in 2..=10 {
        let variant_path = format!("{}/{}_{}.png", subdir, name, i);
        if let Some(texture) = load_decoration(&variant_path).await {
            variants.push(texture);
        } else {
            // Stop looking for more variants once we hit a gap
            break;
        }
    }

    variants
}

/// Load a decoration texture from embedded assets
async fn load_decoration(path: &str) -> Option<Texture2D> {
    if let Some(bytes) = DecorationAssets::get_image(path) {
        let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Linear);
        Some(texture)
    } else {
        None
    }
}

/// Draw a decoration using its texture
pub fn draw_decoration_texture(
    texture: &Texture2D,
    position: Vec2,
    scale: f32,
) {
    // Base size for decorations (textures are 128x128)
    let base_size = 24.0; // Similar to primitive decoration sizes
    let size = base_size * scale;

    // Center the texture on the position
    let draw_x = position.x - size / 2.0;
    let draw_y = position.y - size / 2.0;

    draw_texture_ex(
        texture,
        draw_x,
        draw_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
}

/// Generate a seed from a position for consistent variant selection
pub fn position_seed(x: f32, y: f32) -> u32 {
    // Simple hash combining x and y to get a consistent seed
    let ix = (x * 100.0) as i32;
    let iy = (y * 100.0) as i32;
    ((ix.wrapping_mul(73856093)) ^ (iy.wrapping_mul(19349663))) as u32
}

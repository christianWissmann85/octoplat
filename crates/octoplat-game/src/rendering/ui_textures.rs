//! UI texture management for loading, title, and menu screens
//!
//! Loads FLUX-generated textures for UI screens with graceful fallback
//! to procedural rendering when textures are not available.

use macroquad::prelude::*;
use octoplat_core::procgen::BiomeId;
use std::collections::HashMap;

use crate::assets::UiAssets;

/// Textures for the loading screen
pub struct LoadingScreenTextures {
    /// Underwater background image
    pub background: Option<Texture2D>,
    /// Cute octopus mascot sprite
    pub octopus: Option<Texture2D>,
}

/// Textures for the title screen
pub struct TitleScreenTextures {
    /// Underwater background image
    pub background: Option<Texture2D>,
    /// Game logo/title art
    pub logo: Option<Texture2D>,
}

/// Textures for menu screens
pub struct MenuTextures {
    /// Main menu background
    pub main_menu: Option<Texture2D>,
    /// Settings menu background
    pub settings: Option<Texture2D>,
}

/// Textures for HUD elements
pub struct HudTextures {
    /// Full heart icon for lives display
    pub heart_full: Option<Texture2D>,
    /// Empty heart icon for lives display
    pub heart_empty: Option<Texture2D>,
    /// Gem/diamond icon for collectibles counter
    pub gem: Option<Texture2D>,
    /// Decorative frame for the stamina bar
    pub stamina_frame: Option<Texture2D>,
    /// Jet boost ability icon
    pub jet_icon: Option<Texture2D>,
    /// Ink cloud ability icon
    pub ink_icon: Option<Texture2D>,
}

/// Textures for additional UI screens (level complete, game over, pause)
pub struct AdditionalUiTextures {
    /// Level complete celebration banner/background
    pub level_complete_banner: Option<Texture2D>,
    /// Game over screen background
    pub game_over_background: Option<Texture2D>,
    /// Pause menu overlay/frame
    pub pause_overlay: Option<Texture2D>,
    /// Minimap decorative frame
    pub minimap_frame: Option<Texture2D>,
    /// Biome name card background
    pub biome_card: Option<Texture2D>,
}

/// Biome thumbnail textures for biome selection
pub struct BiomeThumbnails {
    /// Thumbnails indexed by biome
    thumbnails: HashMap<BiomeId, Texture2D>,
}

impl BiomeThumbnails {
    pub fn new() -> Self {
        Self {
            thumbnails: HashMap::new(),
        }
    }

    /// Get thumbnail for a biome
    pub fn get(&self, biome: BiomeId) -> Option<&Texture2D> {
        self.thumbnails.get(&biome)
    }

    /// Check if a biome has a thumbnail
    pub fn has(&self, biome: BiomeId) -> bool {
        self.thumbnails.contains_key(&biome)
    }

    /// Check if any thumbnails are loaded
    pub fn has_any(&self) -> bool {
        !self.thumbnails.is_empty()
    }
}

impl Default for BiomeThumbnails {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for UI screen textures
pub struct UiTextureManager {
    /// Loading screen assets
    pub loading: LoadingScreenTextures,
    /// Title screen assets
    pub title: TitleScreenTextures,
    /// Menu screen assets
    pub menus: MenuTextures,
    /// HUD element assets
    pub hud: HudTextures,
    /// Additional UI screen assets (level complete, game over, pause)
    pub additional: AdditionalUiTextures,
    /// Biome thumbnails for biome selection
    pub biome_thumbnails: BiomeThumbnails,
    /// Whether textures have been loaded
    loaded: bool,
}

impl UiTextureManager {
    /// Create a new UI texture manager (textures not yet loaded)
    pub fn new() -> Self {
        Self {
            loading: LoadingScreenTextures {
                background: None,
                octopus: None,
            },
            title: TitleScreenTextures {
                background: None,
                logo: None,
            },
            menus: MenuTextures {
                main_menu: None,
                settings: None,
            },
            hud: HudTextures {
                heart_full: None,
                heart_empty: None,
                gem: None,
                stamina_frame: None,
                jet_icon: None,
                ink_icon: None,
            },
            additional: AdditionalUiTextures {
                level_complete_banner: None,
                game_over_background: None,
                pause_overlay: None,
                minimap_frame: None,
                biome_card: None,
            },
            biome_thumbnails: BiomeThumbnails::new(),
            loaded: false,
        }
    }

    /// Load all UI textures
    /// Call this early during startup, before showing the loading screen
    pub async fn load_all(&mut self) {
        if self.loaded {
            return;
        }

        // Loading screen textures
        self.loading.background = load_ui_texture("loading/background.png").await;
        self.loading.octopus = load_ui_texture("loading/octopus_idle.png").await;

        // Title screen textures
        self.title.background = load_ui_texture("title/background.png").await;
        self.title.logo = load_ui_texture("title/logo.png").await;

        // Menu background textures
        self.menus.main_menu = load_ui_texture("menus/main_menu.png").await;
        self.menus.settings = load_ui_texture("menus/settings.png").await;

        // HUD element textures
        self.hud.heart_full = load_ui_texture("hud/heart_full.png").await;
        self.hud.heart_empty = load_ui_texture("hud/heart_empty.png").await;
        self.hud.gem = load_ui_texture("hud/gem.png").await;
        self.hud.stamina_frame = load_ui_texture("hud/stamina_frame.png").await;
        self.hud.jet_icon = load_ui_texture("hud/jet_icon.png").await;
        self.hud.ink_icon = load_ui_texture("hud/ink_icon.png").await;

        // Additional UI screen textures
        self.additional.level_complete_banner =
            load_ui_texture("screens/level_complete_banner.png").await;
        self.additional.game_over_background =
            load_ui_texture("screens/game_over_background.png").await;
        self.additional.pause_overlay = load_ui_texture("screens/pause_overlay.png").await;
        self.additional.minimap_frame = load_ui_texture("hud/minimap_frame.png").await;
        self.additional.biome_card = load_ui_texture("hud/biome_card.png").await;

        // Biome thumbnails
        self.load_biome_thumbnails().await;

        self.loaded = true;
    }

    /// Load biome thumbnail textures
    async fn load_biome_thumbnails(&mut self) {
        let biomes = [
            (BiomeId::OceanDepths, "biomes/ocean_depths.png"),
            (BiomeId::CoralReefs, "biomes/coral_reefs.png"),
            (BiomeId::TropicalShore, "biomes/tropical_shore.png"),
            (BiomeId::Shipwreck, "biomes/shipwreck.png"),
            (BiomeId::ArcticWaters, "biomes/arctic_waters.png"),
            (BiomeId::VolcanicVents, "biomes/volcanic_vents.png"),
            (BiomeId::SunkenRuins, "biomes/sunken_ruins.png"),
            (BiomeId::Abyss, "biomes/abyss.png"),
        ];

        for (biome, path) in biomes {
            if let Some(texture) = load_ui_texture(path).await {
                self.biome_thumbnails.thumbnails.insert(biome, texture);
            }
        }
    }

    /// Check if UI textures have been loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Check if loading screen has any textures
    pub fn has_loading_textures(&self) -> bool {
        self.loading.background.is_some() || self.loading.octopus.is_some()
    }

    /// Check if title screen has any textures
    pub fn has_title_textures(&self) -> bool {
        self.title.background.is_some() || self.title.logo.is_some()
    }

    /// Check if menu backgrounds are available
    pub fn has_menu_textures(&self) -> bool {
        self.menus.main_menu.is_some() || self.menus.settings.is_some()
    }

    /// Check if any HUD textures are available
    pub fn has_hud_textures(&self) -> bool {
        self.hud.heart_full.is_some()
            || self.hud.gem.is_some()
            || self.hud.jet_icon.is_some()
            || self.hud.ink_icon.is_some()
    }

    /// Check if any additional UI screen textures are available
    pub fn has_additional_textures(&self) -> bool {
        self.additional.level_complete_banner.is_some()
            || self.additional.game_over_background.is_some()
            || self.additional.pause_overlay.is_some()
    }
}

impl Default for UiTextureManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Load a UI texture from embedded assets
async fn load_ui_texture(path: &str) -> Option<Texture2D> {
    if let Some(bytes) = UiAssets::get_image(path) {
        let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Linear);
        Some(texture)
    } else {
        None
    }
}

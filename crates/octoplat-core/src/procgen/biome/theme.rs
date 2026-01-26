//! Biome visual theming
//!
//! Defines colors and visual properties for each biome.

use crate::Color;

/// Geometry rendering style for biome-specific platform appearance
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GeometryStyle {
    /// Ocean Depths - wavy edges, rounded corners
    #[default]
    Standard,
    /// Coral Reefs - branching protrusions, organic shapes
    Organic,
    /// Tropical Shore - palm frond shapes, sandy textures
    Tropical,
    /// Shipwreck - tilted fragments, wood grain texture
    Broken,
    /// Arctic Waters - ice crystal formations, frosted edges
    Icy,
    /// Volcanic Vents - sharp edges, lava drip effects
    Jagged,
    /// Sunken Ruins - column shapes, carved stone patterns
    Ancient,
    /// Abyss - angular facets, bioluminescent glow
    Crystalline,
}

/// Visual theme configuration for a biome
#[derive(Clone, Copy, Debug)]
pub struct BiomeTheme {
    /// Background gradient top color
    pub bg_color_top: Color,
    /// Background gradient bottom color
    pub bg_color_bottom: Color,
    /// Color for solid blocks/walls
    pub solid_color: Color,
    /// Color for platforms (one-way, etc.)
    pub platform_color: Color,
    /// Color for hazards (spikes, etc.)
    pub hazard_color: Color,
    /// Accent color for highlights and special elements
    pub accent_color: Color,
    /// Color for ambient particles
    pub particle_color: Color,
    /// Geometry style for platform rendering
    pub geometry_style: GeometryStyle,
    /// Optional glow color for bioluminescent effects
    pub glow_color: Option<Color>,
}

impl BiomeTheme {
    /// Get the background color at a specific Y position (for gradient)
    pub fn bg_color_at(&self, y_ratio: f32) -> Color {
        let y = y_ratio.clamp(0.0, 1.0);
        Color::new(
            lerp(self.bg_color_top.r, self.bg_color_bottom.r, y),
            lerp(self.bg_color_top.g, self.bg_color_bottom.g, y),
            lerp(self.bg_color_top.b, self.bg_color_bottom.b, y),
            1.0,
        )
    }

    /// Get a slightly darker version of the solid color (for borders)
    pub fn solid_border_color(&self) -> Color {
        Color::new(
            self.solid_color.r * 0.7,
            self.solid_color.g * 0.7,
            self.solid_color.b * 0.7,
            self.solid_color.a,
        )
    }

    /// Get a slightly lighter version of the solid color (for highlights)
    pub fn solid_highlight_color(&self) -> Color {
        Color::new(
            (self.solid_color.r * 1.3).min(1.0),
            (self.solid_color.g * 1.3).min(1.0),
            (self.solid_color.b * 1.3).min(1.0),
            self.solid_color.a,
        )
    }

    /// Get platform border color
    pub fn platform_border_color(&self) -> Color {
        Color::new(
            self.platform_color.r * 0.8,
            self.platform_color.g * 0.8,
            self.platform_color.b * 0.8,
            self.platform_color.a,
        )
    }
}

impl Default for BiomeTheme {
    fn default() -> Self {
        // Default ocean theme
        Self {
            bg_color_top: Color::new(0.05, 0.1, 0.2, 1.0),
            bg_color_bottom: Color::new(0.02, 0.05, 0.1, 1.0),
            solid_color: Color::new(0.3, 0.4, 0.5, 1.0),
            platform_color: Color::new(0.4, 0.5, 0.6, 1.0),
            hazard_color: Color::new(0.9, 0.3, 0.3, 1.0),
            accent_color: Color::new(0.5, 0.8, 1.0, 1.0),
            particle_color: Color::new(0.7, 0.9, 1.0, 0.4),
            geometry_style: GeometryStyle::Standard,
            glow_color: None,
        }
    }
}

/// Linear interpolation helper
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

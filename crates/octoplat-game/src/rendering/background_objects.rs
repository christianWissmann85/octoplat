//! Background object system with parallax layers
//!
//! Provides multi-layer parallax silhouettes for visual depth per biome.

use macroquad::prelude::*;

use octoplat_core::procgen::{BiomeId, BiomeTheme};
use octoplat_core::Rng;

/// A single background layer with parallax depth
#[derive(Clone, Debug)]
pub struct BackgroundLayer {
    /// Objects in this layer
    pub objects: Vec<BackgroundObject>,
    /// Parallax depth factor (0.1 = far/slow, 0.9 = near/fast)
    pub depth: f32,
    /// Color darkening multiplier for fog/distance effect
    pub color_mult: f32,
}

impl BackgroundLayer {
    pub fn new(depth: f32) -> Self {
        Self {
            objects: Vec::new(),
            depth,
            color_mult: 1.0 - depth * 0.5, // Farther = darker
        }
    }

    /// Add an object to this layer
    pub fn add(&mut self, obj: BackgroundObject) {
        self.objects.push(obj);
    }
}

/// Types of background objects
#[derive(Clone, Debug)]
pub enum BackgroundObject {
    /// Irregular rock silhouette
    RockFormation {
        x: f32,
        width: f32,
        height: f32,
        seed: u32,
    },
    /// Cluster of underwater plants
    PlantCluster {
        x: f32,
        plant_type: PlantType,
        count: u8,
    },
    /// Distant structure silhouette
    DistantStructure {
        x: f32,
        structure_type: StructureType,
    },
    /// Floating particle clouds
    ParticleCloud {
        x: f32,
        density: u8,
    },
    /// Glowing orb (for Abyss)
    GlowOrb {
        x: f32,
        y_offset: f32,
        size: f32,
    },
}

/// Plant types for background vegetation
#[derive(Clone, Copy, Debug)]
pub enum PlantType {
    Kelp,
    Coral,
    Seaweed,
}

/// Structure types for distant silhouettes
#[derive(Clone, Copy, Debug)]
pub enum StructureType {
    ShipHull,
    ShipMast,
    RockPillar,
    CrystalSpire,
    VolcanicPlume,
}

/// Complete background configuration for a level
#[derive(Clone, Debug)]
pub struct BiomeBackground {
    /// Far layer (slowest parallax)
    pub far_layer: BackgroundLayer,
    /// Mid layer (medium parallax)
    pub mid_layer: BackgroundLayer,
    /// Near layer (fastest parallax, closest to gameplay)
    pub near_layer: BackgroundLayer,
}

impl BiomeBackground {
    /// Create a new biome background with the given layers
    pub fn new(far: BackgroundLayer, mid: BackgroundLayer, near: BackgroundLayer) -> Self {
        Self {
            far_layer: far,
            mid_layer: mid,
            near_layer: near,
        }
    }

    /// Generate background for a biome with the given dimensions
    pub fn generate(biome: BiomeId, level_width: f32, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut next_random = || rng.next_u64();

        match biome {
            BiomeId::OceanDepths => Self::generate_ocean_depths(level_width, &mut next_random),
            BiomeId::CoralReefs => Self::generate_coral_reefs(level_width, &mut next_random),
            BiomeId::TropicalShore => Self::generate_tropical(level_width, &mut next_random),
            BiomeId::Shipwreck => Self::generate_shipwreck(level_width, &mut next_random),
            BiomeId::ArcticWaters => Self::generate_arctic(level_width, &mut next_random),
            BiomeId::VolcanicVents => Self::generate_volcanic(level_width, &mut next_random),
            BiomeId::SunkenRuins => Self::generate_ruins(level_width, &mut next_random),
            BiomeId::Abyss => Self::generate_abyss(level_width, &mut next_random),
        }
    }

    fn generate_ocean_depths(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Rock silhouettes
        let rock_count = (level_width / 200.0) as i32 + 2;
        for i in 0..rock_count {
            let x = i as f32 * 200.0 + (rng() % 100) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 80.0 + (rng() % 60) as f32,
                height: 100.0 + (rng() % 80) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Kelp forests
        let kelp_count = (level_width / 150.0) as i32 + 3;
        for i in 0..kelp_count {
            let x = i as f32 * 150.0 + (rng() % 80) as f32;
            mid.add(BackgroundObject::PlantCluster {
                x,
                plant_type: PlantType::Kelp,
                count: 3 + (rng() % 4) as u8,
            });
        }

        // Near: Floating particles
        let particle_count = (level_width / 300.0) as i32 + 2;
        for i in 0..particle_count {
            let x = i as f32 * 300.0 + (rng() % 150) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 5 + (rng() % 5) as u8,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_coral_reefs(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Reef formations
        let reef_count = (level_width / 180.0) as i32 + 2;
        for i in 0..reef_count {
            let x = i as f32 * 180.0 + (rng() % 90) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 100.0 + (rng() % 80) as f32,
                height: 60.0 + (rng() % 50) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Coral clusters
        let coral_count = (level_width / 120.0) as i32 + 4;
        for i in 0..coral_count {
            let x = i as f32 * 120.0 + (rng() % 60) as f32;
            mid.add(BackgroundObject::PlantCluster {
                x,
                plant_type: PlantType::Coral,
                count: 4 + (rng() % 4) as u8,
            });
        }

        // Near: Swaying seaweed
        let seaweed_count = (level_width / 100.0) as i32 + 3;
        for i in 0..seaweed_count {
            let x = i as f32 * 100.0 + (rng() % 50) as f32;
            near.add(BackgroundObject::PlantCluster {
                x,
                plant_type: PlantType::Seaweed,
                count: 2 + (rng() % 3) as u8,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_shipwreck(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Ship hull silhouettes
        let hull_count = (level_width / 400.0) as i32 + 1;
        for i in 0..hull_count {
            let x = i as f32 * 400.0 + (rng() % 200) as f32;
            far.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::ShipHull,
            });
        }

        // Mid: Mast silhouettes
        let mast_count = (level_width / 200.0) as i32 + 2;
        for i in 0..mast_count {
            let x = i as f32 * 200.0 + (rng() % 100) as f32;
            mid.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::ShipMast,
            });
        }

        // Near: Floating debris particles
        let debris_count = (level_width / 250.0) as i32 + 2;
        for i in 0..debris_count {
            let x = i as f32 * 250.0 + (rng() % 125) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 3 + (rng() % 4) as u8,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_volcanic(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Lava glow / distant volcanic plumes
        let plume_count = (level_width / 350.0) as i32 + 1;
        for i in 0..plume_count {
            let x = i as f32 * 350.0 + (rng() % 175) as f32;
            far.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::VolcanicPlume,
            });
        }

        // Mid: Rock pillars
        let pillar_count = (level_width / 150.0) as i32 + 3;
        for i in 0..pillar_count {
            let x = i as f32 * 150.0 + (rng() % 75) as f32;
            mid.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::RockPillar,
            });
        }

        // Near: Steam/ash particles
        let ash_count = (level_width / 200.0) as i32 + 3;
        for i in 0..ash_count {
            let x = i as f32 * 200.0 + (rng() % 100) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 6 + (rng() % 5) as u8,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_abyss(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Void shapes / distant rocks
        let void_count = (level_width / 300.0) as i32 + 2;
        for i in 0..void_count {
            let x = i as f32 * 300.0 + (rng() % 150) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 120.0 + (rng() % 80) as f32,
                height: 150.0 + (rng() % 100) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Crystal formations
        let crystal_count = (level_width / 180.0) as i32 + 3;
        for i in 0..crystal_count {
            let x = i as f32 * 180.0 + (rng() % 90) as f32;
            mid.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::CrystalSpire,
            });
        }

        // Near: Bioluminescent orbs
        let orb_count = (level_width / 150.0) as i32 + 4;
        for i in 0..orb_count {
            let x = i as f32 * 150.0 + (rng() % 75) as f32;
            let y_offset = ((rng() % 200) as f32) - 100.0;
            near.add(BackgroundObject::GlowOrb {
                x,
                y_offset,
                size: 8.0 + (rng() % 12) as f32,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_tropical(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Island silhouettes (rock formations work for this)
        let island_count = (level_width / 250.0) as i32 + 2;
        for i in 0..island_count {
            let x = i as f32 * 250.0 + (rng() % 125) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 100.0 + (rng() % 80) as f32,
                height: 60.0 + (rng() % 40) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Palm tree silhouettes (using kelp/seaweed clusters)
        let palm_count = (level_width / 130.0) as i32 + 4;
        for i in 0..palm_count {
            let x = i as f32 * 130.0 + (rng() % 65) as f32;
            mid.add(BackgroundObject::PlantCluster {
                x,
                plant_type: PlantType::Kelp, // Kelp works for palm silhouettes
                count: 2 + (rng() % 3) as u8,
            });
        }

        // Near: Gentle particles (bubbles/light motes)
        let particle_count = (level_width / 200.0) as i32 + 3;
        for i in 0..particle_count {
            let x = i as f32 * 200.0 + (rng() % 100) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 3 + (rng() % 4) as u8,
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_arctic(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Iceberg silhouettes
        let iceberg_count = (level_width / 220.0) as i32 + 2;
        for i in 0..iceberg_count {
            let x = i as f32 * 220.0 + (rng() % 110) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 90.0 + (rng() % 70) as f32,
                height: 80.0 + (rng() % 60) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Crystal spires (ice pillars)
        let ice_count = (level_width / 160.0) as i32 + 3;
        for i in 0..ice_count {
            let x = i as f32 * 160.0 + (rng() % 80) as f32;
            mid.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::CrystalSpire,
            });
        }

        // Near: Aurora glow orbs (northern lights effect)
        let aurora_count = (level_width / 180.0) as i32 + 4;
        for i in 0..aurora_count {
            let x = i as f32 * 180.0 + (rng() % 90) as f32;
            let y_offset = ((rng() % 150) as f32) - 120.0; // Higher in the sky
            near.add(BackgroundObject::GlowOrb {
                x,
                y_offset,
                size: 15.0 + (rng() % 20) as f32, // Larger for aurora effect
            });
        }

        // Also add some snow particles
        let snow_count = (level_width / 250.0) as i32 + 2;
        for i in 0..snow_count {
            let x = i as f32 * 250.0 + (rng() % 125) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 8 + (rng() % 6) as u8, // Dense snow
            });
        }

        Self::new(far, mid, near)
    }

    fn generate_ruins(level_width: f32, rng: &mut impl FnMut() -> u64) -> Self {
        let mut far = BackgroundLayer::new(0.15);
        let mut mid = BackgroundLayer::new(0.4);
        let mut near = BackgroundLayer::new(0.7);

        // Far: Ancient temple/structure silhouettes
        let temple_count = (level_width / 350.0) as i32 + 1;
        for i in 0..temple_count {
            let x = i as f32 * 350.0 + (rng() % 175) as f32;
            far.add(BackgroundObject::RockFormation {
                x,
                width: 150.0 + (rng() % 100) as f32,
                height: 120.0 + (rng() % 80) as f32,
                seed: rng() as u32,
            });
        }

        // Mid: Column/pillar silhouettes
        let pillar_count = (level_width / 140.0) as i32 + 4;
        for i in 0..pillar_count {
            let x = i as f32 * 140.0 + (rng() % 70) as f32;
            mid.add(BackgroundObject::DistantStructure {
                x,
                structure_type: StructureType::RockPillar, // Pillars work for columns
            });
        }

        // Near: Mystical glow orbs
        let orb_count = (level_width / 200.0) as i32 + 3;
        for i in 0..orb_count {
            let x = i as f32 * 200.0 + (rng() % 100) as f32;
            let y_offset = ((rng() % 180) as f32) - 90.0;
            near.add(BackgroundObject::GlowOrb {
                x,
                y_offset,
                size: 10.0 + (rng() % 15) as f32,
            });
        }

        // Mystical particles
        let particle_count = (level_width / 300.0) as i32 + 2;
        for i in 0..particle_count {
            let x = i as f32 * 300.0 + (rng() % 150) as f32;
            near.add(BackgroundObject::ParticleCloud {
                x,
                density: 4 + (rng() % 4) as u8,
            });
        }

        Self::new(far, mid, near)
    }
}

/// Draw all background layers with parallax
pub fn draw_biome_background_layers(
    background: &BiomeBackground,
    theme: &BiomeTheme,
    camera_pos: Vec2,
    screen_size: Vec2,
    time: f32,
) {
    // Draw layers back to front (far -> mid -> near)
    draw_layer(&background.far_layer, theme, camera_pos, screen_size, time);
    draw_layer(&background.mid_layer, theme, camera_pos, screen_size, time);
    draw_layer(&background.near_layer, theme, camera_pos, screen_size, time);
}

/// Draw a single parallax layer
fn draw_layer(
    layer: &BackgroundLayer,
    theme: &BiomeTheme,
    camera_pos: Vec2,
    screen_size: Vec2,
    time: f32,
) {
    let parallax_offset_x = camera_pos.x * layer.depth;
    let base_y = screen_size.y * 0.7; // Objects sit in lower portion of screen

    // Calculate fog color based on depth
    let fog_color = Color::new(
        theme.bg_color_bottom.r * layer.color_mult,
        theme.bg_color_bottom.g * layer.color_mult,
        theme.bg_color_bottom.b * layer.color_mult,
        0.6 + layer.depth * 0.3,
    );

    for obj in &layer.objects {
        draw_background_object(obj, parallax_offset_x, base_y, fog_color, theme, time, layer.depth);
    }
}

/// Draw a single background object with parallax offset
fn draw_background_object(
    obj: &BackgroundObject,
    parallax_offset: f32,
    base_y: f32,
    fog_color: Color,
    theme: &BiomeTheme,
    time: f32,
    depth: f32,
) {
    match obj {
        BackgroundObject::RockFormation { x, width, height, seed } => {
            let screen_x = x - parallax_offset;
            draw_rock_silhouette(screen_x, base_y, *width, *height, *seed, fog_color);
        }
        BackgroundObject::PlantCluster { x, plant_type, count } => {
            let screen_x = x - parallax_offset;
            draw_plant_cluster(screen_x, base_y, *plant_type, *count, fog_color, time);
        }
        BackgroundObject::DistantStructure { x, structure_type } => {
            let screen_x = x - parallax_offset;
            draw_structure_silhouette(screen_x, base_y, *structure_type, fog_color, theme, time);
        }
        BackgroundObject::ParticleCloud { x, density } => {
            let screen_x = x - parallax_offset;
            draw_particle_cloud(screen_x, base_y, *density, fog_color, time, depth);
        }
        BackgroundObject::GlowOrb { x, y_offset, size } => {
            let screen_x = x - parallax_offset;
            draw_glow_orb(screen_x, base_y + y_offset, *size, theme, time);
        }
    }
}

fn draw_rock_silhouette(x: f32, base_y: f32, width: f32, height: f32, seed: u32, color: Color) {
    // Use seed to create consistent but varied rock shapes
    let variation = (seed % 10) as f32 / 10.0;

    // Simple irregular rock using overlapping shapes
    let y = base_y + 20.0;
    draw_ellipse(x, y, width * 0.5, height * 0.3, 0.0, color);
    draw_ellipse(x - width * 0.2, y + height * 0.1, width * 0.4, height * 0.4, 0.0, color);
    draw_ellipse(x + width * 0.15, y - height * 0.1 * variation, width * 0.35, height * 0.35, 0.0, color);

    // Top peak
    draw_triangle(
        vec2(x - width * 0.3, y - height * 0.2),
        vec2(x + width * 0.2, y - height * 0.15),
        vec2(x - width * 0.05, y - height * (0.5 + variation * 0.3)),
        color,
    );
}

fn draw_plant_cluster(x: f32, base_y: f32, plant_type: PlantType, count: u8, color: Color, time: f32) {
    for i in 0..count {
        let offset_x = (i as f32 - count as f32 / 2.0) * 15.0;
        let plant_x = x + offset_x;
        let sway = (time * 0.8 + i as f32 * 0.3).sin() * 3.0;

        match plant_type {
            PlantType::Kelp => {
                let height = 60.0 + (i as f32 * 7.0) % 20.0;
                for seg in 0..5 {
                    let seg_sway = (time * 0.6 + seg as f32 * 0.2).sin() * 4.0;
                    let seg_y = base_y - seg as f32 * (height / 5.0);
                    draw_line(
                        plant_x + seg_sway * 0.5,
                        seg_y,
                        plant_x + sway + seg_sway,
                        seg_y - height / 5.0,
                        3.0 - seg as f32 * 0.4,
                        color,
                    );
                }
            }
            PlantType::Coral => {
                let height = 40.0 + (i as f32 * 5.0) % 15.0;
                // Branching coral shape
                draw_line(plant_x, base_y, plant_x + sway * 0.5, base_y - height, 4.0, color);
                draw_line(
                    plant_x + sway * 0.3,
                    base_y - height * 0.5,
                    plant_x + sway + 15.0,
                    base_y - height * 0.7,
                    3.0,
                    color,
                );
                draw_line(
                    plant_x + sway * 0.3,
                    base_y - height * 0.6,
                    plant_x + sway - 12.0,
                    base_y - height * 0.8,
                    2.5,
                    color,
                );
            }
            PlantType::Seaweed => {
                let height = 35.0 + (i as f32 * 4.0) % 12.0;
                for seg in 0..4 {
                    let seg_sway = (time * 1.2 + seg as f32 * 0.4 + i as f32).sin() * 5.0;
                    draw_line(
                        plant_x + seg_sway * 0.3,
                        base_y - seg as f32 * (height / 4.0),
                        plant_x + seg_sway,
                        base_y - (seg + 1) as f32 * (height / 4.0),
                        2.0 - seg as f32 * 0.3,
                        color,
                    );
                }
            }
        }
    }
}

fn draw_structure_silhouette(
    x: f32,
    base_y: f32,
    structure_type: StructureType,
    color: Color,
    theme: &BiomeTheme,
    time: f32,
) {
    match structure_type {
        StructureType::ShipHull => {
            // Ship hull silhouette
            let hull_width = 200.0;
            let hull_height = 80.0;
            let y = base_y + 30.0;

            // Hull body
            draw_triangle(
                vec2(x - hull_width * 0.5, y),
                vec2(x + hull_width * 0.5, y),
                vec2(x + hull_width * 0.3, y + hull_height),
                color,
            );
            draw_rectangle(x - hull_width * 0.4, y - hull_height * 0.3, hull_width * 0.8, hull_height * 0.5, color);
        }
        StructureType::ShipMast => {
            // Broken mast silhouette
            let y = base_y;
            draw_line(x, y, x + 5.0, y - 120.0, 6.0, color);
            // Broken crossbeam
            draw_line(x - 30.0, y - 80.0, x + 35.0, y - 75.0, 4.0, color);
            // Tattered sail suggestion
            let sail_alpha = color.a * 0.5;
            let sail_color = Color::new(color.r, color.g, color.b, sail_alpha);
            draw_triangle(
                vec2(x + 5.0, y - 90.0),
                vec2(x + 5.0, y - 50.0),
                vec2(x + 30.0, y - 60.0),
                sail_color,
            );
        }
        StructureType::RockPillar => {
            let y = base_y + 10.0;
            let height = 100.0;
            // Jagged pillar
            draw_triangle(
                vec2(x - 20.0, y),
                vec2(x + 25.0, y),
                vec2(x + 5.0, y - height),
                color,
            );
            draw_triangle(
                vec2(x - 15.0, y - height * 0.3),
                vec2(x + 10.0, y - height * 0.4),
                vec2(x - 5.0, y - height * 0.8),
                color,
            );
        }
        StructureType::CrystalSpire => {
            let y = base_y;
            let height = 90.0;
            // Main crystal
            draw_triangle(
                vec2(x - 15.0, y),
                vec2(x + 15.0, y),
                vec2(x, y - height),
                color,
            );
            // Side crystals
            draw_triangle(
                vec2(x - 25.0, y),
                vec2(x - 10.0, y),
                vec2(x - 18.0, y - height * 0.6),
                color,
            );
            draw_triangle(
                vec2(x + 10.0, y),
                vec2(x + 22.0, y),
                vec2(x + 15.0, y - height * 0.5),
                color,
            );

            // Glow effect for crystals
            if let Some(glow) = theme.glow_color {
                let pulse = (time * 1.5).sin() * 0.2 + 0.3;
                let glow_color = Color::new(glow.r, glow.g, glow.b, pulse * 0.4);
                draw_circle(x, y - height * 0.5, 15.0, glow_color);
            }
        }
        StructureType::VolcanicPlume => {
            let y = base_y + 40.0;
            // Volcanic vent base
            draw_triangle(
                vec2(x - 60.0, y),
                vec2(x + 60.0, y),
                vec2(x, y - 100.0),
                color,
            );

            // Glowing lava effect
            if let Some(glow) = theme.glow_color {
                let pulse = (time * 2.0).sin() * 0.3 + 0.5;
                let lava_color = Color::new(glow.r, glow.g * 0.5, glow.b * 0.2, pulse * 0.6);
                draw_circle(x, y - 30.0, 25.0, lava_color);

                // Rising heat distortion
                for i in 0..3 {
                    let rise = ((time * 0.5 + i as f32 * 0.3) % 1.0) * 80.0;
                    let alpha = (1.0 - rise / 80.0) * 0.3;
                    draw_circle(
                        x + (time * 2.0 + i as f32).sin() * 10.0,
                        y - 50.0 - rise,
                        10.0 + rise * 0.2,
                        Color::new(lava_color.r, lava_color.g, lava_color.b, alpha),
                    );
                }
            }
        }
    }
}

fn draw_particle_cloud(x: f32, base_y: f32, density: u8, color: Color, time: f32, depth: f32) {
    let particle_color = Color::new(color.r, color.g, color.b, color.a * 0.4);

    for i in 0..density {
        let phase = i as f32 * 0.7;
        let drift_x = (time * 0.3 + phase).sin() * 20.0;
        let drift_y = (time * 0.2 + phase * 1.3).cos() * 15.0;
        let offset_x = ((i as f32 * 17.3) % 60.0) - 30.0;
        let offset_y = ((i as f32 * 23.7) % 80.0) - 40.0;

        let size = 2.0 + (i as f32 * 0.5) % 3.0;
        let px = x + offset_x + drift_x;
        let py = base_y + offset_y + drift_y - 50.0 * (1.0 - depth);

        draw_circle(px, py, size, particle_color);
    }
}

fn draw_glow_orb(x: f32, y: f32, size: f32, theme: &BiomeTheme, time: f32) {
    if let Some(glow) = theme.glow_color {
        let pulse = (time * 1.2).sin() * 0.3 + 0.7;
        let drift_x = (time * 0.5).sin() * 8.0;
        let drift_y = (time * 0.3).cos() * 5.0;

        let orb_x = x + drift_x;
        let orb_y = y + drift_y;

        // Outer glow layers
        for i in 0..4 {
            let layer_size = size * (1.0 + i as f32 * 0.4);
            let alpha = (0.25 - i as f32 * 0.05) * pulse;
            draw_circle(orb_x, orb_y, layer_size, Color::new(glow.r, glow.g, glow.b, alpha));
        }

        // Core
        draw_circle(orb_x, orb_y, size * 0.4, Color::new(glow.r, glow.g, glow.b, 0.8 * pulse));
    }
}

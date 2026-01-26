//! Screen transition effects

use macroquad::prelude::*;

use crate::rendering::easing::{ease_out_quad, ease_in_out_cubic};

/// Screen transition state
pub struct Transition {
    /// Progress (0.0 = start, 1.0 = complete)
    pub progress: f32,
    /// Total duration
    pub duration: f32,
    /// Whether the midpoint has been reached (for state switch)
    pub midpoint_reached: bool,
}

impl Transition {
    pub fn new(duration: f32) -> Self {
        Self {
            progress: 0.0,
            duration,
            midpoint_reached: false,
        }
    }

    /// Update transition, returns true when complete
    pub fn update(&mut self, dt: f32) -> bool {
        self.progress += dt / self.duration;

        if self.progress >= 0.5 && !self.midpoint_reached {
            self.midpoint_reached = true;
        }

        self.progress >= 1.0
    }

    /// Get fade alpha (0.0 = visible, 1.0 = black)
    pub fn fade_alpha(&self) -> f32 {
        if self.progress < 0.5 {
            // Fade out
            self.progress * 2.0
        } else {
            // Fade in
            (1.0 - self.progress) * 2.0
        }
    }

    /// Check if we should switch states (at midpoint)
    pub fn should_switch(&self) -> bool {
        self.midpoint_reached
    }
}

/// Draw a fade overlay (black with alpha)
pub fn draw_fade_overlay(alpha: f32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, alpha),
    );
}

// ============================================================================
// Death/Respawn Transition
// ============================================================================

/// Phase of the death/respawn transition
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathTransitionPhase {
    /// Initial impact - red flash, brief pause
    Impact,
    /// Ink splatter expanding from death point
    InkSplatter,
    /// Fade to black
    FadeOut,
    /// Waiting at black (state switch happens here)
    Hold,
    /// Fade back in at respawn point
    FadeIn,
    /// Swirl effect at respawn location
    RespawnSwirl,
    /// Transition complete
    Complete,
}

/// Death/respawn transition with multiple visual phases
pub struct DeathTransition {
    /// Current phase
    pub phase: DeathTransitionPhase,
    /// Timer for current phase
    pub timer: f32,
    /// Death position (screen space for overlay, world space available)
    pub death_pos: Option<Vec2>,
    /// Respawn position
    pub respawn_pos: Option<Vec2>,
    /// Whether state switch has occurred
    pub state_switched: bool,
}

impl DeathTransition {
    /// Phase durations
    const IMPACT_DURATION: f32 = 0.15;
    const INK_DURATION: f32 = 0.35;
    const FADE_OUT_DURATION: f32 = 0.25;
    const HOLD_DURATION: f32 = 0.1;
    const FADE_IN_DURATION: f32 = 0.3;
    const SWIRL_DURATION: f32 = 0.4;

    pub fn new() -> Self {
        Self {
            phase: DeathTransitionPhase::Complete,
            timer: 0.0,
            death_pos: None,
            respawn_pos: None,
            state_switched: false,
        }
    }

    /// Start the death transition
    pub fn start(&mut self, death_pos: Vec2) {
        self.phase = DeathTransitionPhase::Impact;
        self.timer = Self::IMPACT_DURATION;
        self.death_pos = Some(death_pos);
        self.respawn_pos = None;
        self.state_switched = false;
    }

    /// Set respawn position (called when respawn happens)
    pub fn set_respawn_pos(&mut self, pos: Vec2) {
        self.respawn_pos = Some(pos);
    }

    /// Check if transition is active
    pub fn is_active(&self) -> bool {
        self.phase != DeathTransitionPhase::Complete
    }

    /// Check if we should switch state (respawn)
    pub fn should_switch_state(&self) -> bool {
        self.phase == DeathTransitionPhase::Hold && !self.state_switched
    }

    /// Mark state as switched
    pub fn mark_switched(&mut self) {
        self.state_switched = true;
    }

    /// Update transition, advancing phases
    pub fn update(&mut self, dt: f32) {
        if self.phase == DeathTransitionPhase::Complete {
            return;
        }

        self.timer -= dt;
        if self.timer <= 0.0 {
            // Advance to next phase
            self.phase = match self.phase {
                DeathTransitionPhase::Impact => {
                    self.timer = Self::INK_DURATION;
                    DeathTransitionPhase::InkSplatter
                }
                DeathTransitionPhase::InkSplatter => {
                    self.timer = Self::FADE_OUT_DURATION;
                    DeathTransitionPhase::FadeOut
                }
                DeathTransitionPhase::FadeOut => {
                    self.timer = Self::HOLD_DURATION;
                    DeathTransitionPhase::Hold
                }
                DeathTransitionPhase::Hold => {
                    self.timer = Self::FADE_IN_DURATION;
                    DeathTransitionPhase::FadeIn
                }
                DeathTransitionPhase::FadeIn => {
                    self.timer = Self::SWIRL_DURATION;
                    DeathTransitionPhase::RespawnSwirl
                }
                DeathTransitionPhase::RespawnSwirl => {
                    DeathTransitionPhase::Complete
                }
                DeathTransitionPhase::Complete => DeathTransitionPhase::Complete,
            };
        }
    }

    /// Get phase progress (0.0 to 1.0)
    fn phase_progress(&self) -> f32 {
        let duration = match self.phase {
            DeathTransitionPhase::Impact => Self::IMPACT_DURATION,
            DeathTransitionPhase::InkSplatter => Self::INK_DURATION,
            DeathTransitionPhase::FadeOut => Self::FADE_OUT_DURATION,
            DeathTransitionPhase::Hold => Self::HOLD_DURATION,
            DeathTransitionPhase::FadeIn => Self::FADE_IN_DURATION,
            DeathTransitionPhase::RespawnSwirl => Self::SWIRL_DURATION,
            DeathTransitionPhase::Complete => 1.0,
        };
        if duration > 0.0 {
            1.0 - (self.timer / duration).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// Draw the transition overlay (call in screen space)
    pub fn draw(&self) {
        match self.phase {
            DeathTransitionPhase::Impact => {
                self.draw_impact_phase();
            }
            DeathTransitionPhase::InkSplatter => {
                self.draw_ink_phase();
            }
            DeathTransitionPhase::FadeOut => {
                self.draw_fade_out_phase();
            }
            DeathTransitionPhase::Hold => {
                // Full black
                draw_fade_overlay(1.0);
            }
            DeathTransitionPhase::FadeIn => {
                self.draw_fade_in_phase();
            }
            DeathTransitionPhase::RespawnSwirl => {
                self.draw_swirl_phase();
            }
            DeathTransitionPhase::Complete => {}
        }
    }

    fn draw_impact_phase(&self) {
        let progress = self.phase_progress();

        // Red flash vignette that fades quickly
        let flash_alpha = (1.0 - progress) * 0.6;

        // Draw red vignette from edges
        let sw = screen_width();
        let sh = screen_height();
        let vignette_size = 150.0 * (1.0 - progress * 0.5);

        // Top edge
        draw_rectangle(0.0, 0.0, sw, vignette_size, Color::new(0.8, 0.1, 0.1, flash_alpha * 0.8));
        // Bottom edge
        draw_rectangle(0.0, sh - vignette_size, sw, vignette_size, Color::new(0.8, 0.1, 0.1, flash_alpha * 0.8));
        // Left edge
        draw_rectangle(0.0, 0.0, vignette_size, sh, Color::new(0.8, 0.1, 0.1, flash_alpha * 0.6));
        // Right edge
        draw_rectangle(sw - vignette_size, 0.0, vignette_size, sh, Color::new(0.8, 0.1, 0.1, flash_alpha * 0.6));

        // Brief white flash at very start
        if progress < 0.3 {
            let white_alpha = (0.3 - progress) / 0.3 * 0.4;
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(1.0, 1.0, 1.0, white_alpha));
        }
    }

    fn draw_ink_phase(&self) {
        let progress = ease_out_quad(self.phase_progress());
        let sw = screen_width();
        let sh = screen_height();

        // Ink splatter expanding from center (or death position if available)
        let center = self.death_pos.map(|_| {
            // Convert world pos to rough screen center - just use screen center
            vec2(sw / 2.0, sh / 2.0)
        }).unwrap_or(vec2(sw / 2.0, sh / 2.0));

        // Multiple ink blobs expanding outward
        let blob_count = 12;
        for i in 0..blob_count {
            let angle = (i as f32 / blob_count as f32) * std::f32::consts::TAU;
            let offset_angle = angle + progress * 0.5; // Slight rotation as it expands

            // Each blob expands at slightly different rate
            let blob_progress = (progress + (i as f32 * 0.03)).min(1.0);
            let max_dist = (sw.max(sh)) * 0.8;
            let dist = blob_progress * max_dist;

            // Blob size grows then shrinks at edges
            let size_factor = if blob_progress < 0.7 {
                blob_progress / 0.7
            } else {
                1.0 - (blob_progress - 0.7) / 0.3 * 0.3
            };
            let blob_size = 80.0 + size_factor * 120.0 + (i as f32 * 10.0);

            let blob_pos = center + vec2(offset_angle.cos(), offset_angle.sin()) * dist;

            // Dark purple/black ink color
            let ink_color = Color::new(0.1, 0.05, 0.15, 0.9);
            draw_circle(blob_pos.x, blob_pos.y, blob_size, ink_color);
        }

        // Central ink mass
        let central_size = progress * sw.max(sh) * 0.6;
        draw_circle(center.x, center.y, central_size, Color::new(0.05, 0.02, 0.1, 0.95));

        // Overlay darkening
        let overlay_alpha = progress * 0.7;
        draw_fade_overlay(overlay_alpha);
    }

    fn draw_fade_out_phase(&self) {
        let progress = ease_in_out_cubic(self.phase_progress());
        // Continue from ink phase's darkness level to full black
        let alpha = 0.7 + progress * 0.3;
        draw_fade_overlay(alpha);
    }

    fn draw_fade_in_phase(&self) {
        let progress = ease_in_out_cubic(self.phase_progress());
        // Fade from black to visible
        let alpha = 1.0 - progress;
        draw_fade_overlay(alpha);
    }

    fn draw_swirl_phase(&self) {
        let progress = self.phase_progress();
        let sw = screen_width();
        let sh = screen_height();

        // Respawn swirl at screen center (respawn position would need camera transform)
        let center = vec2(sw / 2.0, sh / 2.0);

        // Swirling particles converging then dispersing
        let particle_count = 16;
        for i in 0..particle_count {
            let base_angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;

            // Particles spiral inward then outward
            let spiral_progress = if progress < 0.5 {
                // Converging
                1.0 - progress * 2.0
            } else {
                // Dispersing
                (progress - 0.5) * 2.0
            };

            let spin_angle = base_angle + progress * std::f32::consts::TAU * 2.0;
            let dist = spiral_progress * 100.0 + 10.0;
            let particle_pos = center + vec2(spin_angle.cos(), spin_angle.sin()) * dist;

            // Particle fades out as it disperses
            let particle_alpha = if progress < 0.5 {
                progress * 2.0
            } else {
                1.0 - (progress - 0.5) * 2.0
            };

            let size = 4.0 + (1.0 - spiral_progress) * 6.0;

            // Cyan/teal respawn color
            draw_circle(
                particle_pos.x,
                particle_pos.y,
                size,
                Color::new(0.3, 0.9, 0.8, particle_alpha * 0.8),
            );
        }

        // Central glow
        if progress < 0.7 {
            let glow_alpha = if progress < 0.5 {
                progress * 2.0 * 0.5
            } else {
                (0.7 - progress) / 0.2 * 0.5
            };
            for layer in 0..4 {
                let layer_radius = 20.0 + layer as f32 * 15.0;
                let layer_alpha = glow_alpha * (1.0 - layer as f32 * 0.2);
                draw_circle(center.x, center.y, layer_radius, Color::new(0.4, 1.0, 0.9, layer_alpha));
            }
        }
    }
}

impl Default for DeathTransition {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Level Transition (Dive/Surface)
// ============================================================================

/// Direction of level transition
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LevelTransitionDirection {
    /// Going deeper (next level)
    Dive,
    /// Going up (previous level / surface)
    Surface,
}

/// Level transition with dive/surface animation
pub struct LevelTransition {
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Direction of transition
    pub direction: LevelTransitionDirection,
    /// Total duration
    pub duration: f32,
    /// Whether midpoint reached (for level switch)
    pub midpoint_reached: bool,
}

impl LevelTransition {
    pub fn new(direction: LevelTransitionDirection, duration: f32) -> Self {
        Self {
            progress: 0.0,
            direction,
            duration,
            midpoint_reached: false,
        }
    }

    /// Update transition, returns true when complete
    pub fn update(&mut self, dt: f32) -> bool {
        self.progress += dt / self.duration;

        if self.progress >= 0.5 && !self.midpoint_reached {
            self.midpoint_reached = true;
        }

        self.progress >= 1.0
    }

    /// Check if we should switch levels (at midpoint)
    pub fn should_switch(&self) -> bool {
        self.midpoint_reached
    }

    /// Draw the level transition effect
    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();

        match self.direction {
            LevelTransitionDirection::Dive => {
                // Wipe from top to bottom then back
                let wipe_progress = if self.progress < 0.5 {
                    ease_out_quad(self.progress * 2.0)
                } else {
                    1.0 - ease_out_quad((self.progress - 0.5) * 2.0)
                };

                let wipe_height = wipe_progress * sh;

                // Dark blue wipe (diving deeper)
                draw_rectangle(0.0, 0.0, sw, wipe_height, Color::new(0.02, 0.05, 0.15, 1.0));

                // Bubble particles rising through the wipe
                if self.progress < 0.6 {
                    for i in 0..8 {
                        let bubble_x = (i as f32 / 8.0) * sw + (self.progress * 50.0).sin() * 20.0;
                        let bubble_y = wipe_height - 30.0 - (i as f32 * 15.0) - self.progress * 100.0;
                        if bubble_y > 0.0 && bubble_y < wipe_height {
                            let size = 3.0 + (i as f32 * 0.5);
                            draw_circle(bubble_x, bubble_y, size, Color::new(0.5, 0.7, 0.9, 0.6));
                        }
                    }
                }
            }
            LevelTransitionDirection::Surface => {
                // Wipe from bottom to top then back
                let wipe_progress = if self.progress < 0.5 {
                    ease_out_quad(self.progress * 2.0)
                } else {
                    1.0 - ease_out_quad((self.progress - 0.5) * 2.0)
                };

                let wipe_height = wipe_progress * sh;

                // Light blue wipe (surfacing)
                draw_rectangle(0.0, sh - wipe_height, sw, wipe_height, Color::new(0.2, 0.5, 0.7, 1.0));

                // Light rays coming down
                if self.progress > 0.3 && self.progress < 0.7 {
                    let ray_alpha = ((self.progress - 0.3) / 0.2).min(1.0) * ((0.7 - self.progress) / 0.2).min(1.0);
                    for i in 0..5 {
                        let ray_x = (i as f32 / 5.0) * sw + sw * 0.1;
                        let ray_width = 30.0 + i as f32 * 10.0;
                        draw_triangle(
                            vec2(ray_x, 0.0),
                            vec2(ray_x - ray_width, sh - wipe_height),
                            vec2(ray_x + ray_width, sh - wipe_height),
                            Color::new(1.0, 1.0, 0.9, ray_alpha * 0.3),
                        );
                    }
                }
            }
        }
    }
}

// ============================================================================
// Menu Slide Transition
// ============================================================================

/// Direction for menu slide transitions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlideDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Menu slide transition state
pub struct MenuSlideTransition {
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Slide direction
    pub direction: SlideDirection,
    /// Duration
    pub duration: f32,
    /// Whether midpoint reached
    pub midpoint_reached: bool,
}

impl MenuSlideTransition {
    pub fn new(direction: SlideDirection, duration: f32) -> Self {
        Self {
            progress: 0.0,
            direction,
            duration,
            midpoint_reached: false,
        }
    }

    /// Update transition, returns true when complete
    pub fn update(&mut self, dt: f32) -> bool {
        self.progress += dt / self.duration;

        if self.progress >= 0.5 && !self.midpoint_reached {
            self.midpoint_reached = true;
        }

        self.progress >= 1.0
    }

    /// Get the slide offset for the old content (slides out)
    pub fn old_content_offset(&self) -> Vec2 {
        let progress = ease_in_out_cubic(self.progress.min(0.5) * 2.0);
        let sw = screen_width();
        let sh = screen_height();

        match self.direction {
            SlideDirection::Left => vec2(-progress * sw, 0.0),
            SlideDirection::Right => vec2(progress * sw, 0.0),
            SlideDirection::Up => vec2(0.0, -progress * sh),
            SlideDirection::Down => vec2(0.0, progress * sh),
        }
    }

    /// Get the slide offset for the new content (slides in)
    pub fn new_content_offset(&self) -> Vec2 {
        if self.progress < 0.5 {
            // New content not visible yet
            let sw = screen_width();
            let sh = screen_height();
            match self.direction {
                SlideDirection::Left => vec2(sw, 0.0),
                SlideDirection::Right => vec2(-sw, 0.0),
                SlideDirection::Up => vec2(0.0, sh),
                SlideDirection::Down => vec2(0.0, -sh),
            }
        } else {
            let progress = ease_in_out_cubic((self.progress - 0.5) * 2.0);
            let sw = screen_width();
            let sh = screen_height();

            match self.direction {
                SlideDirection::Left => vec2(sw * (1.0 - progress), 0.0),
                SlideDirection::Right => vec2(-sw * (1.0 - progress), 0.0),
                SlideDirection::Up => vec2(0.0, sh * (1.0 - progress)),
                SlideDirection::Down => vec2(0.0, -sh * (1.0 - progress)),
            }
        }
    }

    /// Check if at midpoint (time to switch content)
    pub fn should_switch(&self) -> bool {
        self.midpoint_reached
    }

    /// Draw a wipe overlay effect for menu transitions
    pub fn draw_overlay(&self) {
        let sw = screen_width();
        let sh = screen_height();

        // Wipe progress: expands to cover, then retracts
        let wipe_progress = if self.progress < 0.5 {
            ease_in_out_cubic(self.progress * 2.0)
        } else {
            1.0 - ease_in_out_cubic((self.progress - 0.5) * 2.0)
        };

        // Deep blue/teal color for menu wipes
        let wipe_color = Color::new(0.05, 0.12, 0.2, 0.95);

        match self.direction {
            SlideDirection::Left => {
                // Wipe from right to left
                let wipe_width = wipe_progress * sw;
                draw_rectangle(sw - wipe_width, 0.0, wipe_width, sh, wipe_color);
            }
            SlideDirection::Right => {
                // Wipe from left to right
                let wipe_width = wipe_progress * sw;
                draw_rectangle(0.0, 0.0, wipe_width, sh, wipe_color);
            }
            SlideDirection::Up => {
                // Wipe from bottom to top
                let wipe_height = wipe_progress * sh;
                draw_rectangle(0.0, sh - wipe_height, sw, wipe_height, wipe_color);
            }
            SlideDirection::Down => {
                // Wipe from top to bottom
                let wipe_height = wipe_progress * sh;
                draw_rectangle(0.0, 0.0, sw, wipe_height, wipe_color);
            }
        }
    }
}

// ============================================================================
// Biome Transition
// ============================================================================

use octoplat_core::procgen::BiomeId;

/// Biome transition effect shown when entering a new biome
pub struct BiomeTransition {
    /// The biome being entered
    pub to_biome: BiomeId,
    /// Current progress (0-1)
    pub progress: f32,
    /// Total duration
    pub duration: f32,
    /// Text display timer
    pub text_fade: f32,
}

impl BiomeTransition {
    /// Create a new biome transition
    pub fn new(to_biome: BiomeId, duration: f32) -> Self {
        Self {
            to_biome,
            progress: 0.0,
            duration,
            text_fade: 1.0,
        }
    }

    /// Update the transition, returns true when complete
    pub fn update(&mut self, dt: f32) -> bool {
        self.progress += dt / self.duration;
        if self.progress >= 1.0 {
            self.progress = 1.0;
            return true;
        }
        false
    }

    /// Check if we're in the visible text phase (middle portion)
    pub fn is_text_visible(&self) -> bool {
        self.progress > 0.2 && self.progress < 0.8
    }

    /// Get the biome's theme color
    fn get_biome_color(biome: BiomeId) -> Color {
        match biome {
            BiomeId::OceanDepths => Color::new(0.1, 0.2, 0.4, 1.0),      // Deep blue
            BiomeId::CoralReefs => Color::new(0.9, 0.5, 0.6, 1.0),       // Coral pink
            BiomeId::TropicalShore => Color::new(0.2, 0.7, 0.8, 1.0),    // Tropical teal
            BiomeId::Shipwreck => Color::new(0.3, 0.25, 0.2, 1.0),       // Rusty brown
            BiomeId::ArcticWaters => Color::new(0.7, 0.85, 0.95, 1.0),   // Icy blue-white
            BiomeId::VolcanicVents => Color::new(0.8, 0.3, 0.1, 1.0),    // Volcanic orange
            BiomeId::SunkenRuins => Color::new(0.4, 0.5, 0.35, 1.0),     // Ancient green
            BiomeId::Abyss => Color::new(0.05, 0.02, 0.1, 1.0),          // Near black
        }
    }

    /// Get the display name for a biome
    fn get_biome_name(biome: BiomeId) -> &'static str {
        match biome {
            BiomeId::OceanDepths => "Ocean Depths",
            BiomeId::CoralReefs => "Coral Reefs",
            BiomeId::TropicalShore => "Tropical Shore",
            BiomeId::Shipwreck => "Shipwreck",
            BiomeId::ArcticWaters => "Arctic Waters",
            BiomeId::VolcanicVents => "Volcanic Vents",
            BiomeId::SunkenRuins => "Sunken Ruins",
            BiomeId::Abyss => "The Abyss",
        }
    }

    /// Draw the biome transition effect
    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();

        // Color overlay with fade in/out
        let biome_color = Self::get_biome_color(self.to_biome);

        // Overlay alpha: fade in during first 30%, solid middle, fade out last 30%
        let overlay_alpha = if self.progress < 0.3 {
            ease_in_out_cubic(self.progress / 0.3) * 0.5
        } else if self.progress > 0.7 {
            ease_in_out_cubic(1.0 - (self.progress - 0.7) / 0.3) * 0.5
        } else {
            0.5
        };

        // Draw color overlay
        let overlay_color = Color::new(biome_color.r, biome_color.g, biome_color.b, overlay_alpha);
        draw_rectangle(0.0, 0.0, sw, sh, overlay_color);

        // Draw "Entering [Biome]" text in the middle portion
        if self.is_text_visible() {
            let text_progress = (self.progress - 0.2) / 0.6; // 0-1 within text phase
            let text_alpha = if text_progress < 0.2 {
                ease_in_out_cubic(text_progress / 0.2)
            } else if text_progress > 0.8 {
                ease_in_out_cubic(1.0 - (text_progress - 0.8) / 0.2)
            } else {
                1.0
            };

            let biome_name = Self::get_biome_name(self.to_biome);
            let entering_text = "Entering";

            // Draw "Entering" text
            let entering_size = 24.0;
            let entering_dims = measure_text(entering_text, None, entering_size as u16, 1.0);
            let entering_x = (sw - entering_dims.width) / 2.0;
            let entering_y = sh / 2.0 - 20.0;

            draw_text(
                entering_text,
                entering_x,
                entering_y,
                entering_size,
                Color::new(1.0, 1.0, 1.0, text_alpha * 0.8),
            );

            // Draw biome name (larger)
            let name_size = 48.0;
            let name_dims = measure_text(biome_name, None, name_size as u16, 1.0);
            let name_x = (sw - name_dims.width) / 2.0;
            let name_y = sh / 2.0 + 30.0;

            // Draw shadow
            draw_text(
                biome_name,
                name_x + 2.0,
                name_y + 2.0,
                name_size,
                Color::new(0.0, 0.0, 0.0, text_alpha * 0.5),
            );

            // Draw main text with biome tint
            let tint = Color::new(
                0.5 + biome_color.r * 0.5,
                0.5 + biome_color.g * 0.5,
                0.5 + biome_color.b * 0.5,
                text_alpha,
            );
            draw_text(biome_name, name_x, name_y, name_size, tint);
        }

        // Draw swirling particles at edges during transition
        self.draw_particles();
    }

    /// Draw decorative particles for the transition
    fn draw_particles(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let biome_color = Self::get_biome_color(self.to_biome);

        // Particle count based on progress (more in middle)
        let particle_intensity = if self.progress < 0.5 {
            self.progress * 2.0
        } else {
            (1.0 - self.progress) * 2.0
        };

        let num_particles = (20.0 * particle_intensity) as i32;

        for i in 0..num_particles {
            let seed = i as f32 * 137.5 + self.progress * 500.0;
            let x = (seed.sin() * 0.5 + 0.5) * sw;
            let y = (seed.cos() * 0.5 + 0.5) * sh;
            let size = 2.0 + (seed * 0.1).sin().abs() * 4.0;
            let alpha = particle_intensity * 0.6 * ((seed * 0.2).sin() * 0.5 + 0.5);

            draw_circle(
                x, y, size,
                Color::new(biome_color.r, biome_color.g, biome_color.b, alpha),
            );
        }
    }
}

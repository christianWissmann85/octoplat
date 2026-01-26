//! Playing state handler
//!
//! Handles gameplay update logic and rendering.
//!
//! The update logic is split into focused sub-functions for maintainability:
//! - `update_minimap_input` - minimap toggle and zoom controls
//! - `update_death` - death state and respawn/game over transitions
//! - `update_environment` - breakable blocks, moving/crumbling platforms
//! - `update_player_physics` - player movement and collision
//! - `update_hazards` - hazard tile collision detection
//! - `update_enemies` - enemy AI and player-enemy collisions
//! - `update_collectibles` - gem collection and milestone rewards
//! - `update_checkpoints` - checkpoint activation
//! - `update_level_progress` - exit detection, level complete, fall death
//! - `update_effects` - particles and shader effects

use std::sync::atomic::{AtomicBool, Ordering};

use macroquad::prelude::*;

use crate::app::{GameAction, GameActions};
use crate::audio::SoundId;
use crate::compat::{rect_to_mq, ToMqVec2};
use crate::game_state::GameState;
use crate::gameplay;
use crate::rendering;

// ============================================================================
// Update Sub-Functions
// ============================================================================

/// Handle minimap toggle and zoom input
fn update_minimap_input(game: &GameState) -> GameActions {
    let mut actions = GameActions::new();

    if game.input.minimap_toggle_pressed {
        actions.push(GameAction::ToggleMinimap);
    }

    if game.ui.minimap_visible {
        const ZOOM_STEP: f32 = 0.5;

        if game.input.minimap_zoom_in_pressed {
            actions.push(GameAction::AdjustMinimapScale(ZOOM_STEP));
        }
        if game.input.minimap_zoom_out_pressed {
            actions.push(GameAction::AdjustMinimapScale(-ZOOM_STEP));
        }
    }

    actions
}

/// Update the death state
///
/// Returns actions for respawn or game over.
pub fn update_death(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    if game.gameplay.death.is_dead
        && game.gameplay.death.update(dt) {
            if game.progression.lives.is_game_over() {
                actions.push(GameAction::GameOver);
            } else {
                actions.push(GameAction::Respawn);
            }
        }

    actions
}

/// Update environment: breakable blocks and dynamic platforms
fn update_environment(game: &mut GameState, dt: f32) {
    // Check breakable block collision BEFORE player update
    if let Some(tilemap) = game.level.manager.tilemap() {
        gameplay::check_breakable_blocks(
            &mut game.gameplay.player,
            tilemap,
            &mut game.gameplay.level_env.destroyed_blocks,
            &game.gameplay.config,
        );
    }

    // Update moving platforms
    gameplay::update_moving_platforms(
        game.gameplay.level_env.moving_platforms.values_mut(),
        &game.gameplay.config,
        dt,
    );

    // Update crumbling platforms
    gameplay::update_crumbling_platforms(
        game.gameplay.level_env.crumbling_platforms.values_mut(),
        &game.gameplay.config,
        dt,
    );

    // Apply platform movement to player if riding
    gameplay::apply_platform_movement(
        &mut game.gameplay.player,
        game.gameplay.level_env.moving_platforms.values(),
        dt,
    );
}

/// Update player physics and visual effects
fn update_player_physics(game: &mut GameState, dt: f32) {
    if let Some(tilemap) = game.level.manager.tilemap() {
        let crumbling_rects = game.gameplay.level_env.solid_crumbling_rects();
        game.gameplay.player.update(
            &mut game.input,
            tilemap,
            &game.gameplay.level_env.grapple_points,
            &game.gameplay.config,
            dt,
            &game.gameplay.level_env.destroyed_blocks,
            &crumbling_rects,
        );
    }

    game.gameplay.player.update_visual_effects(dt);

    // Handle dynamic platform collisions (after player physics)
    gameplay::handle_platform_collisions(
        &mut game.gameplay.player,
        game.gameplay.level_env.moving_platforms.values(),
        game.gameplay.level_env.crumbling_platforms.values_mut(),
        &game.gameplay.config,
    );
}

/// Check hazard collision - returns TriggerDeath action if hit
fn update_hazards(game: &GameState) -> Option<GameAction> {
    if let Some(tilemap) = game.level.manager.tilemap() {
        if gameplay::check_hazard_collision(&game.gameplay.player, tilemap) {
            return Some(GameAction::TriggerDeath);
        }
    }
    None
}

/// Update enemies and check collisions
///
/// Returns Some(TriggerDeath) if player died, None otherwise.
/// Note: Also handles enemy-killed case internally.
fn update_enemies(game: &mut GameState, dt: f32) -> Option<GameAction> {
    // Update enemy AI
    if let Some(tilemap) = game.level.manager.tilemap() {
        for crab in game.gameplay.level_env.crabs.values_mut() {
            crab.update(tilemap, &game.gameplay.config, dt);
        }
    }
    for puffer in game.gameplay.level_env.pufferfish.values_mut() {
        puffer.update(&game.gameplay.config, dt);
    }

    // Check collisions
    let collision_result = gameplay::check_enemy_collision(
        &mut game.gameplay.player,
        game.gameplay.level_env.crabs.values_mut(),
        game.gameplay.level_env.pufferfish.values_mut(),
        &game.gameplay.config,
    );

    match collision_result {
        gameplay::EnemyCollisionResult::PlayerDied => Some(GameAction::TriggerDeath),
        gameplay::EnemyCollisionResult::EnemyKilled | gameplay::EnemyCollisionResult::None => None,
    }
}

/// Update camera to follow player
fn update_camera(game: &mut GameState, dt: f32, level_bounds: Rect) {
    game.gameplay.camera.update(
        game.gameplay.player.position,
        game.gameplay.player.velocity,
        dt,
        level_bounds,
        &game.gameplay.config,
    );
}

/// Update gem collection and milestone rewards
fn update_collectibles(game: &mut GameState) -> GameActions {
    let mut actions = GameActions::new();

    // Collect gems
    let player_rect = game.gameplay.player.collision_rect();
    let mut gems_collected_this_frame = 0u32;
    for gem in game.gameplay.level_env.gems.values_mut() {
        if gem.check_collection(player_rect) {
            gems_collected_this_frame += 1;
        }
    }
    game.gameplay.level_env.gems_collected += gems_collected_this_frame;

    // RogueLite mode: check for gem milestone extra life
    if game.progression.is_in_roguelite_run() && gems_collected_this_frame > 0 {
        let total_gems = game.progression.roguelite.total_gems + game.gameplay.level_env.gems_collected;
        if game.progression.lives.check_gem_milestone(
            total_gems,
            game.gameplay.config.endless_gem_milestone,
            game.gameplay.config.max_lives,
        ) {
            actions.push(GameAction::PlaySound(SoundId::ExtraLife));
            game.fx.effects.spawn_extra_life(game.gameplay.player.position);
        }
    }

    actions
}

/// Proximity threshold for checkpoint/exit/pool activation (in pixels)
const ACTIVATION_RADIUS: f32 = 24.0;

/// Update checkpoint activation
fn update_checkpoints(game: &mut GameState) -> GameActions {
    let mut actions = GameActions::new();
    let player_pos = game.gameplay.player.position;

    let mut new_checkpoint: Option<Vec2> = None;
    for &checkpoint_pos in &game.gameplay.level_env.checkpoint_positions {
        if (player_pos - checkpoint_pos).length() < ACTIVATION_RADIUS {
            let core_pos = octoplat_core::Vec2::new(checkpoint_pos.x, checkpoint_pos.y);
            if game.fx.feedback.prev_checkpoint != Some(core_pos) {
                new_checkpoint = Some(checkpoint_pos);
            }
            game.level.manager.set_checkpoint(checkpoint_pos);
        }
    }

    if let Some(checkpoint_pos) = new_checkpoint {
        let core_pos = octoplat_core::Vec2::new(checkpoint_pos.x, checkpoint_pos.y);
        game.fx.feedback.set_checkpoint(Some(core_pos));
        actions.push(GameAction::PlaySound(SoundId::Checkpoint));
        game.fx.effects.spawn_checkpoint(checkpoint_pos);

        // Award extra life at checkpoints (RogueLite mode handles this via gem milestones)
        if !game.progression.is_in_roguelite_run() {
            actions.push(GameAction::AwardExtraLife);
        }
    }

    actions
}

/// Check water pools for charge refill
fn update_water_pools(game: &mut GameState) {
    for &pool_pos in &game.gameplay.level_env.water_pool_positions {
        if (game.gameplay.player.position - pool_pos).length() < ACTIVATION_RADIUS {
            game.gameplay.player.refill_charges(&game.gameplay.config);
        }
    }
}

/// Update level progress: exit detection, level complete, fall death
fn update_level_progress(game: &mut GameState, dt: f32, level_bounds: Rect) -> GameActions {
    let mut actions = GameActions::new();

    // Check level exit
    if let Some(exit_pos) = game.gameplay.level_env.exit_position {
        let at_exit = (game.gameplay.player.position - exit_pos).length() < ACTIVATION_RADIUS;
        if at_exit && !game.gameplay.level_env.level_complete {
            actions.push(GameAction::MarkLevelComplete);
        }
    }

    // Note: Level complete sound is now played immediately in MarkLevelComplete action handler

    // Handle level complete transition (with small delay for visual feedback)
    if game.gameplay.level_env.level_complete {
        game.gameplay.level_env.show_level_text -= dt;
        if game.gameplay.level_env.show_level_text <= 2.0 {
            if game.progression.is_in_roguelite_run() {
                actions.push(GameAction::CompleteRogueliteLevel);
            } else {
                actions.push(GameAction::NextLevel);
            }
        }
    }

    // Update level text timer
    if game.gameplay.level_env.show_level_text > 0.0 {
        game.gameplay.level_env.show_level_text -= dt;
    }

    // Die if fallen off level
    if gameplay::check_fall_death(&game.gameplay.player, level_bounds) {
        actions.push(GameAction::TriggerDeath);
    }

    actions
}

/// Update visual and shader effects
fn update_effects(game: &mut GameState, dt: f32) {
    game.fx.effects.update(dt);
    game.fx.shaders.update_chromatic(dt);
}

// ============================================================================
// Main Update Function
// ============================================================================

/// Update gameplay for Playing state
///
/// Returns actions to execute. This is the core gameplay loop.
pub fn update(game: &mut GameState, dt: f32) -> GameActions {
    let mut actions = GameActions::new();

    // Handle minimap input
    actions.extend(update_minimap_input(game));

    // Handle death state first (blocks other updates)
    if game.gameplay.death.is_dead {
        return update_death(game, dt);
    }

    // Get level bounds
    let level_bounds = game.level.manager
        .tilemap()
        .map(|tm| rect_to_mq(tm.bounds()))
        .unwrap_or(Rect::new(0.0, 0.0, 800.0, 600.0));

    // Update environment (breakables, platforms)
    update_environment(game, dt);

    // Update player physics
    update_player_physics(game, dt);

    // Check hazard collision (early return if dead)
    if let Some(death_action) = update_hazards(game) {
        actions.push(death_action);
        return actions;
    }

    // Update enemies (early return if player died)
    if let Some(death_action) = update_enemies(game, dt) {
        actions.push(death_action);
        return actions;
    }

    // Update camera
    update_camera(game, dt, level_bounds);

    // Update collectibles (gems, milestones)
    actions.extend(update_collectibles(game));

    // Update checkpoints
    actions.extend(update_checkpoints(game));

    // Update water pools
    update_water_pools(game);

    // Update level progress (exit, complete, fall death)
    actions.extend(update_level_progress(game, dt, level_bounds));

    // Update visual effects
    update_effects(game, dt);

    // Update cached HUD strings (only if values changed)
    update_hud_cache(game);

    actions
}

/// Render gameplay
pub fn render(game: &GameState, time: f32) {
    let level_bounds = game.level.manager
        .tilemap()
        .map(|tm| rect_to_mq(tm.bounds()))
        .unwrap_or(Rect::new(0.0, 0.0, 800.0, 600.0));

    // Draw background - use biome theme in roguelite mode (the only game mode now)
    let active_theme = if game.progression.is_in_roguelite_run() {
        Some(&game.progression.roguelite.biome_progression.current().theme)
    } else {
        None
    };

    if let Some(theme) = active_theme {
        let current_biome = game.progression.roguelite.biome_progression.current().id;
        let screen_size = vec2(screen_width(), screen_height());

        // Draw background: texture if available, otherwise gradient + procedural layers
        if let Some(texture) = game.level.background_textures.get(current_biome) {
            // FLUX texture already has depth - no procedural layers needed
            rendering::draw_textured_background(texture, game.gameplay.camera.position, screen_size);
        } else {
            // Fallback: gradient + procedural silhouettes for depth
            rendering::draw_biome_background(theme);
            if let Some(ref biome_bg) = game.level.biome_background {
                rendering::draw_biome_background_layers(
                    biome_bg,
                    theme,
                    game.gameplay.camera.position,
                    screen_size,
                    time,
                );
            }
        }
    } else {
        clear_background(Color::new(0.05, 0.1, 0.2, 1.0));
    }

    // Draw parallax background (screen space, before camera) - campaign mode only
    if active_theme.is_none() {
        if let Some(ref bg) = game.level.background {
            bg.draw(
                game.gameplay.camera.position,
                vec2(level_bounds.w, level_bounds.h),
                time,
            );
        }
    }

    // Set game camera for world rendering (with screen shake offset if enabled)
    let mut cam = game.gameplay.camera.to_camera2d();
    if game.progression.save_manager.data.screen_shake_enabled {
        cam.target.x += game.fx.effects.shake.offset.x;
        cam.target.y += game.fx.effects.shake.offset.y;
    }
    set_camera(&cam);

    // Draw level tiles - use biome theme in roguelite mode or editor playtest
    if let Some(tilemap) = game.level.manager.tilemap() {
        if let Some(theme) = active_theme {
            let current_biome = game.progression.roguelite.biome_progression.current().id;
            let tile_texture = game.level.tile_textures.get(current_biome);
            let spike_texture = game.level.tile_textures.get_spike();
            rendering::draw_tilemap_themed(tilemap, &game.gameplay.level_env.destroyed_blocks, theme, time, tile_texture, spike_texture);
        } else {
            rendering::draw_tilemap(tilemap, &game.gameplay.level_env.destroyed_blocks);
        }
    }

    // Draw decorations (after tilemap, before grapple points)
    // Debug: log decoration count once per session
    #[cfg(debug_assertions)]
    {
        static DECO_DEBUG_LOGGED: AtomicBool = AtomicBool::new(false);
        if !DECO_DEBUG_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[Decorations] level_env has {} decorations to draw",
                game.gameplay.level_env.decorations.len()
            );
        }
    }

    if !game.gameplay.level_env.decorations.is_empty() {
        // Use active theme or fall back to default Ocean Depths theme
        let default_theme = crate::procgen::BiomeId::OceanDepths.definition().theme;
        let theme = active_theme.unwrap_or(&default_theme);
        // Use texture-based decorations when available, fall back to primitives
        rendering::draw_decorations_with_textures(
            &game.gameplay.level_env.decorations,
            theme,
            time,
            &game.level.decoration_textures,
        );
    }

    // Draw water pools
    rendering::draw_water_pools(&game.gameplay.level_env.water_pool_positions, time);

    // Draw checkpoints
    rendering::draw_checkpoints(
        &game.gameplay.level_env.checkpoint_positions,
        game.level.manager.checkpoint(),
        time,
    );

    // Draw exit
    rendering::draw_exit(game.gameplay.level_env.exit_position, time);

    // Draw grapple points
    rendering::draw_grapple_points(&game.gameplay.level_env.grapple_points, &game.gameplay.player, &game.gameplay.config);

    // Draw dynamic platforms
    for platform in game.gameplay.level_env.moving_platforms.values() {
        rendering::draw_moving_platform(platform, time);
    }
    for platform in game.gameplay.level_env.crumbling_platforms.values() {
        rendering::draw_crumbling_platform(platform, time);
    }

    // Draw enemies
    for crab in game.gameplay.level_env.crabs.values() {
        rendering::draw_crab(crab, time);
    }
    for puffer in game.gameplay.level_env.pufferfish.values() {
        rendering::draw_pufferfish(puffer, time);
    }

    // Draw gems
    for gem in game.gameplay.level_env.gems.values() {
        rendering::draw_gem(gem, time);
    }

    // Draw tentacle line if swinging
    rendering::draw_tentacle(&game.gameplay.player);

    // Draw player (unless dead)
    if !game.gameplay.death.is_dead {
        rendering::draw_player(&game.gameplay.player, &game.gameplay.config, time);
    }

    // Draw particle effects
    game.fx.effects.draw();

    // Draw death effect
    if game.gameplay.death.is_dead {
        if let Some(death_pos) = game.gameplay.death.position {
            let progress = game.gameplay.death.animation_progress(game.gameplay.config.death_animation_time);
            rendering::draw_death_effect(death_pos.to_mq_vec2(), progress);
        }
    }

    // Reset to screen space for UI
    set_default_camera();

    // Apply chromatic aberration post-effect
    game.fx.shaders.apply_chromatic();

    // Draw ambient biome particles when themed (behind HUD)
    if let Some(theme) = active_theme {
        rendering::draw_biome_particles(theme, time, 30);
    }

    // Draw HUD
    rendering::draw_hud(
        game.gameplay.level_env.gems_collected,
        game.gameplay.level_env.total_gems,
        &game.gameplay.player,
        &game.gameplay.config,
        game.progression.lives.current,
        game.progression.is_in_roguelite_run(),
        Some(&game.level.ui_textures.hud),
    );

    // Draw minimap (if visible and tilemap exists)
    if game.ui.minimap_visible {
        if let Some(tilemap) = game.level.manager.tilemap() {
            rendering::draw_minimap(
                tilemap,
                &game.gameplay.player,
                &game.gameplay.level_env,
                game.progression.save_manager.data.minimap_size,
                game.progression.save_manager.data.minimap_scale,
                game.progression.save_manager.data.minimap_opacity,
                time,
                game.level.ui_textures.additional.minimap_frame.as_ref(),
            );
        }
    }

    // Draw biome name card when starting level
    if game.gameplay.level_env.show_level_text > 0.0 && game.gameplay.level_env.show_level_text < 3.0 {
        draw_biome_name_card(game, time);
    }

    // Draw level complete text
    if game.gameplay.level_env.level_complete {
        let text = "Level Complete!";
        let text_width = measure_text(text, None, 64, 1.0).width;
        draw_text(
            text,
            (screen_width() - text_width) / 2.0,
            screen_height() / 2.0,
            64.0,
            Color::new(1.0, 0.9, 0.3, 1.0),
        );
    }

    // Draw roguelite info in top-right corner
    render_roguelite_hud(game);

    // Draw seed input dialog
    render_seed_input(game);

    // Draw keybind hints (bottom-left, only when not in seed input)
    if !game.ui.seed_input.active {
        let hints = if game.progression.is_in_roguelite_run() {
            "F2-4:Difficulty F5:Export F6:Seed F8:Exit Run"
        } else {
            "F2:Standard F3:Casual F4:Challenge F5:Export F6:Seed F7:RogueLite"
        };
        draw_text(hints, 10.0, screen_height() - 10.0, 12.0, Color::new(0.5, 0.6, 0.7, 0.6));
    }

    // Debug info
    #[cfg(debug_assertions)]
    rendering::draw_debug(&format!("{:?}", game.gameplay.player.state), game.gameplay.player.velocity, get_fps());
}

/// Update cached HUD strings (only when values change to avoid per-frame allocations)
fn update_hud_cache(game: &mut GameState) {
    if !game.progression.is_in_roguelite_run() {
        return;
    }

    let cache = &mut game.ui.hud_cache;

    // Update roguelite text if values changed
    let level = game.progression.roguelite.biome_progression.total_levels() + 1;
    let total_gems = game.progression.roguelite.total_gems + game.gameplay.level_env.gems_collected;
    let roguelite_key = (level, total_gems);
    if cache.roguelite_key != roguelite_key {
        cache.roguelite_text = format!(
            "ROGUELITE - Level {} | Total Gems: {}",
            level, total_gems
        );
        cache.roguelite_key = roguelite_key;
    }

    // Update biome text if values changed
    let biome = game.progression.roguelite.biome_progression.current();
    let biome_progress = (game.progression.roguelite.biome_progression.biome_progress() * 100.0) as u32;
    let run_progress = (game.progression.roguelite.biome_progression.run_progress() * 100.0) as u32;
    let is_boss = game.progression.roguelite.biome_progression.is_boss_level();
    let biome_key = (biome.id, biome_progress, run_progress, is_boss);
    if cache.biome_key != biome_key {
        let boss_indicator = if is_boss { " [BOSS]" } else { "" };
        let all_biomes = crate::procgen::BiomeId::all();
        let biome_index = all_biomes.iter().position(|&b| b == biome.id).unwrap_or(0) + 1;
        cache.biome_text = format!(
            "{} ({}/{}) {} ({}%) | Run: {}%",
            biome.name, biome_index, all_biomes.len(), boss_indicator, biome_progress, run_progress
        );
        cache.biome_key = biome_key;
    }

    // Update seed text if values changed
    let seed_key = (game.procgen_seed, game.progression.roguelite.preset);
    if cache.seed_key != seed_key {
        if let Some(seed) = game.procgen_seed {
            let preset_str = match game.progression.roguelite.preset {
                crate::procgen::DifficultyPreset::Casual => "Casual",
                crate::procgen::DifficultyPreset::Standard => "Standard",
                crate::procgen::DifficultyPreset::Challenge => "Challenge",
            };
            cache.seed_text = format!("Seed: {} ({})", seed, preset_str);
        } else {
            cache.seed_text.clear();
        }
        cache.seed_key = seed_key;
    }
}

/// Render roguelite HUD info (uses cached strings from update phase)
fn render_roguelite_hud(game: &GameState) {
    if !game.progression.is_in_roguelite_run() {
        return;
    }

    let cache = &game.ui.hud_cache;
    let mut y_offset = 20.0;

    // RogueLite mode stats
    if !cache.roguelite_text.is_empty() {
        let text_width = measure_text(&cache.roguelite_text, None, 18, 1.0).width;
        draw_text(
            &cache.roguelite_text,
            screen_width() - text_width - 10.0,
            y_offset,
            18.0,
            Color::new(1.0, 0.8, 0.3, 0.9),
        );
        y_offset += 20.0;
    }

    // Biome progress display
    if !cache.biome_text.is_empty() {
        let text_width = measure_text(&cache.biome_text, None, 16, 1.0).width;
        draw_text(
            &cache.biome_text,
            screen_width() - text_width - 10.0,
            y_offset,
            16.0,
            Color::new(0.6, 0.9, 0.8, 0.85),
        );
        y_offset += 20.0;
    }

    // Seed display
    if !cache.seed_text.is_empty() {
        let text_width = measure_text(&cache.seed_text, None, 16, 1.0).width;
        draw_text(
            &cache.seed_text,
            screen_width() - text_width - 10.0,
            y_offset,
            16.0,
            Color::new(0.7, 0.8, 0.9, 0.8),
        );
    }
}

/// Render seed input dialog
fn render_seed_input(game: &GameState) {
    if !game.ui.seed_input.active {
        return;
    }

    // Darken background
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

    // Dialog box
    let box_w = 300.0;
    let box_h = 100.0;
    let box_x = (screen_width() - box_w) / 2.0;
    let box_y = (screen_height() - box_h) / 2.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.15, 0.2, 0.3, 0.95));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.4, 0.5, 0.6, 1.0));

    // Title
    draw_text("Enter Seed:", box_x + 20.0, box_y + 30.0, 20.0, WHITE);

    // Input field
    let input_display = if game.ui.seed_input.buffer.is_empty() {
        "_ _ _ _ _ _ _ _ _ _".to_string()
    } else {
        game.ui.seed_input.buffer.clone()
    };
    draw_text(&input_display, box_x + 20.0, box_y + 55.0, 24.0, Color::new(0.9, 1.0, 0.9, 1.0));

    // Instructions
    draw_text("Enter=Confirm  Esc=Cancel", box_x + 20.0, box_y + 85.0, 14.0, Color::new(0.6, 0.7, 0.8, 0.8));
}

/// Draw the biome name card when entering a level
fn draw_biome_name_card(game: &GameState, time: f32) {
    let show_time = game.gameplay.level_env.show_level_text;

    // Calculate alpha with smooth fade in/out
    // Fade in for first 0.5s, hold for 1.5s, fade out for last 1s
    let alpha = if show_time > 2.5 {
        (3.0 - show_time) / 0.5 // Fade in
    } else if show_time > 1.0 {
        1.0 // Hold
    } else {
        show_time // Fade out
    };

    let sw = screen_width();
    let sh = screen_height();

    // Card dimensions
    let card_w = 400.0;
    let card_h = 100.0;
    let card_x = (sw - card_w) / 2.0;
    let card_y = sh * 0.25;

    // Slide-in effect during fade-in
    let slide_offset = if show_time > 2.5 {
        (3.0 - show_time) / 0.5 * -20.0 + 20.0
    } else {
        0.0
    };
    let card_y = card_y + slide_offset;

    // Get biome info
    let biome = game.progression.roguelite.biome_progression.current();
    let biome_name = biome.name;
    // Use solid_color as the representative biome color
    let core_color = biome.theme.solid_color;
    let biome_color = Color::new(core_color.r, core_color.g, core_color.b, 1.0);
    let level_name = game.level.manager.level_name();

    // Draw card background with texture or procedural
    if let Some(card_texture) = game.level.ui_textures.additional.biome_card.as_ref() {
        draw_texture_ex(
            card_texture,
            card_x,
            card_y,
            Color::new(1.0, 1.0, 1.0, alpha),
            DrawTextureParams {
                dest_size: Some(vec2(card_w, card_h)),
                ..Default::default()
            },
        );
    } else {
        // Procedural card background
        // Outer glow
        draw_rectangle(
            card_x - 4.0,
            card_y - 4.0,
            card_w + 8.0,
            card_h + 8.0,
            Color::new(biome_color.r, biome_color.g, biome_color.b, alpha * 0.3),
        );

        // Main card background
        draw_rectangle(
            card_x,
            card_y,
            card_w,
            card_h,
            Color::new(0.05, 0.1, 0.15, alpha * 0.9),
        );

        // Border with biome color
        draw_rectangle_lines(
            card_x,
            card_y,
            card_w,
            card_h,
            3.0,
            Color::new(biome_color.r, biome_color.g, biome_color.b, alpha * 0.8),
        );

        // Inner highlight line
        draw_rectangle(
            card_x + 3.0,
            card_y + 3.0,
            card_w - 6.0,
            2.0,
            Color::new(1.0, 1.0, 1.0, alpha * 0.2),
        );
    }

    // Draw biome name (large, centered)
    let biome_text_size = 36.0;
    let biome_text_dims = measure_text(biome_name, None, biome_text_size as u16, 1.0);
    draw_text(
        biome_name,
        card_x + (card_w - biome_text_dims.width) / 2.0,
        card_y + 40.0,
        biome_text_size,
        Color::new(biome_color.r, biome_color.g, biome_color.b, alpha),
    );

    // Draw level name (smaller, below)
    let level_text_size = 18.0;
    let level_text_dims = measure_text(level_name, None, level_text_size as u16, 1.0);
    draw_text(
        level_name,
        card_x + (card_w - level_text_dims.width) / 2.0,
        card_y + 70.0,
        level_text_size,
        Color::new(0.7, 0.8, 0.9, alpha * 0.8),
    );

    // Add subtle animated sparkles
    let sparkle_count = 3;
    for i in 0..sparkle_count {
        let offset = i as f32 * 2.1;
        let sparkle_x = card_x + 30.0 + i as f32 * (card_w - 60.0) / (sparkle_count - 1) as f32;
        let sparkle_y = card_y + 85.0 + (time * 2.0 + offset).sin() * 3.0;
        let sparkle_alpha = ((time * 3.0 + offset).sin() * 0.5 + 0.5) * alpha;
        draw_circle(sparkle_x, sparkle_y, 2.0, Color::new(1.0, 1.0, 1.0, sparkle_alpha * 0.6));
    }
}

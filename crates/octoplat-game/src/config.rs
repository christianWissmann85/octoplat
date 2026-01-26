use macroquad::prelude::*;

/// All tunable game constants in one place for easy iteration
pub struct GameConfig {
    // Physics
    pub gravity: f32,
    pub terminal_velocity: f32,

    // Movement
    pub move_speed: f32,
    pub air_control: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub air_acceleration: f32,

    // Sprint
    pub sprint_speed: f32,
    pub sprint_acceleration: f32,
    pub sprint_air_bonus: f32,
    pub sprint_stamina_drain: f32,

    // Jumping
    pub jump_velocity: f32,
    /// Jump cut allows variable jump height by reducing velocity when button released mid-jump.
    /// 0.5 means releasing early gives ~50% of full jump height, making platforming more precise.
    pub jump_cut_multiplier: f32,
    /// Coyote time: grace period after leaving ground where jump still works.
    /// 0.1s (100ms) feels responsive without being exploitable - allows last-second jumps.
    pub coyote_time: f32,
    /// Jump buffer: pressing jump slightly before landing still registers.
    /// 0.12s (120ms) makes landings forgiving without feeling unresponsive.
    pub jump_buffer_time: f32,

    // Corner correction (helps with tight passages)
    pub corner_correction_threshold: f32,  // Max pixels to nudge when clipping corners

    // Landing
    pub hard_landing_threshold: f32,  // Velocity threshold for hard landing
    pub landing_recovery_time: f32,   // Duration of movement slowdown
    pub landing_recovery_factor: f32, // Movement speed multiplier during recovery (0.3 = 30%)

    // Wall mechanics
    pub wall_stamina_max: f32,
    pub wall_stamina_regen_rate: f32,
    pub wall_jump_velocity: Vec2,
    pub wall_jump_climb_horizontal: f32, // Multiplier for climb jump horizontal push
    pub wall_jump_climb_vertical: f32,   // Multiplier for climb jump vertical boost
    pub wall_jumps_max: u8,
    pub wall_jump_cooldown: f32,  // Time after wall jump where grabs are ignored
    pub same_wall_cooldown: f32,  // Longer cooldown for re-grabbing the same wall
    pub same_wall_threshold: f32, // How close (in pixels) counts as "same wall"

    // Water Jet Boost
    pub jet_boost_speed: f32,
    pub jet_boost_duration: f32,
    pub jet_max_charges: u8,
    pub jet_regen_rate: f32,           // Seconds per charge regen
    pub jet_downward_speed_mult: f32,  // Multiplier for downward jet speed

    // Ink Cloud
    pub ink_duration: f32,
    pub ink_max_charges: u8,

    // Tentacle Swing
    pub grapple_range: f32,
    #[allow(dead_code)] // Future: animate grapple hook shooting out
    pub grapple_shoot_speed: f32,
    pub swing_gravity: f32,
    pub swing_damping: f32,
    pub swing_pump_strength: f32,
    pub rope_retract_speed: f32,
    pub rope_min_length: f32,      // Minimum rope length when retracting
    pub swing_release_boost: f32,
    pub swing_stamina_drain: f32,

    // Camera
    pub camera_smoothing: f32,
    pub camera_lookahead: f32,
    pub camera_lookahead_speed_mult: f32,  // Extra lookahead when moving fast
    pub camera_vertical_bias: f32,         // Look down when falling
    pub camera_deadzone: Vec2,

    // Player
    pub player_hitbox: Vec2, // Width, height in pixels
    pub hit_flash_duration: f32,
    pub input_deadzone: f32, // Movement input threshold

    // HP system
    pub player_max_hp: u8,
    pub invincibility_duration: f32,
    pub enemy_speed_multiplier: f32,

    // Hazard damage values
    pub spike_damage: u8,
    pub crab_damage: u8,
    pub pufferfish_damage: u8,

    // Hazards
    pub death_animation_time: f32,

    // Enemies
    pub crab_speed: f32,
    pub pufferfish_amplitude: f32,
    pub pufferfish_speed: f32,

    // Platforms
    pub bounce_velocity: f32,
    pub moving_platform_speed: f32,
    pub crumble_shake_time: f32,
    pub crumble_respawn_time: f32,

    // Lives system
    pub starting_lives: u32,
    pub max_lives: u32,
    pub endless_gem_milestone: u32,

    // Gamepad
    pub gamepad_stick_deadzone: f32,

    // Minimap (visibility default only - other settings are in SaveData)
    pub minimap_default_visible: bool,
}

impl GameConfig {
    /// Validate all config values and return errors for any invalid settings.
    ///
    /// Call this after loading or creating a config to catch invalid values early.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Physics validation
        if self.gravity <= 0.0 {
            errors.push("gravity must be positive".into());
        }
        if self.terminal_velocity <= 0.0 {
            errors.push("terminal_velocity must be positive".into());
        }

        // Movement validation
        if self.move_speed <= 0.0 {
            errors.push("move_speed must be positive".into());
        }
        if self.air_control < 0.0 || self.air_control > 1.0 {
            errors.push("air_control must be between 0.0 and 1.0".into());
        }
        if self.acceleration <= 0.0 {
            errors.push("acceleration must be positive".into());
        }
        if self.deceleration <= 0.0 {
            errors.push("deceleration must be positive".into());
        }

        // Jump validation
        if self.jump_velocity >= 0.0 {
            errors.push("jump_velocity must be negative (upward)".into());
        }
        if self.jump_cut_multiplier < 0.0 || self.jump_cut_multiplier > 1.0 {
            errors.push("jump_cut_multiplier must be between 0.0 and 1.0".into());
        }
        if self.coyote_time < 0.0 {
            errors.push("coyote_time must be >= 0".into());
        }
        if self.jump_buffer_time < 0.0 {
            errors.push("jump_buffer_time must be >= 0".into());
        }

        // Wall mechanics validation
        if self.wall_stamina_max <= 0.0 {
            errors.push("wall_stamina_max must be positive".into());
        }
        if self.wall_stamina_regen_rate < 0.0 {
            errors.push("wall_stamina_regen_rate must be >= 0".into());
        }
        if self.wall_jumps_max == 0 {
            errors.push("wall_jumps_max must be >= 1".into());
        }
        if self.wall_jump_cooldown < 0.0 {
            errors.push("wall_jump_cooldown must be >= 0".into());
        }
        if self.same_wall_cooldown < 0.0 {
            errors.push("same_wall_cooldown must be >= 0".into());
        }

        // Jet boost validation
        if self.jet_boost_speed <= 0.0 {
            errors.push("jet_boost_speed must be positive".into());
        }
        if self.jet_boost_duration <= 0.0 {
            errors.push("jet_boost_duration must be positive".into());
        }
        if self.jet_max_charges == 0 {
            errors.push("jet_max_charges must be >= 1".into());
        }
        if self.jet_regen_rate <= 0.0 {
            errors.push("jet_regen_rate must be positive".into());
        }

        // Grapple validation
        if self.grapple_range <= 0.0 {
            errors.push("grapple_range must be positive".into());
        }
        if self.rope_min_length <= 0.0 {
            errors.push("rope_min_length must be positive".into());
        }

        // Player validation
        if self.player_hitbox.x <= 0.0 || self.player_hitbox.y <= 0.0 {
            errors.push("player_hitbox dimensions must be positive".into());
        }
        if self.death_animation_time <= 0.0 {
            errors.push("death_animation_time must be positive".into());
        }

        // Enemy validation
        if self.crab_speed <= 0.0 {
            errors.push("crab_speed must be positive".into());
        }

        // Lives validation
        if self.starting_lives == 0 {
            errors.push("starting_lives must be >= 1".into());
        }
        if self.max_lives < self.starting_lives {
            errors.push("max_lives must be >= starting_lives".into());
        }

        // Gamepad validation
        if self.gamepad_stick_deadzone < 0.0 || self.gamepad_stick_deadzone > 1.0 {
            errors.push("gamepad_stick_deadzone must be between 0.0 and 1.0".into());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            // Physics
            gravity: 2400.0,
            terminal_velocity: 750.0,

            // Movement
            move_speed: 200.0,
            air_control: 0.9,
            acceleration: 8000.0,
            deceleration: 12000.0,
            air_acceleration: 6000.0,

            // Sprint
            sprint_speed: 400.0,
            sprint_acceleration: 10000.0,
            sprint_air_bonus: 0.5,
            sprint_stamina_drain: 0.6,

            // Jumping - negative velocity because Y increases downward
            jump_velocity: -800.0,      // Initial upward velocity (tuned with gravity 2400)
            jump_cut_multiplier: 0.5,   // Variable jump: releasing early = 50% height
            coyote_time: 0.1,           // 100ms grace period after leaving platform
            jump_buffer_time: 0.12,     // 120ms buffer before landing

            // Corner correction (helps with tight passages)
            corner_correction_threshold: 6.0,  // Nudge up to 6px to slip through gaps

            // Landing
            hard_landing_threshold: 550.0,  // Fall speed that triggers hard landing
            landing_recovery_time: 0.08,    // Brief slowdown after hard landing
            landing_recovery_factor: 0.3,   // 30% movement during recovery

            // Wall mechanics
            wall_stamina_max: 2.0,
            wall_stamina_regen_rate: 0.5,
            wall_jump_velocity: vec2(280.0, -520.0),
            wall_jump_climb_horizontal: 0.45, // Climb jump: less horizontal push
            wall_jump_climb_vertical: 1.3,    // Climb jump: more vertical boost
            wall_jumps_max: 2,
            wall_jump_cooldown: 0.3,  // 300ms grace period after wall jump
            same_wall_cooldown: 0.45, // 450ms before can re-grab the same wall
            same_wall_threshold: 16.0, // Within 16px counts as "same wall"

            // Water Jet Boost
            jet_boost_speed: 500.0,
            jet_boost_duration: 0.15,
            jet_max_charges: 3,
            jet_regen_rate: 2.0,           // 2 seconds per charge
            jet_downward_speed_mult: 1.6,  // 800 speed when downward (matching old dive)

            // Ink Cloud
            ink_duration: 1.5,
            ink_max_charges: 2,

            // Tentacle Swing
            grapple_range: 200.0,
            grapple_shoot_speed: 800.0,
            swing_gravity: 600.0,
            swing_damping: 0.995,          // Higher = less energy loss, more momentum
            swing_pump_strength: 4.0,      // How much player input affects swing
            rope_retract_speed: 100.0,
            rope_min_length: 50.0,         // Can't retract shorter than this
            swing_release_boost: 1.2,
            swing_stamina_drain: 0.8,      // Stamina per second while swinging

            // Camera
            camera_smoothing: 5.0,
            camera_lookahead: 50.0,
            camera_lookahead_speed_mult: 0.15,  // Adds up to 15% of speed to lookahead
            camera_vertical_bias: 60.0,          // Look 60px down when falling fast
            camera_deadzone: vec2(30.0, 20.0),

            // Player
            player_hitbox: vec2(24.0, 30.0),
            hit_flash_duration: 0.12,
            input_deadzone: 0.1, // Movement input threshold

            // HP system (defaults for Normal/Treading Water difficulty)
            player_max_hp: 3,
            invincibility_duration: 1.0,
            enemy_speed_multiplier: 1.0,

            // Hazard damage values
            spike_damage: 1,
            crab_damage: 1,
            pufferfish_damage: 2,

            // Hazards
            death_animation_time: 0.5,

            // Enemies
            crab_speed: 60.0,
            pufferfish_amplitude: 40.0,
            pufferfish_speed: 2.0,

            // Platforms
            bounce_velocity: 550.0, // Strong upward launch
            moving_platform_speed: 60.0,
            crumble_shake_time: 0.6,
            crumble_respawn_time: 3.0,

            // Lives system
            starting_lives: 5,
            max_lives: 9,
            endless_gem_milestone: 50,

            // Gamepad
            gamepad_stick_deadzone: 0.2,

            // Minimap
            minimap_default_visible: true,
        }
    }
}

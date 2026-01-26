# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build
cargo build                  # Debug build
cargo build --release        # Release build

# Test
cargo test                   # Run all tests
cargo test --test {name}     # Run specific test file (e.g., cargo test --test procgen)

# Lint and Check
cargo clippy                 # Lint
cargo check                  # Type check without building

# Run the game
cargo run --release          # Run in release mode
```

Cross-compile for Windows: `./scripts/build-windows.sh`

## Architecture

Octoplat is a procedurally-generated roguelite platformer built with Rust and macroquad.

### Two-Crate Workspace

**octoplat-core** - Pure Rust library with no graphics dependencies:
- `level/` - Level data structures, tilemaps, decorations, markers
- `procgen/` - Procedural generation, biomes, archetype pooling, level validation
- `state/` - Game state types (difficulty, lives, roguelite progression)
- `physics/` - Collision detection and feedback tracking
- `types/` - Primitive types (Vec2, Rect, Color)

**octoplat-game** - Complete game runtime using macroquad:
- `app/` - Action system, handlers, keybindings
- `gameplay/` - Physics engine, collision handling
- `rendering/` - Tile rendering, shaders, effects
- `player/` - Player state machine, movement, abilities (wall jump, jet boost, ink cloud)
- `level/` - Level management, environment visuals
- `roguelite/` - Roguelite mode with linked segment progression
- `ui/` - Menus, screens, transitions
- `platforms/` - Moving and crumbling platforms
- `hazards/` - Enemies (Crab, Pufferfish)

### Key Patterns

**State Machine:** AppState enum drives game flow (Title, MainMenu, Playing, Paused, GameOver, Settings, BiomeSelect, LevelComplete, RogueLiteLeaderboard, Error). StateController handles transitions with fade animations.

**Modular Controllers:** GameState delegates to focused subsystems:
- StateController - App state transitions
- EffectsController - Audio, particles, feedback
- GameplayEngine - Player, level, camera
- ProgressionManager - Saves, lives, roguelite tracking
- UIState - Menus, minimap

**Action-Based Updates:** GameActions enum for state transitions and side effects, avoiding scattered mutation.

**Compatibility Layer:** `compat` module converts between octoplat_core types and macroquad types.

### Procedural Generation

- Archetype-based level design using hand-crafted segments in `/assets/roguelite/{biome}/`
- 4 biomes: OceanDepths, Shipwreck, CrystalCave, CoralReef
- Level validation ensures reachability and proper mechanics usage
- Difficulty scales with asymptotic curve (rate: 0.05)

### Testing

Integration tests live in `crates/{crate}/tests/` (not inline):
- `procgen.rs` - Generation and validation
- `player.rs` - Player state machine
- `scenarios.rs` - Gameplay scenarios
- `physics.rs` - Collision detection

### Constants

Game tuning constants centralized in:
- `octoplat_core::constants` - Shared constants
- `octoplat_game::config::GameConfig` - Physics, movement, abilities

### Platform Notes

- Audio gracefully degrades on systems without ALSA (e.g., WSL)
- Window: 1280x720, resizable

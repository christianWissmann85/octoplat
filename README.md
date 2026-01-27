# Octoplat

<p align="center">
  <strong>A procedurally-generated roguelite platformer starring the world's most determined octopus!</strong>
</p>

<p align="center">
  <em>Dive deep. Jump high. Ink your way to victory.</em>
</p>

---

## About

**Octoplat** is an underwater roguelite platformer where you guide an adorable pink octopus through procedurally-generated levels across 8 unique biomes. Master wall jumps, jet boosts, and ink clouds as you navigate treacherous depths filled with crabs, pufferfish, and plenty of spikes.

Built with Rust and powered by catchy tunes and a blend of generative AI art with procedural level design, Octoplat offers tight platforming action wrapped in an ocean of charm.

## Features

### Tight Platforming Controls
- **Wall Jumping** - Cling to walls and leap between surfaces with stamina-based climbing
- **Jet Boost** - Charge up and blast through the air (or dive down to smash enemies!)
- **Ink Cloud** - Collect ink power-ups for temporary invincibility
- **Tentacle Swing** - Grapple and swing across wide gaps

### 8 Beautiful Biomes
Journey from the tranquil **Ocean Depths** through vibrant **Coral Reefs**, sunny **Tropical Shores**, eerie **Shipwrecks**, frozen **Arctic Waters**, dangerous **Volcanic Vents**, mysterious **Sunken Ruins**, and finally into the ultimate challenge: **The Abyss**.

### Roguelite Progression
- Procedurally-generated levels using hand-crafted archetype segments
- Progressive difficulty scaling across biomes
- Gem collection and run tracking
- Leaderboards for your best attempts

### Four Difficulty Modes
From chill exploration to tentacle-twisting challenge:

| Difficulty | HP | Lives | For Players Who... |
|------------|-----|-------|-------------------|
| **Drifting** | 5 | 7 | Want to enjoy the scenery |
| **Treading Water** | 3 | 5 | Like a balanced challenge |
| **OctoHard** | 2 | 4 | Crave punishment |
| **The Kraken** | 1 | 3 | Have something to prove |

### Polished Details
- Squash & stretch animations that make movement feel satisfying
- Bloom effects, particles, and smooth camera transitions
- Catchy music tracks unique to each biome
- Crunchy sound effects for every jump, boost, and gem collected
- Minimap with adjustable zoom
- Gamepad support

## Building & Running

### Requirements
- Rust (Edition 2021)
- Cargo

### Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/octoplat.git
cd octoplat

# Build and run (release mode recommended for best performance)
cargo run --release
```

### Other Commands

```bash
cargo build                  # Debug build
cargo build --release        # Release build (optimized)
cargo test                   # Run all tests
cargo clippy                 # Lint check
```

### Cross-Compile for Windows

```bash
./scripts/build-windows.sh
```

## Controls

| Action | Keyboard | Gamepad |
|--------|----------|---------|
| Move | A/D or Arrow Keys | Left Stick |
| Jump | Space | A Button |
| Wall Jump | Space (while on wall) | A Button |
| Jet Boost | Hold & Release Space | Hold & Release A |
| Sprint | Shift | B Button |
| Pause | Escape | Start |

## Architecture

Octoplat uses a clean two-crate workspace design:

```
octoplat/
├── crates/
│   ├── octoplat-core/    # Pure Rust library (no graphics)
│   │   ├── level/        # Level data structures
│   │   ├── procgen/      # Procedural generation
│   │   ├── state/        # Game state types
│   │   └── physics/      # Collision detection
│   │
│   └── octoplat-game/    # Game runtime (macroquad)
│       ├── player/       # Player state machine & abilities
│       ├── rendering/    # Tiles, shaders, effects
│       ├── roguelite/    # Roguelite progression
│       └── ui/           # Menus & screens
│
└── assets/
    └── roguelite/        # Hand-crafted level segments
```
## Tech Stack

- **Language:** Rust
- **Graphics:** [Macroquad](https://macroquad.rs/)
- **Audio:** Macroquad audio (gracefully degrades on systems without ALSA)
- **Serialization:** Serde + JSON
- **Input:** Keyboard + Gamepad support

## Contributing

Contributions welcome! Whether it's new biome segments, bug fixes, or tentacle-related puns, feel free to open an issue or PR.

---

<p align="center">
  <strong>Happy platforming!</strong> 🐙
</p>

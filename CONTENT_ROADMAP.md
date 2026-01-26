# Octoplat Content Expansion Roadmap

**Goal:** Transform Octoplat from a functional alpha into a visually polished game inspired by Shantae and the Pirate's Curse aesthetic.

**Art Direction:** Vibrant, colorful underwater adventure with expressive characters, rich parallax backgrounds, and polished audio.

**Resources:**
- FLUX Schnell (local stable-diffusion.cpp) for AI art generation
- CC0/Free asset packs for music and SFX
- Procedural primitives for animated decorations
- Image processing tools for background removal

---

## ✅ Phase 1: Audio Foundation [COMPLETE]

**Objective:** Add music and improve SFX to transform the game's atmosphere.

### 1.1 Music System Architecture
- [x] Design music manager with track loading, crossfading, and biome switching
- [x] Implement music volume control in settings
- [x] Add fade transitions between biomes/menus

### 1.2 Source CC0 Music
- [x] **Web Search:** Find CC0/free underwater/ocean ambient tracks
- [x] **Web Search:** Find CC0/free adventure/exploration music
- [x] **Web Search:** Find CC0/free action/tension music for difficult sections
- [x] **Web Search:** Find CC0/free menu/title screen music
- [x] Download and organize tracks by mood/biome fit
- [x] Document attribution (even for CC0, good practice)

### 1.3 Biome Music Mapping
- [x] Ocean Depths: Calm, mysterious ambient
- [x] Coral Reefs: Bright, cheerful exploration
- [x] Tropical Shore: Upbeat, Caribbean vibes
- [x] Shipwreck: Eerie, nautical adventure
- [x] Arctic Waters: Cold, ethereal ambient
- [x] Volcanic Vents: Intense, dramatic
- [x] Sunken Ruins: Ancient, mystical
- [x] Abyss: Deep, ominous, minimal

### 1.4 Source CC0 SFX
- [x] **Web Search:** Find CC0 platformer SFX packs
- [x] **Web Search:** Find CC0 underwater/bubble SFX
- [x] **Web Search:** Find CC0 UI/menu SFX
- [x] Curate and select best fits for each sound type
- [x] Replace/supplement synthesized sounds where beneficial

### 1.5 SFX Integration
- [x] Integrate new jump/land sounds
- [x] Integrate new collectible sounds (gems, powerups)
- [x] Integrate new enemy/hazard sounds
- [x] Integrate new UI sounds (menu, select, back)
- [x] Integrate ambient underwater sounds (bubbles, currents)

### 1.6 Audio Polish
- [x] Balance volume levels across all audio
- [x] Test audio in gameplay scenarios
- [x] Add audio settings (master, music, SFX sliders)

---

## ✅ Phase 2: Background Art (FLUX Generation) [COMPLETE]

**Objective:** Create rich, painted parallax backgrounds for each biome using AI generation with FLUX Schnell.

### 2.1 Background System Enhancement
- [x] Add texture-based background layer support alongside procedural
- [x] Implement 3-layer parallax system (far, mid, near)
- [x] Add biome-specific background asset loading

### 2.2 Ocean Depths Backgrounds
- [x] **FLUX Generate:** Far layer - deep blue gradient with distant rock silhouettes
- [x] **FLUX Generate:** Mid layer - floating particles, distant kelp forests
- [x] **FLUX Generate:** Near layer - foreground rock formations, coral hints
- [x] Integrate into game with parallax scrolling

### 2.3 Coral Reefs Backgrounds
- [x] **FLUX Generate:** Far layer - bright turquoise water, distant coral mountains
- [x] **FLUX Generate:** Mid layer - colorful coral formations, tropical fish silhouettes
- [x] **FLUX Generate:** Near layer - vibrant coral foreground, anemones
- [x] Integrate and test

### 2.4 Tropical Shore Backgrounds
- [x] **FLUX Generate:** Far layer - sunset sky, distant palm islands
- [x] **FLUX Generate:** Mid layer - shallow water, sandy bottom visible
- [x] **FLUX Generate:** Near layer - beach transition, tropical plants
- [x] Integrate and test

### 2.5 Shipwreck Backgrounds
- [x] **FLUX Generate:** Far layer - murky water, distant ship hulls
- [x] **FLUX Generate:** Mid layer - broken masts, floating debris
- [x] **FLUX Generate:** Near layer - wood planks, chains, barnacles
- [x] Integrate and test

### 2.6 Arctic Waters Backgrounds
- [x] **FLUX Generate:** Far layer - icy blue depths, aurora hints through ice
- [x] **FLUX Generate:** Mid layer - ice formations, frozen structures
- [x] **FLUX Generate:** Near layer - ice crystals, snow particles
- [x] Integrate and test

### 2.7 Volcanic Vents Backgrounds
- [x] **FLUX Generate:** Far layer - dark with distant lava glow
- [x] **FLUX Generate:** Mid layer - volcanic rock formations, steam vents
- [x] **FLUX Generate:** Near layer - glowing lava cracks, ash particles
- [x] Integrate and test

### 2.8 Sunken Ruins Backgrounds
- [x] **FLUX Generate:** Far layer - mysterious temple silhouettes
- [x] **FLUX Generate:** Mid layer - crumbling columns, glowing runes
- [x] **FLUX Generate:** Near layer - overgrown stone, mystical orbs
- [x] Integrate and test

### 2.9 Abyss Backgrounds
- [x] **FLUX Generate:** Far layer - near-black with faint bioluminescence
- [x] **FLUX Generate:** Mid layer - strange deep-sea formations
- [x] **FLUX Generate:** Near layer - bioluminescent creatures, crystal hints
- [x] Integrate and test

---

## ✅ Phase 3: Decoration Expansion [COMPLETE]

**Objective:** Add more visual variety with new procedural decorations and FLUX-generated props.

### ✅ 3.1 New Procedural Decorations (Per Biome)

#### Ocean Depths Additions
- [ ] Add: Jellyfish (floating, pulsing glow)
- [ ] Add: Sea urchin (spiky ball on surfaces)
- [ ] Add: Clam (opening/closing animation)
- [ ] Add: Tube worm (swaying with particles)

#### Coral Reefs Additions
- [ ] Add: Sea fan (large swaying)
- [ ] Add: Clownfish (swimming pattern)
- [ ] Add: Brain coral (bumpy surface)
- [ ] Add: Sea sponge (tube clusters)

#### Tropical Shore Additions
- [ ] Add: Seagull (perched, occasional flap)
- [ ] Add: Crab holes (sand mounds)
- [ ] Add: Driftwood (weathered wood pieces)
- [ ] Add: Message bottle (glass with paper)

#### Shipwreck Additions
- [ ] Add: Lantern (swaying, dim glow)
- [ ] Add: Rope coils (hanging/piled)
- [ ] Add: Cannon (mounted, decorative)
- [ ] Add: Treasure coins (scattered gold)

#### Arctic Waters Additions
- [ ] Add: Ice stalactites (hanging from ceilings)
- [ ] Add: Frozen fish (encased in ice)
- [ ] Add: Snow drifts (on platforms)
- [ ] Add: Penguin (idle animation)

#### Volcanic Vents Additions
- [ ] Add: Lava drips (falling particles)
- [ ] Add: Obsidian shards (sharp crystals)
- [ ] Add: Sulfur deposits (yellow crusts)
- [ ] Add: Fire coral (glowing, dangerous look)

#### Sunken Ruins Additions
- [ ] Add: Statue fragments (broken heads, hands)
- [ ] Add: Hieroglyphics (wall markings)
- [ ] Add: Ancient pot (cracked vessels)
- [ ] Add: Magic circle (glowing floor pattern)

#### Abyss Additions
- [ ] Add: Anglerfish light (distant lure glow)
- [ ] Add: Giant eye (watching, blinking)
- [ ] Add: Void tendrils (reaching from darkness)
- [ ] Add: Pressure cracks (stressed rock lines)

### 3.2 FLUX-Generated Props
- [ ] **FLUX Generate:** Treasure chest variations (closed, open, empty)
- [ ] **FLUX Generate:** Collectible gems/pearls
- [ ] **FLUX Generate:** Signposts/markers
- [ ] **FLUX Generate:** Power-up items (jet fuel, ink refill)
- [ ] Process images: Remove white backgrounds, make transparent
- [ ] Integrate as texture-based decorations

### 3.3 Decoration System Improvements
- [ ] Increase decoration density options
- [ ] Add rare/special decoration spawning
- [ ] Improve decoration placement (avoid overlap, better distribution)
- [ ] Add decoration layering (some behind tiles, some in front)

---

## ✅ Phase 4: Tile Texturing [COMPLETE]

**Objective:** Add optional texture overlays to enhance the procedural tiles.

### 4.1 Tile Texture System
- [ ] Design texture overlay system that combines with procedural colors
- [ ] Implement texture atlas loading
- [ ] Add blend modes for texture + procedural color

### 4.2 Generate Tile Textures (FLUX)
- [ ] **FLUX Generate:** Sandy/rocky texture for Ocean Depths
- [ ] **FLUX Generate:** Coral/organic texture for Coral Reefs
- [ ] **FLUX Generate:** Sandy/tropical texture for Tropical Shore
- [ ] **FLUX Generate:** Wood plank texture for Shipwreck
- [ ] **FLUX Generate:** Ice/frost texture for Arctic Waters
- [ ] **FLUX Generate:** Volcanic rock texture for Volcanic Vents
- [ ] **FLUX Generate:** Stone brick texture for Sunken Ruins
- [ ] **FLUX Generate:** Dark crystal texture for Abyss
- [ ] Process: Make seamlessly tileable
- [ ] Process: Create normal/highlight variations

### 4.3 Integration
- [ ] Apply textures to solid blocks
- [ ] Apply textures to platforms
- [ ] Test visual cohesion with existing auto-tiling
- [ ] Add texture quality setting (off/low/high)

---

## ✅ Phase 5: UI Polish [COMPLETE]

**Objective:** Upgrade menus and HUD with illustrated elements.

**Read:** `/home/chris/octoplat/flux-guide.md` Flux Schnell Guide

### 5.1 Title Screen [COMPLETE]

- [x] **FLUX Generate:** Loading screen background (underwater scene with octopus)
- [x] **FLUX Generate:** Cute Octopus with a coded spinner Animation
- [x] Process and integrate
- [x] Add animated elements (Spinner, funny Claude Code Like Messages, Octoplat Themed)

- [x] **FLUX Generate:** Title screen background (underwater scene with octopus)
- [x] **FLUX Generate:** Game logo/title art
- [x] Process and integrate
- [x] Add animated elements (bubbles, light rays)

### 5.2 Menu Backgrounds [COMPLETE]
- [x] **FLUX Generate:** Main menu background
- [x] **FLUX Generate:** Settings menu background
- [x] **FLUX Generate:** Biome select backgrounds (8 thumbnails)
- [x] Process and integrate

### 5.3 HUD Elements [COMPLETE]
- [x] **FLUX Generate:** Heart icons (full, empty)
- [x] **FLUX Generate:** Gem/pearl icon
- [x] **FLUX Generate:** Stamina bar frame
- [x] **FLUX Generate:** Ability icons (jet, ink)
- [x] Process: Remove backgrounds, make transparent
- [x] Integrate replacing procedural shapes

### 5.4 Additional UI [COMPLETE]
- [x] **FLUX Generate:** Level complete banner
- [x] **FLUX Generate:** Game over screen
- [x] **FLUX Generate:** Pause menu overlay
- [x] Design and implement improved minimap frame
- [x] Add biome name cards on entry
- [x] Reorganized HUD layout (lives top-left, abilities bottom-right, minimap bottom-left)

### 5.5 HAZARDS [COMPLETE]
- [x] **FLUX Generate:** Spikey Hazard
- [x] Design and implement improved Spikey Hazard
- [x] Integrate replacing procedural shapes

---

## Phase 6: Juice & Effects

**Objective:** Add polish through particles, screen effects, and feedback.

### 6.1 Particle System Expansion
- [ ] Add ink cloud particles (ability use)
- [ ] Add jet boost flame particles
- [ ] Add gem collection sparkles
- [ ] Add footstep/landing dust
- [ ] Add enemy defeat particles

### 6.2 Screen Effects
- [ ] Implement screen shake (hit, explosion, landing)
- [ ] Add speed lines (fast movement, jet boost)
- [ ] Add underwater distortion shader (subtle wave)
- [ ] Add depth-of-field blur for backgrounds
- [ ] Add bloom for glowing elements

### 6.3 Transitions
- [ ] Design level transition animation (dive/surface)
- [ ] Improve menu transitions (slide, fade)
- [ ] Add biome transition effect (color shift, particles)
- [ ] Add death/respawn transition

### 6.4 Camera Polish
- [ ] Implement camera lookahead (show more in movement direction)
- [ ] Add camera smoothing improvements
- [ ] Add camera shake dampening
- [ ] Implement zoom effects (boss areas, dramatic moments)

### 6.5 Animation Polish
- [ ] Add squash/stretch to player
- [ ] Add anticipation frames (pre-jump crouch)
- [ ] Add overshoot on landings
- [ ] Improve enemy animations
- [ ] Add decoration idle animations

---

## Asset Organization

```
assets/
├── audio/
│   ├── music/
│   │   ├── menu_theme.ogg
│   │   ├── ocean_depths.ogg
│   │   ├── coral_reefs.ogg
│   │   └── ... (per biome)
│   └── sfx/
│       ├── jump.ogg
│       ├── gem_collect.ogg
│       └── ... (per sound)
├── backgrounds/
│   ├── ocean_depths/
│   │   ├── far.png
│   │   ├── mid.png
│   │   └── near.png
│   └── ... (per biome)
├── textures/
│   ├── tiles/
│   │   ├── ocean_depths.png
│   │   └── ... (per biome)
│   └── props/
│       ├── treasure_chest.png
│       └── ...
├── ui/
│   ├── title_screen.png
│   ├── icons/
│   │   ├── heart_full.png
│   │   ├── gem.png
│   │   └── ...
│   └── menus/
│       └── ...
└── icons/
    └── ... (existing app icons)
```

---

## Progress Tracking

### Phase 1: Audio Foundation
- **Status:** COMPLETE
- **Blockers:** None
- **Notes:**
  - Music system implemented with crossfading and biome mapping
  - Chiptunes tracks organized and mapped to biomes
  - SFX integration complete with procedural and file-based sounds
  - Music transitions added for: Title, Biome start, Pause/Resume, Game Over, Return to Menu
  - Animated loading screen with octopus during asset loading

### Phase 2: Background Art
- **Status:** COMPLETE
- **Blockers:** None
- **Notes:**
  - Created texture-based background system (`background_textures.rs`)
  - Generated 24 parallax layers using FLUX Schnell with ghibli-style-flux LoRA
  - All 8 biomes have far/mid/near layers (~35MB total assets)
  - Integrated into game with automatic fallback to procedural backgrounds
  - Backgrounds loaded during startup loading screen

### Phase 3: Decoration Expansion
- **Status:** Not Started
- **Blockers:** None
- **Notes:**

### Phase 4: Tile Texturing
- **Status:** Not Started
- **Blockers:** None (Phase 2 background system can be reused)
- **Notes:**

### Phase 5: UI Polish
- **Status:** COMPLETE
- **Blockers:** None
- **Notes:**
  - 5.1 Title Screen complete: loading screen, title screen backgrounds, octopus mascot, logo
  - Created UiAssets and UiTextureManager for texture loading
  - Added animated bubbles, light rays, and procedural fallback rendering
  - 5.2 Menu Backgrounds complete: main menu, settings, and 8 biome thumbnails
  - Extended UiTextureManager with MenuTextures and BiomeThumbnails
  - All menus gracefully fall back to procedural rendering when textures missing
  - 5.3 HUD Elements complete: heart icons, gem icon, stamina frame, jet/ink ability icons
  - Extended UiTextureManager with HudTextures struct
  - All HUD elements have transparent backgrounds and graceful fallback
  - 5.4 Additional UI complete: level complete banner, game over background, pause overlay
  - Extended UiTextureManager with AdditionalUiTextures struct (minimap_frame, biome_card)
  - Biome name card displays on level entry with animated fade-in/out and biome colors
  - HUD layout reorganized: lives top-left, abilities/gems bottom-right, minimap bottom-left
  - 5.5 Hazards complete: spike texture generated with FLUX, integrated into tilemap rendering
  - Added HazardTextureAssets for spike texture loading
  - Spike textures tinted per-biome using hazard_color from BiomeTheme

### Phase 6: Juice & Effects
- **Status:** Not Started
- **Blockers:** None (can work in parallel with other phases)
- **Notes:**

---

## CC0 Resource Links (To Be Populated)

### Music Sources
- [ ] OpenGameArt.org - Music section
- [ ] Freesound.org - Ambient/Music
- [ ] incompetech.com - Kevin MacLeod
- [ ] (Add more as discovered)

### SFX Sources
- [ ] Freesound.org
- [ ] OpenGameArt.org - Sound Effects
- [ ] Kenney.nl - Audio packs
- [ ] (Add more as discovered)

### Art Style References
- Shantae and the Pirate's Curse (primary reference)
- Shantae: Half-Genie Hero
- Rayman Legends (parallax backgrounds)
- Hollow Knight (atmospheric backgrounds)

---

**Created:** January 2026
**Last Updated:** January 2026

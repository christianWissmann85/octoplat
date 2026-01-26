# FLUX Model Guide for Octoplat Game Art
## Setup and Usage with stable-diffusion.cpp

---

## 1. Model Overview

**FLUX** models are state-of-the-art image generation models (2024-2026) that produce significantly better quality than SDXL models. They offer modern image quality comparable to DALL-E 3 and Gemini.

### Available Models

- **FLUX.1-dev** (High Quality): Best quality, slower generation (~20 steps, ~2-3 minutes)
- **FLUX.1-schnell** (Fast): Good quality, rapid prototyping (~4 steps, ~30 seconds) - **RECOMMENDED for game assets**

---

## 2. File Structure

Your models should be organized in `~/.ai-assets/models/flux/`:

```
~/.ai-assets/models/flux/
├── flux-dev-q8.gguf              # 12GB - Main model (high quality)
├── flux-schnell-q4.gguf          # 6.4GB - Fast model
├── ae.safetensors                # 320MB - VAE (shared)
├── clip_l.safetensors            # 235MB - CLIP encoder (shared)
├── t5-Q5_K_M.gguf               # ~2GB - T5 text encoder (faster, recommended)
└── t5xxl_fp8_e4m3fn.safetensors # 4.6GB - T5 text encoder (alternative)
```

LoRAs should be in `~/.ai-assets/loras/`:
```
~/.ai-assets/loras/
├── pixel-art-flux.safetensors          # For retro pixel art sprites
├── cartoon-flux.safetensors            # For warm cartoon illustrations
├── ghibli-style-flux.safetensors       # For hand-drawn animation style
└── anime-game-art.safetensors          # For anime-style game characters
```

**Recommended LoRAs for Octoplat (download from Civitai):**
- **Pixel Art Schnell Flux LoRA** - Perfect for retro game sprites
- **Cartoon Flux LoRA | Line Warm Illustration** - Great for vibrant cartoon characters
- **Ghibli Style Flux** - Excellent for hand-drawn, painterly backgrounds
- **Cartoon Saloon Flux** - Good for animation-style characters

---

## 3. Command Structure

### Base Command (FLUX Dev - High Quality)

```bash
cd ~/stable-diffusion.cpp

./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-dev-q8.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "YOUR_PROMPT_HERE" \
  --cfg-scale 1.0 \
  --sampling-method euler \
  --steps 20 \
  -H 896 -W 704 \
  -o ~/.ai-assets/output/OUTPUT_NAME.png
```

**Key Parameters:**
- `--steps 20`: More steps = better quality (15-30 recommended for dev)
- `--cfg-scale 1.0`: Always use 1.0 for FLUX models
- `-H 896 -W 704`: Portrait card aspect ratio
- `-H 704 -W 896`: Landscape orientation

---

## 4. Octoplat Art Direction

### Core Art Style
- **Shantae / Metroidvania platformer aesthetic**
- **Vibrant, colorful underwater adventure theme**
- **Pixel art style or high-resolution sprite art** suitable for 2D sidescrollers
- **Biome-specific visual themes** (ocean depths, coral reefs, tropical shores, shipwrecks, volcanic vents, etc.)
- **Cute, expressive character designs** with personality

### Asset Types Needed
1. **Character Sprites** - Octopus protagonist, NPCs
2. **Enemy Sprites** - Crabs, pufferfish, sea creatures per biome
3. **Background Art** - Parallax layers for each biome
4. **Tileset Elements** - Platforms, walls, decorative tiles
5. **Props & Decorations** - Coral, treasure chests, kelp, bubbles
6. **UI Elements** - Icons, buttons, health bars

### Prompt Template Structure

```
[ASSET TYPE] for 2D platformer game, [SUBJECT DESCRIPTION], 
[STYLE NOTES], underwater/ocean theme, [BIOME SPECIFIC], 
vibrant colors, [game art style reference], side view/top-down/isometric
```

---

## 5. Biome-Specific Asset Prompts

### 🌊 OCEAN DEPTHS (Starting Biome - Tutorial)

**Background Art - Deep Ocean:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Deep ocean underwater background for 2D platformer game, dark blue gradient water, gentle light rays from surface, floating particles and bubbles, peaceful atmosphere, distant silhouettes of underwater rock formations, parallax scrolling layer, Shantae art style, vibrant game art" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/ocean_depths_bg.png
```

**Enemy - Ocean Crab:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Cute angry crab enemy sprite for 2D platformer, red shell with blue accents, side view, cartoon style, expressive eyes, small claws raised, underwater creature, game character sprite, Shantae art style, transparent background suitable for sprite sheet" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/ocean_crab_sprite.png
```

---

### 🪸 CORAL REEFS (Colorful & Vertical)

**Background Art - Coral Gardens:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Vibrant coral reef background for 2D platformer, colorful pink and orange coral formations, tropical fish swimming, bright turquoise water, sun rays penetrating, lush underwater garden, parallax layer for side-scrolling game, cheerful atmosphere, Shantae game art style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/coral_reefs_bg.png
```

**Tileset - Coral Platforms:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "Platform tileset for 2D game, pink and orange coral formations, textured surface for walking, modular 32x32 pixel style tiles, top view perspective, colorful underwater platforms, seamless tiling edges, game asset sheet, vibrant colors <lora:pixel-art-flux:0.6>" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 1024 -o ~/.ai-assets/output/coral_platform_tiles.png
```

---

### 🏝️ TROPICAL SHORE (Warm & Coastal)

**Background Art - Beach Sunset:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "Tropical beach background for 2D platformer, golden sunset sky with pink clouds, palm trees silhouettes, warm orange and yellow lighting, gentle waves, paradise island atmosphere, parallax scrolling layer, Shantae style vibrant game art <lora:ghibli-style-flux:0.5>" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/tropical_shore_bg.png
```

**Decoration - Treasure Chest:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Pirate treasure chest game object, wooden chest with brass lock, gold coins spilling out, side view, cartoon style, colorful, collectible item for 2D platformer, single game asset, white background, Shantae art style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/treasure_chest.png
```

---

### ⚓ SHIPWRECK (Dark & Enclosed)

**Background Art - Sunken Ship Interior:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Sunken pirate ship interior background for 2D platformer, dark wooden hull with holes letting light through, broken masts and barrels, mysterious atmosphere, teal underwater lighting, floating debris, side-scrolling game layer, adventure game art style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/shipwreck_bg.png
```

**Enemy - Ghost Pirate Fish:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Ghostly spectral fish enemy sprite for 2D game, translucent glowing blue and green, pirate hat, spooky but cute, side view floating pose, underwater ghost, game character, Shantae art style, white background for sprite" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/ghost_pirate_fish.png
```

---

### ❄️ ARCTIC WATERS (Ice & Cold)

**Background Art - Frozen Ocean:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Arctic underwater background for 2D platformer, icy blue water, floating ice chunks, northern lights aurora visible through ice ceiling, cold color palette blues and whites, frozen ocean atmosphere, parallax game layer, beautiful fantasy game art" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/arctic_waters_bg.png
```

**Tileset - Ice Platforms:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Ice platform tileset for 2D platformer, frozen blue ice blocks, slippery surface texture, modular game tiles, top view, crystalline ice formations, seamless tiling, 32x32 pixel style, arctic theme game assets" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 1024 -o ~/.ai-assets/output/ice_platform_tiles.png
```

---

### 🌋 VOLCANIC VENTS (Fire & Danger)

**Background Art - Underwater Volcano:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Underwater volcanic vent background for 2D platformer, glowing orange lava flows, dark rocky formations, red and orange hot water currents, bubbling magma, dangerous atmosphere, dramatic lighting, parallax game layer, adventure game art" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/volcanic_vents_bg.png
```

**Hazard - Lava Bubble:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Glowing lava bubble hazard sprite for 2D platformer game, orange and yellow hot bubble rising, animated game object, danger element, bright glow effect, single game asset, white background, cartoon style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 256 -W 256 -o ~/.ai-assets/output/lava_bubble.png
```

---

### 🏛️ SUNKEN RUINS (Ancient Mystery)

**Background Art - Ancient Temple:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Ancient sunken ruins background for 2D platformer, crumbling stone columns and arches, mysterious glowing runes, blue-green mystical light, overgrown with coral and seaweed, atmospheric underwater temple, parallax game layer, fantasy adventure art" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/sunken_ruins_bg.png
```

---

### 🕳️ THE ABYSS (Final Boss Area)

**Background Art - Dark Abyss:**
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Deep abyss background for 2D platformer final level, almost completely dark water, faint bioluminescent creatures in distance, eerie purple and blue glow, ominous atmosphere, crushing depth feeling, parallax game layer, dramatic final area art" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 704 -W 1280 -o ~/.ai-assets/output/abyss_bg.png
```

---

## 6. LoRA Usage for Game Art Styles

### Recommended LoRAs and Their Uses

**Pixel Art LoRA** - For retro sprite-based assets:
```bash
<lora:pixel-art-flux:0.8>  # Strong pixel art effect for sprites
<lora:pixel-art-flux:0.5>  # Subtle pixelation for backgrounds
```

**Cartoon LoRA** - For vibrant, warm character designs:
```bash
<lora:cartoon-flux:0.7>    # Cartoon illustration style
<lora:cartoon-flux:0.5>    # Subtle cartoon influence
```

**Ghibli Style LoRA** - For hand-drawn, painterly aesthetics:
```bash
<lora:ghibli-style-flux:0.6>  # Soft hand-drawn backgrounds
<lora:ghibli-style-flux:0.4>  # Subtle watercolor effect
```

**Anime Game Art LoRA** - For anime-style characters:
```bash
<lora:anime-game-art:0.7>  # Strong anime character style
<lora:anime-game-art:0.5>  # Balanced game art approach
```

### Mixed LoRAs (Combine styles)
```bash
<lora:cartoon-flux:0.5> <lora:ghibli-style-flux:0.3>  # Cartoon with hand-drawn feel
<lora:pixel-art-flux:0.4> <lora:cartoon-flux:0.3>     # Pixelated cartoon hybrid
```

**Strength Recommendations:**
- `0.3-0.5`: Subtle influence, good for backgrounds
- `0.6-0.7`: Moderate influence, good for characters
- `0.8-1.0`: Strong influence, good for consistent style

### When to Use LoRAs:
- **Sprites**: Pixel art or cartoon LoRA (0.7-0.8)
- **Backgrounds**: Ghibli or cartoon LoRA (0.4-0.6)
- **UI Elements**: Cartoon LoRA (0.5-0.6)
- **Tilesets**: Pixel art LoRA (0.6-0.8)

---

## 7. Character & Protagonist Assets

### Main Character - Octopus Hero:
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "Cute cartoon octopus character sprite for 2D platformer game, purple and blue gradient colors, large expressive eyes, friendly personality, side view idle pose, eight tentacles, game protagonist, Shantae art style, white background for sprite sheet <lora:cartoon-flux:0.7>" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/octopus_hero_idle.png
```

### Player Animation - Jump Pose:
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Cute cartoon octopus jumping pose sprite, purple and blue gradient, tentacles spread upward, excited expression, side view for 2D platformer, game character animation frame, white background, same style as idle sprite" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/octopus_hero_jump.png
```

### Enemy - Pufferfish:
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Grumpy pufferfish enemy sprite for 2D platformer, inflated spiky ball, yellow with orange spots, angry cartoon face, side view, cute but dangerous, underwater game enemy, Shantae art style, white background for sprite" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/pufferfish_enemy.png
```

---

## 8. UI & Icon Assets

### Health Icon:
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Heart icon for game UI, cute cartoon style, red and pink gradient, shiny glossy surface, small game icon, white background, 64x64 pixel icon design" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 256 -W 256 -o ~/.ai-assets/output/health_icon.png
```

### Collectible Pearl:
```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Glowing pearl collectible for 2D platformer game, shiny white pearl with iridescent sheen, sparkle effect, single game object, white background, pickup item sprite" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 256 -W 256 -o ~/.ai-assets/output/pearl_collectible.png
```

---

## 9. Rapid Asset Creation Workflow

**FLUX Schnell is PERFECT for game assets** - use it for everything:

```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "YOUR_GAME_ASSET_PROMPT_HERE <lora:STYLE_NAME:0.7>" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H HEIGHT -W WIDTH -o ~/.ai-assets/output/OUTPUT_NAME.png
```

**Example - Generate a sprite with cartoon style:**
```bash
cd ~/stable-diffusion.cpp && ./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "Cute jellyfish enemy sprite for 2D platformer, glowing pink and blue, floating animation, side view, game character <lora:cartoon-flux:0.7>" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/jellyfish_enemy.png
```

**Recommended Resolutions:**
- **Sprites**: `-H 512 -W 512` (characters, enemies)
- **Backgrounds**: `-H 704 -W 1280` (wide parallax layers)
- **Tilesets**: `-H 512 -W 1024` (platform tiles)
- **Icons**: `-H 256 -W 256` (UI elements)
- **Small Objects**: `-H 256 -W 256` (collectibles, pickups)

**Workflow:**
1. Generate 3-4 variations with Schnell (~2 minutes total)
2. Pick the best one
3. (Optional) Refine with Dev model if needed - but Schnell is usually perfect for game assets!

**Speed:**
- **Schnell**: ~30 seconds per sprite/background
- You can create 10+ assets in 5 minutes!

---

## 10. Troubleshooting

### Issue: "unknown format" errors
**Solution:** Ensure all model files are properly downloaded (not 0 bytes)
```bash
ls -lh ~/.ai-assets/models/flux/
```

### Issue: Out of VRAM
**Solutions:**
- Use the Q5 T5 encoder instead of fp8 (saves ~2GB)
- Use flux-schnell-q4 instead of flux-dev-q8 (saves ~6GB)
- Reduce image resolution: `-H 768 -W 640`

### Issue: LoRA not loading
**Solution:** Ensure LoRA filename matches exactly (without `.safetensors` extension)
```bash
ls ~/.ai-assets/loras/
# Use: <lora:classical-painting:0.7>
# Not: <lora:classical-painting.safetensors:0.7>
```

---

## 11. Performance Tips

### Optimize Generation Speed:
1. **Use Q5 T5 encoder** (faster, less VRAM): `t5-Q5_K_M.gguf`
2. **Lower steps for testing**: `--steps 15` (still good quality)
3. **Batch similar prompts**: Generate all characters for one faction together
4. **Use Schnell for iterations**: Only use Dev for final art

### Memory Usage:
- **Schnell Q4 + Q5 T5**: ~8GB VRAM (fastest)
- **Dev Q8 + Q5 T5**: ~14GB VRAM (best quality)
- **Dev Q8 + FP8 T5**: ~16GB VRAM (alternative)

---

## 12. Quick Reference Commands

### Test Your Setup:
```bash
cd ~/stable-diffusion.cpp
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "Cute cartoon octopus character for 2D platformer game, Shantae art style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/test.png
```

### Check Model Sizes:
```bash
ls -lh ~/.ai-assets/models/flux/
```

### View Recent Output:
```bash
ls -lht ~/.ai-assets/output/ | head -10
```

### Quick Asset Generation:
```bash
# Character sprite
cd ~/stable-diffusion.cpp && ./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "DESCRIBE_YOUR_ASSET_HERE, 2D platformer game art, Shantae style" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o ~/.ai-assets/output/$(date +%s)_asset.png
```

---

## 13. Asset Integration Tips

### For Octoplat Project:
1. **Save assets** to `/home/chris/octoplat/assets/` directory
2. **Organize by type**: `sprites/`, `backgrounds/`, `tiles/`, `ui/`
3. **Name consistently**: `biome_type_variant.png` (e.g., `ocean_bg_deep.png`)
4. **Use appropriate formats**:
   - PNG with transparency for sprites
   - JPEG for large backgrounds (smaller file size)
   - Consider sprite sheets for animations

### Post-Processing:
- Remove backgrounds in GIMP/Krita if needed
- Resize to game resolution (sprites typically 32x32, 64x64, or 128x128)
- Create sprite sheets by combining multiple poses
- Add transparency where needed

---

## Notes

- **Always use `--cfg-scale 1.0`** for FLUX models
- **Schnell is perfect** for game assets - fast and high quality
- **Common sprite sizes**: 512x512 for generation, scale down as needed
- **Background layers**: Wide aspect ratio (1280x704) for parallax scrolling
- Navigate to `~/stable-diffusion.cpp` before running commands
- Generate multiple variations quickly and pick the best

---

## 14. Automated Asset Generation Pipeline

The `flux-enhancement/` directory contains a Python pipeline for automated asset generation with background removal - perfect for batch-creating game sprites and decorations.

### Setup with uv

The project uses [uv](https://github.com/astral-sh/uv) for Python dependency management. Dependencies are pinned in `pyproject.toml` for reproducibility.

```bash
cd /home/chris/octoplat/flux-enhancement

# Install uv if not already installed
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install all dependencies (creates .venv automatically)
uv sync

# Verify rembg is working
uv run python -c "from rembg import remove; print('rembg OK')"
```

**Key Dependencies (managed via pyproject.toml):**
- Python 3.9 (required for numba/rembg compatibility)
- `rembg>=2.0.50` - Background removal
- `pillow>=10.0.0` - Image processing
- `numpy<2` - Required for numba compatibility
- `numba>=0.58` - JIT compilation for pymatting
- `onnxruntime<1.17` - ML inference for rembg

### Pipeline Scripts

**`simple_asset_gen.py`** - Single asset generation with automatic background removal:
```bash
uv run python simple_asset_gen.py "small rock prop for 2D game, hand-drawn style" rock_test.png
```

**`batch_ui_gen.py`** - Batch generation for UI assets:
```bash
uv run python batch_ui_gen.py ui_hud_batch.json
```

**`batch_decorations.py`** - Batch generation for decoration assets:
```bash
uv run python batch_decorations.py decorations_batch.json
```

### JSON Batch Specification Format

Create a JSON file to generate multiple assets:

```json
{
  "output_dir": "/home/chris/octoplat/assets/decorations",
  "assets": [
    {
      "name": "small_rock",
      "subdir": "ocean_depths",
      "prompt": "<lora:ghibli-style-flux:0.5><lora:cartoon-flux:0.5> small mossy rock prop for 2D underwater game, hand-drawn 2D game art, stylized cartoon illustration, single object, centered, white background",
      "width": 512,
      "height": 512
    },
    {
      "name": "shell",
      "subdir": "coral_reefs",
      "prompt": "<lora:ghibli-style-flux:0.5><lora:cartoon-flux:0.5> pink seashell spiral shell prop for 2D underwater game, hand-drawn 2D game art, stylized cartoon illustration, single object, centered, white background",
      "width": 512,
      "height": 512
    }
  ]
}
```

### Octoplat Art Direction - PROVEN LoRA Combination

After extensive testing, this LoRA combination produces the best results matching Octoplat's hand-drawn art style:

```
<lora:ghibli-style-flux:0.5><lora:cartoon-flux:0.5>
```

**Key prompt elements for consistent art direction:**
- `hand-drawn 2D game art` - Ensures non-photorealistic output
- `stylized cartoon illustration` - Maintains cartoon aesthetic
- `single object, centered, white background` - Clean sprites for background removal

### Example: Decoration Asset Prompt

```
<lora:ghibli-style-flux:0.5><lora:cartoon-flux:0.5> wooden barrel prop for 2D underwater shipwreck game, hand-drawn 2D game art, stylized cartoon illustration, single object, centered, white background
```

### Background Removal with rembg

The pipeline uses `rembg` for automatic background removal:

```python
from rembg import remove
from PIL import Image

# Remove background from generated image
input_image = Image.open("raw_asset.png")
output_image = remove(input_image)
output_image.save("transparent_asset.png")
```

**Output:** 128x128 PNG sprites with transparent backgrounds, ready for game integration.

### Integration with Octoplat

Generated assets are placed in the appropriate directories:

```
assets/decorations/
├── ocean_depths/
│   └── small_rock.png
├── coral_reefs/
│   └── shell.png
├── tropical_shore/
│   ├── coconut.png
│   └── starfish.png
├── shipwreck/
│   ├── wood_debris.png
│   ├── barrel.png
│   └── anchor.png
├── arctic_waters/
│   └── frosted_rock.png
└── sunken_ruins/
    ├── broken_column.png
    └── ancient_tile.png
```

The game loads these automatically via `DecorationAssets` in `assets.rs`:

```rust
// Embedded decoration assets (sprites for level decorations)
#[derive(RustEmbed)]
#[folder = "../../assets/decorations/"]
pub struct DecorationAssets;
```

### Workflow Summary

1. **Define assets** in a JSON batch spec
2. **Run batch generation**: `python batch_asset_gen.py spec.json`
3. **Review outputs** - pipeline handles FLUX generation + background removal
4. **Copy to game assets** folder
5. **Rebuild game** - rust_embed automatically picks up new assets

---

## 15. Parallax Background Notes

### Single-Layer Backgrounds

FLUX generates complete, opaque images rather than transparent layered sprites. For parallax backgrounds, use a **single background per biome** with subtle scrolling:

```rust
// Background scrolling with depth 0.3 for subtle movement
let parallax_offset = camera_x * 0.3;
draw_texture_ex(
    background,
    -parallax_offset % texture_width,
    0.0,
    WHITE,
    DrawTextureParams { dest_size: Some(screen_size), ..Default::default() }
);
```

### Background Asset Location

```
assets/backgrounds/
├── ocean_depths/background.png
├── coral_reefs/background.png
├── tropical_shore/background.png
├── shipwreck/background.png
├── arctic_waters/background.png
├── volcanic_vents/background.png
├── sunken_ruins/background.png
└── abyss/background.png
```

---

**Created:** January 2026
**Updated:** January 2026 - Added Python pipeline documentation
**Game:** Octoplat - 2D Underwater Platformer
**Art Style:** Shantae-inspired vibrant adventure game aesthetic
**Theme:** Ocean depths exploration with cute octopus protagonist

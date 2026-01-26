# Quick Start Guide: Autonomous Game Asset Generation

## 🚀 Getting Started in 10 Minutes

### Step 1: Install Dependencies (2 minutes)

This project uses [uv](https://github.com/astral-sh/uv) for Python dependency management.

```bash
cd flux-enhancement

# Install uv if not already installed
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install dependencies (creates .venv and installs packages)
uv sync

# Verify FLUX is working
cd ~/stable-diffusion.cpp
./build/bin/sd-cli --help
```

### Step 2: Test Individual Asset Generation (3 minutes)

```bash
# Generate a simple sprite (note: use 'uv run' to run Python scripts)
uv run python simple_asset_gen.py generate-single \
  "cute purple octopus character, side view, 2D game sprite, white background <lora:cartoon-flux:0.7>" \
  test_sprite.png \
  512 512

# Remove background
uv run python simple_asset_gen.py remove-bg \
  test_sprite.png \
  test_sprite_transparent.png

# Verify output
ls -lh test_sprite*.png
```

### Step 3: Generate Complete Character Set (5 minutes)

```bash
# Use the example character spec
uv run python simple_asset_gen.py generate-character examples/octopus_hero_spec.json

# Check results
ls -R output/octopus_hero/
```

Expected output structure:
```
output/octopus_hero/
├── idle_f00.png
├── idle_f00_transparent.png
├── idle_f01.png
├── idle_f01_transparent.png
├── idle_f02.png
├── idle_f02_transparent.png
├── idle_f03.png
├── idle_f03_transparent.png
├── idle_sheet.png
├── walk_f00.png
├── walk_f00_transparent.png
├── ... (walk frames)
├── walk_sheet.png
├── jump_f00.png
├── ... (jump frames)
├── jump_sheet.png
├── attack_f00.png
├── ... (attack frames)
├── attack_sheet.png
└── metadata.json
```

---

## 📖 Usage Examples

### Generate Single Asset

```bash
# Basic syntax
uv run python simple_asset_gen.py generate-single PROMPT OUTPUT [WIDTH] [HEIGHT] [SEED]

# Example: Enemy sprite
uv run python simple_asset_gen.py generate-single \
  "angry red crab enemy, raised claws, cartoon style, white background <lora:cartoon-flux:0.6>" \
  crab_enemy.png \
  512 512 100

# Example: Background
uv run python simple_asset_gen.py generate-single \
  "deep ocean background, light rays, parallax layer <lora:ghibli-style-flux:0.5>" \
  ocean_bg.png \
  1280 704
```

### Generate Batch Assets

```bash
# Run batch generation
uv run python simple_asset_gen.py generate-batch examples/batch_assets_example.json

# This will generate:
# - Multiple enemy sprites
# - Collectible items
# - UI icons
# - Background layers
# - Platform tiles
```

### Generate UI Assets (for Octoplat)

```bash
# Generate menu backgrounds and biome thumbnails
uv run python batch_ui_gen.py ui_menus_batch.json

# Generate HUD elements (hearts, gems, ability icons)
uv run python batch_ui_gen.py ui_hud_batch.json
```

### Create Sprite Sheet Manually

```bash
# Generate 4 frames manually
for i in {1..4}; do
  uv run python simple_asset_gen.py generate-single \
    "octopus idle frame $i, cartoon style <lora:cartoon-flux:0.7>" \
    "frame_$i.png" \
    512 512 42
done

# Remove backgrounds
for i in {1..4}; do
  uv run python simple_asset_gen.py remove-bg "frame_$i.png" "frame_${i}_trans.png"
done

# Assemble sprite sheet
uv run python simple_asset_gen.py create-sheet \
  frame_1_trans.png frame_2_trans.png frame_3_trans.png frame_4_trans.png \
  -o idle_animation.png -c 4
```

---

## 🎯 Create Your Own Character Spec

### Template: `my_character_spec.json`

```json
{
  "character": {
    "name": "my_character_id",
    "description": "detailed physical description of your character",
    "style": "lora-name:strength",
    "seed": 42
  },
  "animations": {
    "animation_name": {
      "frames": 4,
      "prompts": [
        "first frame description",
        "second frame description",
        "third frame description",
        "fourth frame description"
      ]
    }
  },
  "resolution": [512, 512]
}
```

### Example: Create a Seahorse Character

```json
{
  "character": {
    "name": "seahorse_guardian",
    "description": "majestic seahorse character with golden scales, wearing royal armor, long curled tail, cartoon style",
    "style": "cartoon-flux:0.7",
    "seed": 55
  },
  "animations": {
    "idle": {
      "frames": 3,
      "prompts": [
        "floating in place, tail gently curled",
        "slight upward drift, tail relaxed",
        "returning to neutral position"
      ]
    },
    "swim": {
      "frames": 4,
      "prompts": [
        "beginning swim motion, tail propelling",
        "mid-swim, moving forward",
        "continuing swim stroke",
        "completing swim cycle"
      ]
    }
  },
  "resolution": [512, 512]
}
```

Generate:
```bash
uv run python simple_asset_gen.py generate-character seahorse_spec.json
```

---

## 🛠️ Advanced Usage

### Batch Asset Generation

Create `my_batch.json`:

```json
{
  "assets": [
    {
      "type": "sprite",
      "prompt": "your prompt here <lora:style:0.7>",
      "output": "output/category/filename.png",
      "width": 512,
      "height": 512,
      "seed": 100,
      "remove_bg": true
    }
  ]
}
```

Run:
```bash
uv run python simple_asset_gen.py generate-batch my_batch.json
```

### Different Art Styles

**Pixel Art:**
```json
{
  "prompt": "character description, pixel art style <lora:pixel-art-flux:0.7>",
  "style": "pixel-art-flux:0.7"
}
```

**Ghibli Style:**
```json
{
  "prompt": "character description, hand-drawn animation style <lora:ghibli-style-flux:0.5>",
  "style": "ghibli-style-flux:0.5"
}
```

**Cartoon:**
```json
{
  "prompt": "character description, vibrant cartoon illustration <lora:cartoon-flux:0.7>",
  "style": "cartoon-flux:0.7"
}
```

### Consistent Characters Across Poses

The key is using the same **seed** and **base description** for all animations:

```json
{
  "character": {
    "seed": 42,  // KEEP THIS THE SAME for all poses
    "description": "extremely detailed character description..."
  }
}
```

The more detailed your base description, the more consistent the character will be across different poses.

---

## 📊 Asset Resolutions Guide

| Asset Type | Resolution | Example |
|------------|-----------|---------|
| Small Sprite | 256×256 | Collectibles, small enemies |
| Standard Sprite | 512×512 | Characters, enemies |
| Large Sprite | 1024×1024 | Bosses, large objects |
| UI Icon | 64×64 to 256×256 | Health, mana, items |
| Background (Wide) | 1280×704 | Parallax layers |
| Background (Tall) | 704×1280 | Vertical scrolling |
| Tileset | 1024×512 | Platform tiles |

---

## 🎨 Prompt Engineering Tips

### For Character Sprites

**Good Prompt Structure:**
```
[Character description], [pose/action], [view angle],
[art style], [background], [lora]

Example:
"cute purple octopus with big eyes, idle standing pose,
side view, cartoon style 2D game sprite, white background
<lora:cartoon-flux:0.7>"
```

### For Backgrounds

**Good Prompt Structure:**
```
[Environment description], [lighting], [atmosphere],
[technical specs], [style]

Example:
"deep ocean underwater environment, gentle light rays from surface,
peaceful atmosphere, parallax scrolling layer for 2D platformer,
vibrant game art style <lora:ghibli-style-flux:0.5>"
```

### Key Phrases That Work Well

- "white background" → Makes background removal easier
- "side view" → Consistent sprite orientation
- "2D game sprite" → Optimizes for game art style
- "cartoon style" → Maintains game aesthetic
- "transparent background suitable for sprite sheet" → Hints at transparency

---

## 🔧 Troubleshooting

### Dependencies Not Installing

**Problem:** `uv sync` fails

```bash
# Clear cache and retry
rm -rf .venv uv.lock
uv sync

# Check Python version
uv run python --version  # Should be 3.9.x
```

### Background Removal Not Working

**Problem:** `rembg` not working

```bash
# Test rembg
uv run python -c "from rembg import remove; print('OK')"

# If fails, try reinstalling
uv pip install --force-reinstall rembg
```

### Character Looks Different Across Poses

**Solution:** Ensure same seed and more detailed description
```json
{
  "seed": 42,  // CRITICAL: Keep same
  "description": "very detailed description including:
    - exact colors (e.g., 'bright purple' not just 'purple')
    - specific features (e.g., '8 tentacles')
    - accessories (e.g., 'red bandana tied on left side')
    - distinguishing marks"
}
```

### FLUX Generation Fails

**Check:**
```bash
# Verify models exist
ls -lh ~/.ai-assets/models/flux/

# Verify LoRAs exist
ls -lh ~/.ai-assets/loras/

# Test basic generation
cd ~/stable-diffusion.cpp
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  -p "test" \
  --cfg-scale 1.0 --sampling-method euler --steps 4 \
  -H 512 -W 512 -o test.png
```

---

## 🚀 Next Steps

### 1. Test the Basic Workflow
- Generate one sprite
- Remove its background
- Create a simple sprite sheet

### 2. Create Your First Character
- Design a character spec JSON
- Generate all animations
- Review results

### 3. Build an Asset Library
- Create batch specs for enemies, items, backgrounds
- Generate in bulk
- Organize into game-ready folders

### 4. Experiment with Styles
- Try different LoRAs
- Mix LoRAs (e.g., pixel-art + cartoon)
- Find your unique art style

### 5. Integrate with Claude Code
- Claude Code can now read your spec files
- Modify prompts programmatically
- Generate asset variations automatically

---

## 📦 Recommended Workflow for New Game

```bash
# 1. Create project structure
mkdir -p my_game/specs
mkdir -p my_game/output/{characters,enemies,items,backgrounds,ui,tiles}

# 2. Create character specs
# Edit specs/hero.json, specs/enemy1.json, etc.

# 3. Generate all characters
for spec in specs/*.json; do
  uv run python simple_asset_gen.py generate-character "$spec"
done

# 4. Create batch spec for environments
# Edit specs/batch_environments.json

# 5. Generate environments
uv run python simple_asset_gen.py generate-batch specs/batch_environments.json

# 6. Organize assets
mv output/* my_game/output/
```

---

## 🎓 Understanding the Pipeline

```
User Input (JSON Spec)
        ↓
Prompt Builder
        ↓
FLUX Generator ──→ image.png
        ↓
Background Remover ──→ image_transparent.png
        ↓
Multiple Frames ──→ Sprite Sheet Assembler
        ↓
Final Assets (organized by type)
```

**Key Insight:** Each component is modular. You can:
- Use FLUX generator alone for static art
- Use background remover on any images
- Assemble sprite sheets from any source images
- Chain them together for complete automation

---

## 💡 Pro Tips

1. **Start Simple:** Generate one sprite manually first to test prompts
2. **Iterate on Prompts:** Small wording changes = big visual differences
3. **Use Seeds:** For consistent characters, always use same seed
4. **Batch Similar Items:** Generate all enemies together, all backgrounds together
5. **Name Systematically:** Use `category_variant_frame.png` naming
6. **Keep Specs:** Save your JSON specs for future regeneration
7. **Test Schnell First:** Use schnell for rapid prototyping (4 steps)
8. **Quality Pass:** Use dev model (20 steps) for final art if needed

---

## 🎯 Ready to Go?

Run this to verify everything is working:

```bash
# Quick test
uv run python simple_asset_gen.py generate-character examples/octopus_hero_spec.json

# If successful, you should see:
# - Progress messages
# - Generated frames in output/octopus_hero/
# - Sprite sheets assembled
# - metadata.json created
```

**Success?** You're ready for autonomous game asset generation! 🎮

**Problems?** Check the troubleshooting section or verify your FLUX setup with the original guides.

---

**Happy Asset Generating!** 🎨✨

# Autonomous Game Asset Generation System
## Design Document for FLUX-Based Pipeline

---

## 🎯 Executive Summary

**Current State:** You have a working FLUX pipeline for generating individual high-quality images (card art, character sprites, backgrounds).

**The Challenge:** Transforming this into an autonomous system that can generate complete game asset packages including:
- Animated sprite sheets (multiple frames in one image)
- Character consistency across different poses
- Transparent backgrounds for sprites
- Organized asset libraries
- Batch generation capabilities

**The Solution:** Build a hybrid Python toolkit + bash automation system that Claude Code can autonomously operate.

---

## 📊 Gap Analysis

### ✅ What You Already Have
- ✅ FLUX models (dev & schnell) with stable-diffusion.cpp
- ✅ Quality LoRAs (pixel-art, cartoon, classical painting, etc.)
- ✅ Excellent prompt templates for different asset types
- ✅ Individual asset generation working perfectly
- ✅ Good understanding of game art requirements

### ❌ What's Missing
- ❌ **Sprite sheet generation** - No way to create animation frames in grid layouts
- ❌ **Background removal** - FLUX outputs have backgrounds, sprites need transparency
- ❌ **Character consistency** - No system to maintain same character across poses
- ❌ **Batch automation** - Manual one-by-one generation
- ❌ **Asset organization** - Manual file management
- ❌ **Post-processing pipeline** - No automated resizing, cropping, format conversion

---

## 🏗️ Proposed Architecture

### Three-Tier System

```
┌─────────────────────────────────────────────────────┐
│  Tier 1: Asset Specification (JSON)                 │
│  - Define asset types, poses, styles                │
│  - Claude Code reads/writes these specs             │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│  Tier 2: Generation Engine (Python + Bash)          │
│  - FLUX image generation (existing)                 │
│  - New: Background removal, sprite assembly         │
│  - New: Consistency management (seed tracking)      │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│  Tier 3: Asset Library (Organized Output)           │
│  - Categorized folders by asset type                │
│  - Metadata tracking (prompts, seeds, variants)     │
│  - Ready for game engine import                     │
└─────────────────────────────────────────────────────┘
```

---

## 🛠️ Required Components

### 1. **Background Removal Pipeline** (CRITICAL)
**Problem:** FLUX generates images with backgrounds. Game sprites need transparency.

**Solutions:**
- **Option A:** `rembg` library (AI-based, best quality)
- **Option B:** ImageMagick color-based removal (fast, good for solid backgrounds)
- **Option C:** Prompt engineering ("white background" + threshold removal)

**Recommendation:** Hybrid approach
- Generate with "white background" or "transparent background" in prompt
- Use `rembg` for automatic high-quality removal
- Fallback to ImageMagick for simple cases

**Implementation:**
```python
# pip install rembg pillow
from rembg import remove
from PIL import Image

def remove_background(input_path, output_path):
    input_img = Image.open(input_path)
    output_img = remove(input_img)
    output_img.save(output_path)
```

---

### 2. **Sprite Sheet Assembly** (ESSENTIAL)
**Problem:** Animations require multiple frames in grid layouts.

**Solution:** Generate individual frames, then assemble into sprite sheet.

**Two Approaches:**

#### Approach A: Grid Assembly (Traditional)
```
┌─────┬─────┬─────┬─────┐
│ F1  │ F2  │ F3  │ F4  │  Idle animation
├─────┼─────┼─────┼─────┤
│ F5  │ F6  │ F7  │ F8  │  Walk animation
├─────┼─────┼─────┼─────┤
│ F9  │ F10 │ F11 │ F12 │  Jump animation
└─────┴─────┴─────┴─────┘
```

**Implementation:**
```python
from PIL import Image

def create_sprite_sheet(frame_paths, output_path, rows=3, cols=4):
    frames = [Image.open(path) for path in frame_paths]
    frame_w, frame_h = frames[0].size
    
    sheet = Image.new('RGBA', (frame_w * cols, frame_h * rows), (0, 0, 0, 0))
    
    for idx, frame in enumerate(frames):
        x = (idx % cols) * frame_w
        y = (idx // cols) * frame_h
        sheet.paste(frame, (x, y))
    
    sheet.save(output_path)
```

#### Approach B: Single-Image Sprite Sheet (AI-Generated)
**Experimental:** Prompt FLUX to generate entire sprite sheet in one image.

**Prompt Example:**
```
"Character sprite sheet showing 4 animation frames, idle pose, walk cycle,
attack motion, all on white background, 2D game sprite style, side view,
evenly spaced frames in horizontal row"
```

**Pros:** Single generation, potentially consistent
**Cons:** Less control, may not align perfectly, experimental

**Recommendation:** Start with Approach A (reliable), test Approach B as optimization.

---

### 3. **Character Consistency System** (IMPORTANT)
**Problem:** Same character needs to look identical across different poses.

**Solution:** Seed-based consistency + detailed character description.

**Strategy:**
```python
# Define a character once
character_spec = {
    "name": "octopus_hero",
    "base_description": "cute purple octopus with big eyes, friendly smile, 
                         wearing red bandana, 8 tentacles",
    "seed": 42,  # Keep same seed for consistency
    "style_lora": "cartoon-flux:0.7"
}

# Generate multiple poses with same base
poses = ["idle", "walk_frame1", "walk_frame2", "jump", "attack"]

for pose in poses:
    prompt = f"{character_spec['base_description']}, {pose} animation frame, 
              side view, game sprite, white background <lora:{character_spec['style_lora']}>"
    
    # Use same seed for all variations
    generate_with_seed(prompt, seed=character_spec['seed'])
```

**Key Techniques:**
1. **Consistent seed** across all poses
2. **Detailed base description** as prefix to all prompts
3. **Reference image** (generate first pose, use as style reference)
4. **LoRA consistency** - same LoRA settings

---

### 4. **Batch Generation Automation**
**Problem:** Generating 50+ assets manually is tedious.

**Solution:** Asset specification files that define entire asset sets.

**Example: Character Asset Specification (JSON)**
```json
{
  "asset_type": "character_sprite_sheet",
  "character": {
    "name": "octopus_hero",
    "description": "cute purple octopus, big expressive eyes, red bandana, 8 tentacles",
    "style": "cartoon-flux:0.7",
    "seed": 42
  },
  "animations": {
    "idle": {
      "frames": 4,
      "prompts": [
        "standing still, relaxed pose",
        "slight bounce, one tentacle raised",
        "back to neutral",
        "other tentacle wave"
      ]
    },
    "walk": {
      "frames": 6,
      "prompts": [
        "moving forward, tentacles extended",
        "mid-stride, tentacles pulling",
        "forward position reached",
        "tentacles extend again",
        "mid-stride opposite side",
        "cycle complete"
      ]
    }
  },
  "output": {
    "individual_frames": true,
    "sprite_sheet": true,
    "resolution": [512, 512],
    "transparent_bg": true
  }
}
```

**Automation Script:**
```python
def generate_character_asset_set(spec_file):
    spec = load_json(spec_file)
    
    for animation, details in spec['animations'].items():
        frames = []
        
        for i, frame_prompt in enumerate(details['prompts']):
            # Build full prompt
            full_prompt = f"{spec['character']['description']}, {frame_prompt}, 
                          side view, 2D game sprite, white background 
                          <lora:{spec['character']['style']}>"
            
            # Generate frame
            output_path = f"output/{spec['character']['name']}_{animation}_f{i}.png"
            generate_flux_image(full_prompt, output_path, seed=spec['character']['seed'])
            
            # Remove background
            transparent_path = output_path.replace('.png', '_transparent.png')
            remove_background(output_path, transparent_path)
            
            frames.append(transparent_path)
        
        # Assemble sprite sheet
        sprite_sheet_path = f"output/{spec['character']['name']}_{animation}_sheet.png"
        create_sprite_sheet(frames, sprite_sheet_path, rows=1, cols=len(frames))
```

---

## 📦 Recommended LoRAs for Game Assets

### Must-Have LoRAs (Download from Civitai/HuggingFace)

1. **Pixel Art LoRAs**
   - `pixel-art-flux-schnell.safetensors` - For retro game sprites
   - `pixel-art-xl.safetensors` - Alternative pixel style
   
2. **Sprite Sheet Specific**
   - `sprite-sheet-generator-flux.safetensors` - If available
   - `game-asset-flux.safetensors` - General game art
   
3. **Animation LoRAs**
   - `animation-frames-flux.safetensors` - For frame consistency
   
4. **Style LoRAs** (You already have some)
   - `cartoon-flux.safetensors` ✅ (You have this)
   - `ghibli-style-flux.safetensors` ✅ (You have this)
   - `pixel-art-flux.safetensors` ✅ (You have this)

5. **Utility LoRAs**
   - `transparent-background-flux.safetensors` - Helps with alpha channels
   - `character-consistency-flux.safetensors` - If available

**Where to Find:**
- Civitai.com (search "FLUX game assets", "FLUX sprite sheet")
- HuggingFace (search "FLUX LoRA pixel art")
- GitHub repositories for FLUX LoRAs

**Critical Note:** As of Jan 2026, FLUX LoRAs for sprite sheets are relatively new. You may need to:
- Use general game art LoRAs
- Rely more on prompt engineering
- Potentially train your own LoRA for specific styles

---

## 🎬 Animated Sprite Sheet Generation Workflow

### Option 1: Individual Frame Generation → Assembly (RECOMMENDED)

```bash
# Step 1: Generate individual frames
for i in {1..6}; do
  ./build/bin/sd-cli \
    --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
    --vae ~/.ai-assets/models/flux/ae.safetensors \
    --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
    --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
    --lora-model-dir ~/.ai-assets/loras/ \
    -p "cute purple octopus, walk cycle frame $i, side view, game sprite, 
        white background <lora:cartoon-flux:0.7>" \
    --seed 42 \
    --cfg-scale 1.0 --sampling-method euler --steps 4 \
    -H 512 -W 512 -o frame_$i.png
done

# Step 2: Remove backgrounds (Python)
python remove_backgrounds.py frame_*.png

# Step 3: Assemble sprite sheet (Python)
python create_sprite_sheet.py --input frame_*_transparent.png \
                               --output octopus_walk_cycle.png \
                               --cols 6 --rows 1
```

### Option 2: AI-Generated Complete Sprite Sheet (EXPERIMENTAL)

```bash
./build/bin/sd-cli \
  --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
  --vae ~/.ai-assets/models/flux/ae.safetensors \
  --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
  --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
  --lora-model-dir ~/.ai-assets/loras/ \
  -p "Complete sprite sheet showing cute purple octopus character, 
      6 frames of walk cycle animation, evenly spaced in horizontal row, 
      consistent character across all frames, side view, 2D game art, 
      white background, pixel art style <lora:pixel-art-flux:0.7>" \
  --cfg-scale 1.0 --sampling-method euler --steps 8 \
  -H 512 -W 3072 -o octopus_walk_sheet_ai.png
  
# Then split into individual frames if needed
python split_sprite_sheet.py --input octopus_walk_sheet_ai.png \
                              --frames 6 --output-dir frames/
```

**Pros/Cons:**

| Approach | Pros | Cons |
|----------|------|------|
| **Option 1** (Individual → Assembly) | ✅ Full control over each frame<br>✅ Easy to fix specific frames<br>✅ Perfect alignment<br>✅ Proven technique | ❌ More generations (slower)<br>❌ Requires post-processing |
| **Option 2** (AI Full Sheet) | ✅ Single generation (faster)<br>✅ Natural flow between frames<br>✅ Less processing | ❌ Less control<br>❌ May not align perfectly<br>❌ Harder to edit individual frames<br>❌ Experimental/unreliable |

**Recommendation:** Start with **Option 1** for reliability. Test **Option 2** once you have the pipeline working.

---

## 💻 Python Toolkit Implementation

### Core Modules Needed

```
game_asset_toolkit/
├── __init__.py
├── generators/
│   ├── __init__.py
│   ├── flux_generator.py       # Wrapper for FLUX CLI calls
│   └── batch_generator.py      # Batch processing from JSON specs
├── processors/
│   ├── __init__.py
│   ├── background_remover.py   # rembg integration
│   ├── sprite_assembler.py     # Grid-based sprite sheet creation
│   └── image_processor.py      # Resize, crop, format conversion
├── utils/
│   ├── __init__.py
│   ├── prompt_builder.py       # Template-based prompt generation
│   ├── seed_manager.py         # Consistent seed tracking
│   └── asset_organizer.py      # File naming and organization
└── cli.py                       # Command-line interface for Claude Code
```

### Example: `flux_generator.py`

```python
import subprocess
import os

class FluxGenerator:
    def __init__(self, models_dir="~/.ai-assets/models/flux/",
                 loras_dir="~/.ai-assets/loras/"):
        self.models_dir = os.path.expanduser(models_dir)
        self.loras_dir = os.path.expanduser(loras_dir)
        self.sd_cpp_path = os.path.expanduser("~/stable-diffusion.cpp/build/bin/sd-cli")
    
    def generate(self, prompt, output_path, 
                 model="flux-schnell-q4",
                 width=512, height=512, 
                 steps=4, seed=None, cfg_scale=1.0):
        
        cmd = [
            self.sd_cpp_path,
            "--diffusion-model", f"{self.models_dir}/{model}.gguf",
            "--vae", f"{self.models_dir}/ae.safetensors",
            "--clip_l", f"{self.models_dir}/clip_l.safetensors",
            "--t5xxl", f"{self.models_dir}/t5-Q5_K_M.gguf",
            "--lora-model-dir", self.loras_dir,
            "-p", prompt,
            "--cfg-scale", str(cfg_scale),
            "--sampling-method", "euler",
            "--steps", str(steps),
            "-H", str(height),
            "-W", str(width),
            "-o", output_path
        ]
        
        if seed is not None:
            cmd.extend(["--seed", str(seed)])
        
        subprocess.run(cmd, check=True)
        return output_path
```

---

## 🚀 Implementation Roadmap

### Phase 1: Core Infrastructure (Week 1)
- [ ] Set up Python virtual environment
- [ ] Install dependencies (rembg, Pillow, etc.)
- [ ] Create basic FluxGenerator wrapper
- [ ] Implement background removal
- [ ] Test individual asset generation

### Phase 2: Sprite Sheet System (Week 1-2)
- [ ] Implement sprite sheet assembler
- [ ] Create animation frame generator
- [ ] Test multi-pose character generation with seed consistency
- [ ] Build prompt template system

### Phase 3: Batch Automation (Week 2)
- [ ] Design JSON specification format
- [ ] Implement batch generator
- [ ] Create asset organizer
- [ ] Test full pipeline with sample character

### Phase 4: Claude Code Integration (Week 2-3)
- [ ] Create CLI interface for toolkit
- [ ] Write documentation for autonomous operation
- [ ] Test Claude Code's ability to use the system
- [ ] Refine based on testing

### Phase 5: Advanced Features (Week 3-4)
- [ ] Tileset generation
- [ ] Parallax background layers
- [ ] UI element generation
- [ ] Asset variation system

---

## 🎯 Claude Code Integration Strategy

### How Claude Code Will Use This System

**Example Workflow:**

```bash
# 1. Claude Code creates asset specification
cat > octopus_hero_spec.json << 'EOF'
{
  "character": "octopus_hero",
  "description": "cute purple octopus, big eyes, red bandana",
  "animations": ["idle", "walk", "jump", "attack"],
  "style": "cartoon-flux:0.7",
  "resolution": [512, 512]
}
EOF

# 2. Claude Code runs batch generator
python -m game_asset_toolkit generate-character octopus_hero_spec.json

# 3. System automatically:
#    - Generates all animation frames
#    - Removes backgrounds
#    - Assembles sprite sheets
#    - Organizes output

# 4. Claude Code can inspect results
ls output/octopus_hero/
# idle_sheet.png
# walk_sheet.png
# jump_sheet.png
# attack_sheet.png
# metadata.json
```

**Key Features for Autonomy:**
1. **JSON-driven**: Claude Code can generate specifications programmatically
2. **CLI interface**: Simple commands for complex operations
3. **Error handling**: Clear error messages for Claude Code to interpret
4. **Metadata output**: JSON files tracking what was generated and how
5. **Resume capability**: Can continue from partial generations

---

## 📝 Asset Specification Format

```json
{
  "project": "Octoplat",
  "asset_categories": {
    "characters": [
      {
        "id": "octopus_hero",
        "base_description": "cute purple octopus protagonist with big expressive eyes, red bandana",
        "seed": 42,
        "style_lora": "cartoon-flux:0.7",
        "animations": {
          "idle": {"frames": 4, "fps": 8},
          "walk": {"frames": 6, "fps": 12},
          "jump": {"frames": 4, "fps": 10},
          "attack": {"frames": 5, "fps": 15}
        },
        "variants": ["normal", "powered_up", "hurt"]
      }
    ],
    "enemies": [
      {
        "id": "ocean_crab",
        "base_description": "angry red crab with blue accents, raised claws",
        "seed": 100,
        "style_lora": "cartoon-flux:0.6",
        "animations": {
          "idle": {"frames": 2, "fps": 4},
          "walk": {"frames": 4, "fps": 8},
          "attack": {"frames": 3, "fps": 12}
        }
      }
    ],
    "backgrounds": [
      {
        "id": "ocean_depths_bg",
        "description": "deep ocean background with dark blue gradient, light rays from surface",
        "layers": ["far", "mid", "near"],
        "resolution": [1280, 704],
        "style_lora": "ghibli-style-flux:0.5"
      }
    ],
    "ui": [
      {
        "id": "health_heart",
        "description": "cute cartoon heart icon, glossy red",
        "variants": ["full", "half", "empty"],
        "resolution": [64, 64]
      }
    ]
  }
}
```

---

## ⚡ Quick Start Implementation

### Minimal Viable Toolkit (Start Here)

**File: `simple_asset_gen.py`**
```python
#!/usr/bin/env python3
"""
Simple Game Asset Generator
Minimal toolkit to get started with automated asset generation
"""

import subprocess
import json
import os
from pathlib import Path

# 1. FLUX Generator
def generate_asset(prompt, output_path, width=512, height=512, seed=None):
    """Generate a single asset using FLUX"""
    cmd = f"""
    cd ~/stable-diffusion.cpp && ./build/bin/sd-cli \
      --diffusion-model ~/.ai-assets/models/flux/flux-schnell-q4.gguf \
      --vae ~/.ai-assets/models/flux/ae.safetensors \
      --clip_l ~/.ai-assets/models/flux/clip_l.safetensors \
      --t5xxl ~/.ai-assets/models/flux/t5-Q5_K_M.gguf \
      --lora-model-dir ~/.ai-assets/loras/ \
      -p "{prompt}" \
      --cfg-scale 1.0 --sampling-method euler --steps 4 \
      -H {height} -W {width} \
      {f'--seed {seed}' if seed else ''} \
      -o {output_path}
    """
    subprocess.run(cmd, shell=True, check=True)

# 2. Background Removal (Install: pip install rembg pillow)
def remove_background(input_path, output_path):
    """Remove background from sprite"""
    try:
        from rembg import remove
        from PIL import Image
        
        with open(input_path, 'rb') as i:
            input_img = Image.open(i)
            output_img = remove(input_img)
            output_img.save(output_path)
        return True
    except ImportError:
        print("rembg not installed. Run: pip install rembg")
        return False

# 3. Sprite Sheet Assembly
def create_sprite_sheet(frame_paths, output_path, cols=4):
    """Combine frames into sprite sheet"""
    from PIL import Image
    
    frames = [Image.open(p) for p in frame_paths]
    w, h = frames[0].size
    rows = (len(frames) + cols - 1) // cols
    
    sheet = Image.new('RGBA', (w * cols, h * rows), (0, 0, 0, 0))
    
    for idx, frame in enumerate(frames):
        x = (idx % cols) * w
        y = (idx // cols) * h
        sheet.paste(frame, (x, y), frame if frame.mode == 'RGBA' else None)
    
    sheet.save(output_path)

# 4. Character Asset Set Generator
def generate_character_set(spec_file):
    """Generate complete character asset set from JSON spec"""
    with open(spec_file) as f:
        spec = json.load(f)
    
    char = spec['character']
    output_dir = Path(f"output/{char['name']}")
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for anim_name, anim_data in spec['animations'].items():
        frames = []
        
        for i in range(anim_data['frames']):
            # Generate frame
            prompt = f"{char['description']}, {anim_name} animation frame {i+1}, " \
                    f"side view, 2D game sprite, white background <lora:{char['style']}>"
            
            frame_path = output_dir / f"{anim_name}_f{i}.png"
            generate_asset(prompt, str(frame_path), seed=char['seed'])
            
            # Remove background
            trans_path = output_dir / f"{anim_name}_f{i}_trans.png"
            if remove_background(str(frame_path), str(trans_path)):
                frames.append(str(trans_path))
            else:
                frames.append(str(frame_path))
        
        # Create sprite sheet
        sheet_path = output_dir / f"{anim_name}_sheet.png"
        create_sprite_sheet(frames, str(sheet_path), cols=anim_data['frames'])
        
        print(f"✓ Generated {anim_name} animation ({anim_data['frames']} frames)")

# CLI
if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        generate_character_set(sys.argv[1])
    else:
        print("Usage: python simple_asset_gen.py <spec.json>")
```

**Usage:**
```bash
# 1. Install dependencies
pip install rembg pillow

# 2. Create character spec
cat > octopus_spec.json << 'EOF'
{
  "character": {
    "name": "octopus_hero",
    "description": "cute purple octopus with big eyes and red bandana",
    "style": "cartoon-flux:0.7",
    "seed": 42
  },
  "animations": {
    "idle": {"frames": 4},
    "walk": {"frames": 6}
  }
}
EOF

# 3. Generate
python simple_asset_gen.py octopus_spec.json

# 4. Check output
ls output/octopus_hero/
```

---

## 🎓 Answer to Your Question

### Can Claude Code Do This Autonomously?

**Short Answer:** Not yet, but you're very close! You need to implement the missing pieces.

**Current Capability:** 60%
- ✅ FLUX can generate high-quality individual images
- ✅ Good prompt templates exist
- ✅ LoRAs for various styles work
- ❌ No sprite sheet assembly
- ❌ No background removal
- ❌ No batch automation
- ❌ No consistency management

### What You Need to Build

**Priority 1 (Essential):**
1. **Background removal script** - 1-2 hours
2. **Sprite sheet assembler** - 2-3 hours
3. **Basic batch generator** - 3-4 hours

**Priority 2 (Important):**
4. **JSON specification format** - 2 hours
5. **Seed-based consistency** - 1 hour
6. **Asset organization** - 2 hours

**Priority 3 (Enhancement):**
7. **Advanced LoRA management** - Research needed
8. **AI-generated full sprite sheets** - Experimental
9. **Tileset generation** - 3-4 hours

### Recommended LoRAs to Find

**Search on Civitai/HuggingFace for:**
1. `flux sprite sheet` - Specialized LoRAs
2. `flux game asset` - General game art
3. `flux pixel art` - For retro games (you have this)
4. `flux character sheet` - Character turnarounds
5. `flux animation frames` - Frame consistency

**Realistically:** FLUX sprite sheet LoRAs are still emerging. You'll likely rely more on:
- Good prompt engineering
- Post-processing Python scripts
- Seed consistency techniques

### Implementation Timeline

**Week 1:** Build Python toolkit (10-15 hours)
**Week 2:** Test and refine, create templates (8-10 hours)
**Week 3:** Claude Code integration and automation (5-8 hours)

**Total:** 23-33 hours to full autonomy

---

## 🚦 Recommendation: Start Simple, Scale Up

### Phase 1: Prove Concept (This Weekend)
```bash
# 1. Install dependencies
pip install rembg pillow

# 2. Test single sprite generation + background removal
python -c "
from rembg import remove
from PIL import Image
img = Image.open('test_sprite.png')
result = remove(img)
result.save('test_sprite_transparent.png')
"

# 3. Test sprite sheet assembly with 4 manual frames
python create_sprite_sheet.py frame1.png frame2.png frame3.png frame4.png -o sheet.png
```

### Phase 2: Build Automation (Next Week)
- Implement the `simple_asset_gen.py` script above
- Create character spec JSON files
- Test batch generation

### Phase 3: Full Autonomy (Week After)
- Expand to all asset types
- Add error handling and recovery
- Create comprehensive documentation for Claude Code
- Test Claude Code's ability to operate the system

---

## ✅ Action Items (Prioritized)

1. **[HIGH]** Implement background removal (`rembg` or ImageMagick)
2. **[HIGH]** Create sprite sheet assembler script
3. **[HIGH]** Build minimal batch generator
4. **[MEDIUM]** Design JSON specification format
5. **[MEDIUM]** Test seed-based consistency
6. **[MEDIUM]** Search for FLUX sprite sheet LoRAs on Civitai
7. **[LOW]** Explore AI-generated full sprite sheets
8. **[LOW]** Build tileset generation system

---

## 🎯 The Königsdisziplin Achievement Path

**Your Goal:** Claude Code autonomously generates complete game asset packages

**What Makes This "Königsdisziplin":**
- ✨ Full animation sprite sheets
- ✨ Character consistency across poses
- ✨ Transparent backgrounds
- ✨ Organized asset libraries
- ✨ Batch generation from high-level descriptions
- ✨ Zero manual intervention

**You're 60% there.** The missing 40% is post-processing automation and orchestration - which is totally achievable with the Python toolkit outlined above.

**Key Insight:** FLUX is amazing at generating individual high-quality images. The "magic" comes from:
1. Smart prompting (seed consistency, detailed descriptions)
2. Post-processing (background removal, assembly)
3. Orchestration (batch automation, organization)

This is **not** a LoRA problem - it's an **automation and pipeline** problem. You need to build the glue code.

---

**Ready to start?** I recommend beginning with the `simple_asset_gen.py` script and testing it with a single character. Once that works, scaling up is straightforward.

Would you like me to implement any specific component first?

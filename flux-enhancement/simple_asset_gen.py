#!/usr/bin/env python3
"""
Simple Game Asset Generator
Minimal toolkit to get started with automated FLUX-based asset generation

Usage:
    python simple_asset_gen.py generate-single "prompt" output.png
    python simple_asset_gen.py generate-character character_spec.json
    python simple_asset_gen.py remove-bg input.png output.png
    python simple_asset_gen.py create-sheet frame1.png frame2.png ... -o sheet.png
"""

import subprocess
import json
import os
import sys
from pathlib import Path
from typing import List, Optional

# ============================================================================
# FLUX GENERATOR
# ============================================================================

class FluxGenerator:
    """Wrapper for stable-diffusion.cpp FLUX generation"""
    
    def __init__(self):
        self.sd_path = Path.home() / "stable-diffusion.cpp" / "build" / "bin" / "sd-cli"
        self.models_dir = Path.home() / ".ai-assets" / "models" / "flux"
        self.loras_dir = Path.home() / ".ai-assets" / "loras"
        
    def generate(self, 
                 prompt: str,
                 output_path: str,
                 width: int = 512,
                 height: int = 512,
                 model: str = "flux-schnell-q4",
                 steps: int = 4,
                 seed: Optional[int] = None,
                 cfg_scale: float = 1.0) -> str:
        """
        Generate a single image using FLUX
        
        Args:
            prompt: Text description of what to generate
            output_path: Where to save the image
            width: Image width in pixels
            height: Image height in pixels
            model: Which FLUX model to use (flux-schnell-q4 or flux-dev-q8)
            steps: Number of diffusion steps
            seed: Random seed for reproducibility
            cfg_scale: CFG scale (always 1.0 for FLUX)
        
        Returns:
            Path to generated image
        """
        cmd = [
            str(self.sd_path),
            "--diffusion-model", str(self.models_dir / f"{model}.gguf"),
            "--vae", str(self.models_dir / "ae.safetensors"),
            "--clip_l", str(self.models_dir / "clip_l.safetensors"),
            "--t5xxl", str(self.models_dir / "t5-Q5_K_M.gguf"),
            "--lora-model-dir", str(self.loras_dir),
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
        
        print(f"🎨 Generating: {Path(output_path).name}")
        print(f"   Prompt: {prompt[:80]}...")
        
        subprocess.run(cmd, check=True)
        print(f"✓ Saved to: {output_path}")
        
        return output_path


# ============================================================================
# BACKGROUND REMOVAL
# ============================================================================

def remove_background(input_path: str, output_path: str) -> bool:
    """
    Remove background from sprite using AI
    
    Requires: pip install rembg pillow
    
    Args:
        input_path: Path to input image with background
        output_path: Path to save transparent image
    
    Returns:
        True if successful, False if rembg not installed
    """
    try:
        from rembg import remove
        from PIL import Image
        
        print(f"🔍 Removing background from: {Path(input_path).name}")
        
        input_img = Image.open(input_path)
        output_img = remove(input_img)
        output_img.save(output_path)
        
        print(f"✓ Transparent image saved: {output_path}")
        return True
        
    except ImportError:
        print("❌ Error: rembg not installed")
        print("   Install with: pip install rembg pillow")
        return False


# ============================================================================
# SPRITE SHEET ASSEMBLY
# ============================================================================

def create_sprite_sheet(frame_paths: List[str], 
                       output_path: str,
                       cols: int = 4,
                       transparent: bool = True) -> str:
    """
    Combine multiple frames into a sprite sheet grid
    
    Args:
        frame_paths: List of paths to individual frames
        output_path: Where to save the sprite sheet
        cols: Number of columns in grid
        transparent: Whether to maintain transparency
    
    Returns:
        Path to created sprite sheet
    """
    try:
        from PIL import Image
    except ImportError:
        print("❌ Error: Pillow not installed")
        print("   Install with: pip install Pillow")
        return ""
    
    print(f"🎞️  Creating sprite sheet from {len(frame_paths)} frames")
    
    # Load all frames
    frames = []
    for path in frame_paths:
        if not Path(path).exists():
            print(f"⚠️  Warning: Frame not found: {path}")
            continue
        frames.append(Image.open(path))
    
    if not frames:
        print("❌ Error: No valid frames found")
        return ""
    
    # Calculate grid dimensions
    frame_w, frame_h = frames[0].size
    rows = (len(frames) + cols - 1) // cols
    
    # Create sprite sheet
    mode = 'RGBA' if transparent else 'RGB'
    bg_color = (0, 0, 0, 0) if transparent else (255, 255, 255)
    
    sheet = Image.new(mode, (frame_w * cols, frame_h * rows), bg_color)
    
    # Paste frames into grid
    for idx, frame in enumerate(frames):
        x = (idx % cols) * frame_w
        y = (idx // cols) * frame_h
        
        # Handle transparency
        if transparent and frame.mode == 'RGBA':
            sheet.paste(frame, (x, y), frame)
        else:
            sheet.paste(frame, (x, y))
    
    sheet.save(output_path)
    print(f"✓ Sprite sheet created: {output_path}")
    print(f"   Grid: {cols} cols × {rows} rows ({len(frames)} frames)")
    
    return output_path


# ============================================================================
# CHARACTER ASSET SET GENERATOR
# ============================================================================

def generate_character_set(spec_file: str) -> None:
    """
    Generate complete character asset set from JSON specification
    
    Spec file format:
    {
      "character": {
        "name": "character_id",
        "description": "detailed character description",
        "style": "lora-name:strength",
        "seed": 42
      },
      "animations": {
        "animation_name": {
          "frames": 4,
          "prompts": ["frame 1 desc", "frame 2 desc", ...]
        }
      },
      "resolution": [512, 512]
    }
    
    Args:
        spec_file: Path to JSON specification file
    """
    # Load specification
    with open(spec_file) as f:
        spec = json.load(f)
    
    char = spec['character']
    output_dir = Path("output") / char['name']
    output_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"\n{'='*60}")
    print(f"Generating Character Asset Set: {char['name']}")
    print(f"{'='*60}\n")
    
    generator = FluxGenerator()
    resolution = spec.get('resolution', [512, 512])
    
    # Generate each animation
    for anim_name, anim_data in spec['animations'].items():
        print(f"\n--- Animation: {anim_name} ---")
        
        frames = []
        frame_prompts = anim_data.get('prompts', [])
        num_frames = anim_data.get('frames', len(frame_prompts))
        
        # Generate individual frames
        for i in range(num_frames):
            # Build frame-specific prompt
            if i < len(frame_prompts):
                frame_desc = frame_prompts[i]
            else:
                frame_desc = f"{anim_name} frame {i+1}"
            
            full_prompt = (f"{char['description']}, {frame_desc}, "
                          f"side view, 2D game sprite, white background "
                          f"<lora:{char['style']}>")
            
            # Generate frame
            frame_path = output_dir / f"{anim_name}_f{i:02d}.png"
            generator.generate(
                prompt=full_prompt,
                output_path=str(frame_path),
                width=resolution[0],
                height=resolution[1],
                seed=char.get('seed')
            )
            
            # Remove background
            trans_path = output_dir / f"{anim_name}_f{i:02d}_transparent.png"
            if remove_background(str(frame_path), str(trans_path)):
                frames.append(str(trans_path))
            else:
                frames.append(str(frame_path))
        
        # Create sprite sheet
        sheet_path = output_dir / f"{anim_name}_sheet.png"
        create_sprite_sheet(frames, str(sheet_path), cols=num_frames)
    
    # Save metadata
    metadata = {
        "character": char,
        "generated_animations": list(spec['animations'].keys()),
        "total_frames": sum(a['frames'] for a in spec['animations'].values()),
        "output_directory": str(output_dir)
    }
    
    metadata_path = output_dir / "metadata.json"
    with open(metadata_path, 'w') as f:
        json.dump(metadata, f, indent=2)
    
    print(f"\n{'='*60}")
    print(f"✓ Character set complete!")
    print(f"  Output: {output_dir}")
    print(f"  Animations: {', '.join(spec['animations'].keys())}")
    print(f"  Total frames: {metadata['total_frames']}")
    print(f"{'='*60}\n")


# ============================================================================
# BATCH ASSET GENERATOR
# ============================================================================

def generate_batch_assets(batch_file: str) -> None:
    """
    Generate multiple assets from a batch specification file
    
    Batch file format:
    {
      "assets": [
        {
          "type": "sprite",
          "prompt": "description",
          "output": "filename.png",
          "width": 512,
          "height": 512,
          "seed": 42,
          "remove_bg": true
        },
        ...
      ]
    }
    """
    with open(batch_file) as f:
        batch = json.load(f)
    
    generator = FluxGenerator()
    
    print(f"\n{'='*60}")
    print(f"Batch Asset Generation: {len(batch['assets'])} assets")
    print(f"{'='*60}\n")
    
    for i, asset in enumerate(batch['assets'], 1):
        print(f"\n[{i}/{len(batch['assets'])}] {asset.get('type', 'asset')}: {asset['output']}")
        
        # Generate asset
        output_path = asset['output']
        generator.generate(
            prompt=asset['prompt'],
            output_path=output_path,
            width=asset.get('width', 512),
            height=asset.get('height', 512),
            seed=asset.get('seed'),
            steps=asset.get('steps', 4)
        )
        
        # Optional background removal
        if asset.get('remove_bg', False):
            base, ext = os.path.splitext(output_path)
            trans_path = f"{base}_transparent{ext}"
            remove_background(output_path, trans_path)
    
    print(f"\n{'='*60}")
    print(f"✓ Batch generation complete!")
    print(f"{'='*60}\n")


# ============================================================================
# CLI INTERFACE
# ============================================================================

def main():
    """Command-line interface"""
    
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    
    command = sys.argv[1]
    
    # Generate single asset
    if command == "generate-single":
        if len(sys.argv) < 4:
            print("Usage: python simple_asset_gen.py generate-single PROMPT OUTPUT_PATH [OPTIONS]")
            sys.exit(1)
        
        prompt = sys.argv[2]
        output = sys.argv[3]
        
        # Parse optional arguments
        width = int(sys.argv[4]) if len(sys.argv) > 4 else 512
        height = int(sys.argv[5]) if len(sys.argv) > 5 else 512
        seed = int(sys.argv[6]) if len(sys.argv) > 6 else None
        
        generator = FluxGenerator()
        generator.generate(prompt, output, width, height, seed=seed)
    
    # Generate character set
    elif command == "generate-character":
        if len(sys.argv) < 3:
            print("Usage: python simple_asset_gen.py generate-character SPEC_FILE.json")
            sys.exit(1)
        
        spec_file = sys.argv[2]
        generate_character_set(spec_file)
    
    # Batch generation
    elif command == "generate-batch":
        if len(sys.argv) < 3:
            print("Usage: python simple_asset_gen.py generate-batch BATCH_FILE.json")
            sys.exit(1)
        
        batch_file = sys.argv[2]
        generate_batch_assets(batch_file)
    
    # Remove background
    elif command == "remove-bg":
        if len(sys.argv) < 4:
            print("Usage: python simple_asset_gen.py remove-bg INPUT OUTPUT")
            sys.exit(1)
        
        input_path = sys.argv[2]
        output_path = sys.argv[3]
        remove_background(input_path, output_path)
    
    # Create sprite sheet
    elif command == "create-sheet":
        if len(sys.argv) < 4:
            print("Usage: python simple_asset_gen.py create-sheet FRAME1 FRAME2 ... -o OUTPUT")
            sys.exit(1)
        
        # Find output flag
        try:
            o_idx = sys.argv.index('-o')
            output = sys.argv[o_idx + 1]
            frames = sys.argv[2:o_idx]
        except (ValueError, IndexError):
            print("Error: Must specify output with -o flag")
            sys.exit(1)
        
        # Optional columns
        cols = 4
        if '-c' in sys.argv:
            c_idx = sys.argv.index('-c')
            cols = int(sys.argv[c_idx + 1])
        
        create_sprite_sheet(frames, output, cols=cols)
    
    else:
        print(f"Unknown command: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()

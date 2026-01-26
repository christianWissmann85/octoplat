#!/usr/bin/env python3
"""
Batch Tile Texture Generator for Octoplat
Generates seamlessly tileable textures for biome blocks

Usage:
    python batch_tiles.py ../assets/textures/tiles/batch_tiles.json
"""

import json
import sys
from pathlib import Path
from simple_asset_gen import FluxGenerator

def generate_tiles(batch_file: str) -> None:
    """
    Generate tile textures from batch specification

    Expected format:
    {
      "metadata": {
        "loras": "<lora:...>",
        "width": 128,
        "height": 128,
        "steps": 30
      },
      "images": [
        {
          "filename": "ocean_depths.png",
          "prompt": "..."
        }
      ]
    }
    """
    with open(batch_file) as f:
        batch = json.load(f)

    batch_path = Path(batch_file)
    output_dir = batch_path.parent
    metadata = batch['metadata']
    images = batch['images']

    loras = metadata.get('loras', '')
    width = metadata.get('width', 128)
    height = metadata.get('height', 128)
    steps = metadata.get('steps', 30)

    generator = FluxGenerator()

    print(f"\n{'='*60}")
    print(f"Tile Texture Batch Generation: {len(images)} textures")
    print(f"Output directory: {output_dir}")
    print(f"Size: {width}x{height}, Steps: {steps}")
    print(f"{'='*60}\n")

    successful = 0
    failed = 0

    for i, image in enumerate(images, 1):
        filename = image['filename']
        prompt = image['prompt']

        # Add LoRAs to prompt
        full_prompt = f"{loras} {prompt}" if loras else prompt

        output_path = output_dir / filename

        print(f"\n[{i}/{len(images)}] {filename}")
        print(f"   Prompt: {prompt[:60]}...")

        try:
            generator.generate(
                prompt=full_prompt,
                output_path=str(output_path),
                width=width,
                height=height,
                steps=steps
            )
            print(f"   ✓ Created: {output_path}")
            successful += 1

        except Exception as e:
            print(f"   ✗ Failed: {e}")
            failed += 1

    print(f"\n{'='*60}")
    print(f"Batch generation complete!")
    print(f"  Successful: {successful}")
    print(f"  Failed: {failed}")
    print(f"{'='*60}\n")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    batch_file = sys.argv[1]
    if not Path(batch_file).exists():
        print(f"Error: Batch file not found: {batch_file}")
        sys.exit(1)

    generate_tiles(batch_file)


if __name__ == "__main__":
    main()

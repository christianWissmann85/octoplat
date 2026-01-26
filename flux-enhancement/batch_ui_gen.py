#!/usr/bin/env python3
"""
Batch UI Asset Generator for Octoplat
Generates UI assets with optional background removal

Usage:
    python batch_ui_gen.py ui_screens_batch.json
"""

import json
import sys
from pathlib import Path
from simple_asset_gen import FluxGenerator, remove_background

def generate_ui_assets(batch_file: str) -> None:
    """
    Generate UI assets from batch specification

    Expected format:
    {
      "output_dir": "/path/to/assets/ui",
      "assets": [
        {
          "name": "background",
          "subdir": "loading",
          "prompt": "...",
          "width": 1280,
          "height": 720,
          "remove_bg": false
        }
      ]
    }
    """
    with open(batch_file) as f:
        batch = json.load(f)

    output_dir = Path(batch['output_dir'])
    assets = batch['assets']

    generator = FluxGenerator()

    print(f"\n{'='*60}")
    print(f"UI Asset Batch Generation: {len(assets)} assets")
    print(f"Output directory: {output_dir}")
    print(f"{'='*60}\n")

    successful = 0
    failed = 0

    for i, asset in enumerate(assets, 1):
        name = asset['name']
        subdir = asset['subdir']
        prompt = asset['prompt']
        width = asset.get('width', 512)
        height = asset.get('height', 512)
        should_remove_bg = asset.get('remove_bg', False)

        # Create output subdirectory
        asset_dir = output_dir / subdir
        asset_dir.mkdir(parents=True, exist_ok=True)

        # Paths
        raw_path = asset_dir / f"{name}_raw.png"
        final_path = asset_dir / f"{name}.png"

        print(f"\n[{i}/{len(assets)}] {subdir}/{name}")
        print(f"   Size: {width}x{height}")
        print(f"   Remove BG: {should_remove_bg}")
        print(f"   Prompt: {prompt[:70]}...")

        try:
            # Generate with FLUX
            generator.generate(
                prompt=prompt,
                output_path=str(raw_path),
                width=width,
                height=height,
                steps=4
            )

            if should_remove_bg:
                # Remove background
                if remove_background(str(raw_path), str(final_path)):
                    # Clean up raw file
                    raw_path.unlink()
                    print(f"   ✓ Created (with BG removal): {final_path}")
                    successful += 1
                else:
                    # Keep raw if bg removal failed
                    raw_path.rename(final_path)
                    print(f"   ⚠ BG removal failed, kept raw: {final_path}")
                    successful += 1
            else:
                # No BG removal needed
                raw_path.rename(final_path)
                print(f"   ✓ Created: {final_path}")
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
        print("\nAvailable batch files:")
        for f in Path(".").glob("*_batch.json"):
            print(f"  - {f}")
        sys.exit(1)

    batch_file = sys.argv[1]
    if not Path(batch_file).exists():
        print(f"Error: Batch file not found: {batch_file}")
        sys.exit(1)

    generate_ui_assets(batch_file)


if __name__ == "__main__":
    main()

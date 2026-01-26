#!/bin/bash
# Build for Windows and copy to Desktop

echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "Build successful!" 

else
    echo "Build failed!"
    exit 1
fi

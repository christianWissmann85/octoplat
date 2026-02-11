#!/bin/bash
# Build script for Octoplat WebAssembly deployment

set -e

echo "Building Octoplat for WebAssembly..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if wasm32-unknown-unknown target is installed
echo -e "${BLUE}Checking for wasm32-unknown-unknown target...${NC}"
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build the WASM binary
echo -e "${BLUE}Building WASM binary with release optimizations...${NC}"
cargo build --release --target wasm32-unknown-unknown

# Create docs directory if it doesn't exist
mkdir -p docs

# Copy the WASM binary
echo -e "${BLUE}Copying WASM binary to docs/...${NC}"
cp target/wasm32-unknown-unknown/release/octoplat.wasm docs/

# Download the macroquad JS loader
echo -e "${BLUE}Downloading macroquad JS loader...${NC}"
curl -sL https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o docs/mq_js_bundle.js

# Get file sizes
WASM_SIZE=$(du -h docs/octoplat.wasm | cut -f1)
echo -e "${GREEN}WASM build complete!${NC}"
echo -e "  Binary size: ${WASM_SIZE}"

# Optional: Run wasm-opt if available for further optimization
if command -v wasm-opt &> /dev/null; then
    echo -e "${BLUE}Running wasm-opt for additional optimization...${NC}"
    wasm-opt -Oz docs/octoplat.wasm -o docs/octoplat.wasm
    OPTIMIZED_SIZE=$(du -h docs/octoplat.wasm | cut -f1)
    echo -e "${GREEN}Optimized size: ${OPTIMIZED_SIZE}${NC}"
else
    echo -e "${BLUE}Tip: Install binaryen (wasm-opt) for better compression:${NC}"
    echo "  sudo apt-get install binaryen"
fi

echo ""
echo -e "${GREEN}Web build ready in docs/ directory!${NC}"
echo "To test locally, run:"
echo "  python3 -m http.server 8080 --directory docs"
echo "Then open: http://localhost:8080"

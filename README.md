# Web Build (docs/)

This directory contains the web build configuration for Octoplat.

## 🌐 Play Online

Once deployed, the game will be available at:
**https://christianWissmann85.github.io/octoplat/**

## 🔨 Building Locally

```bash
# Run the build script
./scripts/build-web.sh

# Test locally with Python's HTTP server
python3 -m http.server 8080 --directory docs

# Open in browser
# http://localhost:8080
```

## 🚀 Deployment

Deployment is automated via GitHub Actions:

1. **Push to master** triggers automatic build and deployment
2. The workflow builds the WASM binary with optimizations
3. Built files are deployed to the `gh-pages` branch
4. GitHub Pages serves the game

## 📂 Files

- `index.html` - Game wrapper with loading UI and controls (committed to repo)
- `octoplat.wasm` - Compiled game binary (generated during build, not committed)
- `gl.js` - macroquad JavaScript loader (generated during build, not committed)

## ⚙️ Manual Deployment

If you need to deploy manually:

```bash
# Build
./scripts/build-web.sh

# The workflow will handle deployment automatically on push
# Or enable GitHub Pages in Settings > Pages > Source: gh-pages branch
```

## 🐛 Troubleshooting

**Build fails with "target not found":**
```bash
rustup target add wasm32-unknown-unknown
```

**WASM file too large:**
- Install binaryen for wasm-opt: `sudo apt-get install binaryen`
- Or use cargo: `cargo install wasm-opt`
- The build script will use it automatically if available

**Game doesn't load in browser:**
- Check browser console for errors
- Ensure WASM file is being served with correct MIME type
- GitHub Pages handles this automatically

#!/bin/bash
# Run octoplat with audio thread error messages suppressed

cd "$(dirname "$0")/.." || exit 1
cargo run 2>&1 | grep -v "Audio thread died"

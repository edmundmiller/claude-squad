#!/usr/bin/env bash
set -euo pipefail

# Ensure rustup in PATH for non-interactive shells
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  . "$HOME/.cargo/env"
fi

rustup target add wasm32-wasip1 || true
cargo build --target wasm32-wasip1 --release

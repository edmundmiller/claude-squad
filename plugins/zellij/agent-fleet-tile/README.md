# Agent Fleet Zellij Tile (experimental)

A minimal Zellij plugin that creates a git worktree for the current repo and launches your preferred agent (Claude, Aider, Gemini, or a custom command) in that worktree.

## Prerequisites
- zellij >= 0.41
- Rust toolchain
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`

## Build

```bash
rustup target add wasm32-wasip1  # idempotent
cargo build --target wasm32-wasip1 --release
```

Output: `target/wasm32-wasip1/release/agent_fleet_tile.wasm`

## Run via layout

```bash
zellij --layout layouts/agent-fleet.kdl
```

## Notes
- Press Enter to create the worktree and launch.
- Branch defaults to `<user>/zellij-plugin`.
- Worktree defaults to `../<repo>-af-<short-id>`.
- Base defaults to `origin/main`.

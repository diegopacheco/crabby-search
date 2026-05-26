#!/usr/bin/env bash
set -e
root="$(cd "$(dirname "$0")" && pwd)"
cargo build --release --manifest-path "$root/engine/Cargo.toml"
cd "$root/web"
bun install
bun run build
echo "engine binary: $root/engine/target/release/crabby-search"
echo "web bundle: $root/web/dist"

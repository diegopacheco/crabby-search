#!/usr/bin/env bash
set -e
root="$(cd "$(dirname "$0")" && pwd)"
binary="$root/engine/target/release/crabby-search"
if [ ! -f "$binary" ]; then
  cargo build --release --manifest-path "$root/engine/Cargo.toml"
fi
nohup "$binary" > "$root/engine.log" 2>&1 &
echo $! > "$root/.engine.pid"
until bash -c "echo > /dev/tcp/127.0.0.1/7700" 2>/dev/null; do
  sleep 1
done
cd "$root/web"
if [ ! -d node_modules ]; then
  bun install
fi
nohup bun run dev > "$root/web.log" 2>&1 &
echo $! > "$root/.web.pid"
until bash -c "echo > /dev/tcp/127.0.0.1/5173" 2>/dev/null; do
  sleep 1
done
echo "engine: http://localhost:7700"
echo "web: http://localhost:5173"

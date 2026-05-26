#!/usr/bin/env bash
root="$(cd "$(dirname "$0")" && pwd)"

kill_tree() {
  local pid="$1"
  local child
  for child in $(pgrep -P "$pid" 2>/dev/null); do
    kill_tree "$child"
  done
  kill "$pid" 2>/dev/null
}

for name in web engine; do
  pidfile="$root/.$name.pid"
  if [ -f "$pidfile" ]; then
    kill_tree "$(cat "$pidfile")"
    rm -f "$pidfile"
  fi
done
echo "stopped"

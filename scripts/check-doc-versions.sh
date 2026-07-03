#!/usr/bin/env bash
# Fail when user-facing docs contain stale dependency pins.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "=== doc version lint ==="
stale=false

if grep -rn 'odcs = "0\.7"' docs/user README.md 2>/dev/null; then
  echo "error: stale odcs = \"0.7\" pin found in user docs"
  stale=true
fi

if grep -rn 'version = "0\.7"' docs/user README.md 2>/dev/null; then
  echo "error: stale version = \"0.7\" pin found in user docs"
  stale=true
fi

if [[ "$stale" == true ]]; then
  exit 1
fi

echo "Doc version lint passed."

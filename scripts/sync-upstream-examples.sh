#!/usr/bin/env bash
# Sync curated upstream ODCS examples into tests/fixtures/upstream/.
#
# Usage: ./scripts/sync-upstream-examples.sh [upstream-commit-sha]
#
# Examples are copied from https://github.com/bitol-io/open-data-contract-standard.
# Only apiVersion v3.1.0 examples are included. Upstream document revision
# values (e.g. version: 1.0.0) are preserved as-is.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/tests/fixtures/upstream"
UPSTREAM_REPO="https://raw.githubusercontent.com/bitol-io/open-data-contract-standard"
REF="${1:-main}"

mkdir -p "$DEST"

EXAMPLES=(
  "docs/examples/quality/column-accuracy.odcs.yaml"
  "docs/examples/quality/column-custom.odcs.yaml"
  "docs/examples/quality/column-validity.odcs.yaml"
)

for path in "${EXAMPLES[@]}"; do
  name="$(basename "$path")"
  curl -fsSL "$UPSTREAM_REPO/$REF/$path" -o "$DEST/$name"
done

cat >"$DEST/SOURCE.txt" <<EOF
Upstream examples synced from bitol-io/open-data-contract-standard @ ${REF}
$(date -u +"%Y-%m-%dT%H:%M:%SZ")

Files:
$(printf '  - %s\n' "${EXAMPLES[@]}")

Document version fields are preserved from upstream (typically version: 1.0.0).
Examples that fail odcs validation due to stricter semantic rules are documented in tests.
EOF

echo "Synced ${#EXAMPLES[@]} examples to $DEST"

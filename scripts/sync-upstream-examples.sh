#!/usr/bin/env bash
# Sync curated upstream ODCS examples into tests/fixtures/upstream/.
#
# Usage: ./scripts/sync-upstream-examples.sh [upstream-commit-sha]
#
# Examples are copied from https://github.com/bitol-io/open-data-contract-standard
# and normalized for odcs 0.4 conformance testing:
#   - version field set to "3.1.0" (upstream examples often use 1.0.0)
#   - only apiVersion v3.1.0 examples are included

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
  # Normalize contract version for odcs 3.1.0-targeted validation.
  sed -i.bak 's/^version: 1\.0\.0/version: "3.1.0"/' "$DEST/$name"
  rm -f "$DEST/$name.bak"
done

cat >"$DEST/SOURCE.txt" <<EOF
Upstream examples synced from bitol-io/open-data-contract-standard @ ${REF}
$(date -u +"%Y-%m-%dT%H:%M:%SZ")

Files:
$(printf '  - %s\n' "${EXAMPLES[@]}")

Normalization: version field set to "3.1.0" for odcs conformance tests.
EOF

echo "Synced ${#EXAMPLES[@]} examples to $DEST"

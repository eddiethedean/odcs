#!/usr/bin/env bash
# Full CI parity checks — see CONTRIBUTING.md
set -euo pipefail

cd "$(dirname "$0")/.."

echo "=== cargo fmt ==="
cargo fmt --all -- --check

echo "=== cargo clippy ==="
cargo clippy --all-targets -- -D warnings

echo "=== cargo doc ==="
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked

echo "=== cargo test ==="
cargo test --locked

echo "=== cargo build ==="
cargo build --locked --all-targets

echo "=== Python tests ==="
if [[ ! -d .venv ]]; then
  python3 -m venv .venv
fi
# shellcheck disable=SC1091
source .venv/bin/activate
python -m pip install -q --upgrade pip maturin pytest
maturin develop --features python --locked
pytest python/tests -v

echo "=== maturin build ==="
maturin build --features python --locked

echo "=== mkdocs ==="
python -m pip install -q -r docs/requirements.txt
mkdocs build --strict

echo "All checks passed."

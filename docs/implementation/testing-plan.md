# Testing Plan

## Automated test suites

| Suite | Location | Covers |
|-------|----------|--------|
| Integration | [`tests/skeleton.rs`](../../tests/skeleton.rs) | Parsing, validation, round-trip, section fixtures |
| CLI | [`tests/cli.rs`](../../tests/cli.rs) | Exit codes, JSON output, all commands |
| JSON Schema conformance | [`tests/json_schema_conformance.rs`](../../tests/json_schema_conformance.rs) | Valid fixtures vs pinned upstream schema |
| Python | [`python/tests/test_pyodcs.py`](../../python/tests/test_pyodcs.py) | API and CLI parity |
| Doc tests | [`src/lib.rs`](../../src/lib.rs) | Public API example |

## Running tests

```bash
# Rust (default features)
cargo test --locked

# Rust formatting and lint
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked

# Python
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --features python --locked
pytest python/tests -v
```

CI runs the above via [`.github/workflows/checks.yml`](../../.github/workflows/checks.yml).

## Coverage by category

| Category | Status |
|----------|--------|
| Parse valid ODCS YAML | Covered (`skeleton.rs`) |
| Parse valid ODCS JSON | Covered |
| Reject malformed YAML/JSON | Covered |
| Reject missing required fields | Covered |
| Validate schema object names | Covered (validation phase) |
| Validate quality rules | Covered (library metrics, rule types) |
| Validate SLA structures | Parse fixtures; limited semantic validation |
| Preserve custom properties | Covered |
| Deterministic diagnostics | Covered |
| CLI success/failure exit codes | Covered (`cli.rs`) |
| JSON output format | Covered |
| JSON Schema conformance | Covered (`json_schema_conformance.rs`) |

## Fixtures

Valid and invalid fixtures live in [`tests/fixtures/`](../../tests/fixtures/). Key examples are also copied to [`examples/`](../../examples/) for user documentation.

Add new fixtures based on official ODCS examples when extending conformance coverage.

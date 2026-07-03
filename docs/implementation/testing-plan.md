# Testing Plan

## Automated test suites

| Suite | Location | Covers |
|-------|----------|--------|
| Integration | [`tests/skeleton.rs`](../../tests/skeleton.rs) | Parsing, validation, round-trip, section fixtures |
| CLI | [`tests/cli.rs`](../../tests/cli.rs) | Exit codes, JSON output, all commands |
| JSON Schema conformance | [`tests/json_schema_conformance.rs`](../../tests/json_schema_conformance.rs) | Valid/invalid fixtures vs `schema/odcs-v3.1.0.json`; upstream corpus |
| Shared fixtures | [`tests/common/mod.rs`](../../tests/common/mod.rs) | Canonical valid fixture list and JSON Schema helpers |
| Cross-file / contract set | [`tests/cross_file.rs`](../../tests/cross_file.rs) | `--dep`, `--include`, duplicate contract id rejection |
| Compatibility | [`tests/compatibility.rs`](../../tests/compatibility.rs) | Contract diff and breaking-change classification |
| Validation negative | [`tests/validation_negative.rs`](../../tests/validation_negative.rs) | Parser hardening, duplicate keys, server typos, dedup |
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

MkDocs site (Read the Docs): `pip install -r docs/requirements.txt && mkdocs build --strict`

## Coverage by category

| Category | Status |
|----------|--------|
| Parse valid ODCS YAML | Covered (`skeleton.rs`) |
| Parse valid ODCS JSON | Covered |
| Reject malformed YAML/JSON | Covered |
| Reject missing required fields | Covered |
| Validate schema object names | Covered (validation phase) |
| Validate quality rules | Covered (library metrics, rule types) |
| Validate SLA structures | Parse fixtures; cross-field SLA element refs in structural phase; scheduler/schedule pairing in sections phase |
| Section semantics (0.8.0) | Roles, support, SLA, pricing negative fixtures in `validation_negative.rs` |
| Preserve custom properties | Covered |
| Structural cross-field rules | Covered (`validation_negative.rs`, structural fixtures) |
| CLI success/failure exit codes | Covered (`cli.rs`) |
| JSON output format | Covered |
| JSON Schema conformance | Covered (`json_schema_conformance.rs`, default validation since 0.4.0) |
| Spec parity (default vs JSON Schema) | Covered (`json_schema_conformance.rs` via `tests/common`) |
| Schema export | Covered (`cli.rs`) |
| Deprecated `--strict` alias | Covered (`cli.rs`, `validation_negative.rs`, Python tests) |
| Nested duplicate-key detection | Covered (`validation_negative.rs`, `cli.rs`, Python tests) |
| JSON Schema dedup vs Rust validators | Covered (`validation_negative.rs`) |

## Fixtures

Valid and invalid fixtures live in [`tests/fixtures/`](../../tests/fixtures/). Upstream examples are in [`tests/fixtures/upstream/`](../../tests/fixtures/upstream/) (sync via [`scripts/sync-upstream-examples.sh`](../../scripts/sync-upstream-examples.sh)).

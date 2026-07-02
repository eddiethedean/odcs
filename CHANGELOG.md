# Changelog

## 0.2.0

First published release.

- Phase 1 skeleton milestone (module stubs, fixtures, CLI tests, exit codes)
- Rust crate `odcs` and Python package `pyodcs`
- CI and release workflows

## 0.1.0

Phase 1 — Skeleton (complete).

- Professional repository layout aligned with DTCS conventions
- Rust crate skeleton with full module tree per `crate-layout.md`
- Basic YAML and JSON parsing for minimal ODCS contracts
- Extension field preservation on parse
- CLI: `validate`, `inspect`, `diagnostics`, `schema`, `version` with `--json`
- CLI exit codes: 0 valid, 1 validation error, 2 parse/IO failure
- Expanded fixture suite and integration tests (`tests/skeleton.rs`, `tests/cli.rs`)
- `pyodcs` Python package with PyO3 bindings
- CI pipeline (fmt, clippy, test) plus release workflow
- Documentation reorganized under `docs/implementation/`

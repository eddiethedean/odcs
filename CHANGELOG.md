# Changelog

## 0.3.0

Phase 2 and Phase 3 milestone — full v3.1.0 Canonical Object Model and hardened parsing.

**Breaking changes:**

- Root-level `quality` removed; quality checks belong under `schema[]`
- Root `extensions` flatten removed; use `customProperties` arrays
- Required root fields: `version`, `apiVersion`, `kind`, `id`, `status`
- Unknown root and nested fields are rejected at parse time (`deny_unknown_fields`)

**Added:**

- Typed COM for schema, quality, SLA, servers, team, roles, pricing, support
- Parse diagnostics with object references and `odcs:unknown-field`
- Section fixtures and YAML/JSON round-trip tests
- Upstream `odcs-json-schema-v3.1.0.json` pinned in `tests/fixtures/`

### Fixed

- Relationship `type` field serde mapping and array `items` on schema properties
- `DataQuality.implementation` accepts object values for custom rules
- Nested `deny_unknown_fields` on document model types
- Phase-based validation pipeline with required-field, quality, reference, and extension checks
- `ParseResult::into_contract()` now rejects validation-invalid contracts
- Python CLI exit codes, inspect JSON parity, and `schema` subcommand
- JSON Schema conformance tests for valid fixtures
- Documentation aligned with the `0.3.0` API and milestone status

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

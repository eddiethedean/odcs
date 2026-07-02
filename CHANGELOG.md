# Changelog

## 0.4.0

Validation maturity and JSON Schema parity release.

**Added:**

- `--strict` validation mode runs the Rust pipeline plus pinned ODCS v3.1.0 JSON Schema checks
- `ValidationOptions`, `validate_with_options()`, and `validate_strict()` library APIs
- `odcs schema` exports the pinned JSON Schema (default stdout; `--json` metadata; `--url-only` URL)
- `odcs:json-schema-violation` diagnostic code for strict-mode schema errors
- Pinned schema asset at [`schema/odcs-v3.1.0.json`](schema/odcs-v3.1.0.json)
- Upstream example corpus under `tests/fixtures/upstream/` with [`scripts/sync-upstream-examples.sh`](scripts/sync-upstream-examples.sh)
- Broader JSON Schema negative parity tests and reference validation fixtures
- Python `validate(strict=True)`, `parse_and_validate(strict=True)`, `pinned_schema()`, and CLI parity

**Changed:**

- Default `validate()` behavior is unchanged (non-strict); strict checks are opt-in
- `odcs schema` default output is full JSON Schema JSON (use `--url-only` for the previous URL-only behavior)

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
- Quality validation bypasses (omitted `type`, deprecated `rule`, empty strings, unknown types, invalid ranges)
- `quality_rules()` traversal into array `items` schemas
- Reference validation (empty composite members, format patterns, length parity, dangling refs)
- Server validation for missing canonical fields and misplaced `details` keys
- Extension validation across all `customProperties` sites with duplicate-key detection
- Schema cross-field rules (`array` requires `items`, `INVALID_SCHEMA` diagnostics, primary key positions)
- Section validators for team, roles, support, SLA, and authoritative definitions
- `StableId` pattern validation
- Parser hardening: `parse_strict`, file size limits, duplicate-key detection, `serde_path_to_error`
- Versioning fixes: narrowed `SUPPORTED_API_VERSIONS`, exact `apiVersion` coupling, `SlaValue` integer precedence
- Python `validate_result` shape handling and idempotency, proper I/O exceptions, `BrokenPipeError` handling
- Rust CLI JSON output error propagation

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
- Extension field preservation on parse (superseded in 0.3.0 by `deny_unknown_fields`; see [docs/user/migration-0.3.md](docs/user/migration-0.3.md))
- CLI: `validate`, `inspect`, `diagnostics`, `schema`, `version` with `--json`
- CLI exit codes: 0 valid, 1 validation error, 2 parse/IO failure
- Expanded fixture suite and integration tests (`tests/skeleton.rs`, `tests/cli.rs`)
- `pyodcs` Python package with PyO3 bindings
- CI pipeline (fmt, clippy, test) plus release workflow
- Documentation reorganized under `docs/implementation/`

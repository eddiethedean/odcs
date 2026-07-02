# Changelog

## 0.4.0

Spec parity and validation maturity release — default validation is schema-complete for ODCS v3.1.0.

**Breaking changes:**

- `validate()` always runs JSON Schema validation (not opt-in)
- `version` is now the contract revision (e.g. `1.0.0`); only `apiVersion` gates spec support (`v3.1.0`)
- Stricter default validation: quality dimensions, logical types, server types, library comparison operators
- Removed `primaryKeyPosition >= 0` rule that rejected schema-valid upstream examples

**Added:**

- `--strict` flag and `validate(strict=True)` retained as deprecated no-op aliases (JSON Schema always runs)
- `ValidationOptions`, `validate_with_options()`, and `validate_strict()` library APIs
- `odcs schema` exports the pinned JSON Schema (default stdout; `--json` metadata; `--url-only` URL)
- `odcs:json-schema-violation` diagnostic code for JSON Schema errors
- Pinned schema asset at [`schema/odcs-v3.1.0.json`](schema/odcs-v3.1.0.json)
- Upstream example corpus under `tests/fixtures/upstream/` with [`scripts/sync-upstream-examples.sh`](scripts/sync-upstream-examples.sh)
- SLA fields `description` and `scheduler` on `ServiceLevelAgreementProperty`
- Rust validators for `logicalType`, `quality.dimension`, server `type` enum, and type-specific required fields (Snowflake, Kafka, Postgres)
- Library quality rules require a comparison operator (`mustBe`, `mustBeBetween`, etc.)
- Relationship `type` enum validation (`foreignKey`)
- `AuthoritativeDefinition.type` non-empty validation
- Broader JSON Schema negative parity tests and reference validation fixtures
- Section fixtures: tenant, tags, domain, description, dataProduct, contractCreatedTs, authoritativeDefinitions, property relationships, Kafka/Postgres servers
- `tests/spec_parity.rs` harness; upstream examples preserved without version rewriting
- Python `validate(strict=True)`, `parse_and_validate(strict=True)`, `pinned_schema()`, and CLI parity
- Python `CODES` diagnostic constants export
- Read the Docs site via MkDocs ([`mkdocs.yml`](mkdocs.yml), [`.readthedocs.yaml`](.readthedocs.yaml))

**Changed:**

- `validate_strict()` is an alias for `validate()`
- Upstream sync script no longer rewrites `version: 1.0.0` to `3.1.0`
- `odcs schema` default output is full JSON Schema JSON (use `--url-only` for URL-only output)

## 0.3.0

ODCS v3.1.0 canonical object model and parsing release.

**Breaking changes:**

- Full ODCS v3.1.0 canonical object model replaces minimal skeleton types
- Unknown fields rejected at root and nested objects (`deny_unknown_fields`, `odcs:unknown-field`)
- Quality rules must be nested under `schema[]` (root-level `quality` no longer supported)
- Library quality metrics use v3.1.0 names (`nullValues`, not `not_null`)

**Added:**

- YAML and JSON parsing with structured parse diagnostics
- Section modules: SLA, servers, team, roles, pricing, support, relationships
- Parse helpers: `ParseResult`, `into_contract()`, `parse_strict()`
- Expanded examples and integration test fixtures
- Pinned upstream JSON Schema reference fixture under `tests/fixtures/`

**Changed:**

- `version` field semantics aligned with upstream (contract revision, not spec version)

## 0.2.0

Early validation and Python bindings release.

**Added:**

- CI and release workflows (crates.io, PyPI)
- PyO3 bindings and `pyodcs` package
- CLI commands: `validate`, `inspect`, `diagnostics`, `schema`, `version`
- Standardized exit codes (0 valid, 1 validation, 2 parse/IO)
- Basic YAML and JSON parsing for minimal contracts

## 0.1.0

Initial skeleton release.

**Added:**

- Rust crate layout and module skeleton
- CLI entry point
- Examples and test fixture directories
- Apache 2.0 license

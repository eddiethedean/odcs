# Changelog

## Unreleased — 1.0.0 prep

**Breaking (1.0.0):**

- Removed deprecated `--strict` CLI flag (Rust and Python)
- Removed `ValidationOptions`, `validate_with_options()`, and `validate_strict()` from Rust API
- Removed Python `strict=` parameters from validate helpers
- Internal modules (`parser`, `validation`, `model`, `registry`, `schema`, `diagnostics`) marked `#[doc(hidden)]` — use root re-exports only

**Added:**

- [API stability policy](docs/implementation/api-stability.md)
- Registry path confinement: reject indexed paths whose canonical form escapes the registry root
- Documented YAML alias/depth limits in [SECURITY.md](SECURITY.md)
- Production hardening: capped `parse_file` reads, registry scan limits (10k files, 16 MiB index), symlink cycle detection, atomic index writes, `ODCS_VERBOSE` index progress

**Changed:**

- Documentation sync for 0.9/1.0 reality (architecture, crate layout, enterprise evaluation, migration guide)
- CLI `diff` / `registry` no longer panic on broken pipe (exit 2 instead)
- Cross-file `--dep` / registry paths fail loudly when `canonicalize()` fails (no silent skip)

## 0.9.0 — 2026-07-02

Local contract registry release — index, lookup, and `--registry` cross-file validation.

**Added:**

- Local registry: recursive directory index to `<dir>/.odcs/registry.json` with SHA-256 content hashes
- `Registry`, `RegistryEntry`, `index_registry`, `index_and_save_registry`, `load_registry`
- `ContractSet::from_paths_with_registry` and `parse_and_validate_set_with_registry`
- CLI: `odcs registry index|lookup|list` subcommands
- CLI: `odcs validate --registry <dir>` for registry-backed FQN resolution
- Python: `registry_index`, `registry_index_and_save`, `registry_load`, `registry_lookup`, `registry_list`
- Python: `parse_and_validate_paths(..., registry=...)` parameter
- ADR: [docs/implementation/registry.md](docs/implementation/registry.md)
- Fixtures and tests under `tests/fixtures/registry/` and `tests/registry.rs`

**Changed:**

- Cross-file load order extended: primary → `--dep` → `--registry` → `--include`
- `SPEC.md` documents local registry policy (0.9.0)

## 0.8.0 — 2026-07-02

Section semantics, cross-file references, and compatibility analysis release.

**Added:**

- Section semantics validation: unique `roles[].id`, support URL requirements, SLA scheduler/schedule pairing, pricing currency and amount rules
- Cross-file FQN resolution via `ContractSet`, indexed by contract `id`
- CLI: `odcs validate --dep` and `--include` for multi-document validation
- CLI: `odcs diff <old> <new>` with breaking-change classification
- Library: `parse_and_validate_set`, `validate_set`, `load_set`, `ContractSet`
- Library: `compatibility::diff` with `CompatibilityReport`
- Python: `parse_and_validate_paths()` and `diff()`
- Diagnostic codes: `odcs:compatibility-breaking`, `odcs:compatibility-additive`, `odcs:compatibility-deprecated`, `odcs:compatibility-unchanged`
- ADR: [docs/implementation/cross-file-references.md](docs/implementation/cross-file-references.md)
- Fixtures and tests for section semantics, cross-file, and compatibility workflows

**Changed:**

- Pinned JSON Schema `FullyQualifiedReference` pattern accepts canonical 3-segment FQN form with hyphens in contract ids
- `SPEC.md` documents section semantics, cross-file references, and compatibility analysis (0.8.0)

## 0.7.0 — 2026-07-02

Structural validation release — cross-field rules for schema, server, and SLA references.

**Added:**

- Structural validation phase: unique `schema[].name` and `servers[].server` values
- `slaProperties[].element` and `slaDefaultElement` must reference existing schema object names (comma-separated tokens supported)
- Negative fixtures and tests for each structural rule
- SPEC.md structural validation (0.7.0) policy section

**Fixed:**

- `slaDefaultElement` now uses the same comma-split and trim semantics as `slaProperties[].element`
- Empty contract `id` no longer emits a duplicate `odcs:invalid-extension` diagnostic from the IDs phase
- Quality rules without an inferrable type now emit `odcs:invalid-quality` from Rust validation
- Python CLI text output includes `phase:` when `validationPhase` is set (Rust parity)

**Changed:**

- `with-sla-default-element.yaml` fixture uses `slaDefaultElement: "customers"` to match schema object name semantics
- ROADMAP rebaselined: Phases 12–14 target `0.8.0`; Phase 15 targets `0.9.0`

## 0.6.0 — 2026-07-02

Diagnostics metadata release — validation pipeline phase on every validation diagnostic.

**Added:**

- Optional `validationPhase` field on validation-stage diagnostics (JSON camelCase; omitted for parse-stage diagnostics)
- `ValidationPhase` enum with variants matching the validation pipeline (`document`, `schema`, `quality`, `jsonSchema`, …)
- CLI text output includes `phase:` line when `validationPhase` is set
- Python `VALIDATION_PHASES` constants and `validation_phases()` native helper
- Fixture-wide tests asserting `validationPhase` coverage on validation diagnostics

**Changed:**

- `validation_error` builder now requires a `ValidationPhase` argument (compile-time enforcement)
- `ValidationPhase` moved to `src/diagnostics/validation_phase.rs` (re-exported from `validation::phases`)

## 0.5.0 — 2026-07-02

Parser hardening release — nested duplicate-key detection for YAML and JSON.

**Added:**

- Nested YAML duplicate-key detection via `unsafe-libyaml` event walk before `serde_yaml` deserialization
- `DuplicateKeyFinding` with path-aware `object_ref` (e.g. `schema[0].name`) for JSON and YAML parse errors
- Explicit `unsafe-libyaml = "0.2.11"` dependency
- Fixtures and tests: `invalid-nested-duplicate-key.yaml` / `.json`; CLI exit code `2` for duplicate-key parse failures

**Changed:**

- `odcs:duplicate-key` diagnostics now use dotted paths for nested duplicates (root keys unchanged, e.g. `id`)
- JSON duplicate-key detection reports path-aware `object_ref` (aligned with YAML)
- YAML duplicate-key scanner fails closed on libyaml scan errors
- Unknown nested fields report full dotted `object_ref` paths
- Server property typos in flattened details emit `odcs:unknown-field`
- JSON Schema diagnostics deduplicated when Rust validators report the same field
- Python `is_valid()` accepts parse result dicts (reads `report.diagnostics`)
- CLI `--strict` help text corrected (deprecated no-op)

**Documentation:**

- Adoption-focused docs: What is ODCS primer, API decision guide, troubleshooting, fix-invalid-contract tutorial, glossary, enterprise evaluation brief
- Release status page; MkDocs nav restructure (removed misleading `upstream/` paths)
- README sharpened; CI examples pin published version; diagnostic examples corrected for `apiVersion` and dotted `object_ref` paths
- Expanded architecture guide for contributors

## 0.4.0 — 2026-07-02

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
- User guides: Rust API, contract authoring, migration, CI/CD integration
- [`SECURITY.md`](SECURITY.md) vulnerability reporting policy

**Changed:**

- `validate_strict()` is an alias for `validate()`
- Upstream sync script no longer rewrites `version: 1.0.0` to `3.1.0`
- `odcs schema` default output is full JSON Schema JSON (use `--url-only` for URL-only output)
- Minimal examples use `version: 1.0.0` with `apiVersion: v3.1.0` (contract revision vs spec version)
- PyPI `pyodcs` development status classifier: Pre-Alpha → Alpha
- Quick start and README examples work without a repository clone

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

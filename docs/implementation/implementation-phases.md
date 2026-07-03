# Implementation Phases

All reference-implementation phases through **Phase 15 (local registry)** are **complete** as of `0.9.0`. See [ROADMAP.md](../../ROADMAP.md) and [SPEC.md](../../SPEC.md) spec parity policy.

## Phase 1 — Skeleton

**Status:** Complete (`0.1.0`).

- Create Rust crate.
- Add `src` module layout (including stubs for future modules).
- Add CLI entry point.
- Add examples and tests folders.

## Phase 2 — Canonical Object Model

**Status:** Complete (`0.3.0`).

Model ODCS sections:

- fundamentals
- schema
- quality
- SLA
- stakeholders
- team
- roles
- servers
- pricing
- custom properties

## Phase 3 — Parsing

**Status:** Complete (`0.3.0`).

- Parse YAML.
- Parse JSON.
- Reject unknown fields at root and nested document objects.
- Return structured errors and diagnostics.

## Phase 4 — Diagnostics

**Status:** Complete (`0.4.0`).

Mirror DTCS diagnostic style:

- identifier
- severity
- category
- stage
- message
- object reference
- remediation
- stable `odcs:` codes including JSON Schema violations (`odcs:json-schema-violation`)

## Phase 5 — Validation

**Status:** Complete (`0.4.0`). Default validation includes JSON Schema conformance.

Implemented validation phases:

1. Document validation
2. Structural validation
3. Schema validation
4. Quality validation
5. Reference validation
6. Extension validation
7. JSON Schema validation (default since 0.4.0)

## Phase 6 — CLI

**Status:** Complete (`0.4.0`).

Commands:

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
odcs schema
odcs version
```

Since 0.4.0, JSON Schema validation always runs in default `validate()`. `--strict` is a deprecated no-op alias. `odcs schema` exports the pinned schema.

## Phase 7 — JSON Schema parity

**Status:** Complete (`0.4.0`). JSON Schema validation runs in default `validate()`.

Conformance tests in [`tests/json_schema_conformance.rs`](../../tests/json_schema_conformance.rs) validate fixtures against `schema/odcs-v3.1.0.json`. Upstream examples are synced via [`scripts/sync-upstream-examples.sh`](../../scripts/sync-upstream-examples.sh).

## Phase 8 — Python bindings

**Status:** Complete (`0.4.0`).

PyO3 bindings and the `pyodcs` package expose parse, validate, inspect, schema export, and CLI helpers with parity to the Rust `odcs` CLI.

## Phase 9 — Parser hardening

**Status:** Complete (`0.5.0`).

- Nested YAML duplicate-key detection via `unsafe-libyaml` event walk
- Path-aware `object_ref` for nested JSON and YAML duplicate keys
- Fail-closed behavior on libyaml scan errors

## Phase 10 — Diagnostics metadata

**Status:** Complete (`0.6.0`).

- `validationPhase` on every validation-stage diagnostic (JSON + CLI text)
- `ValidationPhase` enum aligned with validator modules
- Compile-time phase wiring in `validation_error` builder

## Phase 11 — Structural validation

**Status:** Complete (`0.7.0`).

Cross-field rules in [`structural.rs`](../../src/validation/structural.rs):

- Unique non-empty `schema[].name` and `servers[].server` values
- `slaProperties[].element` and `slaDefaultElement` must reference existing schema object names (comma-separated tokens supported)
- Documented in [SPEC.md](../../SPEC.md) structural validation (0.7.0) policy

## Phase 12 — Section semantics

**Status:** Complete (`0.8.0`).

Business rules in [`sections.rs`](../../src/validation/sections.rs): unique `roles[].id`, support URL requirements, SLA scheduler/schedule pairing, pricing currency and amount rules.

## Phase 13 — Cross-file references

**Status:** Complete (`0.8.0`).

Multi-document FQN resolution via `ContractSet`, CLI `--dep` / `--include`, and library `load_set`.

## Phase 14 — Compatibility analysis

**Status:** Complete (`0.8.0`).

`compatibility::diff`, CLI `odcs diff`, and Python `pyodcs.diff()`.

## Phase 15 — Local registry

**Status:** Complete (`0.9.0`).

Recursive directory index to `.odcs/registry.json`, lookup APIs, CLI `odcs registry` subcommands, `validate --registry`, and Python registry bindings. See [registry.md](registry.md).

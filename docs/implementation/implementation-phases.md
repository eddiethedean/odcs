# Implementation Phases

## Phase 1 — Skeleton

**Status:** Complete (`0.1.0`).

- Create Rust crate.
- Add `src` module layout (including stubs for Phase 2+ modules).
- Add CLI entry point.
- Add examples and tests folders.

Phase 6 CLI polish (`--strict`, schema export) continues in later milestones.

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

**Status:** Largely complete (`0.3.0`).

Mirror DTCS diagnostic style:

- identifier
- severity
- category
- stage
- message
- object reference
- remediation

## Phase 5 — Validation

**Status:** In progress (`0.3.0`).

Implemented validation phases:

1. Document validation
2. Structural validation
3. Schema validation
4. Quality validation
5. Reference validation
6. Extension validation

Remaining: deeper semantic checks, `--strict` mode, and additional conformance coverage.

## Phase 6 — CLI

**Status:** Largely complete (`0.3.0`).

Commands:

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
odcs schema
odcs version
```

`pyodcs` provides a matching Python CLI.

## Phase 7 — JSON Schema parity

**Status:** Baseline started (`0.3.0`).

Conformance tests in [`tests/json_schema_conformance.rs`](../../tests/json_schema_conformance.rs) validate pinned fixtures against `tests/fixtures/odcs-json-schema-v3.1.0.json`.

## Phase 8 — Python bindings

**Status:** Largely complete (`0.3.0`).

PyO3 bindings and the `pyodcs` package expose parse, validate, inspect, and CLI helpers.

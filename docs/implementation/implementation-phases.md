# Implementation Phases

## Phase 1 — Skeleton

**Status:** Complete (`0.1.0`).

- Create Rust crate.
- Add `src` module layout (including stubs for Phase 2+ modules).
- Add CLI entry point.
- Add examples and tests folders.

Phase 6 CLI polish (`--strict`, schema export) continues in later milestones.

## Phase 2 — Canonical Object Model

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

- Parse YAML.
- Parse JSON.
- Preserve unknown extension fields.
- Return structured errors and diagnostics.

## Phase 4 — Diagnostics

Mirror DTCS diagnostic style:

- identifier
- severity
- category
- stage
- message
- object reference
- remediation

## Phase 5 — Validation

Implement validation phases:

1. Document validation
2. Canonical Object Model validation
3. Structural validation
4. Schema validation
5. Quality validation
6. Reference validation
7. Extension validation

## Phase 6 — CLI

Commands:

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
odcs schema
odcs version
```

## Phase 7 — JSON Schema Parity

Compare behavior against official ODCS JSON Schema examples.

## Phase 8 — Python Bindings

Add PyO3 bindings after Rust API stabilizes.

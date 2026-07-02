# Implementation Phases

## Phase 1 — Skeleton

- Create Rust crate.
- Add `src` module layout.
- Add CLI entry point.
- Add examples and tests folders.

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

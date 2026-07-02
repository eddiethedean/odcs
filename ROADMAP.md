# ODCS Roadmap

Reference-implementation milestones for the Open Data Contract Standard. This roadmap tracks the Rust crate in [`src/`](src/).

The [upstream ODCS specification](https://github.com/bitol-io/open-data-contract-standard) is the source of truth for semantics. When this roadmap and the upstream specification disagree, the upstream specification wins.

---

## Status overview

| Phase | Name | Focus | Status |
|-------|------|-------|--------|
| **1** | [Skeleton](#phase-1--skeleton) | Crate layout, CLI entry point, examples, tests | **Complete** (`0.1.0`) |
| **2** | [Canonical Object Model](#phase-2--canonical-object-model) | ODCS sections as Rust types | Planned |
| **3** | [Parsing](#phase-3--parsing) | YAML and JSON parsing with extension preservation | Planned |
| **4** | [Diagnostics](#phase-4--diagnostics) | Structured diagnostics aligned with DTCS style | Planned |
| **5** | [Validation](#phase-5--validation) | Phase-based validation pipeline | Planned |
| **6** | [CLI](#phase-6--cli) | `validate`, `inspect`, `diagnostics`, `schema`, `version` | Planned |
| **7** | [JSON Schema parity](#phase-7--json-schema-parity) | Conformance against official ODCS JSON Schema | Planned |
| **8** | [Python bindings](#phase-8--python-bindings) | PyO3 bindings after Rust API stabilizes | Planned |

## Dependencies

```text
Phase 1  Skeleton
             │
             ├──► Phase 2  Canonical Object Model
             │         │
             │         └──► Phase 3  Parsing
             │                    │
             │                    └──► Phase 4  Diagnostics
             │                               │
             │                               └──► Phase 5  Validation
             │                                          │
             │                                          ├──► Phase 6  CLI
             │                                          │
             │                                          └──► Phase 7  JSON Schema parity
             │                                                     │
             │                                                     └──► Phase 8  Python bindings
```

---

## Phase 1 — Skeleton

**Target:** `0.1.0` — **Complete**

- [x] Repository layout aligned with DTCS conventions
- [x] Rust crate with full module skeleton per `crate-layout.md`
- [x] CLI entry point with `validate`, `inspect`, `diagnostics`, `schema`, and `version`
- [x] Basic YAML and JSON parsing for minimal contracts
- [x] Examples and expanded test fixtures (valid, invalid, malformed, extensions)
- [x] Integration and CLI test coverage
- [x] CLI exit codes aligned with `cli-spec.md` (0 valid, 1 validation, 2 parse/IO)
- [x] CI pipeline (fmt, clippy, test)

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

- Parse YAML and JSON
- Preserve unknown extension fields
- Return structured errors and diagnostics

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

## Phase 7 — JSON Schema parity

Compare behavior against official ODCS JSON Schema examples and conformance fixtures.

## Phase 8 — Python bindings

Add PyO3 bindings after the Rust API stabilizes.

# ODCS Roadmap

Reference-implementation milestones for the Open Data Contract Standard. This roadmap tracks the Rust crate in [`src/`](src/).

The [upstream ODCS specification](https://github.com/bitol-io/open-data-contract-standard) is the source of truth for semantics. When this roadmap and the upstream specification disagree, the upstream specification wins.

---

## Status overview

| Phase | Name | Focus | Status |
|-------|------|-------|--------|
| **1** | [Skeleton](#phase-1--skeleton) | Crate layout, CLI entry point, examples, tests | **Complete** (`0.1.0`) |
| **2** | [Canonical Object Model](#phase-2--canonical-object-model) | ODCS sections as Rust types | **Complete** (`0.3.0`) |
| **3** | [Parsing](#phase-3--parsing) | YAML and JSON parsing with diagnostics | **Complete** (`0.3.0`) |
| **4** | [Diagnostics](#phase-4--diagnostics) | Structured diagnostics aligned with DTCS style | **Largely complete** (`0.3.0`) |
| **5** | [Validation](#phase-5--validation) | Phase-based validation pipeline | **In progress** (`0.3.0`) |
| **6** | [CLI](#phase-6--cli) | `validate`, `inspect`, `diagnostics`, `schema`, `version` | **Largely complete** (`0.3.0`) |
| **7** | [JSON Schema parity](#phase-7--json-schema-parity) | Conformance against official ODCS JSON Schema | **Baseline started** (`0.3.0`) |
| **8** | [Python bindings](#phase-8--python-bindings) | PyO3 bindings after Rust API stabilizes | **Largely complete** (`0.3.0`) |

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

**Target:** `0.3.0` — **Complete**

- [x] Shared types (`StableId`, `Tags`, `CustomProperty`, `AuthoritativeDefinitions`, `ContractDescription`)
- [x] Root `DataContract` with v3.1.0 required fields
- [x] `SchemaObject` / `SchemaProperty` with nested quality
- [x] Section modules: SLA, servers, team (object + legacy array), roles, pricing, support
- [x] `stakeholders` documented as N/A for v3.1.0

## Phase 3 — Parsing

**Target:** `0.3.0` — **Complete**

- [x] YAML and JSON parsing via serde
- [x] Parse helpers (`success` / `failure_from_serde`)
- [x] Parse diagnostics with paths and unknown-field detection
- [x] Fixture migration and round-trip tests
- [x] Upstream JSON Schema reference fixture pinned under `tests/fixtures/`

## Phase 4 — Diagnostics

**Target:** `0.3.0` — **Largely complete**

- [x] Structured `Diagnostic` records with id, severity, category, stage, message
- [x] `object_ref` and `remediation` support
- [x] Stable `odcs:` diagnostic codes
- [x] CLI text and JSON output

## Phase 5 — Validation

**Target:** `0.3.0` — **In progress**

- [x] Document validation (required root fields, version checks)
- [x] Structural validation (version/apiVersion consistency)
- [x] Schema validation (required schema/property names)
- [x] Quality validation (library metrics, rule-type constraints)
- [x] Reference validation (relationship endpoints)
- [x] Extension validation (custom property keys)
- [ ] `--strict` mode semantics
- [ ] Deeper reference resolution

## Phase 6 — CLI

**Target:** `0.3.0` — **Largely complete**

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
odcs schema
odcs version
```

- [x] Rust CLI with exit codes per `cli-spec.md`
- [x] Python `pyodcs` CLI parity
- [ ] Full `--strict` enforcement
- [ ] JSON Schema export from `odcs schema`

## Phase 7 — JSON Schema parity

**Target:** `0.3.0` — **Baseline started**

- [x] Pinned upstream schema fixture
- [x] Conformance tests for valid section fixtures
- [ ] Broader negative-case parity
- [ ] Example corpus from upstream repository

## Phase 8 — Python bindings

**Target:** `0.3.0` — **Largely complete**

- [x] PyO3 bindings via maturin (`pyodcs._native`)
- [x] Parse, validate, inspect helpers
- [x] Python CLI

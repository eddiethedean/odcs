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
| **4** | [Diagnostics](#phase-4--diagnostics) | Structured diagnostics aligned with DTCS style | **Complete** (`0.4.0`) |
| **5** | [Validation](#phase-5--validation) | Phase-based validation pipeline | **Complete** (`0.4.0`) |
| **6** | [CLI](#phase-6--cli) | `validate`, `inspect`, `diagnostics`, `schema`, `version` | **Complete** (`0.4.0`) |
| **7** | [JSON Schema parity](#phase-7--json-schema-parity) | Conformance against official ODCS JSON Schema | **Complete** (`0.4.0`) |
| **8** | [Python bindings](#phase-8--python-bindings) | PyO3 bindings after Rust API stabilizes | **Complete** (`0.4.0`) |

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
- [x] Upstream JSON Schema reference fixture pinned under `schema/` and `tests/fixtures/`

## Phase 4 — Diagnostics

**Target:** `0.4.0` — **Complete**

- [x] Structured `Diagnostic` records with id, severity, category, stage, message
- [x] `object_ref` and `remediation` support
- [x] Stable `odcs:` diagnostic codes (including `odcs:json-schema-violation` for strict mode)
- [x] CLI text and JSON output

## Phase 5 — Validation

**Target:** `0.4.0` — **Complete**

- [x] Document validation (required root fields, version checks)
- [x] Structural validation (version/apiVersion consistency)
- [x] Schema validation (required schema/property names)
- [x] Quality validation (library metrics, rule-type constraints)
- [x] Reference validation (relationship endpoints)
- [x] Extension validation (custom property keys)
- [x] `--strict` mode semantics (JSON Schema validation phase)
- [x] Deeper reference resolution (schema-level `from`, nested property shorthand)

## Phase 6 — CLI

**Target:** `0.4.0` — **Complete**

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
odcs schema
odcs version
```

- [x] Rust CLI with exit codes per `cli-spec.md`
- [x] Python `pyodcs` CLI parity
- [x] Full `--strict` enforcement
- [x] JSON Schema export from `odcs schema`

## Phase 7 — JSON Schema parity

**Target:** `0.4.0` — **Complete**

- [x] Pinned upstream schema fixture (`schema/odcs-v3.1.0.json`)
- [x] Conformance tests for valid section fixtures
- [x] Broader negative-case parity
- [x] Example corpus from upstream repository (`tests/fixtures/upstream/`, `scripts/sync-upstream-examples.sh`)
- [x] Strict-mode JSON Schema validation phase

## Phase 8 — Python bindings

**Target:** `0.4.0` — **Complete**

- [x] PyO3 bindings via maturin (`pyodcs._native`)
- [x] Parse, validate, inspect helpers
- [x] Strict validation (`strict=True`) and `validate_result(strict=True)`
- [x] `pinned_schema()` and schema CLI export
- [x] Python CLI with full parity to Rust `odcs`

---

## Future milestones (0.5+)

Phases 1–8 cover the reference-implementation core. The following are explicitly **out of scope** for 0.4.0 per [docs/implementation/non-goals.md](docs/implementation/non-goals.md) and remain stub modules or backlog items:

| Area | Module | Notes |
|------|--------|-------|
| Registry server | [`src/registry/mod.rs`](src/registry/mod.rs) | Contract registry / discovery |
| Compatibility analysis | [`src/compatibility/mod.rs`](src/compatibility/mod.rs) | Cross-version contract diffing |
| Cross-file reference resolution | validation backlog | Shorthand refs within a single document only |
| Nested YAML duplicate-key detection | parser backlog | Root-level duplicate keys covered in 0.3.0 |
| `ValidationPhase` on diagnostics | diagnostics polish | Phase identifiers defined; not yet attached to every diagnostic |

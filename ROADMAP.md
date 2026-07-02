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
| **9** | [Parser hardening](#phase-9--parser-hardening) | Nested YAML duplicate-key detection | **Complete** (`0.5.0`) |
| **10** | [Diagnostics metadata](#phase-10--diagnostics-metadata) | `validationPhase` on validation diagnostics | **Complete** (`0.6.0`) |
| **11** | [Structural validation](#phase-11--structural-validation) | Cross-field rules in `structural.rs` | **Complete** (`0.7.0`) |
| **12** | [Section semantics](#phase-12--section-semantics) | Roles, SLA, pricing, support validators | Planned (`0.7.0`) |
| **13** | [Cross-file references](#phase-13--cross-file-references) | Multi-document FQN resolution | Planned (`0.7.0`) |
| **14** | [Compatibility analysis](#phase-14--compatibility-analysis) | Contract diff and breaking-change report | Planned (`0.7.0`) |
| **15** | [Registry](#phase-15--registry) | Local contract index and lookup | Planned (`0.8.0`) |
| **16** | [1.0 release](#phase-16--10-release) | API stabilization and upstream sync | Planned (`1.0.0`) |

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
             │                                                                │
             │                    ┌───────────────────────────────────────────┤
             │                    │                                           │
             │                    ▼                                           ▼
             │           Phase 9  Parser hardening              Phase 10  Diagnostics metadata
             │                    │                                           │
             │                    └───────────────────┬───────────────────────┘
             │                                        ▼
             │                              Phase 11  Structural validation
             │                                        │
             │                          ┌─────────────┴─────────────┐
             │                          ▼                           ▼
             │                Phase 12  Section semantics   Phase 13  Cross-file references
             │                          │                           │
             │                          └─────────────┬─────────────┘
             │                                        ▼
             │                              Phase 14  Compatibility analysis
             │                                        │
             │                                        ▼
             │                              Phase 15  Registry
             │                                        │
             │                                        ▼
             │                              Phase 16  1.0 release
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

## Spec parity (0.4.0) — Complete

- [x] Default `validate()` includes JSON Schema conformance
- [x] Upstream `version` / `apiVersion` semantics aligned
- [x] SLA model complete (`description`, `scheduler`)
- [x] Enum and server type validation in default mode
- [x] Expanded section fixture matrix and upstream corpus without normalization
- [x] Spec parity policy documented in [`SPEC.md`](SPEC.md)

---

## Future milestones (0.5+)

Phases 1–9 deliver schema-complete ODCS v3.1.0 document parsing and validation, including nested duplicate-key detection. Phases 10–16 deepen observability, multi-document workflows, and ecosystem tooling on the path to `1.0.0`.

| Release | Phases | Theme |
|---------|--------|-------|
| `0.5.0` | 9 ✓ | Parser hardening (nested duplicate-key detection) |
| `0.6.0` | 10 ✓ | Diagnostics metadata (`validationPhase`) |
| `0.7.0` | 11, 12, 13 | Structural validation, section semantics, cross-file references |
| `0.8.0` | 14 | Contract evolution and compatibility reporting |
| `0.9.0` | 15 | Local registry and discovery |
| `1.0.0` | 16 | Stable public API, deprecation cleanup, upstream alignment |

Out of scope for this repository (see [docs/implementation/non-goals.md](docs/implementation/non-goals.md)): data quality execution, DTCS/DPCS transformation semantics, SQL generation, ETL, and runtime engines.

---

## Phase 9 — Parser hardening

**Target:** `0.5.0` — **Complete**

**Goal:** Detect duplicate keys at any YAML nesting depth before serde deserialization, matching JSON behavior in [`src/parser/duplicate_keys.rs`](src/parser/duplicate_keys.rs).

**Context:** Implemented via `find_yaml_duplicate_key` using an `unsafe-libyaml` event walk (pre-`serde_yaml` deserialize). JSON uses `DupeDetectVisitor` with a path stack. Both return `DuplicateKeyFinding { key, object_ref }` (e.g. `schema[0].name`). Flow-style mappings and YAML anchors/aliases remain out of scope.

**Deliverables:**

- [x] Extend [`src/parser/duplicate_keys.rs`](src/parser/duplicate_keys.rs) with nested YAML duplicate-key detection (`unsafe-libyaml` event walk; path-aware)
- [x] Invoke nested check from [`src/parser/yaml.rs`](src/parser/yaml.rs) before `serde_path_to_error::deserialize`
- [x] Emit `odcs:duplicate-key` via `failure_duplicate_key` with dotted `object_ref` paths (e.g. `schema[0].name`)
- [x] Fixtures: [`tests/fixtures/invalid-nested-duplicate-key.yaml`](tests/fixtures/invalid-nested-duplicate-key.yaml) and [`.json`](tests/fixtures/invalid-nested-duplicate-key.json)
- [x] Tests in [`tests/validation_negative.rs`](tests/validation_negative.rs); CLI exit code `2` in [`tests/cli.rs`](tests/cli.rs)
- [x] Python parse test in [`python/tests/test_pyodcs.py`](python/tests/test_pyodcs.py); explicit `unsafe-libyaml = "0.2.11"` in [`Cargo.toml`](Cargo.toml)

**Out of scope:** Duplicate keys inside YAML flow scalars or anchors/aliases (documented in module).

**Done when:** Nested YAML duplicate keys fail parse with `odcs:duplicate-key` and a non-root `object_ref`; CI green. ✓

---

## Phase 10 — Diagnostics metadata

**Target:** `0.6.0` — **Complete**

**Goal:** Attach the validation pipeline phase to every validation diagnostic so CI and tooling can filter by origin without parsing messages.

**Context:** [`ValidationPhase`](src/validation/phases.rs) exists but [`Diagnostic`](src/diagnostics/diagnostic.rs) only records coarse `stage` (`parse` | `validation` | …). [`validation_error`](src/diagnostics/builders.rs) does not accept a phase.

**Deliverables:**

- [x] Add optional `validation_phase: Option<ValidationPhase>` to `Diagnostic` (serde: `validationPhase`, camelCase)
- [x] Extend `validation_error` (or add `phase_validation_error`) to require `ValidationPhase` for validation-stage diagnostics
- [x] Wire phase through all validators: `document`, `structural`, `schema`, `quality`, `references`, `extensions`, `servers`, `sections`, `ids`, `json_schema`
- [x] Leave parse-stage diagnostics without `validationPhase` (field omitted in JSON)
- [x] CLI text/JSON output includes `validationPhase` when set; update [`docs/user/diagnostics.md`](docs/user/diagnostics.md)
- [x] Export phase name constants in Python diagnostic docs (no separate `CODES` entry — phases are metadata, not error ids)
- [x] Snapshot or assertion tests that every validation diagnostic in fixture runs includes `validationPhase`

**Out of scope:** Repurposing `DiagnosticStage` to encode validation phases; reserved stages (`analysis`, `runtime`, …) stay for future use.

**Done when:** `odcs validate --json` emits `validationPhase` on all validation errors; existing diagnostic `id` values unchanged.

---

## Phase 11 — Structural validation

**Target:** `0.7.0` — **Complete**

**Goal:** Implement cross-field constraints in [`src/validation/structural.rs`](src/validation/structural.rs) that require reading multiple sections of a contract and are not owned by a single-section validator.

**Context:** Root-field checks live in [`document.rs`](src/validation/document.rs); section-specific checks are split across `schema`, `extensions`, `sections`, etc. Phase 11 fills the gap for **inter-section** rules.

**Adopted rules** (confirmed against upstream spec + pinned schema):

- [x] Unique non-empty `schema[].name` values within a contract
- [x] `slaDefaultElement`, when set, references an existing `schema[].name` (element path notation; deprecated field)
- [x] `slaProperties[].element`, when set, references an existing `schema[].name` (comma-separated tokens supported)
- [x] Unique non-empty `servers[].server` values
- [x] ~~`servers[].schema`~~ — **not adopted** (database/catalog schema string in server details, not an ODCS `schema[]` reference)

**Deliverables:**

- [x] Spec audit note in [`SPEC.md`](SPEC.md) listing adopted structural rules and any intentional extensions
- [x] Implement confirmed rules in `structural.rs` using existing `validation_error` + phase metadata (Phase 10)
- [x] Valid/invalid fixtures per rule under `tests/fixtures/`
- [x] Tests in [`tests/validation_negative.rs`](tests/validation_negative.rs)

**Out of scope:** Rules already enforced by JSON Schema or a single-section module (move only if logically cross-field); relationship endpoint resolution (Phase 5 / Phase 13).

**Done when:** `structural.rs` emits diagnostics for all adopted rules; no duplicate enforcement elsewhere.

---

## Phase 12 — Section semantics

**Target:** `0.7.0` — **Planned**

**Goal:** Add Rust-side semantic validation for sections where JSON Schema coverage is thin and [`sections.rs`](src/validation/sections.rs) only checks team usernames today.

**Context:** [`extensions.rs`](src/validation/extensions.rs) validates some support/SLA empty fields and custom properties; [`ids.rs`](src/validation/ids.rs) validates optional stable IDs. Phase 12 adds **business semantics** per section model.

**Deliverables:**

| Section | Module | Rules |
|---------|--------|-------|
| Team | `sections.rs` | *(existing)* non-empty `team.members[].username` |
| Roles | `sections.rs` or `roles.rs` | Non-empty `roles[].role`; unique `roles[].id` when present |
| Support | `sections.rs` | Non-empty `channel` *(existing)*; require `url` when channel is URL-bearing per spec enum |
| SLA | `sections.rs` or `sla.rs` | Non-empty `property` *(existing in extensions)*; validate `scheduler`/`schedule` pairing if spec defines constraints |
| Pricing | `sections.rs` or `pricing.rs` | When `priceAmount` is set, require `priceCurrency`; reject negative amounts if spec disallows |

- [ ] Implement validators; prefer extending `sections.rs` unless a section grows large enough to split
- [ ] Negative fixtures for each new rule
- [ ] Update [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md) SLA row from “limited semantic validation” to covered items
- [ ] All new diagnostics use `validationPhase` and stable existing codes where possible (`missing-required-field`, `invalid-schema`, etc.)

**Out of scope:** Re-validating fields already fully constrained by pinned JSON Schema; quality rule execution.

**Done when:** Each section in the table has at least one semantic rule beyond parse + JSON Schema; tests pass.

---

## Phase 13 — Cross-file references

**Target:** `0.7.0` — **Planned**

**Goal:** Resolve fully-qualified relationship endpoints across a loaded set of contracts; fail unresolved refs with actionable diagnostics.

**Context:** [`references.rs`](src/validation/references.rs) validates shorthand `table.column` against an in-document index and accepts FQN strings via regex without resolving them. [`SPEC.md`](SPEC.md) documents single-document resolution as the 0.4.0 policy.

**Design decisions** (resolve before coding):

- [ ] ADR or `docs/implementation/cross-file-references.md` covering: contract index key (`id` vs filename), FQN grammar (reuse existing regex), and load order
- [ ] `ContractSet` (or equivalent) type: parse + index multiple documents from paths
- [ ] Extend reference validation to resolve FQN endpoints against the set
- [ ] CLI: `odcs validate <path> --include <dir>` or repeated `--dep <path>` (update [`docs/implementation/cli-spec.md`](docs/implementation/cli-spec.md))
- [ ] Library: `validate_set(&ContractSet)` or `parse_and_validate_paths(&[Path])`
- [ ] Python: `parse_and_validate_paths(...)` binding
- [ ] Fixtures: two-contract valid/invalid pairs under `tests/fixtures/cross-file/`

**Out of scope for MVP:** Remote URL fetching, registry-backed resolution (Phase 15), workspace manifests.

**Done when:** A relationship `from`/`to` referencing `other-contract/table.column` validates when `other-contract` is included and fails with `odcs:unresolved-reference` when omitted.

---

## Phase 14 — Compatibility analysis

**Target:** `0.7.0` — **Planned**

**Goal:** Compare two parsed contracts and produce a structured breaking-change report for contract evolution workflows.

**Context:** Stub [`src/compatibility/mod.rs`](src/compatibility/mod.rs). `DiagnosticCategory::Compatibility` already exists but is used only for unsupported `apiVersion`.

**Deliverables:**

- [ ] `CompatibilityReport` with classified changes: `breaking`, `additive`, `deprecated`, `unchanged`
- [ ] Compare dimensions:
  - Root metadata (`id`, `status`, `version` — informational, not breaking by default)
  - Schema objects: added/removed/renamed; property added/removed; `logicalType` change; `required` toggle
  - Quality rules: added/removed; metric or operator change
  - Relationships: added/removed; endpoint change
- [ ] Stable codes: `odcs:compatibility-breaking`, `odcs:compatibility-additive`, … (document in diagnostics guide)
- [ ] CLI: `odcs diff <old> <new>` with text + `--json`; exit `0` if no breaking changes, `1` if breaking
- [ ] Python: `pyodcs.diff(old, new)` returning report dict
- [ ] Fixtures: pairs under `tests/fixtures/compatibility/`

**Out of scope:** Automatic migration or contract rewriting; semver inference for `version` field.

**Done when:** `odcs diff` correctly classifies a fixture pair with known breaking schema removal; tests and CLI spec updated.

---

## Phase 15 — Registry

**Target:** `0.8.0` — **Planned**

**Goal:** Provide a local contract index for discovery and optional integration with Phase 13 cross-file resolution.

**Context:** Stub [`src/registry/mod.rs`](src/registry/mod.rs). Deferred from the first-repo milestone per [non-goals](docs/implementation/non-goals.md).

**Deliverables:**

- [ ] `RegistryEntry` model: `id`, `version`, `path`, optional `tags`, `apiVersion`, content hash
- [ ] Local backend: index file (e.g. `.odcs/registry.json`) + scanned contract directory
- [ ] API: `register`, `lookup(id)`, `lookup(id, version)`, `list`
- [ ] CLI: `odcs registry index <dir>`, `odcs registry lookup <id>` (exact names TBD in cli-spec)
- [ ] Optional: `odcs validate --registry <dir>` loads index for FQN resolution (builds on Phase 13)
- [ ] Python bindings for lookup/list

**Out of scope for MVP:** HTTP remote registry, auth, publish/subscribe, write-through to external systems.

**Done when:** Indexing a directory of contracts enables lookup by `id` and powers cross-file validation without explicit `--include` for indexed paths.

---

## Phase 16 — 1.0 release

**Target:** `1.0.0` — **Planned**

**Goal:** Ship a stable, semver-major API with deprecated surfaces removed and documented upstream alignment policy.

**Breaking cleanup** (requires major bump):

- [ ] Remove `--strict` from Rust and Python CLIs ([`cli-spec.md`](docs/implementation/cli-spec.md) already marks deprecated)
- [ ] Remove `ValidationOptions::strict`, `validate_strict()`, and Python `strict=` parameters
- [ ] Migration note in [`docs/user/migration.md`](docs/user/migration.md) (0.4.x → 1.0)

**Upstream alignment** (when upstream releases beyond 3.1.0):

- [ ] Follow [SPEC.md](SPEC.md) synchronization workflow: pin schema, update model/validators, refresh fixtures via `scripts/sync-upstream-examples.sh`
- [ ] Document supported `apiVersion` values per release
- [ ] Add `stakeholders` model if upstream introduces the section (currently N/A — see [`stakeholders.rs`](src/model/stakeholders.rs))

**Release gate:**

- [ ] Public API review: [`docs/implementation/public-api.md`](docs/implementation/public-api.md) matches exported surface
- [ ] All phases 9–15 complete or explicitly deferred with changelog entries
- [ ] CHANGELOG and release notes for `1.0.0`
- [ ] Crates.io + PyPI publish per [docs/maintainer/releasing.md](docs/maintainer/releasing.md)

**Done when:** `1.0.0` published; no deprecated strict API remains; README and SPEC reflect supported upstream version.

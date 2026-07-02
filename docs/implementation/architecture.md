# Architecture

!!! info "Contributor documentation"
    This page describes the internal pipeline. End users can start with [What is ODCS?](../user/what-is-odcs.md) and [Getting started](../user/getting-started.md).

The reference implementation mirrors the `dtcs` processing architecture:

```text
ODCS Document
        │
        ▼
     Parser          ← duplicate-key scan (YAML), serde deserialization
        │
        ▼
Canonical Object Model   ← typed DataContract graph
        │
        ▼
    Validator         ← phase-based Rust checks + pinned JSON Schema
        │
        ▼
   Diagnostics        ← stable odcs:* codes, object_ref, remediation
```

ODCS is dataset-contract focused. [DTCS](https://github.com/eddiethedean/dtcs) is transformation-contract focused. Do not introduce transformation semantics into this crate.

## Parse stage

| Step | Module | Notes |
|------|--------|-------|
| Format detection | `parser/mod.rs` | YAML (`.yaml`, `.yml`) or JSON (`.json`) |
| Duplicate-key scan | `parser/duplicate_keys.rs` | YAML block mappings and JSON objects; fails closed on scanner errors |
| Deserialization | `parser/yaml.rs`, `parser/json.rs` | `serde` + `deny_unknown_fields` on model types |
| Size limit | `parser/mod.rs` | 16 MiB maximum (`MAX_PARSE_BYTES`) |

Parse failures emit diagnostics with stage `parse` (exit code `2` from CLI).

### YAML duplicate-key limitations

Not fully scanned: flow-style mappings, anchors, and aliases. Documented in [Diagnostics — Duplicate-key limitations](../user/diagnostics.md#duplicate-key-limitations-050).

## Validation pipeline

`validate()` in [`src/validation/mod.rs`](../../src/validation/mod.rs) runs phases sequentially and merges reports:

| Phase | Module | Checks |
|-------|--------|--------|
| Document | `document.rs` | Required root fields, supported `apiVersion` |
| Structural | `structural.rs` | Cross-field rules: unique schema/server names, SLA element references |
| Schema | `schema.rs` | Schema names, `logicalType`, array/object shape |
| Quality | `quality.rs` | Rule types, metrics, dimensions, operators |
| References | `references.rs` | Relationship endpoints and types |
| Extensions | `extensions.rs` | Custom property keys |
| Servers | `servers.rs` | Server name, `type` enum, known detail fields |
| Sections | `sections.rs` | Team, roles, support, SLA |
| IDs | `ids.rs` | StableId patterns |
| JSON Schema | `json_schema.rs` | Pinned `schema/odcs-v3.1.0.json` (always runs since 0.4.0) |
| Dedup | `dedup.rs` | Suppress JSON Schema errors when Rust validators already report the same path |

Validation failures use stage `validation` (CLI exit code `1`).

## Diagnostics

| Component | Role |
|-----------|------|
| `diagnostics/mod.rs` | `Diagnostic`, `DiagnosticReport`, severity/stage/category |
| `diagnostics/codes.rs` | Stable `odcs:*` identifiers |
| `diagnostics/inspect.rs` | Contract summary for CLI/API |

Each diagnostic may include `object_ref` (dotted path or JSON pointer), `remediation`, and since 0.6.0 `validationPhase` (validation-stage only).

## Python binding boundary

```text
Python (pyodcs)  →  PyO3  →  Rust odcs crate (same parser + validator)
```

The Python package does not reimplement validation. All semantics match the Rust core.

Build: `maturin develop --features python --locked`. See [Python API](../user/python.md).

## CLI

Feature-gated binary in `src/cli/` (`cli` feature, default on). Subcommands delegate to the same parse/validate pipeline as the library.

## Reserved modules (not implemented)

| Module | Planned purpose |
|--------|-----------------|
| `registry/` | Contract registry / discovery |
| `compatibility/` | Cross-version contract diffing |

Do not depend on these until a roadmap milestone ships. See [Non-goals](non-goals.md).

## Module map

| Module | Role |
|--------|------|
| `parser/` | YAML/JSON deserialization into `DataContract` |
| `model/` | Canonical Object Model types |
| `validation/` | Phase-based validation pipeline |
| `diagnostics/` | Structured error records and codes |
| `schema/` | Pinned ODCS JSON Schema asset |
| `cli/` | `odcs` binary (feature `cli`) |

See [crate-layout.md](crate-layout.md) for file-level layout and [relationship-to-dtcs.md](relationship-to-dtcs.md) for ecosystem positioning.

## Further reading

- [Validation guide](validation-guide.md)
- [Diagnostics guide](diagnostics-guide.md)
- [Enterprise evaluation brief](../user/enterprise-evaluation.md)

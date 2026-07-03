# Validation Guide

Validation is deterministic and phase-based. The `validate()` function in [`src/validation/mod.rs`](../../src/validation/mod.rs) orchestrates these phases:

| Phase | Module | Checks |
|-------|--------|--------|
| Document | `document.rs` | Required root fields, `apiVersion` support |
| Structural | `structural.rs` | Cross-field rules: unique schema/server names, SLA element references |
| Schema | `schema.rs` | Schema names, `logicalType` enums, array/object shape |
| Quality | `quality.rs` | Rule types, metrics, dimensions, operators |
| References | `references.rs` | Relationship endpoints and types |
| Extensions | `extensions.rs` | Custom property keys, authoritative definitions |
| Servers | `servers.rs` | Server name, `type` enum, type-specific required fields |
| Sections | `sections.rs` | Team usernames; unique `roles[].id`; support URL when tool requires it; SLA scheduler/schedule pairing; pricing currency and amount |
| IDs | `ids.rs` | StableId patterns |
| JSON Schema | `json_schema.rs` | Pinned ODCS v3.1.0 schema (always runs in 0.4.0+) |

`ValidationPhase` in [`src/diagnostics/validation_phase.rs`](../../src/diagnostics/validation_phase.rs) identifies the validator that produced each validation diagnostic. Since 0.6.0, every validation-stage diagnostic includes `validationPhase` in JSON and CLI output.

## Return type

`validate()` returns a `DiagnosticReport` (also available as `ValidationReport`). Use `report.is_valid()` to check for errors.

```rust
use odcs::{parse, validate, DocumentFormat};

let result = parse(content, DocumentFormat::Yaml);
let contract = result.contract.expect("parse succeeded");
let report = validate(&contract);
assert!(report.is_valid());
```

## Version fields

- `version` — contract document revision (any non-empty string, e.g. `1.0.0`)
- `apiVersion` — ODCS spec release; this implementation requires `v3.1.0`

## Intentional extensions

See [SPEC.md](../../SPEC.md) spec parity policy for rules stricter than JSON Schema (relationship resolution, composite parity).

## User-facing reference

For diagnostic codes and CLI integration, see [../user/diagnostics.md](../user/diagnostics.md).

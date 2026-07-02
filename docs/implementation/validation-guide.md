# Validation Guide

Validation is deterministic and phase-based. The `validate()` function in [`src/validation/mod.rs`](../../src/validation/mod.rs) orchestrates these phases:

| Phase | Module | Checks |
|-------|--------|--------|
| Document | `document.rs` | Required root fields, version/apiVersion support |
| Structural | `structural.rs` | version/apiVersion consistency |
| Schema | `schema.rs` | Required schema object and property names |
| Quality | `quality.rs` | Library metric enum, rule-type constraints |
| References | `references.rs` | Relationship endpoint integrity |
| Extensions | `extensions.rs` | Custom property key validation |

`ValidationPhase` in [`src/validation/phases.rs`](../../src/validation/phases.rs) identifies phases for future extension.

## Return type

`validate()` returns a `DiagnosticReport` (also available as `ValidationReport`). Use `report.is_valid()` to check for errors.

```rust
use odcs::{parse, validate, DocumentFormat};

let result = parse(content, DocumentFormat::Yaml);
let report = result.validate(); // parse + validation merged
assert!(report.is_valid());
```

Do not panic on invalid contracts.

## Not yet implemented

- `--strict` CLI mode (reserved)
- Deep reference resolution across files
- Full JSON Schema parity for all negative cases

See [ROADMAP.md](../../ROADMAP.md) Phase 5 and Phase 7.

## User-facing reference

For diagnostic codes and CLI integration, see [../user/diagnostics.md](../user/diagnostics.md).

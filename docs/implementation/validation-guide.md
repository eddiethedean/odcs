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
| JSON Schema | `json_schema.rs` | Strict mode only — pinned upstream schema |

`ValidationPhase` in [`src/validation/phases.rs`](../../src/validation/phases.rs) identifies phases for future extension.

## Return type

`validate()` returns a `DiagnosticReport` (also available as `ValidationReport`). Use `report.is_valid()` to check for errors.

```rust
use odcs::{parse, validate, validate_strict, DocumentFormat, ValidationOptions};

let result = parse(content, DocumentFormat::Yaml);
let contract = result.contract.expect("parse succeeded");
let report = validate(&contract);
assert!(report.is_valid());

// Strict mode (Rust pipeline + JSON Schema)
let strict_report = validate_strict(&contract);
```

## Validation options

```rust
use odcs::{validate_with_options, ValidationOptions};

let report = validate_with_options(
    &contract,
    ValidationOptions { strict: true },
);
```

Default `validate()` uses `ValidationOptions::default_options()` (`strict: false`).

## Future validation work

See [ROADMAP.md](../../ROADMAP.md#future-milestones-05) for post-0.4 backlog (cross-file references, registry, compatibility analysis).

## User-facing reference

For diagnostic codes and CLI integration, see [../user/diagnostics.md](../user/diagnostics.md).

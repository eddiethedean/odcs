# API stability policy

Policy for the `odcs` Rust crate and `pyodcs` Python package as of the 1.0 stabilization release.

## Stable (semver-major protected)

These surfaces follow [Semantic Versioning](https://semver.org/) after 1.0.0:

| Surface | Examples |
|---------|----------|
| Diagnostic codes | `odcs::codes::*`, `pyodcs.CODES` |
| CLI exit codes | `0` valid, `1` validation failure, `2` parse/I/O failure |
| CLI subcommands | `validate`, `inspect`, `diagnostics`, `schema`, `version`, `diff`, `registry` |
| `validationPhase` strings | `document`, `structural`, `schema`, `quality`, … |
| Registry index v1 | `.odcs/registry.json` with `registryVersion: "1"` |
| Root re-exports in [`src/lib.rs`](../../src/lib.rs) | `parse`, `validate`, `parse_and_validate`, `ContractSet`, `Registry`, `diff`, … |

Breaking changes to these require a major version bump.

## Additive (minor releases)

| Surface | Policy |
|---------|--------|
| `DataContract` and model fields | New optional fields may be added when upstream ODCS adds them |
| New diagnostic codes | New `odcs:*` codes may be added; existing codes remain stable |
| New CLI flags | Non-breaking when optional |
| New Python dict keys in reports | Additive only |

## Unstable (may change in minor releases)

| Surface | Notes |
|---------|-------|
| `odcs::parser`, `odcs::validation`, `odcs::model`, … | Marked `#[doc(hidden)]`; use root re-exports |
| Diagnostic `message` and `remediation` text | Match on `id` and `object_ref`, not prose |
| Internal module layout | Refactors do not require major bump if root API unchanged |

## Removed in 1.0 (breaking from 0.9.x)

| Removed | Migration |
|---------|-----------|
| CLI `--strict` | Remove flag; JSON Schema always runs in `validate` |
| `ValidationOptions`, `validate_with_options()` | Use `validate()` |
| `validate_strict()` | Use `validate()` |
| `validate_set_with_options()` | Use `validate_set()` |
| Python `strict=` on `validate`, `parse_and_validate`, … | Remove keyword; behavior unchanged |

`parse_strict()` **remains** — it means parse + validate + return typed contract, not deprecated CLI strict mode.

## MSRV

Minimum supported Rust version: **1.75** (see `rust-version` in `Cargo.toml`). MSRV may increase in minor releases with notice in the changelog.

## Upstream ODCS alignment

Each release documents the supported upstream `apiVersion` (currently `v3.1.0`). Supporting a new upstream major may require a major `odcs` release when model or validation semantics change.

See also [public-api.md](public-api.md) and [migration.md](../user/migration.md).

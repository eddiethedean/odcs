# FAQ

## General

### Is this the ODCS specification?

No. This repository is a **reference implementation** (Rust crate `odcs`, Python package `pyodcs`). The normative standard is maintained at [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).

### What problem does this solve?

It lets you parse ODCS v3.1.0 contracts into a typed object model and validate them locally or in CI — without running a data platform or quality engine.

### Is it production-ready?

**Pre-release** (`0.4.0`, Pre-Alpha on PyPI). Default validation is schema-complete for ODCS v3.1.0. See [Roadmap](../upstream/roadmap.md).

## Installation and usage

### Why does `odcs validate` fail with "command not found"?

Install the binary: `cargo install odcs`. Or run from a checkout: `cargo run -- validate contract.yaml`.

### How do I use Python?

```bash
pip install pyodcs
pyodcs validate contract.yaml
```

See [python.md](python.md).

### What's the difference between `odcs` and `pyodcs`?

- `odcs` — Rust crate and CLI
- `pyodcs` — Python package and CLI wrapping the same Rust core via PyO3

Semantics and exit codes are aligned.

## Validation errors

### Why does my contract fail with `odcs:unknown-field`?

Version 0.3.0 rejects unknown fields at the root and in nested objects. Use `customProperties` for extensions:

```yaml
customProperties:
  - property: "myExtension"
    value: "value"
```

### Why did `quality` at the root stop working?

Quality rules must be nested under `schema[]` in v3.1.0 (root-level `quality` was removed in 0.3.0).

### Why does `metric: not_null` fail?

v3.1.0 uses `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, or `rowCount`. Use `nullValues` instead of `not_null`.

### What do exit codes 0, 1, and 2 mean?

| Code | Meaning |
|------|---------|
| 0 | Valid |
| 1 | Validation errors |
| 2 | Parse or I/O failure |

### What does `--strict` do?

Since 0.4.0, nothing extra — JSON Schema validation always runs in default `validate()`. `--strict` is a deprecated no-op alias.

## Versions

### What is the difference between `version` and `apiVersion`?

- `version` — contract document revision (e.g. `1.0.0`, `2.3.1`); any non-empty string
- `apiVersion` — ODCS specification release (e.g. `v3.1.0`)

This implementation targets ODCS **apiVersion v3.1.0**. Upstream examples commonly use `version: 1.0.0` with `apiVersion: v3.1.0`.

### How does this relate to DTCS and DPCS?

- **ODCS** — what data is (this repo)
- **DTCS** — how data changes
- **DPCS** — how transformations compose

See [../implementation/relationship-to-dtcs.md](../implementation/relationship-to-dtcs.md).

## Contributing

### Where do I start as a contributor?

Read [Contributing](../upstream/contributing.md) and [Implementation overview](../implementation/overview.md).

### The upstream spec and this repo disagree — which wins?

The upstream ODCS specification always wins. See [Specification](../upstream/spec.md).

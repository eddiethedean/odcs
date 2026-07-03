# FAQ

## General

### Is this the ODCS specification?

No. This repository is a **reference implementation** (Rust crate `odcs`, Python package `pyodcs`). The normative standard is maintained at [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).

### What problem does this solve?

It lets you parse ODCS v3.1.0 contracts into a typed object model and validate them locally or in CI — without running a data platform or quality engine.

### Is it production-ready?

**0.9.0** is published on [crates.io](https://crates.io/crates/odcs) and [PyPI](https://pypi.org/project/pyodcs/). **1.0.0** stabilization is complete on `main`; the next release will commit to semver stability per [API stability policy](../implementation/api-stability.md).

| Area | Status in 0.9.0 |
|------|-----------------|
| Parse and validate ODCS v3.1.0 documents | Supported |
| CLI and library APIs | Supported |
| Local contract registry (index, lookup, `--registry`) | Supported |
| Compatibility diff (`odcs diff`) | Supported |
| Remote registry server | Out of scope (see [non-goals](../implementation/non-goals.md)) |

See [Release status](../project/release-status.md), [Roadmap](../roadmap.md), and [non-goals](../implementation/non-goals.md).

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

### Why does my contract fail with `odcs:duplicate-key`?

Since 0.5.0, duplicate mapping keys at any depth fail parse with exit code `2`. The `object_ref` uses a dotted path (e.g. `schema[0].name`). Fix duplicate keys so each field appears once. See [Migration](migration.md#04x-050).

### Why does my contract fail structural validation?

Since 0.7.0, cross-field rules run in the `structural` validation phase:

- Duplicate `schema[].name` or `servers[].server` → `odcs:invalid-schema`
- `slaProperties[].element` or `slaDefaultElement` referencing a missing schema object → `odcs:unresolved-reference`

Filter diagnostics with `validationPhase: structural` or see [Migration](migration.md#06x-070).

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

### JSON Schema validation

Since 0.4.0, JSON Schema validation always runs in `validate()`. The deprecated `--strict` flag was removed in 1.0.

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

### How do I author a new contract from scratch?

See [Authoring contracts](authoring.md) for a minimal template. The normative ODCS specification is at [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).

### How do I upgrade from an older release?

See [Migration](migration.md).

## Contributing

### Where do I start as a contributor?

Read [Contributing](../contributing.md) and [Implementation overview](../implementation/overview.md).

### The upstream spec and this repo disagree — which wins?

The upstream ODCS specification always wins. See [Upstream sync policy](../upstream-sync-policy.md).

### Where is the security policy?

See [SECURITY.md](../../SECURITY.md).

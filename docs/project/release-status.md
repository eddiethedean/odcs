# Release status

This page clarifies what is on `main` versus what is published to registries.

## Current versions

| Source | Version | Notes |
|--------|---------|-------|
| `main` branch (`Cargo.toml`, `pyproject.toml`) | **0.5.0** | Documentation on [Read the Docs](https://odcs.readthedocs.io/) reflects this tree |
| [crates.io](https://crates.io/crates/odcs) | **0.4.0** | Latest published Rust crate (until `v0.5.0` tag) |
| [PyPI](https://pypi.org/project/pyodcs/) | **0.4.0** | Latest published Python package (until `v0.5.0` tag) |

!!! note "Why badges may show 0.4.0"
    README badges read from crates.io and PyPI. They update when maintainers push the `v0.5.0` release tag.

## What 0.5.0 adds over 0.4.0

- Nested YAML/JSON duplicate-key detection with path-style `object_ref`
- Fail-closed YAML duplicate-key scanner
- Server property typo detection in flattened `details`
- JSON Schema diagnostic deduplication when Rust validators report the same field
- Python `is_valid()` accepts parse-result dicts

See [Changelog](../changelog.md) for the full list.

## How to install a specific version

### Published release (0.4.0 today)

```bash
cargo install odcs --version 0.4.0 --locked
pip install pyodcs==0.4.0
```

### Latest `main` (0.5.0) from source

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo install --path . --locked
# Python:
maturin develop --features python --locked
```

## Pinning in CI

Pin the **published** version for reproducible CI until `v0.5.0` is released:

```bash
cargo install odcs --version 0.4.0 --locked
pip install pyodcs==0.4.0
```

After `v0.5.0` ships, update pins to `0.5.0`. See [CI/CD integration](../user/ci-cd.md).

Maintainers: see [Releasing](../maintainer/releasing.md).

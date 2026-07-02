# Release status

Current release: **0.5.0** (on `main`, ready to publish).

## Version alignment

| Source | Version | Status |
|--------|---------|--------|
| `Cargo.toml` | **0.5.0** | Aligned |
| `pyproject.toml` | **0.5.0** | Aligned |
| `CHANGELOG.md` | **0.5.0** | Release notes present |
| Git tag | *(none yet)* | Push `v0.5.0` to publish |

## Registry status

| Registry | Latest published | After `v0.5.0` tag |
|----------|------------------|---------------------|
| [crates.io](https://crates.io/crates/odcs) | 0.4.0 | 0.5.0 (via release workflow) |
| [PyPI](https://pypi.org/project/pyodcs/) | 0.4.0 | 0.5.0 (via release workflow) |

## Install 0.5.0

### After release (crates.io / PyPI)

```bash
cargo install odcs --version 0.5.0 --locked
pip install pyodcs==0.5.0
```

### From source (before or after tag)

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo install --path . --locked
maturin develop --features python --locked   # Python editable install
```

## Pin in CI

```bash
cargo install odcs --version 0.5.0 --locked
pip install pyodcs==0.5.0
```

See [CI/CD integration](../user/ci-cd.md).

## Publish checklist (maintainers)

Before pushing `v0.5.0`:

- [x] `Cargo.toml` / `pyproject.toml` = `0.5.0`
- [x] `CHANGELOG.md` has 0.5.0 release notes
- [x] Full CI parity passes locally (fmt, clippy, doc, test, pytest, mkdocs, publish dry-run)
- [x] `main` CI green
- [ ] GitHub secrets `CARGO_REGISTRY_TOKEN` and `PYPI_API_TOKEN` configured
- [ ] No existing `v0.5.0` tag on remote

To publish:

```bash
git tag v0.5.0
git push origin v0.5.0
```

Monitor [Release workflow](../../.github/workflows/release.yml). See [Releasing](../maintainer/releasing.md) for post-release verification.

## What changed in 0.5.0

See [Changelog](../changelog.md). Highlights:

- Nested YAML/JSON duplicate-key detection with path-style `object_ref`
- Fail-closed YAML duplicate-key scanner
- Server property typo detection; JSON Schema diagnostic dedup
- Documentation adoption improvements

## Previous release

**0.4.0** — schema-complete default validation for ODCS v3.1.0 (JSON Schema always on).

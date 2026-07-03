# Release Process

Releases are triggered by pushing a version tag. CI publishes to crates.io and PyPI.

## Prerequisites

- All checks pass on `main` ([`.github/workflows/checks.yml`](../../.github/workflows/checks.yml))
- `Cargo.toml`, `pyproject.toml`, and `CHANGELOG.md` updated for the new version
- GitHub secrets configured: `CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`

## Version alignment

These files must agree on the version number:

| File | Field |
|------|-------|
| `Cargo.toml` | `[package] version` |
| `pyproject.toml` | `[project] version` |
| Git tag | `vX.Y.Z` (e.g. `v0.9.1`) |

The release workflow verifies tag ↔ Cargo.toml ↔ pyproject.toml alignment.

## Pre-release checklist

Before pushing a tag:

```bash
# Full CI parity (see CONTRIBUTING.md)
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
cargo test --locked
maturin develop --features python --locked && pytest python/tests -v
maturin build --features python --locked
pip install -r docs/requirements.txt && mkdocs build --strict
./scripts/check-doc-versions.sh

# Publish dry runs
cargo publish --dry-run --locked
```

Confirm:

- [ ] `CHANGELOG.md` has release notes for the target version
- [ ] `Cargo.toml`, `pyproject.toml`, and tag name all match
- [ ] `main` is green in CI
- [ ] GitHub secrets `CARGO_REGISTRY_TOKEN` and `PYPI_API_TOKEN` are set
- [ ] No existing `vX.Y.Z` tag on the remote (check with `git ls-remote --tags origin`)

## Steps

1. Update `CHANGELOG.md` with release notes.
2. Bump versions in `Cargo.toml` and `pyproject.toml`.
3. Commit and push to `main`.
4. Create and push the tag:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

5. Monitor [`.github/workflows/release.yml`](../../.github/workflows/release.yml):
   - Runs checks
   - Publishes `odcs` to crates.io
   - Builds Python wheels (Linux, musllinux, Windows, macOS)
   - Publishes `pyodcs` to PyPI

## Dry runs

```bash
cargo publish --dry-run --locked --allow-dirty
maturin build --features python --locked
```

## Post-release

- Verify [crates.io/crates/odcs](https://crates.io/crates/odcs) and [pypi.org/project/pyodcs](https://pypi.org/project/pyodcs/)
- Verify [odcs.readthedocs.io](https://odcs.readthedocs.io/) built the new tag (Read the Docs webhook)
- Update [ROADMAP.md](../../ROADMAP.md) milestone status if needed

## Upstream ODCS sync

When bumping for a new upstream ODCS release, follow [SPEC.md](../../SPEC.md) synchronization workflow:

1. Review upstream changelog and JSON Schema
2. Update pinned schema in `schema/odcs-v3.1.0.json` (and `tests/fixtures/` copy if kept)
3. Run `./scripts/sync-upstream-examples.sh <upstream-commit>` to refresh example corpus
4. Update `UPSTREAM_SPEC_VERSION` in `src/lib.rs`
5. Update conformance tests and documentation

# Release status

Current release: **0.9.0** (published 2026-07-03).

## Version alignment

| Source | Version | Status |
|--------|---------|--------|
| `Cargo.toml` | **0.9.0** | Aligned |
| `pyproject.toml` | **0.9.0** | Aligned |
| `CHANGELOG.md` | **0.9.0** | Release notes present |
| Git tag | **v0.9.0** | Published |

## Registry status

| Registry | Latest published |
|----------|------------------|
| [crates.io](https://crates.io/crates/odcs) | **0.9.0** |
| [PyPI](https://pypi.org/project/pyodcs/) | **0.9.0** |

Release workflow: [actions/runs/28630711719](https://github.com/eddiethedean/odcs/actions/runs/28630711719) (success).

## Install 0.9.0

### From crates.io / PyPI (after release)

```bash
cargo install odcs --version 0.9.0 --locked
pip install pyodcs==0.9.0
```

### From source

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo install --path . --locked
maturin develop --features python --locked
```

## What changed in 0.9.0

See [Changelog](../changelog.md). Highlights:

- Local contract registry: `odcs registry index|lookup|list`
- `odcs validate --registry <dir>` for registry-backed FQN resolution
- Python registry helpers and `parse_and_validate_paths(..., registry=...)`

## Previous release

**0.8.0** — Section semantics, cross-file references, and compatibility analysis.

# Release status

Current release: **0.8.0** (published 2026-07-03).

## Version alignment

| Source | Version | Status |
|--------|---------|--------|
| `Cargo.toml` | **0.8.0** | Aligned |
| `pyproject.toml` | **0.8.0** | Aligned |
| `CHANGELOG.md` | **0.8.0** | Release notes present |
| Git tag | **v0.8.0** | Published |

## Registry status

| Registry | Latest published |
|----------|------------------|
| [crates.io](https://crates.io/crates/odcs) | **0.8.0** |
| [PyPI](https://pypi.org/project/pyodcs/) | **0.8.0** |

Release workflow: [actions/runs/28629406161](https://github.com/eddiethedean/odcs/actions/runs/28629406161) (success).

## Install 0.8.0

### From crates.io / PyPI (after release)

```bash
cargo install odcs --version 0.8.0 --locked
pip install pyodcs==0.8.0
```

### From source

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo install --path . --locked
maturin develop --features python --locked
```

## What changed in 0.8.0

See [Changelog](../changelog.md). Highlights:

- Section semantics: roles, support, SLA, pricing validators
- Cross-file FQN resolution with `--dep` / `--include`
- `odcs diff` and `pyodcs.diff()` compatibility analysis

## Previous release

**0.7.0** — Structural validation phase for schema/server uniqueness and SLA element references.

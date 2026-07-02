# Installation

!!! tip "Published vs main"
    Latest **published** releases are **0.4.0** on crates.io and PyPI. The `main` branch is **0.5.0**. See [Release status](../project/release-status.md).

## Prerequisites

| Component | Requirement |
|-----------|-------------|
| Rust (`odcs`) | Rust **1.75+** ([rustup](https://rustup.rs/)) |
| Python (`pyodcs`) | Python **3.9+** |
| From-source Python build | [maturin](https://www.maturin.rs/) 1.x, Rust toolchain |

## Rust CLI and library

### Install from crates.io

```bash
cargo install odcs
odcs version
```

### Use as a library dependency

Add to `Cargo.toml`:

```toml
[dependencies]
odcs = "0.5"
```

Default features include the CLI. For library-only use:

```toml
odcs = { version = "0.5", default-features = false }
```

### Build from source

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo build --release
cargo install --path . --locked
```

## Python package

### Install from PyPI

```bash
pip install pyodcs
pyodcs version
```

### Editable install from source

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
python -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate
pip install maturin pytest
maturin develop --features python --locked
pytest python/tests -v
```

## Verify installation

Create a minimal contract or clone this repository, then validate:

```bash
# After saving contract.yaml (see Getting started)
odcs validate contract.yaml

# Or, from a repository checkout:
odcs validate examples/minimal.odcs.yaml
pyodcs validate examples/minimal.odcs.yaml
```

Both should print `valid` and exit with code `0`.

## Troubleshooting

### `odcs: command not found`

Install the binary with `cargo install odcs`, or run from a checkout:

```bash
cargo run -- validate examples/minimal.odcs.yaml
```

Ensure `~/.cargo/bin` is on your `PATH`.

### `pyodcs` import fails after editable install

Rebuild the native extension:

```bash
maturin develop --features python --locked
```

### `PackageNotFoundError` for pyodcs

You are importing from source without installing. Run `maturin develop` or `pip install pyodcs`.

### Unsupported file extension

Only `.yaml`, `.yml`, and `.json` are supported. Rename your file or convert the format.

### Validation fails on a contract that worked in 0.2.x

Version 0.5.0 adds nested YAML duplicate-key detection. Version 0.4.0 makes default validation schema-complete. Version 0.3.0 enforces stricter parsing (`deny_unknown_fields`, nested quality). See [migration.md](migration.md), [CHANGELOG.md](../../CHANGELOG.md), and [Troubleshooting](troubleshooting.md).

### `~/.cargo/bin` not on PATH (WSL, macOS, Linux)

Add to your shell profile:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Corporate proxy or air-gapped installs

- **Rust:** Pre-download with `cargo install odcs --locked` on a networked machine, or vendor the crate.
- **Python:** Install wheels from PyPI mirror or use `pip download pyodcs` offline.
- **From source:** Clone the repository and build locally; no network required after dependencies are cached.

### musl / Alpine Linux

Use the musllinux wheel on PyPI, or build from source with `maturin develop --features python --locked`.

### macOS / Linux build errors with Python feature

Ensure a C compiler and Python development headers are available. On Debian/Ubuntu:

```bash
sudo apt install python3-dev
```

## Platform support

- **Rust crate:** any platform supported by the Rust toolchain.
- **Python wheels:** Linux (glibc and musl), macOS (x86_64 and arm64), Windows — built by CI on release tags.

# Contributing to ODCS

Thank you for contributing to the Open Data Contract Standard reference implementation.

## Upstream specification

Normative ODCS semantics are defined in the [upstream specification](https://github.com/bitol-io/open-data-contract-standard). This repository's [SPEC.md](SPEC.md) documents how we track and synchronize with upstream releases.

When implementing behavior:

1. Read the relevant upstream ODCS specification sections.
2. Consult [docs/implementation/](docs/implementation/) for architecture and API guidance.
3. Preserve spec-aligned naming and behavior.
4. Add tests that exercise spec requirements. See [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md).

### Authority

The upstream ODCS specification is the single source of truth for semantics, terminology, and conformance. Implementation docs in [docs/implementation/](docs/implementation/) are illustrative unless explicitly normative.

## Development setup

### Prerequisites

- Rust 1.75+
- Python 3.9+ (for `pyodcs` work)
- [maturin](https://www.maturin.rs/) (for Python editable installs)

### Rust

```bash
git clone https://github.com/eddiethedean/odcs.git
cd odcs
cargo build
cargo test --locked
```

### Python

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin pytest
maturin develop --features python --locked
pytest python/tests -v
```

### Full CI parity

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
cargo test --locked
maturin develop --features python --locked
pytest python/tests -v
maturin build --features python --locked
```

## Implementation changes

The Rust reference crate lives in [src/](src/). Before implementing a module:

1. Read the relevant upstream specification sections.
2. Consult [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md) for module boundaries.
3. Preserve spec-aligned naming and behavior.
4. Add tests for behavioral changes.

### Scope

The crate targets parsing, the canonical object model, validation, and diagnostics. Do not add execution, pipeline composition, or transformation features without an agreed milestone. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

### Code style

- Run `cargo fmt` and `cargo clippy` before submitting changes.
- Keep modules aligned with [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md).
- Prefer conservative behavior when the spec is ambiguous; document open questions with a `TODO` referencing the spec section.

### Documentation

- User-facing changes: update [docs/user/](docs/user/) and [README.md](README.md).
- API changes: update [docs/implementation/public-api.md](docs/implementation/public-api.md).
- Breaking changes: update [CHANGELOG.md](CHANGELOG.md) and [docs/user/migration-0.3.md](docs/user/migration-0.3.md).

## Pull requests

1. Describe whether the change is implementation, documentation, or infrastructure.
2. Link related issues or design discussions when available.
3. Include or update tests for behavioral changes.
4. Ensure `cargo test --locked` passes (and Python tests if touching bindings).

## Releases

Maintainers: see [docs/maintainer/releasing.md](docs/maintainer/releasing.md).

## Questions

For ambiguous upstream spec language, open an issue with the relevant section rather than inventing behavior in code.

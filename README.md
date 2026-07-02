# ODCS — Open Data Contract Standard

Reference Rust implementation for the [Open Data Contract Standard](https://github.com/bitol-io/open-data-contract-standard).

**Status:** Pre-release  
**Upstream ODCS version:** 3.1.0  
**Reference implementation:** 0.3.0 (Phase 2–3 complete; validation, CLI, Python bindings, and JSON Schema conformance baseline in progress)

See [ROADMAP.md](ROADMAP.md) for milestone status.

## Overview

[SPEC.md](SPEC.md) defines the upstream specification policy for this repository. The published ODCS specification remains the normative standard; this crate provides an idiomatic Rust API, deterministic validation, diagnostics, and CLI tooling aligned with upstream semantics.

This repository contains:

| Path | Purpose |
|------|---------|
| [SPEC.md](SPEC.md) | Upstream specification policy and synchronization workflow |
| [docs/](docs/) | Documentation index |
| [docs/implementation/](docs/implementation/) | Reference implementation design and build guides |
| [src/](src/) | Rust crate source (`odcs`) |
| [python/](python/) | Python package source (`pyodcs` on PyPI) |
| [examples/](examples/) | Sample ODCS data contracts |
| [tests/](tests/) | Integration tests and fixtures |
| [ROADMAP.md](ROADMAP.md) | Reference implementation milestones |
| [.github/workflows/](.github/workflows/) | CI pipeline |

## Ecosystem

```text
ODCS defines what data is.
DTCS defines how data changes.
DPCS defines how transformations compose.
```

See [docs/implementation/relationship-to-dtcs.md](docs/implementation/relationship-to-dtcs.md) for positioning alongside [DTCS](https://github.com/dtcs/dtcs).

## Quick start

### Build the Rust crate

```bash
cargo build
cargo test
cargo run -- validate examples/minimal.odcs.yaml
cargo run -- version
```

### Validate a contract

```bash
odcs validate examples/minimal.odcs.yaml
odcs validate examples/minimal.odcs.yaml --json
odcs inspect examples/minimal.odcs.yaml
```

The reference implementation targets this pipeline:

```text
ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, pipeline composition, and transformation semantics remain out of scope. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

## Repository layout

```text
odcs/
├── SPEC.md                 # Upstream specification policy
├── Cargo.toml              # Rust crate manifest
├── pyproject.toml          # Python package manifest (`pyodcs`, maturin)
├── README.md
├── CONTRIBUTING.md
├── LICENSE
├── docs/
│   ├── README.md           # Documentation index
│   └── implementation/     # Reference implementation guides
├── examples/               # Example data contracts
├── python/                 # Python package source (`pyodcs`)
├── src/                    # Rust library, CLI binary, validation
├── tests/                  # Integration tests and fixtures
└── .cursor/prompts/        # Cursor build prompt
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for implementation guidelines and the review process.

When implementation guidance conflicts with the upstream ODCS specification, **the upstream specification wins**. See [docs/implementation/spec-usage.md](docs/implementation/spec-usage.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).

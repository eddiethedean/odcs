# odcs — Rust & Python reference implementation for ODCS

[![CI](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**Validate [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) documents** with a deterministic parser, validator, and CLI.

Use this repository if you want to:

- validate ODCS v3.1.0 contracts in CI/CD
- parse contracts into a typed object model (Rust or Python)
- get structured diagnostics with stable `odcs:*` error codes

**Status:** Pre-release (`0.4.0`) — core parsing and validation; `--strict` JSON Schema mode available; see [ROADMAP.md](ROADMAP.md).

> This repository implements the standard; it is not the ODCS specification itself.

| | |
|---|---|
| **Upstream ODCS** | 3.1.0 |
| **Rust crate** | [`odcs`](https://crates.io/crates/odcs) |
| **Python package** | [`pyodcs`](https://pypi.org/project/pyodcs/) |
| **API docs** | [docs.rs/odcs](https://docs.rs/odcs) |

## Installation

### Prerequisites

- **Rust:** 1.75+ (for the `odcs` crate and CLI)
- **Python:** 3.9+ (for the `pyodcs` package)

### Rust CLI

```bash
cargo install odcs
odcs version
```

### Python

```bash
pip install pyodcs
pyodcs version
```

### From source

```bash
git clone https://github.com/eddiethedean/odcs.git && cd odcs
cargo build --release
cargo install --path . --locked

# Python editable install
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --features python --locked
```

See [docs/user/installation.md](docs/user/installation.md) for troubleshooting.

## Quick start

### Rust CLI

```bash
odcs validate examples/minimal.odcs.yaml
# valid

odcs validate examples/minimal.odcs.yaml --json
odcs inspect examples/minimal.odcs.yaml
odcs diagnostics examples/minimal.odcs.yaml
```

From a checkout without installing the binary:

```bash
cargo run -- validate examples/minimal.odcs.yaml
```

### Rust library

```rust
use odcs::{parse, validate, DocumentFormat};

let yaml = include_bytes!("examples/minimal.odcs.yaml");
let result = parse(yaml, DocumentFormat::Yaml);
let contract = result.into_contract().expect("valid contract");
let report = validate(&contract);
assert!(report.is_valid());
```

### Python

```python
import pyodcs

report = pyodcs.parse_and_validate(open("examples/minimal.odcs.yaml", "rb").read())
assert pyodcs.is_valid(report)

result = pyodcs.parse_file("examples/minimal.odcs.yaml")
print(pyodcs.inspect(result["contract"]))
```

## Documentation

| I want to… | Read |
|------------|------|
| Get started in 5 minutes | [docs/user/getting-started.md](docs/user/getting-started.md) |
| Install Rust or Python | [docs/user/installation.md](docs/user/installation.md) |
| Use the CLI | [docs/user/cli.md](docs/user/cli.md) |
| Use from Python | [docs/user/python.md](docs/user/python.md) |
| Understand error codes | [docs/user/diagnostics.md](docs/user/diagnostics.md) |
| Browse examples | [examples/README.md](examples/README.md) |
| Upgrade from 0.2.x | [docs/user/migration-0.3.md](docs/user/migration-0.3.md) |
| Upgrade to 0.4.x | [docs/user/migration-0.4.md](docs/user/migration-0.4.md) |
| Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Full doc index | [docs/README.md](docs/README.md) |

## Pipeline

```text
ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, pipeline composition, and transformation semantics are out of scope. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

## Ecosystem

```text
ODCS defines what data is.
DTCS defines how data changes.
DPCS defines how transformations compose.
```

See [docs/implementation/relationship-to-dtcs.md](docs/implementation/relationship-to-dtcs.md).

## Repository layout

```text
odcs/
├── docs/user/              # User guides (install, CLI, Python, diagnostics)
├── docs/implementation/    # Maintainer / implementation guides
├── examples/               # Sample data contracts
├── python/pyodcs/          # Python package source
├── src/                    # Rust library and CLI
└── tests/fixtures/         # Integration test fixtures
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). When implementation guidance conflicts with the upstream ODCS specification, **the upstream specification wins**.

## License

Apache License 2.0. See [LICENSE](LICENSE).

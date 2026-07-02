# odcs — Rust & Python reference implementation for ODCS

[![CI](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml)
[![Documentation](https://readthedocs.org/projects/odcs/badge/?version=latest)](https://odcs.readthedocs.io/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**Validate [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) documents** — machine-readable contracts that describe datasets (schemas, quality rules, SLAs, ownership, and server metadata) — with a deterministic parser, validator, and CLI.

This tool checks that your contract *document* is well-formed and conforms to ODCS v3.1.0. It does **not** execute quality checks against live data.

Use this repository if you want to:

- validate ODCS v3.1.0 contracts in CI/CD
- parse contracts into a typed object model (Rust or Python)
- get structured diagnostics with stable `odcs:*` error codes

**Status:** Alpha pre-1.0 (`0.4.0`) — schema-complete default validation for ODCS v3.1.0; see [ROADMAP.md](ROADMAP.md) and the [documentation](https://odcs.readthedocs.io/).

> This repository implements the standard; it is not the ODCS specification itself.

| | |
|---|---|
| **Upstream ODCS** | 3.1.0 |
| **Rust crate** | [`odcs`](https://crates.io/crates/odcs) |
| **Python package** | [`pyodcs`](https://pypi.org/project/pyodcs/) |
| **Documentation** | [odcs.readthedocs.io](https://odcs.readthedocs.io/) |
| **Rust API docs** | [docs.rs/odcs](https://docs.rs/odcs) |

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

See the [installation guide](https://odcs.readthedocs.io/en/latest/user/installation/) ([source](docs/user/installation.md)) for troubleshooting.

## Quick start

Save a minimal contract as `contract.yaml`:

```yaml
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "hello-contract"
status: "draft"
schema:
  - name: customers
    properties:
      - name: customer_id
        logicalType: string
        required: true
```

### Rust CLI

```bash
odcs validate contract.yaml
# valid

odcs validate contract.yaml --json
odcs inspect contract.yaml
odcs diagnostics contract.yaml
```

From a repository checkout (includes `examples/`):

```bash
odcs validate examples/minimal.odcs.yaml
cargo run -- validate examples/minimal.odcs.yaml
```

### Rust library

```rust
use odcs::{parse, validate, DocumentFormat};

let yaml = br#"
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "hello-contract"
status: "draft"
schema:
  - name: customers
    properties:
      - name: customer_id
        logicalType: string
        required: true
"#;

let result = parse(yaml, DocumentFormat::Yaml);
let contract = result.into_contract().expect("valid contract");
let report = validate(&contract);
assert!(report.is_valid());
```

### Python

```python
import pyodcs

content = open("contract.yaml", "rb").read()
report = pyodcs.parse_and_validate(content, format="yaml")
assert pyodcs.is_valid(report)

result = pyodcs.parse(content, format="yaml")
print(pyodcs.inspect(result["contract"]))
```

## Documentation

**Full docs:** [odcs.readthedocs.io](https://odcs.readthedocs.io/)

| I want to… | Read |
|------------|------|
| Get started in 5 minutes | [Getting started](https://odcs.readthedocs.io/en/latest/user/getting-started/) · [source](docs/user/getting-started.md) |
| Install Rust or Python | [Installation](https://odcs.readthedocs.io/en/latest/user/installation/) · [source](docs/user/installation.md) |
| Use the CLI | [CLI](https://odcs.readthedocs.io/en/latest/user/cli/) · [source](docs/user/cli.md) |
| Use from Rust | [Rust](https://odcs.readthedocs.io/en/latest/user/rust/) · [source](docs/user/rust.md) |
| Use from Python | [Python](https://odcs.readthedocs.io/en/latest/user/python/) · [source](docs/user/python.md) |
| Author a contract | [Authoring](https://odcs.readthedocs.io/en/latest/user/authoring/) · [source](docs/user/authoring.md) |
| Integrate in CI/CD | [CI/CD](https://odcs.readthedocs.io/en/latest/user/ci-cd/) · [source](docs/user/ci-cd.md) |
| Understand error codes | [Diagnostics](https://odcs.readthedocs.io/en/latest/user/diagnostics/) · [source](docs/user/diagnostics.md) |
| Upgrade between versions | [Migration](https://odcs.readthedocs.io/en/latest/user/migration/) · [source](docs/user/migration.md) |
| Browse examples | [Examples](https://odcs.readthedocs.io/en/latest/upstream/examples/) · [source](examples/README.md) |
| Contribute | [Contributing](https://odcs.readthedocs.io/en/latest/upstream/contributing/) · [source](CONTRIBUTING.md) |
| Report a security issue | [SECURITY.md](SECURITY.md) |
| Implementation guides | [Implementation](https://odcs.readthedocs.io/en/latest/implementation/overview/) · [source](docs/implementation/overview.md) |
| Rust API reference | [docs.rs/odcs](https://docs.rs/odcs) |

## Pipeline

```text
ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, pipeline composition, and transformation semantics are out of scope. See [non-goals](https://odcs.readthedocs.io/en/latest/implementation/non-goals/) ([source](docs/implementation/non-goals.md)).

## Ecosystem

```text
ODCS defines what data is.
DTCS defines how data changes.
DPCS defines how transformations compose.
```

See [Relationship to DTCS](https://odcs.readthedocs.io/en/latest/implementation/relationship-to-dtcs/) ([source](docs/implementation/relationship-to-dtcs.md)).

## Repository layout

```text
odcs/
├── docs/user/              # User guides (install, CLI, Rust, Python, diagnostics)
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

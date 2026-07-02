# Getting Started

This guide gets you from zero to a validated ODCS contract in about five minutes.

New to ODCS? Read [What is ODCS?](what-is-odcs.md) first.

## What you need to know

- **ODCS** (Open Data Contract Standard) defines machine-readable **data contracts** — schemas, quality rules, SLAs, ownership, and more.
- **This repository** (`odcs` / `pyodcs`) is a **reference implementation** that parses and validates ODCS documents. It is not the specification itself.
- The normative standard lives at [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).

## Step 1 — Install

Choose Rust or Python (or both):

```bash
# Rust CLI (pin 0.7.0 until published to crates.io — see Release status)
cargo install odcs --version 0.7.0 --locked

# Python package + CLI
pip install pyodcs==0.7.0
```

Unpinned `cargo install odcs` / `pip install pyodcs` may install **0.6.0** from registries until tag `v0.7.0` is published. See [Release status](../project/release-status.md).

See [installation.md](installation.md) for prerequisites, from-source setup, and troubleshooting.

## Step 2 — Validate a contract

### If you installed from crates.io or PyPI

Save this minimal contract as `contract.yaml`:

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

Then validate:

```bash
odcs validate contract.yaml
```

### If you cloned this repository

```bash
odcs validate examples/minimal.odcs.yaml
```

Expected output on success:

```text
valid
```

On failure you see structured diagnostics:

```text
[error] odcs:invalid-kind: expected kind 'DataContract', got 'WrongKind'
  at: kind
```

Exit codes: `0` = valid, `1` = validation error, `2` = parse or I/O failure.

## Step 3 — Inspect a contract

```bash
# Use the same path you validated in Step 2
odcs inspect contract.yaml
# or, from a repo checkout:
odcs inspect examples/minimal.odcs.yaml
```

Prints a short summary: id, name, version, schema count, quality rule count.

## Step 4 — Use from code

### Rust

```rust
use odcs::{parse, DocumentFormat};

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
println!("contract id: {}", contract.id);
```

### Python

```python
import pyodcs

content = open("contract.yaml", "rb").read()
report = pyodcs.parse_and_validate(content, format="yaml")
assert pyodcs.is_valid(report)
print(pyodcs.inspect(pyodcs.parse(content, format="yaml")["contract"]))
```

For step-by-step parse then validate, see [API decision guide](api-guide.md).

## Step 5 — Explore more examples

The [examples catalog](../examples.md) includes contracts with SLA, team, servers, relationships, and quality rules.

## What to read next

| Goal | Document |
|------|----------|
| Choose the right API | [api-guide.md](api-guide.md) |
| CLI flags and JSON output | [cli.md](cli.md) |
| Rust API reference | [rust.md](rust.md) |
| Python API reference | [python.md](python.md) |
| Author a contract | [authoring.md](authoring.md) |
| Error codes and remediation | [diagnostics.md](diagnostics.md) |
| CI/CD integration | [ci-cd.md](ci-cd.md) |
| Upgrade guide | [migration.md](migration.md) |
| Fix a broken contract | [tutorials/fix-invalid-contract.md](tutorials/fix-invalid-contract.md) |
| Troubleshoot errors | [troubleshooting.md](troubleshooting.md) |
| Common questions | [faq.md](faq.md) |

## What this tool does not do

- Execute data quality checks against live data
- Run ETL or pipeline transformations
- Host a contract registry server

See [../implementation/non-goals.md](../implementation/non-goals.md).

# Getting Started

This guide gets you from zero to a validated ODCS contract in about five minutes.

## What you need to know

- **ODCS** (Open Data Contract Standard) defines machine-readable **data contracts** — schemas, quality rules, SLAs, ownership, and more.
- **This repository** (`odcs` / `pyodcs`) is a **reference implementation** that parses and validates ODCS documents. It is not the specification itself.
- The normative standard lives at [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).

## Step 1 — Install

Choose Rust or Python (or both):

```bash
# Rust CLI
cargo install odcs

# Python package + CLI
pip install pyodcs
```

See [installation.md](installation.md) for prerequisites, from-source setup, and troubleshooting.

## Step 2 — Validate an example contract

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
odcs inspect examples/minimal.odcs.yaml
```

Prints a short summary: id, name, version, schema count, quality rule count.

## Step 4 — Use from code

### Rust

```rust
use odcs::{parse_file, DocumentFormat};

let result = parse_file("examples/minimal.odcs.yaml")?;
let contract = result.into_contract()?;
println!("contract id: {}", contract.id);
```

### Python

```python
import pyodcs

result = pyodcs.parse_file("examples/minimal.odcs.yaml")
report = pyodcs.validate_result(result)
assert pyodcs.is_valid(report)
print(pyodcs.inspect(result["contract"]))
```

## Step 5 — Explore more examples

The [examples catalog](../upstream/examples.md) includes contracts with SLA, team, servers, relationships, and quality rules.

## What to read next

| Goal | Document |
|------|----------|
| CLI flags and JSON output | [cli.md](cli.md) |
| Python API reference | [python.md](python.md) |
| Error codes and remediation | [diagnostics.md](diagnostics.md) |
| Common questions | [faq.md](faq.md) |

## What this tool does not do

- Execute data quality checks against live data
- Run ETL or pipeline transformations
- Host a contract registry server

See [../implementation/non-goals.md](../implementation/non-goals.md).

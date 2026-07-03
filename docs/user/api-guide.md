# API decision guide

Choose the right entry point for Rust and Python. Both bindings share the same parse and validation semantics.

!!! tip "CLI first"
    For CI and local checks, prefer `odcs validate` or `pyodcs validate`. Use the library APIs when you need programmatic access to the parsed contract or diagnostics.

## Rust

| Goal | Use | Returns |
|------|-----|---------|
| Parse only; inspect parse diagnostics | `parse(content, format)` | `ParseResult { contract, report }` |
| Parse + validate; get all diagnostics | `parse_and_validate(content, format)` or `parse(...).validate()` | `DiagnosticReport` |
| Parse + validate; need typed `DataContract` or fail | `parse(...).into_contract()` or `parse_strict(...)` | `Result<DataContract, DiagnosticReport>` |
| Validate an existing contract | `validate(&contract)` | `DiagnosticReport` |
| Read from file | `parse_file(path)` | `miette::Result<ParseResult>` — see [File I/O](#file-io-and-miette) |

### `parse` → `validate` vs `parse_and_validate`

```rust
use odcs::{parse, parse_and_validate, validate, DocumentFormat};

// One step — diagnostics only
let report = parse_and_validate(yaml, DocumentFormat::Yaml);

// Two steps — access contract before validating
let result = parse(yaml, DocumentFormat::Yaml);
if let Some(contract) = result.contract {
    let report = validate(&contract);
}
```

### `into_contract()` vs `parse_strict()`

Both return `Result<DataContract, DiagnosticReport>` after parse **and** validation.

- **`into_contract()`** — call on a `ParseResult` you already have
- **`parse_strict()`** — convenience when starting from bytes

### Removed in 1.0

| Removed | Use instead |
|---------|-------------|
| `--strict` (CLI) | `odcs validate` (JSON Schema always runs) |
| `validate_strict()` | `validate()` |
| `ValidationOptions`, `validate_with_options()` | `validate()` |
| Python `strict=` | Remove keyword |

See [migration.md](migration.md#09x-100).

## Python

| Goal | Use | Returns |
|------|-----|---------|
| Parse + validate (recommended) | `parse_and_validate(content, format=...)` | `{"diagnostics": [...]}` |
| Parse then validate separately | `parse(...)` then `validate_result(result)` | parse result dict, then report dict |
| Validate a contract dict | `validate(contract)` | `{"diagnostics": [...]}` |
| Check success | `is_valid(report)` | `bool` — accepts validation reports **or** parse results |

### Recommended pattern

```python
import pyodcs
import sys

report = pyodcs.parse_and_validate(open("contract.yaml", "rb").read(), format="yaml")
if not pyodcs.is_valid(report):
    for d in report["diagnostics"]:
        print(f"{d['id']}: {d['message']}", file=sys.stderr)
    sys.exit(1)
```

### Step-by-step alternative

Use when you need the parsed contract dict before validation:

```python
result = pyodcs.parse_file("contract.yaml")
report = pyodcs.validate_result(result)
contract = result["contract"]
```

## File I/O and `miette`

`parse_file()` and `DataContract::from_file()` return `miette::Result` because file read errors are reported through the `miette` crate (a direct dependency of `odcs`).

If you use `?` with these functions, add `miette` to your `Cargo.toml`:

```toml
[dependencies]
odcs = "0.9"
miette = { version = "7", features = ["fancy"] }
```

Or handle I/O without `miette`:

```rust
use std::fs;

let content = fs::read("contract.yaml").expect("read file");
let result = odcs::parse(&content, odcs::DocumentFormat::Yaml);
```

## Diagnostic handling

| Task | Rust | Python |
|------|------|--------|
| List errors | `report.diagnostics` | `report["diagnostics"]` |
| Valid? | `report.is_valid()` | `pyodcs.is_valid(report)` |
| Stable codes | `odcs::codes::*` | `pyodcs.CODES` |

See [Diagnostics reference](diagnostics.md) and [Troubleshooting](troubleshooting.md).

## Further reading

- [Rust API](rust.md)
- [Python API](python.md)
- [CLI reference](cli.md)

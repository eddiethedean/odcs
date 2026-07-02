# Rust API

The `odcs` crate provides parsing, validation, and inspection for ODCS v3.1.0 documents. Generated API docs: [docs.rs/odcs](https://docs.rs/odcs).

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
odcs = "0.4"
```

For library-only use (no CLI binary):

```toml
odcs = { version = "0.4", default-features = false }
```

See [installation.md](installation.md) for `cargo install` and from-source setup.

## Quick start

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

## Parsing

### `parse(content, format)`

Parse from bytes. Returns a `ParseResult` with optional `contract` and parse-time `report`.

```rust
use odcs::{parse, DocumentFormat};

let result = parse(yaml_bytes, DocumentFormat::Yaml);
```

### `parse_file(path)`

Parse from a file path. Infers format from `.yaml`, `.yml`, or `.json` extension.

```rust
use odcs::parse_file;

let result = parse_file("contract.yaml")?;
```

### `DataContract` helpers

```rust
use odcs::DataContract;

let result = DataContract::from_yaml(yaml_text);
let result = DataContract::from_json(json_text);
let result = DataContract::from_file("contract.yaml")?;
```

## ParseResult

```rust
let result = parse(content, DocumentFormat::Yaml);

// Contract only when parse and validation both succeed
let contract = result.into_contract()?;

// Validate without consuming (merges parse + validation diagnostics)
let report = result.validate();

// Parse-time diagnostics only
let parse_ok = result.report.is_valid();
```

`into_contract()` runs validation and returns `Err(DiagnosticReport)` when parse or validation fails.

## Validation

### `validate(contract)`

Validate a parsed `DataContract`. Since 0.4.0, default validation includes pinned ODCS v3.1.0 JSON Schema checks.

```rust
use odcs::validate;

let report = validate(&contract);
if !report.is_valid() {
    for d in &report.diagnostics {
        eprintln!("{}: {}", d.id, d.message);
    }
}
```

### `parse_and_validate(content, format)`

Parse and validate in one step. Returns a `ValidationReport`.

```rust
use odcs::{parse_and_validate, DocumentFormat};

let report = parse_and_validate(yaml_bytes, DocumentFormat::Yaml);
assert!(report.is_valid());
```

### `parse_strict(content, format)`

Parse and validate shortcut. Returns `Result<DataContract, DiagnosticReport>`. Unknown fields are rejected during serde deserialization (not a separate strict mode).

```rust
use odcs::{parse_strict, DocumentFormat};

let contract = parse_strict(yaml_bytes, DocumentFormat::Yaml)?;
```

### Deprecated aliases (0.4.0+)

`validate_strict()`, `validate_with_options()`, and `ValidationOptions::strict()` are retained for compatibility but have no additional effect — JSON Schema validation always runs.

## Inspection

```rust
use odcs::inspect_contract;

let summary = inspect_contract(&contract);
```

Use the CLI `odcs inspect` for JSON output fields (`id`, `schemaCount`, `qualityCount`, etc.).

## Key types

| Type | Purpose |
|------|---------|
| `DataContract` | Root canonical object model |
| `ParseResult` | `{ contract, report }` from parsing |
| `DiagnosticReport` / `ValidationReport` | Collection of `Diagnostic` records |
| `Diagnostic` | Error/warning with `id`, `severity`, `message`, `object_ref`, … |
| `DocumentFormat` | `Yaml` or `Json` |

## Constants and codes

```rust
use odcs::UPSTREAM_SPEC_VERSION; // "3.1.0"
use odcs::codes;                 // odcs::codes::INVALID_KIND, etc.
```

See [diagnostics.md](diagnostics.md) for the full code table.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `cli` | yes | `odcs` binary |
| `python` | no | PyO3 bindings (used by maturin) |

## Error handling pattern

```rust
use odcs::{parse_file, validate};

fn main() -> miette::Result<()> {
    let result = parse_file("contract.yaml")?;
    let contract = match result.into_contract() {
        Ok(c) => c,
        Err(report) => {
            for d in &report.diagnostics {
                eprintln!("{}: {}", d.id, d.message);
            }
            std::process::exit(1);
        }
    };
    let report = validate(&contract);
    if !report.is_valid() {
        std::process::exit(1);
    }
    Ok(())
}
```

## Limits

- Maximum document size: 16 MiB (`MAX_PARSE_BYTES`)
- Supported formats: YAML (`.yaml`, `.yml`) and JSON (`.json`)

## Python equivalent

See [python.md](python.md). Maintainer-oriented API notes: [../implementation/public-api.md](../implementation/public-api.md).

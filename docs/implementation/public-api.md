# Public API

Rust API reference for the `odcs` crate. Generated API docs: [docs.rs/odcs](https://docs.rs/odcs).

User guides: [../user/getting-started.md](../user/getting-started.md).

## Parse and validate

```rust
use odcs::{parse, parse_file, parse_and_validate, validate, DocumentFormat};

// From bytes
let result = parse(yaml_bytes, DocumentFormat::Yaml);

// From file path
let result = parse_file("contract.odcs.yaml")?;

// Parse + validate in one step
let report = parse_and_validate(yaml_bytes, DocumentFormat::Yaml);
assert!(report.is_valid());
```

## ParseResult

```rust
let result = parse(content, DocumentFormat::Yaml);

// Parsed contract (fails if parse or validation errors)
let contract = result.into_contract()?;

// Validation only (consumes result)
let report = result.validate();

// Parse report only
let parse_ok = result.report.is_valid();
```

`into_contract()` requires both successful parsing **and** validation.

## Validate a contract

```rust
use odcs::validate;

let report = validate(&contract);
if !report.is_valid() {
    for d in &report.diagnostics {
        eprintln!("{}: {}", d.id, d.message);
    }
}
```

## DataContract helpers

```rust
use odcs::DataContract;

let result = DataContract::from_yaml(yaml_text);
let result = DataContract::from_json(json_text);
let result = DataContract::from_file("contract.odcs.yaml")?;
```

## Key types

| Type | Purpose |
|------|---------|
| `DataContract` | Root canonical object model |
| `ParseResult` | `{ contract, report }` from parsing |
| `DiagnosticReport` | Collection of `Diagnostic` records |
| `Diagnostic` | Single error/warning with `id`, `severity`, `message`, … |
| `DocumentFormat` | `Yaml` or `Json` |

## Diagnostic codes

Exported as `odcs::codes::*`. See [../user/diagnostics.md](../user/diagnostics.md).

## Constants

```rust
use odcs::UPSTREAM_SPEC_VERSION; // "3.1.0"
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `cli` | yes | `odcs` binary |
| `python` | no | PyO3 bindings (used by maturin) |

```toml
odcs = { version = "0.3", default-features = false }
```

## Python equivalent

See [../user/python.md](../user/python.md).

Keep this API parallel to `dtcs` where practical.

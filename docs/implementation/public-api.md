# Public API

Rust API reference for the `odcs` crate. Generated API docs: [docs.rs/odcs](https://docs.rs/odcs).

Stability guarantees: [api-stability.md](api-stability.md). User guides: [../user/rust.md](../user/rust.md) · [API decision guide](../user/api-guide.md).

## Stable entry points (1.0+)

Use these from application code:

| Category | Symbols |
|----------|---------|
| Parse | `parse`, `parse_file`, `parse_yaml`, `parse_json`, `parse_strict`, `DocumentFormat`, `ParseResult`, `MAX_PARSE_BYTES` |
| Validate | `validate`, `parse_and_validate`, `validate_set`, `validate_with_contract_index` |
| Model | `DataContract` |
| Diagnostics | `Diagnostic`, `DiagnosticReport`, `codes`, `ValidationPhase`, `inspect_contract` |
| Multi-document | `ContractSet`, `load_set`, `load_set_with_registry`, `parse_and_validate_set`, `parse_and_validate_set_with_registry` |
| Registry | `Registry`, `RegistryEntry`, `index_registry`, `index_and_save_registry`, `load_registry` |
| Compatibility | `diff`, `CompatibilityReport`, `ChangeKind` |
| Constants | `UPSTREAM_SPEC_VERSION` |

Internal modules (`parser`, `validation`, `model`, …) are `#[doc(hidden)]` and not covered by semver.

## Parse and validate

```rust
use odcs::{parse, parse_file, parse_and_validate, validate, DocumentFormat};

let result = parse(yaml_bytes, DocumentFormat::Yaml);
let result = parse_file("contract.odcs.yaml")?;
let report = parse_and_validate(yaml_bytes, DocumentFormat::Yaml);
assert!(report.is_valid());
```

## ParseResult

```rust
let result = parse(content, DocumentFormat::Yaml);
let contract = result.into_contract()?;  // parse + validate required
let report = result.validate();
```

## Validate a contract

```rust
use odcs::validate;

let report = validate(&contract);
```

JSON Schema validation against the pinned ODCS v3.1.0 schema always runs in `validate()`.

## `parse_strict`

```rust
use odcs::{parse_strict, DocumentFormat};

let contract = parse_strict(yaml_bytes, DocumentFormat::Yaml)?;
```

Convenience for parse + validate + typed `DataContract`. Unrelated to the removed 0.x `--strict` CLI flag.

## DataContract helpers

```rust
use odcs::DataContract;

let result = DataContract::from_yaml(yaml_text);
let result = DataContract::from_json(json_text);
let result = DataContract::from_file("contract.odcs.yaml")?;
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `cli` | yes | `odcs` binary |
| `python` | no | PyO3 bindings (used by maturin) |

```toml
odcs = { version = "1.0", default-features = false }
```

## Python equivalent

See [../user/python.md](../user/python.md).

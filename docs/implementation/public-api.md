# Public API

Initial Rust API:

```rust
use odcs::{parse_file, validate, DocumentFormat};

let result = parse_file("contract.odcs.yaml")?;
let contract = result.into_contract()?;
let report = validate(&contract);

assert!(report.is_valid());
```

Parse and validate in one step:

```rust
use odcs::{parse_and_validate, DocumentFormat};

let report = parse_and_validate(content.as_bytes(), DocumentFormat::Yaml);
assert!(report.is_valid());
```

Alternative helpers on `DataContract`:

```rust
use odcs::DataContract;

let result = DataContract::from_yaml(yaml_text);
let result = DataContract::from_json(json_text);
let result = DataContract::from_file("contract.odcs.yaml")?;
```

Keep this API parallel to `dtcs` where practical.

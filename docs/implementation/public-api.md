# Public API Draft

Initial Rust API:

```rust
use odcs::{parse_yaml_file, validate};

let contract = parse_yaml_file("contract.odcs.yaml")?;
let report = validate(&contract);

assert!(report.is_valid());
```

Alternative object-oriented API:

```rust
use odcs::DataContract;

let contract = DataContract::from_yaml_file("contract.odcs.yaml")?;
let report = contract.validate();
```

Keep this API parallel to `dtcs` where practical.

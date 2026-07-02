# Recommended Rust Dependencies

Current dependencies in [`Cargo.toml`](../../Cargo.toml):

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
miette = { version = "7", features = ["fancy"] }
semver = { version = "1", features = ["serde"] }
indexmap = { version = "2", features = ["serde"] }
clap = { version = "4", features = ["derive"], optional = true }
pyo3 = { version = "0.23", optional = true, features = ["extension-module", "abi3-py39"] }

[dev-dependencies]
jsonschema = "0.29"
```

Optional later:

```toml
schemars = "0.8"
thiserror = "2"
url = { version = "2", features = ["serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

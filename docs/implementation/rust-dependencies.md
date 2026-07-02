# Recommended Rust Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
thiserror = "2"
miette = { version = "7", features = ["fancy"] }
semver = { version = "1", features = ["serde"] }
indexmap = { version = "2", features = ["serde"] }
clap = { version = "4", features = ["derive"] }
url = { version = "2", features = ["serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

Optional later:

```toml
pyo3 = { version = "0.22", features = ["extension-module"] }
schemars = "0.8"
jsonschema = "0.18"
```

//! Pinned upstream ODCS JSON Schema assets.

use std::sync::OnceLock;

use serde_json::Value;

/// Upstream ODCS GitHub repository URL.
pub const UPSTREAM_REPOSITORY_URL: &str = "https://github.com/bitol-io/open-data-contract-standard";

/// Pinned ODCS v3.1.0 JSON Schema embedded at compile time.
pub const PINNED_SCHEMA_JSON: &str = include_str!("../schema/odcs-v3.1.0.json");

/// Returns the pinned JSON Schema as a parsed [`Value`].
#[must_use]
pub fn pinned_schema_value() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        serde_json::from_str(PINNED_SCHEMA_JSON).expect("pinned JSON Schema must be valid JSON")
    })
}

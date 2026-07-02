//! Root data contract document.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{QualityRule, SchemaObject};

/// Supported upstream ODCS specification versions for this implementation.
pub const SUPPORTED_ODCS_VERSIONS: &[&str] = &["3.1.0"];

/// An ODCS Data Contract — the canonical root object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataContract {
    /// ODCS specification version.
    pub version: String,
    /// Document kind (typically `DataContract`).
    pub kind: String,
    /// Human-readable contract name.
    pub name: String,
    /// Contract lifecycle status.
    pub status: String,
    /// Schema objects describing datasets.
    #[serde(default)]
    pub schema: Vec<SchemaObject>,
    /// Data quality rules.
    #[serde(default)]
    pub quality: Vec<QualityRule>,
    /// Extension fields not yet modeled explicitly.
    #[serde(flatten)]
    pub extensions: IndexMap<String, Value>,
}

impl DataContract {
    /// Returns `true` when the contract version is supported by this crate.
    #[must_use]
    pub fn is_supported_version(&self) -> bool {
        SUPPORTED_ODCS_VERSIONS.contains(&self.version.as_str())
    }
}

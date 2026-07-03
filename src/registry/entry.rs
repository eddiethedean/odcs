//! Registry index entry types.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single contract entry in a local registry index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    /// Stable contract identifier.
    pub id: String,
    /// Contract revision (`version` field).
    pub version: String,
    /// Path relative to the registry root directory.
    pub path: PathBuf,
    /// ODCS API version.
    pub api_version: String,
    /// Optional root tags copied from the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// SHA-256 hex digest of the indexed file bytes.
    pub content_hash: String,
    /// ISO 8601 timestamp when the entry was indexed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<String>,
}

/// On-disk registry index file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegistryIndexFile {
    pub registry_version: String,
    pub entries: Vec<RegistryEntry>,
}

impl RegistryIndexFile {
    pub fn new(entries: Vec<RegistryEntry>) -> Self {
        Self {
            registry_version: "1".to_string(),
            entries,
        }
    }
}

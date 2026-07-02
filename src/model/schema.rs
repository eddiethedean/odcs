//! Schema object types.

use serde::{Deserialize, Serialize};

use super::Field;

/// A schema object describing a dataset within a data contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaObject {
    /// Logical schema name.
    pub name: String,
    /// Physical table or dataset name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_name: Option<String>,
    /// Field definitions.
    #[serde(default)]
    pub properties: Vec<Field>,
}

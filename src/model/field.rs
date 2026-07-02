//! Field types within schema objects.

use serde::{Deserialize, Serialize};

/// A field definition within a schema object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    /// Field name.
    pub name: String,
    /// Logical data type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type: Option<String>,
    /// Whether the field is required.
    #[serde(default)]
    pub required: bool,
    /// Whether the field values must be unique.
    #[serde(default)]
    pub unique: bool,
}

//! Schema object and property types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::quality::DataQualityChecks;
use super::relationships::{RelationshipPropertyLevel, RelationshipSchemaLevel};
use super::shared::SchemaElement;

/// A schema object describing a dataset within a data contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaObject {
    #[serde(flatten)]
    pub element: SchemaElement,
    /// Logical type for schema objects is always `object`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_granularity_description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<SchemaProperty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<RelationshipSchemaLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<DataQualityChecks>,
}

/// A field definition within a schema object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaProperty {
    #[serde(flatten)]
    pub element: SchemaElement,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default = "default_negative_one")]
    pub primary_key_position: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type_options: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_name: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub unique: bool,
    #[serde(default)]
    pub partitioned: bool,
    #[serde(default = "default_negative_one")]
    pub partition_key_position: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transform_source_objects: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_logic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Value>,
    #[serde(default)]
    pub critical_data_element: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<RelationshipPropertyLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<DataQualityChecks>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<SchemaProperty>,
}

const fn default_negative_one() -> i32 {
    -1
}

impl SchemaObject {
    /// Returns the schema object name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.element.name.as_deref()
    }
}

impl SchemaProperty {
    /// Returns the property name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.element.name.as_deref()
    }
}

// Backward-compatible alias used by earlier phases.
pub type Field = SchemaProperty;

//! Schema object and property types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::quality::DataQualityChecks;
use super::relationships::{RelationshipPropertyLevel, RelationshipSchemaLevel};
use super::shared::SchemaElement;

/// A schema object describing a dataset within a data contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SchemaProperty {
    #[serde(flatten)]
    pub element: SchemaElement,
    #[serde(default, skip_serializing_if = "is_false")]
    pub primary_key: bool,
    #[serde(
        default = "default_negative_one",
        skip_serializing_if = "is_default_position"
    )]
    pub primary_key_position: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_type_options: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_name: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub partitioned: bool,
    #[serde(
        default = "default_negative_one",
        skip_serializing_if = "is_default_position"
    )]
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
    #[serde(default, skip_serializing_if = "is_false")]
    pub critical_data_element: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<RelationshipPropertyLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<DataQualityChecks>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<SchemaProperty>,
    /// Item schema for `logicalType: array` properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SchemaProperty>>,
}

const fn default_negative_one() -> i32 {
    -1
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_default_position(value: &i32) -> bool {
    *value == -1
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

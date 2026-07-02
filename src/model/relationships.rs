//! Schema relationship types.

use serde::{Deserialize, Serialize};

use super::shared::CustomProperties;

/// Shared relationship fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

/// Relationship at schema-object level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipSchemaLevel {
    #[serde(flatten)]
    pub base: RelationshipBase,
    pub from: RelationshipEndpoint,
    pub to: RelationshipEndpoint,
}

/// Relationship at property level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipPropertyLevel {
    #[serde(flatten)]
    pub base: RelationshipBase,
    pub to: RelationshipEndpoint,
}

/// Relationship endpoint (single column or composite key).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelationshipEndpoint {
    /// Single column reference.
    Single(String),
    /// Composite key reference.
    Composite(Vec<String>),
}

//! Shared types referenced across ODCS sections.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stable technical identifier (`StableId` in the upstream schema).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StableId(pub String);

/// Tag list used throughout ODCS documents.
pub type Tags = Vec<String>;

/// Custom property key/value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomProperty {
    /// Stable identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    /// Property name.
    pub property: String,
    /// Property value.
    pub value: Value,
    /// Human-readable description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// List of custom properties.
pub type CustomProperties = Vec<CustomProperty>;

/// Link to an authoritative external definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthoritativeDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    pub url: String,
    #[serde(rename = "type")]
    pub definition_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// List of authoritative definition links.
pub type AuthoritativeDefinitions = Vec<AuthoritativeDefinition>;

/// High-level contract description object at the document root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContractDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limitations: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

/// Common element metadata shared by schema objects and properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SchemaElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

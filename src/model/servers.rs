//! Server types.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::roles::Role;
use super::shared::{CustomProperties, StableId};

/// Data source details for where data is physically stored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
    /// Server-type-specific configuration properties.
    #[serde(default, flatten)]
    pub details: IndexMap<String, Value>,
}

//! Role types.

use serde::{Deserialize, Serialize};

use super::shared::{CustomProperties, StableId};

/// IAM role providing access to a dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Role {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_level_approvers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_level_approvers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

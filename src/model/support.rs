//! Support channel types.

use serde::{Deserialize, Serialize};

use super::shared::{CustomProperties, StableId};

/// A support channel entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

/// Support channels at the contract root.
pub type Support = Vec<SupportItem>;

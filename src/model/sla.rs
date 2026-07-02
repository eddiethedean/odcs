//! SLA property types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::shared::StableId;

/// A service level agreement property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceLevelAgreementProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    pub property: String,
    pub value: SlaValue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_ext: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

/// SLA value variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlaValue {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Null,
}

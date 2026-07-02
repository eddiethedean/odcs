//! Data quality rule types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::shared::{AuthoritativeDefinitions, CustomProperties, StableId, Tags};

/// Data quality checks attached to schema objects or properties.
pub type DataQualityChecks = Vec<DataQuality>;

/// A single data quality rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataQuality {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_impact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Quality check type: `text`, `library`, `sql`, or `custom`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Library metric (v3.1.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    /// Deprecated library rule name (pre-v3.1 compatibility in documents).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    /// SQL query for `type: sql` rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Custom engine for `type: custom` rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_not_be: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be_greater_than: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be_greater_or_equal_to: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be_less_than: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be_less_or_equal_to: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_be_between: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_not_be_between: Option<Vec<f64>>,
}

//! Data quality rule types.

use serde::{Deserialize, Serialize};

/// A data quality rule attached to a data contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityRule {
    /// Rule name.
    pub name: String,
    /// Rule implementation type.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// Rule identifier or expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Target field for the rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

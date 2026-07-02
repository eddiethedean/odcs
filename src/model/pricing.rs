//! Pricing types.

use serde::{Deserialize, Serialize};

use super::shared::StableId;

/// Pricing information for a data contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pricing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
}

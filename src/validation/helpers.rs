//! Shared validation helpers.

use crate::model::DataQuality;
use serde_json::Value;

const RULE_TYPES: &[&str] = &["text", "library", "sql", "custom"];

/// Returns `true` when an optional string is absent or empty.
#[must_use]
pub(crate) fn is_blank(opt: &Option<String>) -> bool {
    opt.as_ref().map_or(true, |s| s.is_empty())
}

/// Returns `true` when a JSON value is absent or an empty object/array/null.
#[must_use]
pub(crate) fn is_empty_value(value: &Option<Value>) -> bool {
    match value {
        None => true,
        Some(Value::Null) => true,
        Some(Value::Object(map)) => map.is_empty(),
        Some(Value::Array(items)) => items.is_empty(),
        Some(_) => false,
    }
}

/// Resolves the effective quality rule type, inferring `library` from metric presence.
#[must_use]
pub(crate) fn effective_rule_type(rule: &DataQuality) -> Option<&str> {
    if let Some(rule_type) = rule.rule_type.as_deref() {
        return Some(rule_type);
    }
    if rule.metric.is_some() || rule.rule.is_some() {
        return Some("library");
    }
    if rule.query.is_some() {
        return Some("sql");
    }
    if rule.engine.is_some() || rule.implementation.is_some() {
        return Some("custom");
    }
    None
}

/// Returns the normalized (lowercase) rule type if known.
#[must_use]
pub(crate) fn normalized_rule_type(rule: &DataQuality) -> Option<String> {
    effective_rule_type(rule).map(|t| t.to_ascii_lowercase())
}

/// Returns `true` when the type string is a supported quality rule type (case-insensitive).
#[must_use]
pub(crate) fn is_known_rule_type(rule_type: &str) -> bool {
    RULE_TYPES
        .iter()
        .any(|known| known.eq_ignore_ascii_case(rule_type))
}

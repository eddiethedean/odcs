//! Contract compatibility analysis.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::diagnostics::codes;
use crate::model::{
    DataContract, DataQuality, RelationshipEndpoint, RelationshipSchemaLevel, SchemaObject,
    SchemaProperty,
};

/// Classification of a contract change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeKind {
    /// Breaking change for consumers.
    Breaking,
    /// Additive, backward-compatible change.
    Additive,
    /// Deprecated surface.
    Deprecated,
    /// No effective change.
    Unchanged,
}

impl ChangeKind {
    fn code(self) -> &'static str {
        match self {
            Self::Breaking => codes::COMPATIBILITY_BREAKING,
            Self::Additive => codes::COMPATIBILITY_ADDITIVE,
            Self::Deprecated => codes::COMPATIBILITY_DEPRECATED,
            Self::Unchanged => codes::COMPATIBILITY_UNCHANGED,
        }
    }
}

/// A single classified contract difference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityChange {
    /// Change classification.
    pub kind: ChangeKind,
    /// Dotted object reference path.
    pub path: String,
    /// Human-readable description.
    pub message: String,
    /// Stable diagnostic code.
    pub code: String,
}

impl CompatibilityChange {
    fn new(kind: ChangeKind, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: kind.code().to_string(),
            kind,
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Structured compatibility report for two contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityReport {
    /// Classified changes.
    pub changes: Vec<CompatibilityChange>,
    /// Whether any breaking changes were detected.
    pub has_breaking: bool,
}

impl CompatibilityReport {
    /// Returns `true` when no breaking changes were detected.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        !self.has_breaking
    }

    /// Returns breaking changes only.
    #[must_use]
    pub fn breaking_changes(&self) -> Vec<&CompatibilityChange> {
        self.changes
            .iter()
            .filter(|change| change.kind == ChangeKind::Breaking)
            .collect()
    }
}

/// Compare two parsed contracts and produce a compatibility report.
#[must_use]
pub fn diff(old: &DataContract, new: &DataContract) -> CompatibilityReport {
    let mut changes = Vec::new();

    if old.version != new.version {
        changes.push(CompatibilityChange::new(
            ChangeKind::Unchanged,
            "version",
            format!(
                "contract version changed from '{}' to '{}'",
                old.version, new.version
            ),
        ));
    }
    if old.status != new.status {
        changes.push(CompatibilityChange::new(
            ChangeKind::Unchanged,
            "status",
            format!(
                "contract status changed from '{}' to '{}'",
                old.status, new.status
            ),
        ));
    }
    if old.id != new.id {
        changes.push(CompatibilityChange::new(
            ChangeKind::Unchanged,
            "id",
            format!("contract id changed from '{}' to '{}'", old.id, new.id),
        ));
    }

    diff_schemas(&mut changes, old, new);

    let has_breaking = changes
        .iter()
        .any(|change| change.kind == ChangeKind::Breaking);

    CompatibilityReport {
        changes,
        has_breaking,
    }
}

fn diff_schemas(changes: &mut Vec<CompatibilityChange>, old: &DataContract, new: &DataContract) {
    let old_schemas = schema_map(old);
    let new_schemas = schema_map(new);

    for name in old_schemas.keys() {
        if !new_schemas.contains_key(name.as_str()) {
            changes.push(CompatibilityChange::new(
                ChangeKind::Breaking,
                format!("schema[{name}]"),
                format!("schema object '{name}' was removed"),
            ));
        }
    }

    for (name, new_schema) in &new_schemas {
        let base = format!("schema[{name}]");
        match old_schemas.get(name) {
            None => {
                changes.push(CompatibilityChange::new(
                    ChangeKind::Additive,
                    base,
                    format!("schema object '{name}' was added"),
                ));
            }
            Some(old_schema) => {
                diff_properties(
                    changes,
                    old_schema,
                    new_schema,
                    &format!("{base}.properties"),
                );
                diff_schema_quality(changes, old_schema, new_schema, &base);
                diff_schema_relationships(changes, old_schema, new_schema, &base);
            }
        }
    }
}

fn schema_map(contract: &DataContract) -> HashMap<String, &SchemaObject> {
    contract
        .schema
        .iter()
        .filter_map(|schema| {
            schema
                .element
                .name
                .as_deref()
                .map(|name| (name.to_string(), schema))
        })
        .collect()
}

fn diff_properties(
    changes: &mut Vec<CompatibilityChange>,
    old_schema: &SchemaObject,
    new_schema: &SchemaObject,
    base: &str,
) {
    let old_props = property_map(&old_schema.properties);
    let new_props = property_map(&new_schema.properties);

    for name in old_props.keys() {
        if !new_props.contains_key(name.as_str()) {
            changes.push(CompatibilityChange::new(
                ChangeKind::Breaking,
                format!("{base}[{name}]"),
                format!("property '{name}' was removed"),
            ));
        }
    }

    for (name, new_prop) in &new_props {
        let path = format!("{base}[{name}]");
        match old_props.get(name) {
            None => {
                let kind = if new_prop.required {
                    ChangeKind::Breaking
                } else {
                    ChangeKind::Additive
                };
                changes.push(CompatibilityChange::new(
                    kind,
                    path,
                    format!("property '{name}' was added"),
                ));
            }
            Some(old_prop) => {
                diff_property_details(changes, old_prop, new_prop, &path);
            }
        }
    }
}

fn property_map(properties: &[SchemaProperty]) -> HashMap<String, &SchemaProperty> {
    properties
        .iter()
        .filter_map(|property| {
            property
                .element
                .name
                .as_deref()
                .map(|name| (name.to_string(), property))
        })
        .collect()
}

fn diff_property_details(
    changes: &mut Vec<CompatibilityChange>,
    old_prop: &SchemaProperty,
    new_prop: &SchemaProperty,
    path: &str,
) {
    if old_prop.logical_type != new_prop.logical_type {
        changes.push(CompatibilityChange::new(
            ChangeKind::Breaking,
            format!("{path}.logicalType"),
            format!(
                "logicalType changed from {:?} to {:?}",
                old_prop.logical_type, new_prop.logical_type
            ),
        ));
    }

    if !old_prop.required && new_prop.required {
        changes.push(CompatibilityChange::new(
            ChangeKind::Breaking,
            format!("{path}.required"),
            "property became required".to_string(),
        ));
    } else if old_prop.required && !new_prop.required {
        changes.push(CompatibilityChange::new(
            ChangeKind::Additive,
            format!("{path}.required"),
            "property is no longer required".to_string(),
        ));
    }
}

fn diff_schema_quality(
    changes: &mut Vec<CompatibilityChange>,
    old_schema: &SchemaObject,
    new_schema: &SchemaObject,
    base: &str,
) {
    let old_rules = quality_map(old_schema.quality.as_deref());
    let new_rules = quality_map(new_schema.quality.as_deref());
    diff_quality_rules(changes, &old_rules, &new_rules, &format!("{base}.quality"));
}

fn quality_map(rules: Option<&[DataQuality]>) -> HashMap<String, &DataQuality> {
    rules
        .unwrap_or_default()
        .iter()
        .filter_map(|rule| rule.name.as_ref().map(|name| (name.clone(), rule)))
        .collect()
}

fn diff_quality_rules(
    changes: &mut Vec<CompatibilityChange>,
    old_rules: &HashMap<String, &DataQuality>,
    new_rules: &HashMap<String, &DataQuality>,
    base: &str,
) {
    for name in old_rules.keys() {
        if !new_rules.contains_key(name.as_str()) {
            changes.push(CompatibilityChange::new(
                ChangeKind::Breaking,
                format!("{base}[{name}]"),
                format!("quality rule '{name}' was removed"),
            ));
        }
    }

    for (name, new_rule) in new_rules {
        let path = format!("{base}[{name}]");
        match old_rules.get(name) {
            None => {
                changes.push(CompatibilityChange::new(
                    ChangeKind::Additive,
                    path,
                    format!("quality rule '{name}' was added"),
                ));
            }
            Some(old_rule) => {
                if old_rule.metric != new_rule.metric {
                    changes.push(CompatibilityChange::new(
                        ChangeKind::Breaking,
                        format!("{path}.metric"),
                        format!(
                            "quality metric changed from {:?} to {:?}",
                            old_rule.metric, new_rule.metric
                        ),
                    ));
                }
                if old_rule.rule_type != new_rule.rule_type {
                    changes.push(CompatibilityChange::new(
                        ChangeKind::Breaking,
                        format!("{path}.type"),
                        format!(
                            "quality type changed from {:?} to {:?}",
                            old_rule.rule_type, new_rule.rule_type
                        ),
                    ));
                }
            }
        }
    }
}

fn diff_schema_relationships(
    changes: &mut Vec<CompatibilityChange>,
    old_schema: &SchemaObject,
    new_schema: &SchemaObject,
    base: &str,
) {
    let old_keys = relationship_keys(&old_schema.relationships);
    let new_keys = relationship_keys(&new_schema.relationships);
    let base = format!("{base}.relationships");

    for key in &old_keys {
        if !new_keys.contains(key) {
            changes.push(CompatibilityChange::new(
                ChangeKind::Breaking,
                base.clone(),
                format!("relationship '{key}' was removed"),
            ));
        }
    }

    for key in &new_keys {
        if !old_keys.contains(key) {
            changes.push(CompatibilityChange::new(
                ChangeKind::Additive,
                base.clone(),
                format!("relationship '{key}' was added"),
            ));
        } else {
            let old_rel = find_relationship(&old_schema.relationships, key);
            let new_rel = find_relationship(&new_schema.relationships, key);
            if let (Some(old_rel), Some(new_rel)) = (old_rel, new_rel) {
                if endpoint_key(&old_rel.to) != endpoint_key(&new_rel.to) {
                    changes.push(CompatibilityChange::new(
                        ChangeKind::Breaking,
                        format!("{base}[{key}].to"),
                        "relationship target endpoint changed".to_string(),
                    ));
                }
            }
        }
    }
}

fn relationship_keys(relationships: &[RelationshipSchemaLevel]) -> HashSet<String> {
    relationships
        .iter()
        .map(|relationship| {
            format!(
                "{}->{}:{}",
                endpoint_key(&relationship.from),
                endpoint_key(&relationship.to),
                relationship
                    .base
                    .relationship_type
                    .as_deref()
                    .unwrap_or("foreignKey")
            )
        })
        .collect()
}

fn find_relationship<'a>(
    relationships: &'a [RelationshipSchemaLevel],
    key: &str,
) -> Option<&'a RelationshipSchemaLevel> {
    relationships
        .iter()
        .find(|relationship| relationship_key(relationship) == key)
}

fn relationship_key(relationship: &RelationshipSchemaLevel) -> String {
    format!(
        "{}->{}:{}",
        endpoint_key(&relationship.from),
        endpoint_key(&relationship.to),
        relationship
            .base
            .relationship_type
            .as_deref()
            .unwrap_or("foreignKey")
    )
}

fn endpoint_key(endpoint: &RelationshipEndpoint) -> String {
    match endpoint {
        RelationshipEndpoint::Single(value) => value.clone(),
        RelationshipEndpoint::Composite(values) => values.join(","),
    }
}

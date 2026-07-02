//! Reference validation.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use regex::Regex;

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, RelationshipEndpoint, RelationshipSchemaLevel, SchemaProperty};

fn shorthand_reference_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*$")
            .expect("valid shorthand reference regex")
    })
}

fn fully_qualified_reference_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"^(?:(?:https?://)?[A-Za-z0-9._\-/]+\.yaml#)?/?[A-Za-z_][A-Za-z0-9_]*/[A-Za-z0-9_-]+(?:/[A-Za-z_][A-Za-z0-9_]*/[A-Za-z0-9_-]+)*$",
        )
        .expect("valid fully qualified reference regex")
    })
}

/// Validate relationship and reference constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    let index = SchemaIndex::build(contract);

    for (schema_index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{schema_index}]");
        for (rel_index, relationship) in schema.relationships.iter().enumerate() {
            validate_schema_relationship(
                &mut report,
                relationship,
                &format!("{base_ref}.relationships[{rel_index}]"),
                &index,
            );
        }
        validate_property_relationships(&mut report, &schema.properties, &base_ref, &index);
    }

    report
}

struct SchemaIndex {
    objects: HashMap<String, HashSet<String>>,
}

impl SchemaIndex {
    fn build(contract: &DataContract) -> Self {
        let mut objects = HashMap::new();
        for schema in &contract.schema {
            if let Some(name) = schema.element.name.as_deref() {
                if !name.is_empty() {
                    let mut properties = HashSet::new();
                    collect_property_names(&schema.properties, &mut properties);
                    objects.insert(name.to_string(), properties);
                }
            }
        }
        Self { objects }
    }

    fn resolve_shorthand(&self, reference: &str) -> bool {
        let Some((table, column)) = reference.split_once('.') else {
            return false;
        };
        self.objects
            .get(table)
            .is_some_and(|properties| properties.contains(column))
    }
}

fn collect_property_names(properties: &[SchemaProperty], out: &mut HashSet<String>) {
    for property in properties {
        if let Some(name) = property.element.name.as_deref() {
            if !name.is_empty() {
                out.insert(name.to_string());
            }
        }
        collect_property_names(&property.properties, out);
        if let Some(items) = &property.items {
            collect_property_names(std::slice::from_ref(items), out);
        }
    }
}

fn endpoint_is_invalid(endpoint: &RelationshipEndpoint) -> bool {
    match endpoint {
        RelationshipEndpoint::Single(value) => value.is_empty(),
        RelationshipEndpoint::Composite(values) => {
            values.is_empty() || values.iter().any(|value| value.is_empty())
        }
    }
}

fn endpoint_values(endpoint: &RelationshipEndpoint) -> Vec<&str> {
    match endpoint {
        RelationshipEndpoint::Single(value) => vec![value.as_str()],
        RelationshipEndpoint::Composite(values) => values.iter().map(String::as_str).collect(),
    }
}

fn validate_endpoint(
    report: &mut DiagnosticReport,
    endpoint: &RelationshipEndpoint,
    object_ref: &str,
    index: &SchemaIndex,
) {
    if endpoint_is_invalid(endpoint) {
        emit(
            report,
            validation_error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                "relationship endpoint must not be empty",
            )
            .with_object_ref(object_ref.to_string()),
        );
        return;
    }

    for value in endpoint_values(endpoint) {
        if !is_valid_reference_format(value) {
            emit(
                report,
                validation_error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("relationship endpoint '{value}' is not a valid reference format"),
                )
                .with_object_ref(object_ref.to_string()),
            );
            continue;
        }

        if shorthand_reference_regex().is_match(value) && !index.resolve_shorthand(value) {
            emit(
                report,
                validation_error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("relationship endpoint '{value}' does not resolve to a known schema object and property"),
                )
                .with_object_ref(object_ref.to_string())
                .with_remediation("reference an existing schema object and property name"),
            );
        }
    }
}

fn is_valid_reference_format(value: &str) -> bool {
    shorthand_reference_regex().is_match(value) || fully_qualified_reference_regex().is_match(value)
}

fn validate_composite_parity(
    report: &mut DiagnosticReport,
    from: &RelationshipEndpoint,
    to: &RelationshipEndpoint,
    object_ref: &str,
) {
    match (from, to) {
        (
            RelationshipEndpoint::Composite(from_values),
            RelationshipEndpoint::Composite(to_values),
        ) if from_values.len() != to_values.len() => {
            emit(
                report,
                validation_error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!(
                        "composite relationship endpoints must have equal length (from: {}, to: {})",
                        from_values.len(),
                        to_values.len()
                    ),
                )
                .with_object_ref(object_ref.to_string()),
            );
        }
        _ => {}
    }
}

fn validate_schema_relationship(
    report: &mut DiagnosticReport,
    relationship: &RelationshipSchemaLevel,
    object_ref: &str,
    index: &SchemaIndex,
) {
    validate_endpoint(
        report,
        &relationship.from,
        &format!("{object_ref}.from"),
        index,
    );
    validate_endpoint(report, &relationship.to, &format!("{object_ref}.to"), index);
    validate_composite_parity(report, &relationship.from, &relationship.to, object_ref);
}

fn validate_property_relationships(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
    schema_index: &SchemaIndex,
) {
    for (prop_index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{prop_index}]");
        for (rel_index, relationship) in property.relationships.iter().enumerate() {
            validate_endpoint(
                report,
                &relationship.to,
                &format!("{prop_ref}.relationships[{rel_index}].to"),
                schema_index,
            );
        }
        if !property.properties.is_empty() {
            validate_property_relationships(report, &property.properties, &prop_ref, schema_index);
        }
        if let Some(items) = &property.items {
            validate_property_relationships(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
                schema_index,
            );
        }
    }
}

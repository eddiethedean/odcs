//! Reference validation.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, RelationshipEndpoint, RelationshipSchemaLevel, SchemaProperty};

/// Validate relationship and reference constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for (schema_index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{schema_index}]");
        for (rel_index, relationship) in schema.relationships.iter().enumerate() {
            validate_schema_relationship(
                &mut report,
                relationship,
                &format!("{base_ref}.relationships[{rel_index}]"),
            );
        }
        validate_property_relationships(&mut report, &schema.properties, &base_ref);
    }

    report
}

fn endpoint_is_empty(endpoint: &RelationshipEndpoint) -> bool {
    match endpoint {
        RelationshipEndpoint::Single(value) => value.is_empty(),
        RelationshipEndpoint::Composite(values) => values.is_empty(),
    }
}

fn validate_schema_relationship(
    report: &mut DiagnosticReport,
    relationship: &RelationshipSchemaLevel,
    object_ref: &str,
) {
    if endpoint_is_empty(&relationship.from) {
        emit(
            report,
            validation_error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                "relationship from endpoint must not be empty",
            )
            .with_object_ref(format!("{object_ref}.from")),
        );
    }
    if endpoint_is_empty(&relationship.to) {
        emit(
            report,
            validation_error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                "relationship to endpoint must not be empty",
            )
            .with_object_ref(format!("{object_ref}.to")),
        );
    }
}

fn validate_property_relationships(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        for (rel_index, relationship) in property.relationships.iter().enumerate() {
            if endpoint_is_empty(&relationship.to) {
                emit(
                    report,
                    validation_error(
                        codes::UNRESOLVED_REFERENCE,
                        DiagnosticCategory::Reference,
                        "relationship to endpoint must not be empty",
                    )
                    .with_object_ref(format!("{prop_ref}.relationships[{rel_index}].to")),
                );
            }
        }
        if !property.properties.is_empty() {
            validate_property_relationships(report, &property.properties, &prop_ref);
        }
        if let Some(items) = &property.items {
            validate_property_relationships(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

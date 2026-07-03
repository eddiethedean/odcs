//! Reference validation.

use std::sync::OnceLock;

use regex::Regex;

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::{DataContract, RelationshipEndpoint, RelationshipSchemaLevel, SchemaProperty};
use crate::validation::schema_index::{ContractIndex, SchemaIndex};

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
            r"^(?:(?:https?://)?[A-Za-z0-9._\-/]+\.yaml#)?/?[A-Za-z_][A-Za-z0-9_-]*/[A-Za-z0-9_-]+(?:/[A-Za-z0-9_-]+(?:/[A-Za-z_][A-Za-z0-9_-]*)*)*$",
        )
        .expect("valid fully qualified reference regex")
    })
}

/// Validate relationship and reference constraints within a single contract.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    validate_with_index(contract, None)
}

/// Validate relationship and reference constraints with optional cross-file index.
#[must_use]
pub fn validate_with_index(
    contract: &DataContract,
    contract_index: Option<&ContractIndex>,
) -> DiagnosticReport {
    let index = SchemaIndex::build(contract);
    let mut report = DiagnosticReport::new();

    for (schema_index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{schema_index}]");
        for (rel_index, relationship) in schema.relationships.iter().enumerate() {
            validate_schema_relationship(
                &mut report,
                relationship,
                &format!("{base_ref}.relationships[{rel_index}]"),
                &index,
                contract_index,
            );
        }
        validate_property_relationships(
            &mut report,
            &schema.properties,
            &base_ref,
            &index,
            contract_index,
        );
    }

    report
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
    contract_index: Option<&ContractIndex>,
) {
    if endpoint_is_invalid(endpoint) {
        emit(
            report,
            validation_error(
                ValidationPhase::References,
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
                    ValidationPhase::References,
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
                    ValidationPhase::References,
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("relationship endpoint '{value}' does not resolve to a known schema object and property"),
                )
                .with_object_ref(object_ref.to_string())
                .with_remediation("reference an existing schema object and property name"),
            );
            continue;
        }

        if is_fully_qualified_reference(value) {
            if let Some(contract_index) = contract_index {
                if !contract_index.resolve_fqn(value) {
                    emit(
                        report,
                        validation_error(
                            ValidationPhase::References,
                            codes::UNRESOLVED_REFERENCE,
                            DiagnosticCategory::Reference,
                            format!("relationship endpoint '{value}' does not resolve to a known contract schema object and property"),
                        )
                        .with_object_ref(object_ref.to_string())
                        .with_remediation(
                            "include the referenced contract with --dep or --include",
                        ),
                    );
                }
            }
        }
    }
}

fn is_valid_reference_format(value: &str) -> bool {
    if shorthand_reference_regex().is_match(value) {
        return true;
    }
    if fully_qualified_reference_regex().is_match(value) {
        return true;
    }
    crate::validation::schema_index::parse_fqn_triple(value).is_some()
}

fn is_fully_qualified_reference(value: &str) -> bool {
    fully_qualified_reference_regex().is_match(value)
        || crate::validation::schema_index::parse_fqn_triple(value).is_some()
}

fn validate_relationship_type(
    report: &mut DiagnosticReport,
    relationship_type: &Option<String>,
    object_ref: &str,
) {
    let Some(relationship_type) = relationship_type.as_deref() else {
        return;
    };
    if relationship_type != "foreignKey" {
        emit(
            report,
            validation_error(
                ValidationPhase::References,
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                format!(
                    "unsupported relationship type '{relationship_type}'; expected 'foreignKey'"
                ),
            )
            .with_object_ref(format!("{object_ref}.type")),
        );
    }
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
                    ValidationPhase::References,
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
    contract_index: Option<&ContractIndex>,
) {
    validate_relationship_type(report, &relationship.base.relationship_type, object_ref);
    validate_endpoint(
        report,
        &relationship.from,
        &format!("{object_ref}.from"),
        index,
        contract_index,
    );
    validate_endpoint(
        report,
        &relationship.to,
        &format!("{object_ref}.to"),
        index,
        contract_index,
    );
    validate_composite_parity(report, &relationship.from, &relationship.to, object_ref);
}

fn validate_property_relationships(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
    schema_index: &SchemaIndex,
    contract_index: Option<&ContractIndex>,
) {
    for (prop_index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{prop_index}]");
        for (rel_index, relationship) in property.relationships.iter().enumerate() {
            let rel_ref = format!("{prop_ref}.relationships[{rel_index}]");
            validate_relationship_type(report, &relationship.base.relationship_type, &rel_ref);
            validate_endpoint(
                report,
                &relationship.to,
                &format!("{prop_ref}.relationships[{rel_index}].to"),
                schema_index,
                contract_index,
            );
        }
        if !property.properties.is_empty() {
            validate_property_relationships(
                report,
                &property.properties,
                &prop_ref,
                schema_index,
                contract_index,
            );
        }
        if let Some(items) = &property.items {
            validate_property_relationships(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
                schema_index,
                contract_index,
            );
        }
    }
}

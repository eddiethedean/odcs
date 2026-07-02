//! Schema object and property validation.

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::{DataContract, SchemaObject, SchemaProperty};

const SCHEMA_OBJECT_LOGICAL_TYPES: &[&str] = &["object"];

const PROPERTY_LOGICAL_TYPES: &[&str] = &[
    "string",
    "date",
    "timestamp",
    "time",
    "number",
    "integer",
    "object",
    "array",
    "boolean",
];

/// Validate schema objects and properties.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for (index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{index}]");
        validate_schema_object(&mut report, schema, &base_ref);
    }

    report
}

fn validate_schema_object(report: &mut DiagnosticReport, schema: &SchemaObject, base_ref: &str) {
    if schema
        .element
        .name
        .as_ref()
        .map_or(true, |name| name.is_empty())
    {
        emit(
            report,
            validation_error(
                ValidationPhase::Schema,
                codes::INVALID_SCHEMA,
                DiagnosticCategory::Structure,
                "schema object name must not be empty",
            )
            .with_object_ref(format!("{base_ref}.name")),
        );
    }

    if let Some(logical_type) = schema.logical_type.as_deref() {
        if !SCHEMA_OBJECT_LOGICAL_TYPES.contains(&logical_type) {
            emit(
                report,
                validation_error(
                    ValidationPhase::Schema,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!(
                        "unsupported schema object logicalType '{logical_type}'; expected: {}",
                        SCHEMA_OBJECT_LOGICAL_TYPES.join(", ")
                    ),
                )
                .with_object_ref(format!("{base_ref}.logicalType")),
            );
        }
    }

    validate_properties(report, &schema.properties, base_ref);
}

fn validate_properties(report: &mut DiagnosticReport, properties: &[SchemaProperty], base: &str) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        if property
            .element
            .name
            .as_ref()
            .map_or(true, |name| name.is_empty())
        {
            emit(
                report,
                validation_error(
                    ValidationPhase::Schema,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "schema property name must not be empty",
                )
                .with_object_ref(format!("{prop_ref}.name")),
            );
        }

        if let Some(logical_type) = property.logical_type.as_deref() {
            if !PROPERTY_LOGICAL_TYPES.contains(&logical_type) {
                emit(
                    report,
                    validation_error(
                ValidationPhase::Schema,
                        codes::INVALID_SCHEMA,
                        DiagnosticCategory::Structure,
                        format!(
                            "unsupported property logicalType '{logical_type}'; expected one of: {}",
                            PROPERTY_LOGICAL_TYPES.join(", ")
                        ),
                    )
                    .with_object_ref(format!("{prop_ref}.logicalType")),
                );
            }
        }

        if property.logical_type.as_deref() == Some("array") && property.items.is_none() {
            emit(
                report,
                validation_error(
                    ValidationPhase::Schema,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "array properties require an items schema",
                )
                .with_object_ref(format!("{prop_ref}.items")),
            );
        }

        if property.logical_type.as_deref() == Some("object") && property.properties.is_empty() {
            emit(
                report,
                validation_error(
                    ValidationPhase::Schema,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "object properties require at least one nested property",
                )
                .with_object_ref(format!("{prop_ref}.properties")),
            );
        }

        if !property.properties.is_empty() {
            validate_properties(report, &property.properties, &prop_ref);
        }

        if let Some(items) = &property.items {
            validate_properties(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

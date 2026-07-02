//! Schema object and property validation.

use std::collections::HashSet;

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, SchemaObject, SchemaProperty};

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
                codes::INVALID_SCHEMA,
                DiagnosticCategory::Structure,
                "schema object name must not be empty",
            )
            .with_object_ref(format!("{base_ref}.name")),
        );
    }

    validate_primary_keys(report, &schema.properties, base_ref);
    validate_properties(report, &schema.properties, base_ref);
}

fn validate_primary_keys(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base_ref: &str,
) {
    let mut positions = HashSet::new();
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base_ref}.properties[{index}]");
        if property.primary_key && property.primary_key_position < 0 {
            emit(
                report,
                validation_error(
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "primary key properties require primaryKeyPosition >= 0",
                )
                .with_object_ref(format!("{prop_ref}.primaryKeyPosition")),
            );
        }
        if property.primary_key && !positions.insert(property.primary_key_position) {
            emit(
                report,
                validation_error(
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "primary key positions must be unique within a schema object",
                )
                .with_object_ref(format!("{prop_ref}.primaryKeyPosition")),
            );
        }
    }
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
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "schema property name must not be empty",
                )
                .with_object_ref(format!("{prop_ref}.name")),
            );
        }

        if property.logical_type.as_deref() == Some("array") && property.items.is_none() {
            emit(
                report,
                validation_error(
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

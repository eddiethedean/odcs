//! Schema object and property validation.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, SchemaProperty};

/// Validate schema objects and properties.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for (index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{index}]");
        if schema
            .element
            .name
            .as_ref()
            .map_or(true, |name| name.is_empty())
        {
            emit(
                &mut report,
                validation_error(
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "schema object name must not be empty",
                )
                .with_object_ref(format!("{base_ref}.name")),
            );
        }

        validate_properties(&mut report, &schema.properties, &base_ref);
    }

    report
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
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "schema property name must not be empty",
                )
                .with_object_ref(format!("{prop_ref}.name")),
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

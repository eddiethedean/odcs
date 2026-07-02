//! Extension and custom property validation.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{CustomProperties, DataContract, SchemaProperty};

/// Validate extension and custom property constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    validate_custom_properties(
        &mut report,
        contract.custom_properties.as_ref(),
        "customProperties",
    );

    if let Some(description) = &contract.description {
        validate_custom_properties(
            &mut report,
            description.custom_properties.as_ref(),
            "description.customProperties",
        );
    }

    for (index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{index}]");
        validate_custom_properties(
            &mut report,
            schema.element.custom_properties.as_ref(),
            &format!("{base_ref}.customProperties"),
        );
        validate_property_extensions(&mut report, &schema.properties, &base_ref);
    }

    report
}

fn validate_property_extensions(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        validate_custom_properties(
            report,
            property.element.custom_properties.as_ref(),
            &format!("{prop_ref}.customProperties"),
        );
        if !property.properties.is_empty() {
            validate_property_extensions(report, &property.properties, &prop_ref);
        }
        if let Some(items) = &property.items {
            validate_property_extensions(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

fn validate_custom_properties(
    report: &mut DiagnosticReport,
    properties: Option<&CustomProperties>,
    object_ref: &str,
) {
    let Some(properties) = properties else {
        return;
    };

    for (index, property) in properties.iter().enumerate() {
        if property.property.is_empty() {
            emit(
                report,
                validation_error(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Extension,
                    "custom property key must not be empty",
                )
                .with_object_ref(format!("{object_ref}[{index}].property")),
            );
        }
    }
}

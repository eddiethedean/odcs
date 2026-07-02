//! JSON Schema validation against the pinned upstream ODCS schema.

use std::sync::OnceLock;

use jsonschema::Validator;
use serde_json::Value;

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::DataContract;
use crate::schema;

fn pinned_validator() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| {
        jsonschema::validator_for(schema::pinned_schema_value())
            .expect("pinned ODCS JSON Schema must compile")
    })
}

/// Validate a contract instance against the pinned ODCS JSON Schema.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    let instance = match serde_json::to_value(contract) {
        Ok(value) => value,
        Err(error) => {
            emit(
                &mut report,
                validation_error(
                    ValidationPhase::JsonSchema,
                    codes::JSON_SCHEMA_VIOLATION,
                    DiagnosticCategory::Structure,
                    format!("failed to serialize contract for JSON Schema validation: {error}"),
                ),
            );
            return report;
        }
    };
    validate_instance(&mut report, &instance);
    report
}

fn validate_instance(report: &mut DiagnosticReport, instance: &Value) {
    let validator = pinned_validator();
    for error in validator.iter_errors(instance) {
        let object_ref = error.instance_path.to_string();
        let object_ref = if object_ref.is_empty() {
            None
        } else {
            Some(object_ref)
        };
        let mut diagnostic = validation_error(
            ValidationPhase::JsonSchema,
            codes::JSON_SCHEMA_VIOLATION,
            DiagnosticCategory::Structure,
            error.to_string(),
        );
        if let Some(object_ref) = object_ref {
            diagnostic = diagnostic.with_object_ref(object_ref);
        }
        emit(report, diagnostic);
    }
}

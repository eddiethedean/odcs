//! Structural validation.

use crate::diagnostics::{emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::DataContract;

/// Validate structural constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if contract.version == "3.1.0"
        && !contract.api_version.is_empty()
        && !contract.api_version.starts_with("v3.1")
    {
        emit(
            &mut report,
            validation_error(
                crate::diagnostics::codes::UNSUPPORTED_VERSION,
                DiagnosticCategory::Compatibility,
                format!(
                    "apiVersion '{}' is inconsistent with document version '3.1.0'",
                    contract.api_version
                ),
            )
            .with_object_ref("apiVersion")
            .with_remediation("set apiVersion to a v3.1.x release for ODCS 3.1.0 documents"),
        );
    }

    report
}

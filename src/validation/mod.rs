//! ODCS validation pipeline.

mod document;
mod extensions;
mod phases;
mod quality;
mod references;
mod schema;
mod structural;

pub use phases::ValidationPhase;

use crate::diagnostics::{
    codes, com_error, emit, validation_error, DiagnosticCategory, DiagnosticReport,
};
use crate::model::DataContract;

/// Validate a parsed data contract.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if !contract.is_supported_version() {
        emit(
            &mut report,
            com_error(
                codes::UNSUPPORTED_VERSION,
                DiagnosticCategory::Compatibility,
                format!(
                    "unsupported ODCS version '{}'; supported: {:?}",
                    contract.version,
                    crate::model::SUPPORTED_ODCS_VERSIONS
                ),
            )
            .with_object_ref("version")
            .with_remediation("set version to a supported ODCS release"),
        );
    }

    if !contract.is_supported_api_version() {
        emit(
            &mut report,
            com_error(
                codes::UNSUPPORTED_VERSION,
                DiagnosticCategory::Compatibility,
                format!(
                    "unsupported ODCS apiVersion '{}'; supported: {:?}",
                    contract.api_version,
                    crate::model::SUPPORTED_API_VERSIONS
                ),
            )
            .with_object_ref("apiVersion")
            .with_remediation("set apiVersion to a supported ODCS release"),
        );
    }

    if contract.id.is_empty() {
        emit(
            &mut report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "contract id must not be empty",
            )
            .with_object_ref("id"),
        );
    }

    if contract.api_version.is_empty() {
        emit(
            &mut report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "contract apiVersion must not be empty",
            )
            .with_object_ref("apiVersion"),
        );
    }

    if contract.kind != "DataContract" {
        emit(
            &mut report,
            validation_error(
                codes::INVALID_SCHEMA,
                DiagnosticCategory::Structure,
                format!("expected kind 'DataContract', got '{}'", contract.kind),
            )
            .with_object_ref("kind"),
        );
    }

    report
}

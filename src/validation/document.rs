//! Document-level validation.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::DataContract;

/// Validate document-level constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if contract.version.is_empty() {
        emit(
            &mut report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "contract version must not be empty",
            )
            .with_object_ref("version"),
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
    } else if !contract.is_supported_api_version() {
        emit(
            &mut report,
            validation_error(
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

    if contract.status.is_empty() {
        emit(
            &mut report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "contract status must not be empty",
            )
            .with_object_ref("status"),
        );
    }

    if contract.kind.is_empty() {
        emit(
            &mut report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "contract kind must not be empty",
            )
            .with_object_ref("kind"),
        );
    } else if contract.kind != "DataContract" {
        emit(
            &mut report,
            validation_error(
                codes::INVALID_KIND,
                DiagnosticCategory::Structure,
                format!("expected kind 'DataContract', got '{}'", contract.kind),
            )
            .with_object_ref("kind"),
        );
    }

    report
}

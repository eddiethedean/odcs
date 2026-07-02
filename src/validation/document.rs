//! Document-level validation (stub).
//!
//! Full document validation is planned for Phase 5.

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;

/// Validate document-level constraints.
#[must_use]
#[allow(dead_code)]
pub fn validate(_contract: &DataContract) -> DiagnosticReport {
    DiagnosticReport::new()
}

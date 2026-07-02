//! Structural validation.

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;

/// Validate structural constraints.
#[must_use]
pub fn validate(_contract: &DataContract) -> DiagnosticReport {
    DiagnosticReport::new()
}

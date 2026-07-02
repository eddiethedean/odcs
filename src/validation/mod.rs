//! ODCS validation pipeline.

mod document;
mod extensions;
mod phases;
mod quality;
mod references;
mod schema;
mod structural;

pub use phases::ValidationPhase;

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;

/// Validate a parsed data contract.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.merge(document::validate(contract));
    report.merge(structural::validate(contract));
    report.merge(schema::validate(contract));
    report.merge(quality::validate(contract));
    report.merge(references::validate(contract));
    report.merge(extensions::validate(contract));
    report
}

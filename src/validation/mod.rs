//! ODCS validation pipeline.

mod dedup;
mod document;
mod extensions;
mod helpers;
mod ids;
mod json_schema;
mod phases;
mod quality;
mod references;
mod schema;
mod sections;
mod servers;
mod structural;

pub use phases::ValidationPhase;

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;

/// Options controlling validation behavior.
///
/// As of 0.4.0, JSON Schema validation always runs. The `strict` flag is
/// retained for backward compatibility and has no effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationOptions {
    /// Deprecated: JSON Schema validation is always enabled in 0.4.0+.
    pub strict: bool,
}

impl ValidationOptions {
    /// Default validation options.
    #[must_use]
    pub const fn default_options() -> Self {
        Self { strict: false }
    }

    /// Deprecated alias for default options (strict mode is always on in 0.4.0+).
    #[must_use]
    pub const fn strict() -> Self {
        Self { strict: true }
    }
}

fn run_validation_pipeline(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.merge(document::validate(contract));
    report.merge(structural::validate(contract));
    report.merge(schema::validate(contract));
    report.merge(quality::validate(contract));
    report.merge(references::validate(contract));
    report.merge(extensions::validate(contract));
    report.merge(servers::validate(contract));
    report.merge(sections::validate(contract));
    report.merge(ids::validate(contract));
    report.merge(json_schema::validate(contract));
    dedup::dedup_json_schema_overlap(&mut report);
    report
}

/// Validate a parsed data contract (Rust pipeline + JSON Schema).
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    validate_with_options(contract, ValidationOptions::default_options())
}

/// Validate a parsed data contract.
#[must_use]
pub fn validate_with_options(
    contract: &DataContract,
    _options: ValidationOptions,
) -> DiagnosticReport {
    run_validation_pipeline(contract)
}

/// Validate a parsed data contract (alias for [`validate`] since 0.4.0).
#[must_use]
pub fn validate_strict(contract: &DataContract) -> DiagnosticReport {
    validate(contract)
}

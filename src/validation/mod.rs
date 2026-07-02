//! ODCS validation pipeline.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationOptions {
    /// When true, run JSON Schema validation after the Rust pipeline.
    pub strict: bool,
}

impl ValidationOptions {
    /// Default (non-strict) validation options.
    #[must_use]
    pub const fn default_options() -> Self {
        Self { strict: false }
    }

    /// Strict validation: Rust pipeline plus JSON Schema conformance.
    #[must_use]
    pub const fn strict() -> Self {
        Self { strict: true }
    }
}

/// Validate a parsed data contract with default (non-strict) options.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    validate_with_options(contract, ValidationOptions::default_options())
}

/// Validate a parsed data contract.
#[must_use]
pub fn validate_with_options(
    contract: &DataContract,
    options: ValidationOptions,
) -> DiagnosticReport {
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
    if options.strict {
        report.merge(json_schema::validate(contract));
    }
    report
}

/// Validate a parsed data contract in strict mode (Rust pipeline + JSON Schema).
#[must_use]
pub fn validate_strict(contract: &DataContract) -> DiagnosticReport {
    validate_with_options(contract, ValidationOptions::strict())
}

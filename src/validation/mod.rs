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
mod schema_index;
mod sections;
mod servers;
mod structural;

pub use phases::ValidationPhase;
pub use schema_index::{ContractIndex, SchemaIndex};

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;
use crate::validation::schema_index::ContractIndex as ContractIndexType;

fn run_validation_pipeline(
    contract: &DataContract,
    contract_index: Option<&ContractIndexType>,
) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.merge(document::validate(contract));
    report.merge(structural::validate(contract));
    report.merge(schema::validate(contract));
    report.merge(quality::validate(contract));
    if contract_index.is_some() {
        report.merge(references::validate_with_index(contract, contract_index));
    } else {
        report.merge(references::validate(contract));
    }
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
    run_validation_pipeline(contract, None)
}

/// Validate a parsed data contract with optional cross-file reference index.
#[must_use]
pub fn validate_with_contract_index(
    contract: &DataContract,
    contract_index: Option<&ContractIndexType>,
) -> DiagnosticReport {
    run_validation_pipeline(contract, contract_index)
}

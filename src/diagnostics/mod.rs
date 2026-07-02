//! ODCS diagnostics.

mod builders;
mod category;
pub mod codes;
mod diagnostic;
mod report;
mod severity;
mod stage;
mod validation_phase;

pub use category::DiagnosticCategory;
pub use diagnostic::Diagnostic;
pub use report::{DiagnosticReport, ValidationReport};
pub use severity::Severity;
pub use stage::DiagnosticStage;
pub use validation_phase::ValidationPhase;

use crate::model::DataContract;

pub(crate) use builders::{emit, validation_error};

/// Returns a short human-readable contract summary.
#[must_use]
pub fn inspect_contract(contract: &DataContract) -> String {
    format!(
        "id: {}\nname: {}\nversion: {}\napiVersion: {}\nkind: {}\nstatus: {}\nschema: {}\nquality: {}\n",
        contract.id,
        contract.name.as_deref().unwrap_or("-"),
        contract.version,
        contract.api_version,
        contract.kind,
        contract.status,
        contract.schema.len(),
        contract.quality_rules().len(),
    )
}

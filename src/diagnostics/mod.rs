//! ODCS diagnostics.

mod builders;
mod category;
pub mod codes;
mod diagnostic;
mod report;
mod severity;
mod stage;

pub use category::DiagnosticCategory;
pub use diagnostic::Diagnostic;
pub use report::{DiagnosticReport, ValidationReport};
pub use severity::Severity;
pub use stage::DiagnosticStage;

use crate::model::DataContract;

pub(crate) use builders::{com_error, emit, validation_error};

/// Returns a short human-readable contract summary.
#[must_use]
pub fn inspect_contract(contract: &DataContract) -> String {
    format!(
        "name: {}\nversion: {}\nkind: {}\nstatus: {}\nschema: {}\nquality: {}\n",
        contract.name,
        contract.version,
        contract.kind,
        contract.status,
        contract.schema.len(),
        contract.quality.len(),
    )
}

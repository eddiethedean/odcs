//! JSON document parser.

use crate::diagnostics::{
    codes, emit, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
};
use crate::model::DataContract;

use super::ParseResult;

/// Parse JSON bytes into an ODCS contract.
#[must_use]
pub fn parse_json(content: &[u8]) -> ParseResult {
    match serde_json::from_slice::<DataContract>(content) {
        Ok(contract) => ParseResult {
            contract: Some(contract),
            report: DiagnosticReport::new(),
        },
        Err(error) => {
            let mut report = DiagnosticReport::new();
            emit(
                &mut report,
                Diagnostic::error(
                    codes::PARSE_JSON,
                    DiagnosticCategory::Syntax,
                    DiagnosticStage::Parse,
                    format!("failed to parse JSON: {error}"),
                ),
            );
            ParseResult {
                contract: None,
                report,
            }
        }
    }
}

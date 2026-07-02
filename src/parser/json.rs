//! JSON document parser.

use crate::diagnostics::codes;
use crate::model::DataContract;

use super::{failure_from_serde, success, ParseResult};

/// Parse JSON bytes into an ODCS contract.
#[must_use]
pub fn parse_json(content: &[u8]) -> ParseResult {
    match serde_json::from_slice::<DataContract>(content) {
        Ok(contract) => success(contract),
        Err(error) => {
            let location = match (error.line(), error.column()) {
                (0, 0) => String::new(),
                (line, column) => format!(" (line {line}, column {column})"),
            };
            failure_from_serde(codes::PARSE_JSON, format!("{error}{location}"))
        }
    }
}

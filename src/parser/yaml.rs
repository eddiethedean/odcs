//! YAML document parser.

use crate::diagnostics::codes;
use crate::model::DataContract;

use super::{failure_from_serde, success, ParseResult};

/// Parse YAML bytes into an ODCS contract.
#[must_use]
pub fn parse_yaml(content: &[u8]) -> ParseResult {
    match serde_yaml::from_slice::<DataContract>(content) {
        Ok(contract) => success(contract),
        Err(error) => failure_from_serde(codes::PARSE_YAML, error),
    }
}

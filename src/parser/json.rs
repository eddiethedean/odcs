//! JSON document parser.

use crate::diagnostics::codes;
use crate::model::DataContract;

use super::duplicate_keys::find_json_duplicate_key;
use super::{failure_duplicate_key, failure_from_serde, success, ParseResult};

/// Parse JSON bytes into an ODCS contract.
#[must_use]
pub fn parse_json(content: &[u8]) -> ParseResult {
    if let Some(finding) = find_json_duplicate_key(content) {
        return failure_duplicate_key(finding);
    }

    let mut de = serde_json::Deserializer::from_slice(content);
    match serde_path_to_error::deserialize::<_, DataContract>(&mut de) {
        Ok(contract) => success(contract),
        Err(error) => {
            let location = match (error.inner().line(), error.inner().column()) {
                (0, 0) => String::new(),
                (line, column) => format!(" (line {line}, column {column})"),
            };
            failure_from_serde(codes::PARSE_JSON, format!("{error}{location}"))
        }
    }
}

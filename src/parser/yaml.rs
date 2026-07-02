//! YAML document parser.

use crate::diagnostics::codes;
use crate::model::DataContract;

use super::duplicate_keys::find_yaml_root_duplicate_key;
use super::{failure_duplicate_key, failure_from_serde, success, ParseResult};

/// Parse YAML bytes into an ODCS contract.
#[must_use]
pub fn parse_yaml(content: &[u8]) -> ParseResult {
    let text = match std::str::from_utf8(content) {
        Ok(text) => text,
        Err(error) => return failure_from_serde(codes::PARSE_YAML, error),
    };

    if let Some(key) = find_yaml_root_duplicate_key(text) {
        return failure_duplicate_key(key);
    }

    let de = serde_yaml::Deserializer::from_slice(content);
    match serde_path_to_error::deserialize::<_, DataContract>(de) {
        Ok(contract) => success(contract),
        Err(error) => failure_from_serde(codes::PARSE_YAML, error),
    }
}

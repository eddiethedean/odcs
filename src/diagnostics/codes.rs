//! Standardized `odcs:` diagnostic identifiers.

/// YAML parse failure.
pub const PARSE_YAML: &str = "odcs:parse-yaml";
/// JSON parse failure.
pub const PARSE_JSON: &str = "odcs:parse-json";
/// Unsupported specification version.
pub const UNSUPPORTED_VERSION: &str = "odcs:unsupported-version";
/// Missing required field.
pub const MISSING_REQUIRED_FIELD: &str = "odcs:missing-required-field";
/// Invalid document kind.
pub const INVALID_KIND: &str = "odcs:invalid-kind";
/// Invalid schema object.
pub const INVALID_SCHEMA: &str = "odcs:invalid-schema";
/// Invalid quality rule.
pub const INVALID_QUALITY: &str = "odcs:invalid-quality";
/// Unknown top-level document field.
pub const UNKNOWN_FIELD: &str = "odcs:unknown-field";
/// Unresolved object reference.
pub const UNRESOLVED_REFERENCE: &str = "odcs:unresolved-reference";
/// Invalid extension key.
pub const INVALID_EXTENSION: &str = "odcs:invalid-extension";
/// Duplicate key in document.
pub const DUPLICATE_KEY: &str = "odcs:duplicate-key";
/// Document exceeds size limit.
pub const DOCUMENT_TOO_LARGE: &str = "odcs:document-too-large";

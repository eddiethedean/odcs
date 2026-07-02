//! Version handling for ODCS documents.

/// Supported upstream ODCS `apiVersion` values for this implementation.
pub const SUPPORTED_API_VERSIONS: &[&str] = &[
    "v3.1.0", "v3.0.2", "v3.0.1", "v3.0.0", "v2.2.2", "v2.2.1", "v2.2.0",
];

/// Returns `true` when the API version is supported by this crate.
#[must_use]
pub fn is_supported_api_version(api_version: &str) -> bool {
    SUPPORTED_API_VERSIONS.contains(&api_version)
}

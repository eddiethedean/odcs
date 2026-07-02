//! Reference implementation of the Open Data Contract Standard (ODCS).
//!
//! [`SPEC.md`](../SPEC.md) at the repository root defines the upstream specification
//! policy. This crate implements the foundational pipeline:
//!
//! ```text
//! ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
//! ```
//!
//! # Example
//!
//! ```
//! use odcs::{parse, validate, DocumentFormat};
//!
//! let yaml = br#"
//! version: "3.1.0"
//! apiVersion: "v3.1.0"
//! kind: "DataContract"
//! id: "example"
//! status: "draft"
//! "#;
//!
//! let result = parse(yaml, DocumentFormat::Yaml);
//! let contract = result.contract.expect("parse succeeded");
//! let report = validate(&contract);
//! assert!(report.is_valid());
//! ```

/// Upstream ODCS specification version this crate targets.
pub const UPSTREAM_SPEC_VERSION: &str = "3.1.0";

pub mod compatibility;
pub mod diagnostics;
pub mod model;
pub mod parser;
pub mod registry;
pub mod validation;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "python")]
mod python;

pub use diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity, ValidationReport,
};
pub use model::DataContract;
pub use parser::{parse, parse_file, parse_json, parse_yaml, DocumentFormat, ParseResult};
pub use validation::{validate, ValidationPhase};

/// Parse and validate an ODCS document in one step.
#[must_use]
pub fn parse_and_validate(content: &[u8], format: DocumentFormat) -> ValidationReport {
    parse(content, format).validate()
}

impl DataContract {
    /// Parse a contract from YAML text.
    pub fn from_yaml(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Yaml)
    }

    /// Parse a contract from JSON text.
    pub fn from_json(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Json)
    }

    /// Parse a contract from a file path.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> miette::Result<ParseResult> {
        parse_file(path)
    }
}

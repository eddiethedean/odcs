//! Reference implementation of the Open Data Contract Standard (ODCS).
//!
//! This is a **reference implementation** — not the normative ODCS specification.
//! The upstream standard is maintained at
//! [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard).
//!
//! # User documentation
//!
//! - [Getting started](https://odcs.readthedocs.io/en/latest/user/getting-started/)
//! - [API decision guide](https://odcs.readthedocs.io/en/latest/user/api-guide/)
//! - [CLI reference](https://odcs.readthedocs.io/en/latest/user/cli/)
//! - [API stability policy](https://odcs.readthedocs.io/en/latest/implementation/api-stability/)
//!
//! # Pipeline
//!
//! ```text
//! ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
//! ```
//!
//! # Stable API
//!
//! Use root re-exports only (`parse`, `validate`, `parse_and_validate`, `DataContract`,
//! `ContractSet`, `Registry`, `diff`, …). Internal modules (`parser`, `validation`, `model`, …)
//! are `#[doc(hidden)]` and not semver-stable. See the
//! [public API guide](https://odcs.readthedocs.io/en/latest/implementation/public-api.md).
//!
//! # Error handling
//!
//! - [`parse`] / [`parse_and_validate`] — return [`DiagnosticReport`] (validation) or embed
//!   parse diagnostics in [`ParseResult`]
//! - [`parse_file`] / [`DataContract::from_file`] — return `miette::Result` for I/O errors
//! - [`ParseResult::into_contract`] / [`parse_strict`] — `Result<DataContract, DiagnosticReport>`
//!   after parse and validation
//!
//! Match on diagnostic [`codes`] and `object_ref`, not message text.
//!
//! # Example
//!
//! ```
//! use odcs::{parse, validate, DocumentFormat};
//!
//! let yaml = br#"
//! version: "1.0.0"
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
pub mod contract_set;
#[doc(hidden)]
pub mod diagnostics;
#[doc(hidden)]
pub mod model;
#[doc(hidden)]
pub mod parser;
#[doc(hidden)]
pub mod registry;
#[doc(hidden)]
pub mod schema;
#[doc(hidden)]
pub mod validation;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "python")]
mod python;

pub use compatibility::{diff, ChangeKind, CompatibilityChange, CompatibilityReport};
pub use contract_set::{
    load_set, load_set_with_registry, parse_and_validate_set, parse_and_validate_set_with_registry,
    validate_set, ContractSet,
};
pub use diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity, ValidationPhase, ValidationReport,
};
pub use model::DataContract;
pub use parser::{
    parse, parse_file, parse_json, parse_strict, parse_yaml, DocumentFormat, ParseResult,
    MAX_PARSE_BYTES,
};
pub use registry::{
    index_and_save_registry, index_registry, load_registry, Registry, RegistryEntry,
};
pub use validation::{validate, validate_with_contract_index};

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

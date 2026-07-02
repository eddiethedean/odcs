//! ODCS document parsers.

mod json;
mod yaml;

use std::path::Path;

pub use json::parse_json;
pub use yaml::parse_yaml;

use crate::diagnostics::DiagnosticReport;
use crate::model::DataContract;

/// Result of parsing an ODCS document.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Parsed contract when parsing succeeded.
    pub contract: Option<DataContract>,
    /// Parse-time diagnostics.
    pub report: DiagnosticReport,
}

impl ParseResult {
    /// Returns the parsed contract when parsing succeeded without parse errors.
    pub fn into_contract(self) -> Result<DataContract, DiagnosticReport> {
        match (self.contract, self.report.is_valid()) {
            (Some(contract), true) => Ok(contract),
            (_, false) => Err(self.report),
            (None, true) => Err(self.report),
        }
    }

    /// Parses and validates in one step.
    #[must_use]
    pub fn validate(self) -> DiagnosticReport {
        let mut report = self.report;
        if let Some(contract) = self.contract {
            report.merge(crate::validate(&contract));
        }
        report
    }
}

/// Supported ODCS document serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    /// YAML encoding.
    Yaml,
    /// JSON encoding.
    Json,
}

impl DocumentFormat {
    /// Infers format from a file extension.
    #[must_use]
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "yaml" | "yml" => Some(Self::Yaml),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Parse an ODCS document from bytes.
#[must_use]
pub fn parse(content: &[u8], format: DocumentFormat) -> ParseResult {
    match format {
        DocumentFormat::Yaml => parse_yaml(content),
        DocumentFormat::Json => parse_json(content),
    }
}

/// Parse an ODCS document from a file path.
pub fn parse_file(path: impl AsRef<Path>) -> miette::Result<ParseResult> {
    let path = path.as_ref();
    let content = std::fs::read(path)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    let format = DocumentFormat::from_path(path).ok_or_else(|| {
        miette::miette!(
            "unsupported file extension for {}: expected .yaml, .yml, or .json",
            path.display()
        )
    })?;
    Ok(parse(&content, format))
}

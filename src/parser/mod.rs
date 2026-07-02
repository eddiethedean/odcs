//! ODCS document parsers.

mod json;
mod yaml;

use std::path::Path;

pub use json::parse_json;
pub use yaml::parse_yaml;

use crate::diagnostics::{emit, DiagnosticReport};
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

/// Build a successful parse result.
pub(crate) fn success(contract: DataContract) -> ParseResult {
    ParseResult {
        contract: Some(contract),
        report: DiagnosticReport::new(),
    }
}

/// Build a failed parse result with a diagnostic.
#[allow(dead_code)]
pub(crate) fn failure(code: &str, message: String) -> ParseResult {
    let mut report = DiagnosticReport::new();
    emit(
        &mut report,
        crate::diagnostics::Diagnostic::error(
            code,
            crate::diagnostics::DiagnosticCategory::Syntax,
            crate::diagnostics::DiagnosticStage::Parse,
            message,
        ),
    );
    ParseResult {
        contract: None,
        report,
    }
}

/// Build a failed parse result with an enriched serde diagnostic.
pub(crate) fn failure_from_serde(code: &str, error: impl std::fmt::Display) -> ParseResult {
    let message = error.to_string();
    let mut diagnostic = crate::diagnostics::Diagnostic::error(
        code,
        crate::diagnostics::DiagnosticCategory::Syntax,
        crate::diagnostics::DiagnosticStage::Parse,
        format!("failed to parse document: {message}"),
    );

    if let Some(object_ref) = extract_unknown_field_ref(&message) {
        diagnostic = diagnostic
            .with_object_ref(object_ref)
            .with_remediation("remove the unknown field or use customProperties for extensions");
        diagnostic.id = crate::diagnostics::codes::UNKNOWN_FIELD.to_string();
    } else if let Some(object_ref) = extract_serde_path(&message) {
        diagnostic = diagnostic.with_object_ref(object_ref);
    }

    let mut report = DiagnosticReport::new();
    emit(&mut report, diagnostic);
    ParseResult {
        contract: None,
        report,
    }
}

fn extract_unknown_field_ref(message: &str) -> Option<String> {
    let marker = "unknown field `";
    let start = message.find(marker)? + marker.len();
    let rest = &message[start..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}

fn extract_serde_path(message: &str) -> Option<String> {
    let marker = " at line ";
    if !message.contains(marker) {
        return None;
    }
    message
        .split_whitespace()
        .find(|token| token.contains('.'))
        .map(|token| token.trim_end_matches(',').to_string())
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

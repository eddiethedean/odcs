//! ODCS document parsers.

mod duplicate_keys;
mod json;
mod yaml;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use json::parse_json;
pub use yaml::parse_yaml;

use crate::diagnostics::{
    codes, emit, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
};
use crate::model::DataContract;
use crate::validation::validate;

/// Maximum document size accepted by [`parse_file`] (16 MiB).
pub const MAX_PARSE_BYTES: u64 = 16 * 1024 * 1024;

/// Result of parsing an ODCS document.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Parsed contract when parsing succeeded.
    pub contract: Option<DataContract>,
    /// Parse-time diagnostics.
    pub report: DiagnosticReport,
}

impl ParseResult {
    /// Returns the parsed contract when parsing and validation succeeded.
    pub fn into_contract(self) -> Result<DataContract, DiagnosticReport> {
        match (self.contract, self.report.is_valid()) {
            (Some(contract), true) => {
                let validation_report = crate::validate(&contract);
                if validation_report.is_valid() {
                    Ok(contract)
                } else {
                    let mut report = self.report;
                    report.merge(validation_report);
                    Err(report)
                }
            }
            (_, false) => Err(self.report),
            (None, true) => Err(self.report),
        }
    }

    /// Parses and validates in one step.
    #[must_use]
    pub fn validate(self) -> DiagnosticReport {
        let mut report = self.report;
        if let Some(contract) = self.contract {
            report.merge(validate(&contract));
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
    if content.len() as u64 > MAX_PARSE_BYTES {
        return failure_document_too_large();
    }
    match format {
        DocumentFormat::Yaml => parse_yaml(content),
        DocumentFormat::Json => parse_json(content),
    }
}

/// Parse and validate an ODCS document, returning an error report on failure.
pub fn parse_strict(
    content: &[u8],
    format: DocumentFormat,
) -> Result<DataContract, DiagnosticReport> {
    parse(content, format).into_contract()
}

/// Build a successful parse result.
pub(crate) fn success(contract: DataContract) -> ParseResult {
    ParseResult {
        contract: Some(contract),
        report: DiagnosticReport::new(),
    }
}

/// Build a failed parse result with an enriched serde diagnostic.
pub(crate) fn failure_from_serde(code: &str, error: impl std::fmt::Display) -> ParseResult {
    let message = error.to_string();
    let mut diagnostic = Diagnostic::error(
        code,
        DiagnosticCategory::Syntax,
        DiagnosticStage::Parse,
        format!("failed to parse document: {message}"),
    );

    if let Some(object_ref) = extract_unknown_field_object_ref(&message) {
        diagnostic = diagnostic
            .with_object_ref(object_ref)
            .with_remediation("remove the unknown field or use customProperties for extensions");
        diagnostic.id = codes::UNKNOWN_FIELD.to_string();
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

pub(crate) fn failure_duplicate_key(finding: duplicate_keys::DuplicateKeyFinding) -> ParseResult {
    let mut report = DiagnosticReport::new();
    emit(
        &mut report,
        Diagnostic::error(
            codes::DUPLICATE_KEY,
            DiagnosticCategory::Syntax,
            DiagnosticStage::Parse,
            format!("duplicate key '{}' in document", finding.key),
        )
        .with_object_ref(finding.object_ref)
        .with_remediation("remove duplicate keys so each field appears once"),
    );
    ParseResult {
        contract: None,
        report,
    }
}

fn failure_document_too_large() -> ParseResult {
    let mut report = DiagnosticReport::new();
    emit(
        &mut report,
        Diagnostic::error(
            codes::DOCUMENT_TOO_LARGE,
            DiagnosticCategory::Syntax,
            DiagnosticStage::Parse,
            format!("document exceeds maximum size of {MAX_PARSE_BYTES} bytes"),
        )
        .with_remediation("split the contract or reduce document size"),
    );
    ParseResult {
        contract: None,
        report,
    }
}

fn extract_unknown_field_object_ref(message: &str) -> Option<String> {
    let field = extract_unknown_field_ref(message)?;
    if let Some(marker) = message.find(": unknown field") {
        let prefix = message[..marker].trim();
        if let Some(path) = prefix.rsplit(": ").next() {
            if path.contains('[') || path.contains('.') {
                return Some(format!("{path}.{field}"));
            }
        }
    }
    Some(field)
}

fn extract_unknown_field_ref(message: &str) -> Option<String> {
    let marker = "unknown field `";
    let start = message.find(marker)? + marker.len();
    let rest = &message[start..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}

fn extract_serde_path(message: &str) -> Option<String> {
    if let Some(path) = message.strip_prefix("at ") {
        return Some(path.trim().to_string());
    }
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
    let format = DocumentFormat::from_path(path).ok_or_else(|| {
        miette::miette!(
            "unsupported file extension for {}: expected .yaml, .yml, or .json",
            path.display()
        )
    })?;

    let file = File::open(path)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    let mut content = Vec::new();
    file.take(MAX_PARSE_BYTES.saturating_add(1))
        .read_to_end(&mut content)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;

    if content.len() as u64 > MAX_PARSE_BYTES {
        return Ok(failure_document_too_large());
    }

    Ok(parse(&content, format))
}

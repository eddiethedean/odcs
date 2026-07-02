//! Tests for validationPhase metadata on diagnostics.

use std::fs;
use std::path::PathBuf;

use odcs::{parse, DiagnosticReport, DiagnosticStage, DocumentFormat, ValidationPhase};

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
    )
    .expect("read fixture")
}

fn parse_and_validate_fixture(name: &str) -> DiagnosticReport {
    let format = if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    parse(&fixture_bytes(name), format).validate()
}

fn assert_validation_phase_coverage(report: &DiagnosticReport) {
    for diagnostic in &report.diagnostics {
        match diagnostic.stage {
            DiagnosticStage::Validation => {
                assert!(
                    diagnostic.validation_phase.is_some(),
                    "validation diagnostic missing validationPhase: {diagnostic:?}"
                );
            }
            DiagnosticStage::Parse => {
                assert!(
                    diagnostic.validation_phase.is_none(),
                    "parse diagnostic must omit validationPhase: {diagnostic:?}"
                );
            }
            _ => {}
        }
    }
}

/// Invalid fixtures that parse successfully and produce validation diagnostics.
const VALIDATION_FIXTURES: &[&str] = &[
    "invalid-kind.yaml",
    "invalid-empty-id.yaml",
    "invalid-quality-no-type-bad-metric.yaml",
    "invalid-quality-deprecated-rule-only.yaml",
    "invalid-quality-empty-sql-query.yaml",
    "invalid-quality-unknown-type.yaml",
    "invalid-quality-between-length.yaml",
    "invalid-quality-dimension.yaml",
    "invalid-relationship-empty-composite.yaml",
    "invalid-relationship-bad-format.yaml",
    "invalid-relationship-length-mismatch.yaml",
    "invalid-relationship-dangling.yaml",
    "invalid-relationship-from.yaml",
    "invalid-nested-property-ref.yaml",
    "invalid-server-typo.yaml",
    "invalid-server-type.yaml",
    "invalid-extension-empty-key.yaml",
    "invalid-extension-duplicate.yaml",
    "invalid-schema-array-without-items.yaml",
    "invalid-stable-id.yaml",
    "invalid-logical-type.yaml",
    "invalid-json-schema-only.yaml",
    "invalid-lone-team-member.yaml",
    "unsupported-version.yaml",
];

/// Fixtures that fail during parse (validationPhase must be absent).
const PARSE_ONLY_FIXTURES: &[&str] = &[
    "invalid-duplicate-key.yaml",
    "invalid-duplicate-key.json",
    "invalid-nested-duplicate-key.yaml",
    "invalid-nested-duplicate-key.json",
    "unknown-field.yaml",
    "nested-unknown-field.yaml",
];

#[test]
fn validation_fixtures_include_validation_phase() {
    for name in VALIDATION_FIXTURES {
        let report = parse_and_validate_fixture(name);
        assert!(!report.is_valid(), "fixture {name} should be invalid");
        assert_validation_phase_coverage(&report);
    }
}

#[test]
fn parse_only_fixtures_omit_validation_phase() {
    for name in PARSE_ONLY_FIXTURES {
        let format = if name.ends_with(".json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        let report = parse(&fixture_bytes(name), format).report;
        assert!(!report.is_valid(), "fixture {name} should be invalid");
        assert_validation_phase_coverage(&report);
    }
}

#[test]
fn invalid_kind_uses_document_phase() {
    let report = parse_and_validate_fixture("invalid-kind.yaml");
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.stage == DiagnosticStage::Validation)
        .expect("validation diagnostic");
    assert_eq!(diagnostic.validation_phase, Some(ValidationPhase::Document));
}

#[test]
fn validation_phase_serializes_as_camel_case() {
    let report = parse_and_validate_fixture("invalid-kind.yaml");
    let json = serde_json::to_string(&report.diagnostics).expect("serialize diagnostics");
    assert!(json.contains("\"validationPhase\":\"document\""));
}

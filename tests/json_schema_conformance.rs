//! JSON Schema conformance tests against the pinned upstream ODCS v3.1.0 schema.

mod common;

use std::fs;

use odcs::parser::ParseResult;
use odcs::{codes, parse, validate, DocumentFormat};

use common::{assert_valid_fixture_passes_odcs_and_json_schema, fixture_bytes, format_for, pinned_validator, VALID_FIXTURES};

#[test]
fn valid_fixtures_conform_to_pinned_json_schema() {
    for name in VALID_FIXTURES {
        assert_valid_fixture_passes_odcs_and_json_schema(name);
    }
}

#[test]
fn unsupported_version_fails_validation_before_schema_check() {
    let result = parse(
        &fixture_bytes("unsupported-version.yaml"),
        DocumentFormat::Yaml,
    );
    let report = result.validate();
    assert!(!report.is_valid());
}

#[test]
fn invalid_kind_fails_validation_before_schema_check() {
    let result = parse(&fixture_bytes("invalid-kind.yaml"), DocumentFormat::Yaml);
    let report = result.validate();
    assert!(!report.is_valid());
}

const RUST_MUST_REJECT_FIXTURES: &[&str] = &[
    "invalid-kind.yaml",
    "invalid-quality-unknown-type.yaml",
    "invalid-relationship-dangling.yaml",
    "invalid-relationship-from.yaml",
    "invalid-nested-property-ref.yaml",
    "invalid-quality-dimension.yaml",
    "invalid-logical-type.yaml",
    "invalid-server-type.yaml",
];

#[test]
fn invalid_fixtures_fail_rust_validation() {
    for name in RUST_MUST_REJECT_FIXTURES {
        let format = format_for(name);
        let report = parse(&fixture_bytes(name), format).validate();
        assert!(
            !report.is_valid(),
            "fixture {name}: Rust validation must reject: {:?}",
            report.diagnostics
        );
    }
}

#[test]
fn json_schema_only_fixture_fails_default_validation() {
    let result = parse(
        &fixture_bytes("invalid-json-schema-only.yaml"),
        DocumentFormat::Yaml,
    );
    let contract = result.contract.expect("parsed contract");
    let report = validate(&contract);
    assert!(!report.is_valid());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.id == codes::JSON_SCHEMA_VIOLATION),
        "expected JSON Schema violation: {:?}",
        report.diagnostics
    );
}

#[test]
fn upstream_examples_conform_when_parseable() {
    let upstream_dir = std::path::Path::new("tests/fixtures/upstream");
    if !upstream_dir.exists() {
        return;
    }
    let validator = pinned_validator();
    let mut tested = 0usize;
    for entry in fs::read_dir(upstream_dir).expect("read upstream dir") {
        let entry = entry.expect("read upstream entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name == "SOURCE.txt" || name.starts_with('.') {
            continue;
        }
        let format = if name.ends_with(".json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        let fixture_name = format!("upstream/{name}");
        let ParseResult {
            contract,
            report: parse_report,
        } = parse(&fixture_bytes(&fixture_name), format);
        let mut report = parse_report;
        if let Some(ref contract) = contract {
            report.merge(validate(contract));
        }
        if contract.is_none() {
            continue;
        }
        assert!(
            report.is_valid(),
            "upstream example {name} should pass validation: {:?}",
            report.diagnostics
        );
        tested += 1;
        assert_valid_fixture_passes_odcs_and_json_schema(&fixture_name);
        let contract = contract.expect("parsed upstream example");
        let instance = serde_json::to_value(&contract).expect("serialize contract");
        assert!(
            validator.is_valid(&instance),
            "upstream example {name} failed JSON Schema conformance"
        );
    }
    assert!(
        tested >= 1,
        "expected at least one upstream example to pass odcs validation"
    );
}

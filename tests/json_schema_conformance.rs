//! JSON Schema conformance tests against the pinned upstream ODCS v3.1.0 schema.

use std::fs;

use jsonschema::Validator;
use odcs::parser::ParseResult;
use odcs::{parse, validate, DocumentFormat};
use serde_json::Value;

fn pinned_validator() -> Validator {
    let content = fs::read_to_string("tests/fixtures/odcs-json-schema-v3.1.0.json")
        .expect("read pinned schema");
    let schema_value: Value = serde_json::from_str(&content).expect("parse pinned schema");
    jsonschema::validator_for(&schema_value).expect("compile pinned schema")
}

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(format!("tests/fixtures/{name}")).expect("read fixture")
}

fn assert_fixture_matches_schema(name: &str, format: DocumentFormat) {
    let ParseResult {
        contract,
        report: parse_report,
    } = parse(&fixture_bytes(name), format);
    let mut report = parse_report;
    if let Some(ref contract) = contract {
        report.merge(validate(contract));
    }
    assert!(
        report.is_valid(),
        "fixture {name} should validate before schema check: {:?}",
        report.diagnostics
    );
    let contract = contract.expect("parsed contract");
    let instance = serde_json::to_value(&contract).expect("serialize contract");
    let validator = pinned_validator();
    if !validator.is_valid(&instance) {
        let messages: Vec<String> = validator
            .iter_errors(&instance)
            .map(|error| error.to_string())
            .collect();
        panic!("fixture {name} failed JSON Schema conformance: {messages:?}");
    }
}

const VALID_SCHEMA_FIXTURES: &[&str] = &[
    "minimal.odcs.yaml",
    "minimal.odcs.json",
    "with-sla.yaml",
    "with-team.yaml",
    "with-team-legacy-array.yaml",
    "with-roles.yaml",
    "with-servers.yaml",
    "with-pricing.yaml",
    "with-support.yaml",
    "with-schema-quality.yaml",
    "with-schema-properties.yaml",
    "with-custom-properties.yaml",
    "with-extensions.yaml",
    "with-relationships.yaml",
    "with-schema-array-items.yaml",
    "with-custom-quality-object.yaml",
];

#[test]
fn valid_fixtures_conform_to_pinned_json_schema() {
    for name in VALID_SCHEMA_FIXTURES {
        let format = if name.ends_with(".json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        assert_fixture_matches_schema(name, format);
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

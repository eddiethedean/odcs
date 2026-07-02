//! Integration tests for the ODCS reference implementation.

use std::fs;
use std::path::PathBuf;

use odcs::{codes, parse, parse_file, DocumentFormat, ParseResult, UPSTREAM_SPEC_VERSION};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    let format = if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    parse(&content, format)
}

#[test]
fn upstream_spec_version_is_set() {
    assert_eq!(UPSTREAM_SPEC_VERSION, "3.1.0");
}

#[test]
fn parses_minimal_yaml_fixture() {
    let result = parse_fixture("minimal.odcs.yaml");
    let contract = result.into_contract().expect("parse fixture");
    assert_eq!(contract.name, "customer_data_contract");
    assert_eq!(contract.version, UPSTREAM_SPEC_VERSION);
    assert_eq!(contract.schema.len(), 1);
    assert_eq!(contract.quality.len(), 1);
}

#[test]
fn parses_minimal_json_fixture() {
    let result = parse_fixture("minimal.odcs.json");
    let contract = result.into_contract().expect("parse fixture");
    assert_eq!(contract.name, "customer_data_contract");
    assert_eq!(contract.kind, "DataContract");
}

#[test]
fn validates_minimal_fixture() {
    let result = parse_file("tests/fixtures/minimal.odcs.yaml").expect("read fixture");
    let report = result.validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn parses_example_yaml() {
    let result = parse_file("examples/minimal.odcs.yaml").expect("read example");
    let contract = result.into_contract().expect("parse example");
    assert_eq!(contract.kind, "DataContract");
}

#[test]
fn parses_example_json() {
    let result = parse_file("examples/minimal.odcs.json").expect("read example");
    let contract = result.into_contract().expect("parse example");
    assert_eq!(contract.name, "customer_data_contract");
}

#[test]
fn rejects_malformed_yaml() {
    let result = parse_fixture("malformed.yaml");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::PARSE_YAML));
}

#[test]
fn rejects_malformed_json() {
    let result = parse_fixture("malformed.json");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::PARSE_JSON));
}

#[test]
fn rejects_empty_name() {
    let report = parse_fixture("invalid-empty-name.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_REQUIRED_FIELD));
}

#[test]
fn rejects_invalid_kind() {
    let report = parse_fixture("invalid-kind.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SCHEMA));
}

#[test]
fn rejects_unsupported_version() {
    let report = parse_fixture("unsupported-version.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNSUPPORTED_VERSION));
}

#[test]
fn preserves_extension_fields() {
    let result = parse_fixture("with-extensions.yaml");
    let contract = result.into_contract().expect("parse extensions fixture");
    assert!(contract.extensions.contains_key("customDomain"));
    assert!(contract.extensions.contains_key("metadata"));
}

#[test]
fn diagnostics_are_deterministic_for_invalid_kind() {
    let first = parse_fixture("invalid-kind.yaml").validate();
    let second = parse_fixture("invalid-kind.yaml").validate();
    assert_eq!(first.diagnostics.len(), second.diagnostics.len());
    assert_eq!(first.diagnostics[0].id, second.diagnostics[0].id);
    assert_eq!(first.diagnostics[0].message, second.diagnostics[0].message);
}

#[test]
fn into_contract_requires_valid_parse() {
    let result = parse_fixture("malformed.yaml");
    assert!(result.into_contract().is_err());
}

//! Integration tests for the ODCS reference implementation.

use odcs::{parse_file, DocumentFormat, UPSTREAM_SPEC_VERSION};

#[test]
fn parses_minimal_fixture() {
    let result = parse_file("tests/fixtures/minimal.odcs.yaml").expect("read fixture");
    let contract = result.into_contract().expect("parse fixture");
    assert_eq!(contract.name, "customer_data_contract");
    assert_eq!(contract.version, UPSTREAM_SPEC_VERSION);
    assert_eq!(contract.schema.len(), 1);
    assert_eq!(contract.quality.len(), 1);
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
fn rejects_malformed_yaml() {
    let result = odcs::parse(b"not: [valid: yaml", DocumentFormat::Yaml);
    assert!(!result.report.is_valid());
    assert!(result.contract.is_none());
}

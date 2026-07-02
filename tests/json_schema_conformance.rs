//! JSON Schema conformance tests against the pinned upstream ODCS v3.1.0 schema.

use std::fs;

use jsonschema::Validator;
use odcs::parser::ParseResult;
use odcs::{codes, parse, validate, DocumentFormat};
use serde_json::Value;

fn pinned_validator() -> Validator {
    let content = fs::read_to_string("schema/odcs-v3.1.0.json").expect("read pinned schema");
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
    "with-sla-description.yaml",
    "with-sla-default-element.yaml",
    "with-team.yaml",
    "with-team-legacy-array.yaml",
    "with-roles.yaml",
    "with-servers.yaml",
    "with-server-kafka.yaml",
    "with-server-postgres.yaml",
    "with-pricing.yaml",
    "with-support.yaml",
    "with-schema-quality.yaml",
    "with-schema-properties.yaml",
    "with-custom-properties.yaml",
    "with-extensions.yaml",
    "with-relationships.yaml",
    "with-property-relationships.yaml",
    "with-schema-array-items.yaml",
    "with-custom-quality-object.yaml",
    "with-tenant.yaml",
    "with-root-tags.yaml",
    "with-domain.yaml",
    "with-description.yaml",
    "with-data-product.yaml",
    "with-contract-created-ts.yaml",
    "with-authoritative-definitions.yaml",
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

const INVALID_PARITY_FIXTURES: &[&str] = &[
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
fn invalid_fixtures_fail_rust_or_json_schema_validation() {
    let validator = pinned_validator();
    for name in INVALID_PARITY_FIXTURES {
        let format = if name.ends_with(".json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        let ParseResult {
            contract,
            report: parse_report,
        } = parse(&fixture_bytes(name), format);
        let mut report = parse_report;
        if let Some(ref contract) = contract {
            report.merge(validate(contract));
        }
        let rust_invalid = !report.is_valid();
        let schema_invalid = contract.as_ref().is_some_and(|contract| {
            let instance = serde_json::to_value(contract).expect("serialize contract");
            !validator.is_valid(&instance)
        });
        assert!(
            rust_invalid || schema_invalid,
            "fixture {name} should fail Rust validation and/or JSON Schema"
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
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_QUALITY || d.id == codes::JSON_SCHEMA_VIOLATION));
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
        assert_fixture_matches_schema(&fixture_name, format);
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

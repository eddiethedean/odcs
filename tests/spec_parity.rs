//! Spec parity: default validation agrees with pinned JSON Schema on valid fixtures.

use std::fs;

use jsonschema::Validator;
use odcs::parser::ParseResult;
use odcs::{parse, validate, DocumentFormat};
use serde_json::Value;

fn pinned_validator() -> Validator {
    let content = fs::read_to_string("schema/odcs-v3.1.0.json").expect("read pinned schema");
    let schema_value: Value = serde_json::from_str(&content).expect("parse pinned schema");
    jsonschema::validator_for(&schema_value).expect("compile pinned schema")
}

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(format!("tests/fixtures/{name}")).expect("read fixture")
}

const VALID_FIXTURES: &[&str] = &[
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
fn default_validation_matches_json_schema_on_valid_fixtures() {
    let validator = pinned_validator();
    for name in VALID_FIXTURES {
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
        assert!(
            report.is_valid(),
            "fixture {name} should pass default validation: {:?}",
            report.diagnostics
        );
        let contract = contract.expect("parsed contract");
        let instance = serde_json::to_value(&contract).expect("serialize contract");
        assert!(
            validator.is_valid(&instance),
            "fixture {name} should conform to JSON Schema"
        );
    }
}

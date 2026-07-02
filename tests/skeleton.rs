//! Integration tests for the ODCS reference implementation.

use std::fs;
use std::path::PathBuf;

use odcs::model::DataContract;
use odcs::{
    codes, parse, parse_file, validate, DocumentFormat, ParseResult, UPSTREAM_SPEC_VERSION,
};

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

fn parse_fixture_contract(name: &str) -> DataContract {
    parse_fixture(name).into_contract().expect("parse fixture")
}

const SECTION_FIXTURES: &[&str] = &[
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
fn upstream_spec_version_is_set() {
    assert_eq!(UPSTREAM_SPEC_VERSION, "3.1.0");
}

#[test]
fn parses_minimal_yaml_fixture() {
    let contract = parse_fixture_contract("minimal.odcs.yaml");
    assert_eq!(contract.name.as_deref(), Some("customer_data_contract"));
    assert_eq!(contract.version, UPSTREAM_SPEC_VERSION);
    assert_eq!(contract.api_version, "v3.1.0");
    assert_eq!(contract.id, "customer-data-contract");
    assert_eq!(contract.schema.len(), 1);
    assert_eq!(contract.quality_rules().len(), 1);
}

#[test]
fn parses_minimal_json_fixture() {
    let contract = parse_fixture_contract("minimal.odcs.json");
    assert_eq!(contract.name.as_deref(), Some("customer_data_contract"));
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
    assert_eq!(contract.name.as_deref(), Some("customer_data_contract"));
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
fn rejects_empty_id() {
    let report = parse_fixture("invalid-empty-id.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_REQUIRED_FIELD && d.object_ref.as_deref() == Some("id")));
}

#[test]
fn rejects_invalid_kind() {
    let report = parse_fixture("invalid-kind.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_KIND));
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
fn preserves_custom_properties() {
    let contract = parse_fixture_contract("with-extensions.yaml");
    let root = contract
        .custom_properties
        .as_ref()
        .expect("root custom properties");
    assert!(root.iter().any(|p| p.property == "customDomain"));
    assert!(root.iter().any(|p| p.property == "metadata"));

    let nested = contract.schema[0].properties[0]
        .element
        .custom_properties
        .as_ref()
        .expect("nested custom properties");
    assert!(nested.iter().any(|p| p.property == "sourceSystem"));
}

#[test]
fn rejects_unknown_root_field() {
    let result = parse_fixture("unknown-field.yaml");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNKNOWN_FIELD));
}

#[test]
fn rejects_nested_unknown_field() {
    let result = parse_fixture("nested-unknown-field.yaml");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNKNOWN_FIELD));
}

#[test]
fn rejects_lone_team_member_object() {
    let result = parse_fixture("invalid-lone-team-member.yaml");
    assert!(result.contract.is_none());
    assert!(!result.report.is_valid());
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

#[test]
fn into_contract_rejects_validation_invalid_contract() {
    let result = parse_fixture("invalid-kind.yaml");
    assert!(result.into_contract().is_err());
}

#[test]
fn parses_all_section_fixtures() {
    for name in SECTION_FIXTURES {
        let contract = parse_fixture_contract(name);
        assert!(!contract.id.is_empty(), "fixture {name} missing id");
        assert_eq!(contract.api_version, "v3.1.0");
        let report = validate(&contract);
        assert!(
            report.is_valid(),
            "fixture {name}: {:?}",
            report.diagnostics
        );
    }
}

#[test]
fn parses_relationship_type_field() {
    let contract = parse_fixture_contract("with-relationships.yaml");
    let relationship = &contract.schema[0].relationships[0];
    assert_eq!(
        relationship.base.relationship_type.as_deref(),
        Some("foreignKey")
    );
}

#[test]
fn parses_array_items_property() {
    let contract = parse_fixture_contract("with-schema-array-items.yaml");
    let items = contract.schema[0].properties[0]
        .items
        .as_ref()
        .expect("array items");
    assert_eq!(items.element.name.as_deref(), Some("tag"));
}

#[test]
fn parses_custom_quality_object_implementation() {
    let contract = parse_fixture_contract("with-custom-quality-object.yaml");
    let implementation = contract.schema[0].quality.as_ref().expect("quality rules")[0]
        .implementation
        .as_ref()
        .expect("implementation");
    assert!(implementation.is_object());
}

#[test]
fn team_object_and_legacy_array_forms_parse() {
    let object_form = parse_fixture_contract("with-team.yaml");
    let legacy_form = parse_fixture_contract("with-team-legacy-array.yaml");
    assert_eq!(object_form.team.as_ref().unwrap().members().len(), 1);
    assert_eq!(legacy_form.team.as_ref().unwrap().members().len(), 1);
}

#[test]
fn yaml_round_trip_through_json() {
    let original = parse_fixture_contract("minimal.odcs.yaml");
    let json = serde_json::to_string(&original).expect("serialize json");
    let reparsed = parse(json.as_bytes(), DocumentFormat::Json)
        .into_contract()
        .expect("parse json round-trip");
    assert_eq!(original, reparsed);
}

#[test]
fn json_round_trip_through_yaml() {
    let original = parse_fixture_contract("minimal.odcs.json");
    let yaml = serde_yaml::to_string(&original).expect("serialize yaml");
    let reparsed = parse(yaml.as_bytes(), DocumentFormat::Yaml)
        .into_contract()
        .expect("parse yaml round-trip");
    assert_eq!(original, reparsed);
}

#[test]
fn custom_properties_survive_round_trip() {
    let original = parse_fixture_contract("with-custom-properties.yaml");
    let json = serde_json::to_string(&original).expect("serialize");
    let reparsed = parse(json.as_bytes(), DocumentFormat::Json)
        .into_contract()
        .expect("parse");
    assert_eq!(original, reparsed);
}

#[test]
fn relationship_type_round_trips() {
    let original = parse_fixture_contract("with-relationships.yaml");
    let json = serde_json::to_string(&original).expect("serialize");
    assert!(json.contains("\"type\":\"foreignKey\""));
    let reparsed = parse(json.as_bytes(), DocumentFormat::Json)
        .into_contract()
        .expect("parse");
    assert_eq!(original, reparsed);
}

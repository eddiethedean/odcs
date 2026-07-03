//! Negative validation and parser hardening tests from the 0.3.0 bug audit.

use std::fs;

use odcs::{
    codes, parse, parse_strict, validate_strict, DocumentFormat, ParseResult, ValidationPhase,
    MAX_PARSE_BYTES,
};

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
    )
    .expect("read fixture")
}

fn parse_fixture(name: &str) -> ParseResult {
    let format = if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    parse(&fixture_bytes(name), format)
}

fn assert_invalid_with_code(name: &str, code: &str) {
    let report = parse_fixture(name).validate();
    assert!(
        !report.is_valid(),
        "fixture {name} should be invalid: {:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|d| d.id == code),
        "fixture {name}: expected {code}, got {:?}",
        report.diagnostics
    );
}

fn assert_structural_error(name: &str, code: &str, object_ref: &str) {
    let report = parse_fixture(name).validate();
    assert!(
        !report.is_valid(),
        "fixture {name} should be invalid: {:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|d| {
            d.id == code
                && d.validation_phase == Some(ValidationPhase::Structural)
                && d.object_ref.as_deref() == Some(object_ref)
        }),
        "fixture {name}: expected structural {code} at {object_ref}, got {:?}",
        report.diagnostics
    );
}

fn assert_sections_error(name: &str, code: &str, object_ref: &str) {
    let report = parse_fixture(name).validate();
    assert!(
        !report.is_valid(),
        "fixture {name} should be invalid: {:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|d| {
            d.id == code
                && d.validation_phase == Some(ValidationPhase::Sections)
                && d.object_ref.as_deref() == Some(object_ref)
        }),
        "fixture {name}: expected sections {code} at {object_ref}, got {:?}",
        report.diagnostics
    );
}

#[test]
fn quality_rules_count_includes_items() {
    let contract = parse_fixture("with-schema-quality-items.yaml")
        .into_contract()
        .expect("valid fixture");
    assert_eq!(contract.quality_rules().len(), 1);
}

#[test]
fn rejects_quality_without_type_and_bad_metric() {
    assert_invalid_with_code(
        "invalid-quality-no-type-bad-metric.yaml",
        codes::INVALID_QUALITY,
    );
}

#[test]
fn rejects_quality_with_deprecated_rule_only() {
    assert_invalid_with_code(
        "invalid-quality-deprecated-rule-only.yaml",
        codes::INVALID_QUALITY,
    );
}

#[test]
fn rejects_quality_with_empty_sql_query() {
    assert_invalid_with_code(
        "invalid-quality-empty-sql-query.yaml",
        codes::INVALID_QUALITY,
    );
}

#[test]
fn rejects_quality_with_unknown_type() {
    assert_invalid_with_code("invalid-quality-unknown-type.yaml", codes::INVALID_QUALITY);
}

#[test]
fn rejects_quality_with_invalid_between_length() {
    assert_invalid_with_code(
        "invalid-quality-between-length.yaml",
        codes::INVALID_QUALITY,
    );
}

#[test]
fn rejects_relationship_empty_composite_member() {
    assert_invalid_with_code(
        "invalid-relationship-empty-composite.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_relationship_bad_format() {
    assert_invalid_with_code(
        "invalid-relationship-bad-format.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_relationship_composite_length_mismatch() {
    assert_invalid_with_code(
        "invalid-relationship-length-mismatch.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_relationship_dangling_reference() {
    assert_invalid_with_code(
        "invalid-relationship-dangling.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_server_typo_in_details() {
    let report = parse_fixture("invalid-server-typo.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_REQUIRED_FIELD));
    assert!(report.diagnostics.iter().any(|d| {
        d.id == codes::UNKNOWN_FIELD && d.object_ref.as_deref() == Some("servers[0].sever")
    }));
}

#[test]
fn rejects_extension_empty_key_in_servers() {
    assert_invalid_with_code("invalid-extension-empty-key.yaml", codes::INVALID_EXTENSION);
}

#[test]
fn rejects_extension_duplicate_keys() {
    assert_invalid_with_code("invalid-extension-duplicate.yaml", codes::INVALID_EXTENSION);
}

#[test]
fn rejects_schema_array_without_items() {
    assert_invalid_with_code(
        "invalid-schema-array-without-items.yaml",
        codes::INVALID_SCHEMA,
    );
}

#[test]
fn rejects_invalid_stable_id() {
    assert_invalid_with_code("invalid-stable-id.yaml", codes::INVALID_EXTENSION);
}

#[test]
fn rejects_yaml_duplicate_root_key() {
    let report = parse_fixture("invalid-duplicate-key.yaml").report;
    assert!(!report.is_valid());
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.id == codes::DUPLICATE_KEY)
        .expect("duplicate key diagnostic");
    assert_eq!(diagnostic.object_ref.as_deref(), Some("id"));
}

#[test]
fn rejects_json_duplicate_key() {
    let report = parse_fixture("invalid-duplicate-key.json").report;
    assert!(!report.is_valid());
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.id == codes::DUPLICATE_KEY)
        .expect("duplicate key diagnostic");
    assert_eq!(diagnostic.object_ref.as_deref(), Some("id"));
}

#[test]
fn rejects_yaml_nested_duplicate_key() {
    let report = parse_fixture("invalid-nested-duplicate-key.yaml").report;
    assert!(!report.is_valid());
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.id == codes::DUPLICATE_KEY)
        .expect("duplicate key diagnostic");
    assert_eq!(diagnostic.object_ref.as_deref(), Some("schema[0].name"));
}

#[test]
fn rejects_json_nested_duplicate_key() {
    let report = parse_fixture("invalid-nested-duplicate-key.json").report;
    assert!(!report.is_valid());
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.id == codes::DUPLICATE_KEY)
        .expect("duplicate key diagnostic");
    assert_eq!(diagnostic.object_ref.as_deref(), Some("schema[0].name"));
}

#[test]
fn parse_strict_rejects_invalid_kind() {
    let result = parse_strict(&fixture_bytes("invalid-kind.yaml"), DocumentFormat::Yaml);
    assert!(result.is_err());
}

#[test]
fn rejects_oversized_document() {
    let oversized = vec![b' '; MAX_PARSE_BYTES as usize + 1];
    let report = parse(&oversized, DocumentFormat::Yaml).report;
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::DOCUMENT_TOO_LARGE));
}

#[test]
fn nested_unknown_field_includes_object_ref() {
    let report = parse_fixture("nested-unknown-field.yaml").report;
    assert!(!report.is_valid());
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|d| d.id == codes::UNKNOWN_FIELD)
        .expect("unknown field diagnostic");
    assert_eq!(
        diagnostic.object_ref.as_deref(),
        Some("schema[0].properties[0].requred")
    );
}

#[test]
fn yaml_duplicate_scan_failure_is_parse_error() {
    let yaml = b":\n  bad: [\n";
    let report = parse(yaml, DocumentFormat::Yaml).report;
    assert!(!report.is_valid());
    assert!(report.diagnostics.iter().any(|d| d.id == codes::PARSE_YAML));
}

#[test]
fn json_schema_dedup_when_rust_validator_reports_same_field() {
    let report = parse_fixture("invalid-quality-dimension.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_QUALITY));
    assert!(!report.diagnostics.iter().any(|d| {
        d.id == codes::JSON_SCHEMA_VIOLATION
            && d.object_ref.as_deref() == Some("/schema/0/quality/0/dimension")
    }));
}

#[test]
fn api_version_must_be_supported() {
    let yaml = br#"
version: "1.0.0"
apiVersion: "v3.0.2"
kind: "DataContract"
id: "mismatch"
status: "draft"
schema:
  - name: "customers"
    logicalType: "object"
    properties:
      - name: "customer_id"
        logicalType: "string"
"#;
    let report = parse(yaml, DocumentFormat::Yaml).validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNSUPPORTED_VERSION));
}

#[test]
fn accepts_upstream_document_version_with_supported_api_version() {
    let yaml = br#"
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "upstream-version"
status: "draft"
schema:
  - name: "customers"
    logicalType: "object"
    properties:
      - name: "customer_id"
        logicalType: "string"
"#;
    let report = parse(yaml, DocumentFormat::Yaml).validate();
    assert!(
        report.is_valid(),
        "upstream document version should be accepted: {:?}",
        report.diagnostics
    );
}

#[test]
fn sla_large_integer_preserves_precision() {
    let yaml = br#"
version: "3.1.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "sla-large-int"
status: "active"
slaProperties:
  - property: "freshness"
    value: 9007199254740993
schema:
  - name: "customers"
    logicalType: "object"
    properties:
      - name: "customer_id"
        logicalType: "string"
"#;
    let contract = parse(yaml, DocumentFormat::Yaml)
        .into_contract()
        .expect("valid contract");
    let value = serde_json::to_value(&contract.sla_properties[0].value).expect("serialize sla");
    assert_eq!(value, serde_json::json!(9_007_199_254_740_993_i64));
}

#[test]
fn rejects_property_relationship_invalid_from() {
    assert_invalid_with_code(
        "invalid-relationship-from.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_nested_property_shorthand_reference() {
    assert_invalid_with_code(
        "invalid-nested-property-ref.yaml",
        codes::UNRESOLVED_REFERENCE,
    );
}

#[test]
fn rejects_invalid_server_type() {
    assert_invalid_with_code("invalid-server-type.yaml", codes::INVALID_SCHEMA);
}

#[test]
fn rejects_invalid_quality_dimension_in_default_mode() {
    assert_invalid_with_code("invalid-quality-dimension.yaml", codes::INVALID_QUALITY);
}

#[test]
fn rejects_invalid_logical_type_in_default_mode() {
    assert_invalid_with_code("invalid-logical-type.yaml", codes::INVALID_SCHEMA);
}

#[test]
fn strict_mode_rejects_json_schema_violation() {
    let result = parse_fixture("invalid-json-schema-only.yaml");
    let contract = result.contract.expect("parsed contract");
    let report = validate_strict(&contract);
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_QUALITY || d.id == codes::JSON_SCHEMA_VIOLATION));
}

#[test]
fn sla_description_and_scheduler_round_trip() {
    let contract = parse_fixture("with-sla-description.yaml")
        .into_contract()
        .expect("valid fixture");
    assert_eq!(
        contract.sla_properties[0].description.as_deref(),
        Some("Data available within 24 hours")
    );
    assert_eq!(
        contract.sla_properties[0].scheduler.as_deref(),
        Some("cron")
    );
}

#[test]
fn rejects_duplicate_schema_names() {
    assert_structural_error(
        "invalid-structural-duplicate-schema-name.yaml",
        codes::INVALID_SCHEMA,
        "schema[1].name",
    );
}

#[test]
fn rejects_duplicate_server_names() {
    assert_structural_error(
        "invalid-structural-duplicate-server.yaml",
        codes::INVALID_SCHEMA,
        "servers[1].server",
    );
}

#[test]
fn rejects_dangling_sla_element() {
    assert_structural_error(
        "invalid-structural-sla-element-dangling.yaml",
        codes::UNRESOLVED_REFERENCE,
        "slaProperties[0].element",
    );
}

#[test]
fn rejects_dangling_sla_default_element() {
    assert_structural_error(
        "invalid-structural-sla-default-element-dangling.yaml",
        codes::UNRESOLVED_REFERENCE,
        "slaDefaultElement",
    );
}

#[test]
fn rejects_partial_sla_default_element_list() {
    assert_structural_error(
        "invalid-structural-sla-default-element-partial.yaml",
        codes::UNRESOLVED_REFERENCE,
        "slaDefaultElement",
    );
}

#[test]
fn rejects_empty_quality_rule() {
    assert_invalid_with_code("invalid-quality-empty-rule.yaml", codes::INVALID_QUALITY);
}

#[test]
fn rejects_quality_with_mustbe_only() {
    assert_invalid_with_code("invalid-quality-mustbe-only.yaml", codes::INVALID_QUALITY);
}

#[test]
fn rejects_duplicate_role_ids() {
    assert_sections_error(
        "invalid-roles-duplicate-id.yaml",
        codes::INVALID_SCHEMA,
        "roles[1].id",
    );
}

#[test]
fn rejects_support_missing_url_for_tool() {
    assert_sections_error(
        "invalid-support-missing-url.yaml",
        codes::MISSING_REQUIRED_FIELD,
        "support[0].url",
    );
}

#[test]
fn rejects_sla_schedule_when_scheduler_set() {
    assert_sections_error(
        "invalid-sla-schedule-without-scheduler.yaml",
        codes::MISSING_REQUIRED_FIELD,
        "slaProperties[0].schedule",
    );
}

#[test]
fn rejects_pricing_missing_currency() {
    assert_sections_error(
        "invalid-pricing-missing-currency.yaml",
        codes::MISSING_REQUIRED_FIELD,
        "price.priceCurrency",
    );
}

#[test]
fn rejects_pricing_negative_amount() {
    assert_sections_error(
        "invalid-pricing-negative-amount.yaml",
        codes::INVALID_SCHEMA,
        "price.priceAmount",
    );
}

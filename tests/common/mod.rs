//! Shared helpers and fixture lists for integration tests.

use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use odcs::parser::ParseResult;
use odcs::{parse, validate, DocumentFormat};
use serde_json::Value;

/// Valid ODCS fixtures that should parse and pass default validation.
pub const VALID_FIXTURES: &[&str] = &[
    "minimal.odcs.yaml",
    "minimal.odcs.json",
    "with-sla.yaml",
    "with-sla-description.yaml",
    "with-sla-default-element.yaml",
    "with-sla-default-element-multi.yaml",
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

/// Section coverage fixtures (valid corpus excluding minimal variants).
pub const SECTION_FIXTURES: &[&str] = &[
    "with-sla.yaml",
    "with-sla-description.yaml",
    "with-sla-default-element.yaml",
    "with-sla-default-element-multi.yaml",
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

/// Resolve a path under `tests/fixtures/`.
pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Read fixture bytes from `tests/fixtures/`.
pub fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(fixture_path(name)).expect("read fixture")
}

/// Infer document format from fixture extension.
pub fn format_for(name: &str) -> DocumentFormat {
    if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    }
}

/// Compile the pinned upstream ODCS JSON Schema validator.
pub fn pinned_validator() -> Validator {
    let content = fs::read_to_string("schema/odcs-v3.1.0.json").expect("read pinned schema");
    let schema_value: Value = serde_json::from_str(&content).expect("parse pinned schema");
    jsonschema::validator_for(&schema_value).expect("compile pinned schema")
}

/// Assert a fixture passes odcs validation and conforms to the pinned JSON Schema.
pub fn assert_valid_fixture_passes_odcs_and_json_schema(name: &str) {
    let format = format_for(name);
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
    let validator = pinned_validator();
    if !validator.is_valid(&instance) {
        let messages: Vec<String> = validator
            .iter_errors(&instance)
            .map(|error| error.to_string())
            .collect();
        panic!("fixture {name} failed JSON Schema conformance: {messages:?}");
    }
}

/// Recursively copy a directory tree.
pub fn copy_dir_all(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create destination dir");
    for entry in fs::read_dir(from).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let file_type = entry.file_type().expect("file type");
        let dest = to.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).expect("copy file");
        }
    }
}

//! Compatibility analysis tests.

use std::path::PathBuf;

use odcs::{codes, diff, parse_file, ChangeKind, DataContract};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/compatibility")
        .join(name)
}

fn load_fixture(name: &str) -> DataContract {
    let result = parse_file(fixture_path(name)).expect("parse fixture");
    result.contract.expect("valid fixture")
}

#[test]
fn diff_detects_breaking_property_removal() {
    let old = load_fixture("base.yaml");
    let new = load_fixture("breaking-removed-column.yaml");
    let report = diff(&old, &new);

    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking && change.path == "schema[customers].properties[email]"
    }));
}

#[test]
fn diff_detects_additive_property() {
    let old = load_fixture("base.yaml");
    let new = load_fixture("additive-new-column.yaml");
    let report = diff(&old, &new);

    assert!(!report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Additive && change.path == "schema[customers].properties[phone]"
    }));
}

#[test]
fn diff_is_compatible_without_breaking_changes() {
    let old = load_fixture("base.yaml");
    let new = load_fixture("additive-new-column.yaml");
    assert!(diff(&old, &new).is_compatible());
}

#[test]
fn diff_detects_breaking_when_optional_becomes_required() {
    let old = load_fixture("base.yaml");
    let mut new = old.clone();
    new.schema[0]
        .properties
        .iter_mut()
        .find(|p| p.element.name.as_deref() == Some("email"))
        .expect("email property")
        .required = true;

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking
            && change.path == "schema[customers].properties[email].required"
    }));
}

#[test]
fn diff_detects_breaking_logical_type_change() {
    let old = load_fixture("base.yaml");
    let mut new = old.clone();
    new.schema[0]
        .properties
        .iter_mut()
        .find(|p| p.element.name.as_deref() == Some("email"))
        .expect("email property")
        .logical_type = Some("integer".to_string());

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking
            && change.path == "schema[customers].properties[email].logicalType"
    }));
}

#[test]
fn diff_detects_breaking_quality_rule_removal() {
    let old = load_fixture("base.yaml");
    let mut new = old.clone();
    new.schema[0].quality = None;

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking
            && change.path == "schema[customers].quality[customer_id_not_null]"
    }));
}

#[test]
fn diff_detects_breaking_quality_metric_change() {
    let old = load_fixture("base.yaml");
    let mut new = old.clone();
    new.schema[0].quality.as_mut().expect("quality")[0].metric = Some("duplicateValues".to_string());

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking
            && change.path == "schema[customers].quality[customer_id_not_null].metric"
    }));
}

#[test]
fn diff_detects_breaking_schema_object_removal() {
    let old = load_fixture("base.yaml");
    let mut new = old.clone();
    new.schema.clear();

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking && change.path == "schema[customers]"
    }));
}

#[test]
fn diff_detects_additive_required_property_as_breaking() {
    let old = load_fixture("base.yaml");
    let mut new = load_fixture("additive-new-column.yaml");
    new.schema[0]
        .properties
        .iter_mut()
        .find(|p| p.element.name.as_deref() == Some("phone"))
        .expect("phone property")
        .required = true;

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking && change.path == "schema[customers].properties[phone]"
    }));
}

#[test]
fn diff_detects_breaking_relationship_target_change() {
    let result = parse_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/with-relationships.yaml"),
    )
    .expect("parse relationships fixture");
    let old = result.contract.expect("valid fixture");
    let mut new = old.clone();
    new.schema[0].relationships[0].to =
        odcs::model::RelationshipEndpoint::Single("customers.order_id".to_string());

    let report = diff(&old, &new);
    assert!(report.has_breaking);
    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking && change.path.contains("relationships")
    }));
}

#[test]
fn diff_changes_include_stable_diagnostic_codes() {
    let old = load_fixture("base.yaml");
    let new = load_fixture("breaking-removed-column.yaml");
    let report = diff(&old, &new);

    assert!(report.changes.iter().any(|change| {
        change.kind == ChangeKind::Breaking && change.code == codes::COMPATIBILITY_BREAKING
    }));
}

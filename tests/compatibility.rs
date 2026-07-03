//! Compatibility analysis tests.

use std::path::PathBuf;

use odcs::{diff, parse_file, ChangeKind};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/compatibility")
        .join(name)
}

fn load_fixture(name: &str) -> odcs::DataContract {
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

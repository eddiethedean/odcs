//! Cross-file reference resolution tests.

use std::path::PathBuf;

use odcs::{codes, load_set, parse_and_validate_set, validate_set, ValidationPhase};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn cross_file_fqn_resolves_with_dependency() {
    let primary = fixture_path("cross-file/consumer-valid.yaml");
    let provider = fixture_path("cross-file/provider.yaml");
    let set = load_set(&primary, &[provider], &[]).expect("load set");
    let report = validate_set(&set);
    assert!(
        report.is_valid(),
        "expected valid cross-file set: {:?}",
        report.diagnostics
    );
}

#[test]
fn cross_file_fqn_fails_without_dependency() {
    let primary = fixture_path("cross-file/consumer-invalid.yaml");
    let report = parse_and_validate_set(&primary, &[], &[]);
    assert!(
        !report.is_valid(),
        "expected invalid without dependency: {:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|d| {
            d.id == codes::UNRESOLVED_REFERENCE
                && d.validation_phase == Some(ValidationPhase::References)
                && d.object_ref.as_deref() == Some("schema[0].relationships[0].to")
        }),
        "expected unresolved FQN diagnostic: {:?}",
        report.diagnostics
    );
}

#[test]
fn parse_and_validate_set_with_dep_is_valid() {
    let primary = fixture_path("cross-file/consumer-valid.yaml");
    let provider = fixture_path("cross-file/provider.yaml");
    let report = parse_and_validate_set(&primary, &[provider], &[]);
    assert!(
        report.is_valid(),
        "expected valid with dependency: {:?}",
        report.diagnostics
    );
}

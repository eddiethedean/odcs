//! Cross-file reference resolution tests.

mod common;

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use odcs::{codes, load_set, parse_and_validate_set, ValidationPhase};

use common::fixture_path;

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_include_dir_with_provider() -> PathBuf {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("odcs-include-test-{}-{}", std::process::id(), id));
    fs::create_dir_all(&dir).expect("create include dir");
    fs::copy(
        fixture_path("cross-file/provider.yaml"),
        dir.join("provider.yaml"),
    )
    .expect("copy provider");
    dir
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

#[test]
fn include_dir_resolves_fqn_without_explicit_dep() {
    let include_dir = temp_include_dir_with_provider();
    let primary = fixture_path("cross-file/consumer-valid.yaml");
    let report = parse_and_validate_set(&primary, &[], std::slice::from_ref(&include_dir));
    assert!(
        report.is_valid(),
        "expected valid with include dir: {:?}",
        report.diagnostics
    );
    let _ = fs::remove_dir_all(&include_dir);
}

#[test]
fn duplicate_contract_id_in_set_is_rejected() {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("odcs-dup-id-test-{}-{}", std::process::id(), id));
    fs::create_dir_all(&dir).expect("create temp dir");

    let contract_yaml = r#"version: "3.1.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "duplicate-id-contract"
status: "active"
schema:
  - name: "customers"
    logicalType: "object"
    properties:
      - name: "customer_id"
        logicalType: "string"
"#;
    let primary = dir.join("primary.yaml");
    let dep = dir.join("dep.yaml");
    fs::write(&primary, contract_yaml).expect("write primary");
    fs::write(&dep, contract_yaml).expect("write dep");

    let result = load_set(&primary, std::slice::from_ref(&dep), &[]);
    assert!(result.is_err(), "expected duplicate id rejection");
    let report = result.expect_err("duplicate id report");
    assert!(
        report.diagnostics.iter().any(|d| {
            d.id == codes::INVALID_SCHEMA
                && d.validation_phase == Some(ValidationPhase::Document)
                && d.object_ref.as_deref() == Some("id")
        }),
        "expected duplicate id diagnostic: {:?}",
        report.diagnostics
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn include_dir_not_directory_returns_error() {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let file_path =
        std::env::temp_dir().join(format!("odcs-not-dir-{}-{}", std::process::id(), id));
    fs::write(&file_path, "not a directory").expect("write file");

    let primary = fixture_path("cross-file/consumer-valid.yaml");
    let result = load_set(&primary, &[], std::slice::from_ref(&file_path));
    assert!(result.is_err());
    let _ = fs::remove_file(&file_path);
}

#[test]
fn missing_dep_path_returns_error() {
    let primary = fixture_path("cross-file/consumer-valid.yaml");
    let missing = PathBuf::from("/nonexistent/odcs-dep-path.yaml");
    let result = load_set(&primary, std::slice::from_ref(&missing), &[]);
    assert!(result.is_err(), "expected error for missing dependency path");
    let report = result.expect_err("missing dep report");
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.message.contains("failed to resolve dependency path")),
        "expected resolve failure diagnostic: {:?}",
        report.diagnostics
    );
}

//! CLI integration tests.

use std::path::PathBuf;
use std::process::Command;

use odcs::{codes, UPSTREAM_SPEC_VERSION};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn odcs_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_odcs"))
}

#[test]
fn cli_version_succeeds() {
    let output = odcs_bin().arg("version").output().expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    assert!(stdout.contains(UPSTREAM_SPEC_VERSION));
}

#[test]
fn cli_validate_succeeds_on_minimal() {
    let path = fixture("minimal.odcs.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("valid"));
}

#[test]
fn cli_validate_fails_on_invalid_contract() {
    let path = fixture("invalid-kind.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn cli_validate_parse_failure_exits_2() {
    let path = fixture("malformed.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn cli_validate_json_output() {
    let path = fixture("minimal.odcs.yaml");
    let output = odcs_bin()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\""));
    assert!(stdout.contains("\"diagnostics\""));
}

#[test]
fn cli_inspect_succeeds_on_minimal() {
    let path = fixture("minimal.odcs.yaml");
    let output = odcs_bin()
        .arg("inspect")
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("customer_data_contract"));
}

#[test]
fn cli_diagnostics_json_output_on_invalid_contract() {
    let path = fixture("invalid-kind.yaml");
    let output = odcs_bin()
        .args(["diagnostics", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"diagnostics\""));
    assert!(stdout.contains(codes::INVALID_SCHEMA));
}

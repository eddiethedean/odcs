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
fn cli_version_json_output() {
    let output = odcs_bin()
        .args(["version", "--json"])
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"crateVersion\""));
    assert!(stdout.contains("\"upstreamSpecVersion\""));
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
fn cli_validate_duplicate_key_exits_2() {
    let path = fixture("invalid-nested-duplicate-key.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stderr}{stdout}");
    assert!(combined.contains("duplicate key"));
}

#[test]
fn cli_inspect_parse_failure_exits_2() {
    let path = fixture("malformed.yaml");
    let output = odcs_bin()
        .arg("inspect")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn cli_diagnostics_parse_failure_exits_2() {
    let path = fixture("malformed.yaml");
    let output = odcs_bin()
        .arg("diagnostics")
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
fn cli_inspect_json_output() {
    let path = fixture("minimal.odcs.yaml");
    let output = odcs_bin()
        .args(["inspect", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"id\""));
    assert!(stdout.contains("\"apiVersion\""));
    assert!(stdout.contains("\"qualityCount\""));
    assert!(stdout.contains("\"schemaCount\""));
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
    assert!(stdout.contains(codes::INVALID_KIND));
}

#[test]
fn cli_validate_text_includes_object_ref_for_version_errors() {
    let path = fixture("unsupported-version.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("at: version") || stdout.contains("at: apiVersion"));
}

#[test]
fn cli_schema_command_succeeds() {
    let output = odcs_bin().arg("schema").output().expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"$schema\"") || stdout.contains("\"title\""));
}

#[test]
fn cli_schema_url_only_command_succeeds() {
    let output = odcs_bin()
        .args(["schema", "--url-only"])
        .output()
        .expect("run cli");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Upstream ODCS JSON Schema"));
}

#[test]
fn cli_schema_json_output() {
    let output = odcs_bin()
        .args(["schema", "--json"])
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"schemaVersion\""));
    assert!(stdout.contains("\"schema\""));
}

#[test]
fn cli_validate_strict_fails_on_json_schema_violation() {
    let path = fixture("invalid-json-schema-only.yaml");
    let output = odcs_bin()
        .args(["validate", "--strict"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).contains(codes::JSON_SCHEMA_VIOLATION));
}

#[test]
fn cli_validate_json_includes_validation_phase() {
    let path = fixture("invalid-kind.yaml");
    let output = odcs_bin()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"validationPhase\""));
    assert!(stdout.contains("\"document\""));
}

#[test]
fn cli_validate_parse_failure_omits_validation_phase() {
    let path = fixture("invalid-nested-duplicate-key.yaml");
    let output = odcs_bin()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("\"validationPhase\""));
}

#[test]
fn cli_validate_text_includes_phase_for_validation_errors() {
    let path = fixture("invalid-kind.yaml");
    let output = odcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("phase: document"));
}

#[test]
fn cli_validate_strict_passes_on_minimal() {
    let path = fixture("minimal.odcs.yaml");
    let output = odcs_bin()
        .args(["validate", "--strict"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
}

#[test]
fn cli_diff_reports_breaking_exit_code() {
    let old = fixture("compatibility/base.yaml");
    let new = fixture("compatibility/breaking-removed-column.yaml");
    let output = odcs_bin()
        .args(["diff", &old.to_string_lossy(), &new.to_string_lossy()])
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("odcs:compatibility-breaking"));
}

#[test]
fn cli_validate_cross_file_with_dep_succeeds() {
    let primary = fixture("cross-file/consumer-valid.yaml");
    let provider = fixture("cross-file/provider.yaml");
    let output = odcs_bin()
        .args([
            "validate",
            &primary.to_string_lossy(),
            "--dep",
            &provider.to_string_lossy(),
        ])
        .output()
        .expect("run cli");
    assert!(output.status.success());
}

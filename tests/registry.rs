//! Registry integration tests.

mod common;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use odcs::{
    index_and_save_registry, load_registry, load_set_with_registry, validate_set, Registry,
};

use common::{copy_dir_all, fixture_path as fixture};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn isolated_contracts_root() -> PathBuf {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("odcs-registry-test-{}-{}", std::process::id(), id));
    copy_dir_all(&fixture("registry/contracts"), &dir);
    dir
}

fn index_isolated() -> (PathBuf, Registry) {
    let root = isolated_contracts_root();
    let (registry, report) = index_and_save_registry(&root).expect("index");
    assert!(report.is_valid());
    (root, registry)
}

#[test]
fn recursive_index_finds_nested_contracts() {
    let (root, registry) = index_isolated();
    let ids: Vec<_> = registry.list().iter().map(|e| e.id.as_str()).collect();
    assert!(ids.contains(&"provider-contract"));
    assert!(ids.contains(&"nested-registry-consumer"));
    assert_eq!(registry.list().len(), 3);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn lookup_prefers_highest_semver() {
    let (root, registry) = index_isolated();
    let entry = registry.lookup("provider-contract").expect("entry");
    assert_eq!(entry.version, "2.0.0");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn lookup_version_is_exact() {
    let (root, registry) = index_isolated();
    let entry = registry
        .lookup_version("provider-contract", "1.0.0")
        .expect("entry");
    assert_eq!(entry.path.to_string_lossy(), "provider-v1.yaml");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn load_registry_reads_persisted_index() {
    let root = isolated_contracts_root();
    index_and_save_registry(&root).expect("index");
    let registry = load_registry(&root).expect("load");
    assert!(!registry.list().is_empty());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn validate_with_registry_resolves_fqn_without_dep() {
    let root = isolated_contracts_root();
    index_and_save_registry(&root).expect("index");
    let registry = load_registry(&root).expect("load");
    let primary = fixture("registry/consumer.yaml");
    let set = load_set_with_registry(&primary, &[], &[], Some(&registry)).expect("load set");
    let report = validate_set(&set);
    assert!(
        report.is_valid(),
        "expected valid registry-backed set: {:?}",
        report.diagnostics
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn duplicate_id_version_fails_index() {
    let dir = std::env::temp_dir().join(format!("odcs-registry-dup-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp dir");

    let v1 = dir.join("dup-v1.yaml");
    let v2 = dir.join("dup-v2.yaml");
    std::fs::write(
        &v1,
        r#"version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "dup-contract"
status: "active"
"#,
    )
    .expect("write");
    std::fs::write(
        &v2,
        r#"version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "dup-contract"
status: "active"
"#,
    )
    .expect("write");

    let result = Registry::index_directory(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(result.is_err());
}

#[test]
#[cfg(unix)]
fn dependency_paths_reject_symlink_outside_registry_root() {
    use std::os::unix::fs::symlink;

    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "odcs-registry-escape-test-{}-{}",
        std::process::id(),
        id
    ));
    std::fs::create_dir_all(&root).expect("create root");
    let outside = std::env::temp_dir().join(format!(
        "odcs-registry-outside-{}-{}",
        std::process::id(),
        id
    ));
    std::fs::write(
        &outside,
        r#"version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "outside-contract"
status: "active"
"#,
    )
    .expect("write outside contract");

    symlink(&outside, root.join("escape.yaml")).expect("symlink");

    let result = Registry::index_directory(&root);
    assert!(result.is_err(), "expected index failure for escape symlink");
    let report = result.expect_err("escape report");
    assert!(
        report.diagnostics.iter().any(|d| {
            d.message.contains("outside registry root")
                && d.object_ref.as_deref() == Some("escape.yaml")
        }),
        "expected outside-root diagnostic: {:?}",
        report.diagnostics
    );

    let (isolated_root, registry) = index_isolated();
    assert!(
        registry
            .dependency_paths(&isolated_root.join("missing.yaml"))
            .iter()
            .all(|path| path.starts_with(registry.root())),
        "dependency paths must stay within registry root"
    );

    let _ = std::fs::remove_file(root.join("escape.yaml"));
    let _ = std::fs::remove_file(&outside);
    let _ = std::fs::remove_dir_all(&root);
}

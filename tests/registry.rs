//! Registry integration tests.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use odcs::{
    index_and_save_registry, load_registry, load_set_with_registry, validate_set, Registry,
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn copy_dir_all(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("create temp dir");
    for entry in std::fs::read_dir(from).expect("read fixture dir") {
        let entry = entry.expect("dir entry");
        let file_type = entry.file_type().expect("file type");
        let dest = to.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest);
        } else {
            std::fs::copy(entry.path(), dest).expect("copy fixture file");
        }
    }
}

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

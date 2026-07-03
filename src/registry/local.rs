//! Local filesystem registry backend.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    ValidationPhase,
};
use crate::parser::parse_file;

use super::entry::{RegistryEntry, RegistryIndexFile};
use super::scan::collect_contract_files_recursive;

const REGISTRY_DIR: &str = ".odcs";
const REGISTRY_FILE: &str = "registry.json";

/// Maximum size of a persisted `.odcs/registry.json` file (16 MiB).
pub const MAX_REGISTRY_INDEX_BYTES: u64 = 16 * 1024 * 1024;

/// Local contract registry backed by `.odcs/registry.json`.
#[derive(Debug, Clone)]
pub struct Registry {
    root: PathBuf,
    entries: Vec<RegistryEntry>,
}

impl Registry {
    /// Registry root directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// All indexed entries sorted by `id` then `version`.
    #[must_use]
    pub fn list(&self) -> &[RegistryEntry] {
        &self.entries
    }

    /// Path to the on-disk index file for a registry root.
    #[must_use]
    pub fn index_path(root: &Path) -> PathBuf {
        root.join(REGISTRY_DIR).join(REGISTRY_FILE)
    }

    /// Scan `dir` recursively and build a new registry (does not write to disk).
    pub fn index_directory(dir: &Path) -> Result<(Self, DiagnosticReport), DiagnosticReport> {
        let canonical_root = dir.canonicalize().map_err(|error| {
            io_error_report(&format!("failed to resolve {}: {error}", dir.display()))
        })?;

        if !canonical_root.is_dir() {
            return Err(io_error_report(&format!(
                "registry root is not a directory: {}",
                dir.display()
            )));
        }

        let mut report = DiagnosticReport::new();
        let relative_paths = collect_contract_files_recursive(&canonical_root)
            .map_err(|error| io_error_report(&error))?;

        let mut entries = Vec::new();
        let mut seen_keys = HashSet::new();

        for relative_path in relative_paths {
            verbose_progress(&format!("indexing {}", relative_path.display()));
            let absolute_path = canonical_root.join(&relative_path);
            if resolve_path_within_root(&canonical_root, &absolute_path).is_none() {
                emit(
                    &mut report,
                    validation_error(
                        ValidationPhase::Document,
                        codes::INVALID_SCHEMA,
                        DiagnosticCategory::Structure,
                        format!(
                            "registry entry {} resolves outside registry root",
                            relative_path.display()
                        ),
                    )
                    .with_object_ref(relative_path.to_string_lossy().into_owned()),
                );
                continue;
            }
            match build_entry(&canonical_root, &relative_path, &absolute_path) {
                Ok(entry) => {
                    let key = (entry.id.clone(), entry.version.clone());
                    if !seen_keys.insert(key.clone()) {
                        emit(
                            &mut report,
                            validation_error(
                                ValidationPhase::Document,
                                codes::INVALID_SCHEMA,
                                DiagnosticCategory::Structure,
                                format!(
                                    "duplicate registry entry for id '{}' version '{}'",
                                    entry.id, entry.version
                                ),
                            )
                            .with_object_ref(format!("{}#{}", entry.id, entry.version))
                            .with_remediation(
                                "each contract id and version pair must be unique in the registry",
                            ),
                        );
                        continue;
                    }
                    entries.push(entry);
                }
                Err(entry_report) => {
                    report.merge(entry_report);
                }
            }
        }

        entries.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then_with(|| compare_versions(&left.version, &right.version))
        });

        if !report.is_valid() {
            return Err(report);
        }

        verbose_progress(&format!(
            "indexed {} contract(s) under {}",
            entries.len(),
            canonical_root.display()
        ));

        Ok((
            Self {
                root: canonical_root,
                entries,
            },
            report,
        ))
    }

    /// Scan and persist the registry index to `<dir>/.odcs/registry.json`.
    pub fn index_and_save(dir: &Path) -> Result<(Self, DiagnosticReport), DiagnosticReport> {
        let (registry, report) = Self::index_directory(dir)?;
        registry.save()?;
        Ok((registry, report))
    }

    /// Load a registry from `<dir>/.odcs/registry.json`.
    pub fn load(dir: &Path) -> Result<Self, DiagnosticReport> {
        let canonical_root = dir.canonicalize().map_err(|error| {
            io_error_report(&format!("failed to resolve {}: {error}", dir.display()))
        })?;

        let index_path = Self::index_path(&canonical_root);
        let content = fs::read(&index_path).map_err(|error| {
            io_error_report(&format!(
                "failed to read registry index {}: {error}",
                index_path.display()
            ))
        })?;

        if content.len() as u64 > MAX_REGISTRY_INDEX_BYTES {
            return Err(io_error_report(&format!(
                "registry index {} exceeds maximum size of {MAX_REGISTRY_INDEX_BYTES} bytes",
                index_path.display()
            )));
        }

        let content = String::from_utf8(content).map_err(|error| {
            io_error_report(&format!(
                "registry index {} is not valid UTF-8: {error}",
                index_path.display()
            ))
        })?;

        let index: RegistryIndexFile = serde_json::from_str(&content).map_err(|error| {
            io_error_report(&format!(
                "failed to parse registry index {}: {error}",
                index_path.display()
            ))
        })?;

        let mut entries = index.entries;
        entries.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then_with(|| compare_versions(&left.version, &right.version))
        });

        Ok(Self {
            root: canonical_root,
            entries,
        })
    }

    /// Persist the registry index to disk.
    pub fn save(&self) -> Result<(), DiagnosticReport> {
        let index_dir = self.root.join(REGISTRY_DIR);
        fs::create_dir_all(&index_dir).map_err(|error| {
            io_error_report(&format!(
                "failed to create registry directory {}: {error}",
                index_dir.display()
            ))
        })?;

        let index_path = Self::index_path(&self.root);
        let payload = RegistryIndexFile::new(self.entries.clone());
        let json = serde_json::to_string_pretty(&payload).map_err(|error| {
            io_error_report(&format!("failed to serialize registry index: {error}"))
        })?;

        if json.len() as u64 > MAX_REGISTRY_INDEX_BYTES {
            return Err(io_error_report(&format!(
                "registry index exceeds maximum size of {MAX_REGISTRY_INDEX_BYTES} bytes"
            )));
        }

        let temp_path = index_path.with_extension("json.tmp");
        fs::write(&temp_path, &json).map_err(|error| {
            io_error_report(&format!(
                "failed to write registry index {}: {error}",
                temp_path.display()
            ))
        })?;
        fs::rename(&temp_path, &index_path).map_err(|error| {
            let _ = fs::remove_file(&temp_path);
            io_error_report(&format!(
                "failed to write registry index {}: {error}",
                index_path.display()
            ))
        })?;

        Ok(())
    }

    /// Register a single entry, rejecting duplicate `(id, version)`.
    pub fn register(&mut self, entry: RegistryEntry) -> Result<(), DiagnosticReport> {
        if self
            .entries
            .iter()
            .any(|existing| existing.id == entry.id && existing.version == entry.version)
        {
            let mut report = DiagnosticReport::new();
            emit(
                &mut report,
                validation_error(
                    ValidationPhase::Document,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!(
                        "duplicate registry entry for id '{}' version '{}'",
                        entry.id, entry.version
                    ),
                ),
            );
            return Err(report);
        }
        self.entries.push(entry);
        self.entries.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then_with(|| compare_versions(&left.version, &right.version))
        });
        Ok(())
    }

    /// Lookup the best-matching entry for `id` (highest semver, else lexicographic max).
    #[must_use]
    pub fn lookup(&self, id: &str) -> Option<&RegistryEntry> {
        let matches: Vec<&RegistryEntry> = self.entries.iter().filter(|e| e.id == id).collect();
        select_best_version(matches)
    }

    /// Lookup an exact `(id, version)` entry.
    #[must_use]
    pub fn lookup_version(&self, id: &str, version: &str) -> Option<&RegistryEntry> {
        self.entries
            .iter()
            .find(|entry| entry.id == id && entry.version == version)
    }

    /// Absolute paths for indexed contracts, excluding `exclude` when it matches an entry path.
    ///
    /// When multiple entries share the same `id`, only the best-version entry is included
    /// (same semantics as [`Self::lookup`]).
    #[must_use]
    pub fn dependency_paths(&self, exclude: &Path) -> Vec<PathBuf> {
        let exclude = exclude.canonicalize().ok();
        let mut by_id: HashMap<String, &RegistryEntry> = HashMap::new();

        for entry in &self.entries {
            match by_id.get(&entry.id) {
                None => {
                    by_id.insert(entry.id.clone(), entry);
                }
                Some(current) => {
                    if compare_versions(&entry.version, &current.version)
                        == std::cmp::Ordering::Greater
                    {
                        by_id.insert(entry.id.clone(), entry);
                    }
                }
            }
        }

        let mut paths: Vec<PathBuf> = by_id
            .values()
            .filter_map(|entry| {
                let absolute = self.root.join(entry.path.as_path());
                let canonical = resolve_path_within_root(&self.root, &absolute)?;
                if exclude.as_ref().is_some_and(|ex| ex == &canonical) {
                    return None;
                }
                Some(canonical)
            })
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }
}

fn build_entry(
    _root: &Path,
    relative_path: &Path,
    absolute_path: &Path,
) -> Result<RegistryEntry, DiagnosticReport> {
    let bytes = fs::read(absolute_path).map_err(|error| {
        io_error_report(&format!(
            "failed to read contract {}: {error}",
            absolute_path.display()
        ))
    })?;

    let parse_result = parse_file(absolute_path).map_err(|error| {
        io_error_report(&format!(
            "failed to parse contract {}: {error}",
            absolute_path.display()
        ))
    })?;

    let contract = parse_result.contract.ok_or_else(|| {
        let mut report = parse_result.report;
        if report.is_valid() {
            report = io_error_report(&format!(
                "failed to parse contract {}",
                absolute_path.display()
            ));
        }
        report
    })?;

    if contract.id.is_empty() {
        let mut report = DiagnosticReport::new();
        emit(
            &mut report,
            validation_error(
                ValidationPhase::Document,
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                format!(
                    "contract at {} must have a non-empty id",
                    relative_path.display()
                ),
            )
            .with_object_ref("id"),
        );
        return Err(report);
    }

    Ok(RegistryEntry {
        id: contract.id,
        version: contract.version,
        path: relative_path.to_path_buf(),
        api_version: contract.api_version,
        tags: contract.tags,
        content_hash: sha256_hex(&bytes),
        indexed_at: Some(time::now_rfc3339()),
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn select_best_version(entries: Vec<&RegistryEntry>) -> Option<&RegistryEntry> {
    if entries.is_empty() {
        return None;
    }

    let mut semver_entries: Vec<(semver::Version, &RegistryEntry)> = Vec::new();
    let mut fallback: Option<&RegistryEntry> = None;

    for entry in entries {
        if let Ok(version) = semver::Version::parse(&entry.version) {
            semver_entries.push((version, entry));
        } else if fallback.map_or(true, |current| entry.version > current.version) {
            fallback = Some(entry);
        }
    }

    if let Some((_, entry)) = semver_entries.into_iter().max_by(|a, b| a.0.cmp(&b.0)) {
        Some(entry)
    } else {
        fallback
    }
}

fn compare_versions(left: &str, right: &str) -> std::cmp::Ordering {
    match (semver::Version::parse(left), semver::Version::parse(right)) {
        (Ok(left), Ok(right)) => left.cmp(&right),
        _ => left.cmp(right),
    }
}

/// Returns the canonical path when `candidate` resolves inside `root`.
fn resolve_path_within_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = root.canonicalize().ok()?;
    let canonical = candidate.canonicalize().ok()?;
    if canonical.starts_with(&canonical_root) {
        Some(canonical)
    } else {
        None
    }
}

fn io_error_report(message: &str) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    emit(
        &mut report,
        crate::diagnostics::Diagnostic::error(
            codes::PARSE_YAML,
            DiagnosticCategory::Syntax,
            DiagnosticStage::Parse,
            message.to_string(),
        ),
    );
    report
}

fn verbose_progress(message: &str) {
    if std::env::var_os("ODCS_VERBOSE").is_some() {
        eprintln!("odcs: {message}");
    }
}

mod time {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn now_rfc3339() -> String {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{}Z", duration.as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry/contracts")
    }

    #[test]
    fn index_directory_finds_nested_contracts() {
        let (registry, _report) = Registry::index_directory(&fixture_root()).expect("index");
        assert!(registry.entries.len() >= 3);
    }

    #[test]
    fn lookup_prefers_highest_semver() {
        let (registry, _) = Registry::index_directory(&fixture_root()).expect("index");
        let entry = registry.lookup("provider-contract").expect("entry");
        assert_eq!(entry.version, "2.0.0");
    }

    #[test]
    fn lookup_version_is_exact() {
        let (registry, _) = Registry::index_directory(&fixture_root()).expect("index");
        let entry = registry
            .lookup_version("provider-contract", "1.0.0")
            .expect("entry");
        assert!(entry.path.to_string_lossy().contains("provider-v1"));
    }
}

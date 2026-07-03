//! Cross-file contract loading and validation.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    ValidationPhase,
};
use crate::model::DataContract;
use crate::parser::parse_file;
use crate::registry::is_contract_file;
use crate::registry::Registry;
use crate::validation::{validate_with_contract_index, ContractIndex};

/// A primary contract and its loaded dependencies for cross-file validation.
#[derive(Debug, Clone)]
pub struct ContractSet {
    primary: DataContract,
    dependencies: Vec<DataContract>,
}

impl ContractSet {
    /// Returns the primary contract.
    #[must_use]
    pub fn primary(&self) -> &DataContract {
        &self.primary
    }

    /// Returns loaded dependency contracts (excluding the primary).
    #[must_use]
    pub fn dependencies(&self) -> &[DataContract] {
        &self.dependencies
    }

    /// Returns all contracts in load order: primary first, then dependencies.
    pub fn contracts(&self) -> impl Iterator<Item = &DataContract> {
        std::iter::once(&self.primary).chain(self.dependencies.iter())
    }

    /// Load a contract set from a primary path, explicit dependencies, and include directories.
    pub fn from_paths(
        primary_path: &Path,
        deps: &[PathBuf],
        include_dirs: &[PathBuf],
    ) -> Result<Self, DiagnosticReport> {
        Self::from_paths_with_registry(primary_path, deps, include_dirs, None)
    }

    /// Load a contract set with optional registry-backed dependencies.
    pub fn from_paths_with_registry(
        primary_path: &Path,
        deps: &[PathBuf],
        include_dirs: &[PathBuf],
        registry: Option<&Registry>,
    ) -> Result<Self, DiagnosticReport> {
        let mut report = DiagnosticReport::new();

        let primary_result = parse_file(primary_path).map_err(|error| {
            let mut report = DiagnosticReport::new();
            emit(
                &mut report,
                crate::diagnostics::Diagnostic::error(
                    codes::PARSE_YAML,
                    DiagnosticCategory::Syntax,
                    DiagnosticStage::Parse,
                    error.to_string(),
                ),
            );
            report
        })?;

        let primary = match primary_result.contract {
            Some(contract) => contract,
            None => {
                report.merge(primary_result.report);
                return Err(report);
            }
        };
        report.merge(primary_result.report);

        let mut dependencies = Vec::new();
        let dependency_paths = collect_dependency_paths(primary_path, deps, include_dirs, registry)
            .map_err(|error| {
                let mut report = DiagnosticReport::new();
                emit(
                    &mut report,
                    crate::diagnostics::Diagnostic::error(
                        codes::PARSE_YAML,
                        DiagnosticCategory::Syntax,
                        DiagnosticStage::Parse,
                        error,
                    ),
                );
                report
            })?;

        let primary_canonical = primary_path.canonicalize().ok();

        for path in dependency_paths {
            if primary_canonical
                .as_ref()
                .is_some_and(|primary| path == *primary)
            {
                continue;
            }
            let dep_result = parse_file(&path).map_err(|error| {
                let mut report = DiagnosticReport::new();
                emit(
                    &mut report,
                    crate::diagnostics::Diagnostic::error(
                        codes::PARSE_YAML,
                        DiagnosticCategory::Syntax,
                        DiagnosticStage::Parse,
                        error.to_string(),
                    ),
                );
                report
            })?;
            report.merge(dep_result.report.clone());
            if let Some(contract) = dep_result.contract {
                dependencies.push(contract);
            }
        }

        let set = Self {
            primary,
            dependencies,
        };

        report.merge(validate_duplicate_ids(&set));
        if report.is_valid() {
            Ok(set)
        } else {
            Err(report)
        }
    }
}

fn collect_dependency_paths(
    primary_path: &Path,
    deps: &[PathBuf],
    include_dirs: &[PathBuf],
    registry: Option<&Registry>,
) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let mut seen = HashSet::new();

    let mut push_path = |path: PathBuf| -> Result<(), String> {
        let canonical = path.canonicalize().map_err(|error| {
            format!(
                "failed to resolve dependency path {}: {error}",
                path.display()
            )
        })?;
        if seen.insert(canonical.clone()) {
            paths.push(canonical);
        }
        Ok(())
    };

    for dep in deps {
        push_path(dep.clone())?;
    }

    if let Some(registry) = registry {
        for path in registry.dependency_paths(primary_path) {
            push_path(path)?;
        }
    }

    for dir in include_dirs {
        if !dir.is_dir() {
            return Err(format!(
                "include path is not a directory: {}",
                dir.display()
            ));
        }
        let mut files = Vec::new();
        for entry in fs::read_dir(dir).map_err(|error| {
            format!(
                "failed to read include directory {}: {error}",
                dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read include directory {}: {error}",
                    dir.display()
                )
            })?;
            let path = entry.path();
            if path.is_file() && is_contract_file(&path) {
                files.push(path);
            }
        }
        files.sort();
        for path in files {
            push_path(path)?;
        }
    }

    Ok(paths)
}

fn validate_duplicate_ids(set: &ContractSet) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    let mut seen = HashSet::new();

    for contract in set.contracts() {
        if contract.id.is_empty() {
            continue;
        }
        if !seen.insert(contract.id.clone()) {
            emit(
                &mut report,
                validation_error(
                    ValidationPhase::Document,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!("duplicate contract id '{}'", contract.id),
                )
                .with_object_ref("id")
                .with_remediation("each contract in a loaded set must have a unique id"),
            );
        }
    }

    report
}

/// Validate all contracts in a set with cross-file reference resolution.
#[must_use]
pub fn validate_set(set: &ContractSet) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.merge(validate_duplicate_ids(set));

    let contract_refs: Vec<&DataContract> = set.contracts().collect();
    let contract_index = ContractIndex::from_contracts(&contract_refs);

    for contract in set.contracts() {
        report.merge(validate_with_contract_index(
            contract,
            Some(&contract_index),
        ));
    }

    report
}

/// Parse and validate a contract set from paths.
pub fn parse_and_validate_set(
    primary_path: &Path,
    deps: &[PathBuf],
    include_dirs: &[PathBuf],
) -> DiagnosticReport {
    parse_and_validate_set_with_registry(primary_path, deps, include_dirs, None)
}

/// Parse and validate a contract set with optional registry dependencies.
pub fn parse_and_validate_set_with_registry(
    primary_path: &Path,
    deps: &[PathBuf],
    include_dirs: &[PathBuf],
    registry: Option<&Registry>,
) -> DiagnosticReport {
    match ContractSet::from_paths_with_registry(primary_path, deps, include_dirs, registry) {
        Ok(set) => validate_set(&set),
        Err(report) => report,
    }
}

/// Parse a primary contract and dependencies without validating.
pub fn load_set(
    primary_path: &Path,
    deps: &[PathBuf],
    include_dirs: &[PathBuf],
) -> Result<ContractSet, DiagnosticReport> {
    ContractSet::from_paths(primary_path, deps, include_dirs)
}

/// Parse a primary contract and dependencies with optional registry.
pub fn load_set_with_registry(
    primary_path: &Path,
    deps: &[PathBuf],
    include_dirs: &[PathBuf],
    registry: Option<&Registry>,
) -> Result<ContractSet, DiagnosticReport> {
    ContractSet::from_paths_with_registry(primary_path, deps, include_dirs, registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::index_and_save_registry;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn loads_cross_file_set() {
        let primary = fixture_path("cross-file/consumer-valid.yaml");
        let provider = fixture_path("cross-file/provider.yaml");
        let set = ContractSet::from_paths(&primary, &[provider], &[]).expect("load set");
        assert_eq!(set.primary().id, "consumer-contract");
        assert_eq!(set.dependencies().len(), 1);
    }

    #[test]
    fn loads_registry_backed_set() {
        let contracts_root = fixture_path("registry/contracts");
        let (registry, _) = index_and_save_registry(&contracts_root).expect("index");
        let primary = fixture_path("registry/consumer.yaml");
        let set = ContractSet::from_paths_with_registry(&primary, &[], &[], Some(&registry))
            .expect("load set");
        let report = validate_set(&set);
        assert!(
            report.is_valid(),
            "expected valid registry-backed set: {:?}",
            report.diagnostics
        );
    }
}

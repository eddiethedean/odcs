//! Recursive contract file discovery.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum number of contract files collected during a registry scan.
pub const MAX_REGISTRY_CONTRACT_FILES: usize = 10_000;

/// Returns true when the path looks like an ODCS contract file.
#[must_use]
pub fn is_contract_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("yaml") | Some("yml") | Some("json")
    )
}

fn should_skip_dir(name: &str) -> bool {
    name.starts_with('.')
}

/// Recursively collect contract file paths under `root`, sorted lexicographically.
pub fn collect_contract_files_recursive(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let mut visited = HashSet::new();
    collect_recursive(root, root, &mut files, &mut visited)?;
    files.sort();
    Ok(files)
}

fn collect_recursive(
    root: &Path,
    current: &Path,
    out: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
) -> Result<(), String> {
    let canonical = current.canonicalize().map_err(|error| {
        format!(
            "failed to resolve directory {}: {error}",
            current.display()
        )
    })?;
    if !visited.insert(canonical) {
        return Err(format!(
            "directory cycle detected at {} (symlink loop?)",
            current.display()
        ));
    }

    for entry in fs::read_dir(current)
        .map_err(|error| format!("failed to read directory {}: {error}", current.display()))?
    {
        let entry = entry
            .map_err(|error| format!("failed to read directory {}: {error}", current.display()))?;
        let path = entry.path();
        if path.is_dir() {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if should_skip_dir(name) {
                continue;
            }
            collect_recursive(root, &path, out, visited)?;
        } else if path.is_file() && is_contract_file(&path) {
            if out.len() >= MAX_REGISTRY_CONTRACT_FILES {
                return Err(format!(
                    "registry scan exceeded maximum of {MAX_REGISTRY_CONTRACT_FILES} contract files under {}",
                    root.display()
                ));
            }
            out.push(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry/contracts")
    }

    #[test]
    fn recursive_scan_finds_nested_contracts() {
        let files = collect_contract_files_recursive(&fixture_root()).expect("scan");
        let names: Vec<_> = files
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();
        assert!(names
            .iter()
            .any(|name| name.contains("nested/consumer.yaml")));
        assert!(names.iter().any(|name| name.contains("provider-v1.yaml")));
    }
}

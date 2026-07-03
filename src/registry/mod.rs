//! Local contract registry for discovery and cross-file validation.

mod entry;
mod local;
mod scan;

pub use entry::RegistryEntry;
pub use local::{Registry, MAX_REGISTRY_INDEX_BYTES};
pub use scan::{collect_contract_files_recursive, is_contract_file, MAX_REGISTRY_CONTRACT_FILES};

use std::path::Path;

use crate::diagnostics::DiagnosticReport;

/// Scan a directory recursively and build a registry index in memory.
pub fn index_registry(dir: &Path) -> Result<(Registry, DiagnosticReport), DiagnosticReport> {
    Registry::index_directory(dir)
}

/// Scan a directory and write `<dir>/.odcs/registry.json`.
pub fn index_and_save_registry(
    dir: &Path,
) -> Result<(Registry, DiagnosticReport), DiagnosticReport> {
    Registry::index_and_save(dir)
}

/// Load a registry from `<dir>/.odcs/registry.json`.
pub fn load_registry(dir: &Path) -> Result<Registry, DiagnosticReport> {
    Registry::load(dir)
}

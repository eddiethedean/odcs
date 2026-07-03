"""Reference implementation of the Open Data Contract Standard (ODCS)."""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError, version

from pyodcs._native import diagnostic_codes as _diagnostic_codes
from pyodcs._native import validation_phases as _validation_phases
from pyodcs._native import inspect as _inspect
from pyodcs._native import inspect_summary as _inspect_summary
from pyodcs._native import parse_document as _parse_document
from pyodcs._native import parse_path as _parse_path
from pyodcs._native import pinned_schema as _pinned_schema
from pyodcs._native import quality_rules_count as _quality_rules_count
from pyodcs._native import upstream_spec_version as _upstream_spec_version
from pyodcs._native import validate_contract as _validate_contract
from pyodcs._native import validate_document as _validate_document
from pyodcs._native import diff_contracts as _diff_contracts
from pyodcs._native import parse_and_validate_paths as _parse_and_validate_paths
from pyodcs._native import registry_index as _registry_index
from pyodcs._native import registry_index_and_save as _registry_index_and_save
from pyodcs._native import registry_load as _registry_load
from pyodcs._native import registry_lookup as _registry_lookup
from pyodcs._native import registry_list as _registry_list

UPSTREAM_SPEC_VERSION = _upstream_spec_version()
UPSTREAM_REPOSITORY_URL = "https://github.com/bitol-io/open-data-contract-standard"
CODES = _diagnostic_codes()
VALIDATION_PHASES = _validation_phases()

try:
    __version__ = version("pyodcs")
except PackageNotFoundError:
    __version__ = "0.9.1"


def parse(content: str | bytes, format: str = "yaml") -> dict:
    """Parse an ODCS document from YAML or JSON text."""
    return _parse_document(content, format)


def parse_file(path: str) -> dict:
    """Parse an ODCS document from a file path."""
    return _parse_path(path)


def validate(contract: dict) -> dict:
    """Validate a parsed data contract."""
    return _validate_contract(contract)


def validate_result(result: dict) -> dict:
    """Merge parse-time and validation diagnostics from a parse result."""
    if not isinstance(result, dict):
        raise TypeError("validate_result expects a dict")

    if result.get("_odcs_validated"):
        return {"diagnostics": list(result.get("diagnostics", []))}

    if "report" in result:
        report = result.get("report") or {}
        diagnostics = list(report.get("diagnostics", []))
        contract = result.get("contract")
        if contract is not None:
            validation = _validate_contract(contract)
            diagnostics.extend(validation.get("diagnostics", []))
        merged = {
            "diagnostics": diagnostics,
            "_odcs_validated": True,
        }
        return merged

    if "diagnostics" in result and "contract" not in result:
        return {
            "diagnostics": list(result.get("diagnostics", [])),
            "_odcs_validated": True,
        }

    raise TypeError(
        "validate_result expects a parse result with 'report' or a validation report with 'diagnostics'"
    )


def diff(old: dict, new: dict) -> dict:
    """Compare two parsed contracts and return a compatibility report."""
    return _diff_contracts(old, new)


def parse_and_validate_paths(
    primary: str,
    deps: list[str] | None = None,
    *,
    includes: list[str] | None = None,
    registry: str | None = None,
) -> dict:
    """Parse and validate a primary contract with optional dependency paths."""
    return _parse_and_validate_paths(primary, deps, includes, registry)


def registry_index(directory: str) -> dict:
    """Scan a directory recursively and build a registry index in memory."""
    return _registry_index(directory)


def registry_index_and_save(directory: str) -> dict:
    """Scan a directory and write `.odcs/registry.json`."""
    return _registry_index_and_save(directory)


def registry_load(directory: str) -> list[dict]:
    """Load registry entries from `.odcs/registry.json`."""
    return _registry_load(directory)


def registry_lookup(
    directory: str, id: str, version: str | None = None
) -> dict | None:
    """Look up a registry entry by id (and optional version)."""
    return _registry_lookup(directory, id, version)


def registry_list(directory: str) -> list[dict]:
    """List all entries in a registry index."""
    return _registry_list(directory)


def parse_and_validate(content: str | bytes, format: str = "yaml") -> dict:
    """Parse and validate an ODCS document in one step."""
    return _validate_document(content, format)


def pinned_schema(*, json_metadata: bool = False) -> dict:
    """Return the pinned ODCS JSON Schema."""
    return _pinned_schema(json_metadata)


def inspect(contract: dict) -> str:
    """Return a short human-readable contract summary."""
    return _inspect(contract)


def inspect_summary(contract: dict) -> dict:
    """Return inspect summary fields for a parsed contract."""
    return _inspect_summary(contract)


def quality_rules_count(contract: dict) -> int:
    """Return the number of nested quality rules in a contract."""
    return _quality_rules_count(contract)


def is_valid(report: dict) -> bool:
    """Return True when a diagnostic report contains no error-level diagnostics."""
    if "report" in report and "diagnostics" not in report:
        report = report.get("report") or {}
    return not any(
        diagnostic.get("severity", "error") == "error"
        for diagnostic in report.get("diagnostics", [])
    )


__all__ = [
    "CODES",
    "UPSTREAM_REPOSITORY_URL",
    "UPSTREAM_SPEC_VERSION",
    "__version__",
    "diff",
    "inspect",
    "inspect_summary",
    "is_valid",
    "parse",
    "parse_and_validate",
    "parse_and_validate_paths",
    "registry_index",
    "registry_index_and_save",
    "registry_load",
    "registry_lookup",
    "registry_list",
    "parse_file",
    "pinned_schema",
    "quality_rules_count",
    "validate",
    "validate_result",
]

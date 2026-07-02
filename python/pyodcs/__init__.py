"""Reference implementation of the Open Data Contract Standard (ODCS)."""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError, version

from pyodcs._native import inspect as _inspect
from pyodcs._native import inspect_summary as _inspect_summary
from pyodcs._native import parse_document as _parse_document
from pyodcs._native import parse_path as _parse_path
from pyodcs._native import quality_rules_count as _quality_rules_count
from pyodcs._native import upstream_spec_version as _upstream_spec_version
from pyodcs._native import validate_contract as _validate_contract
from pyodcs._native import validate_document as _validate_document

UPSTREAM_SPEC_VERSION = _upstream_spec_version()

try:
    __version__ = version("pyodcs")
except PackageNotFoundError:
    __version__ = "0.3.0"


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
    diagnostics = list(result.get("report", {}).get("diagnostics", []))
    contract = result.get("contract")
    if contract is not None:
        validation = _validate_contract(contract)
        diagnostics.extend(validation.get("diagnostics", []))
    return {"diagnostics": diagnostics}


def parse_and_validate(content: str | bytes, format: str = "yaml") -> dict:
    """Parse and validate an ODCS document in one step."""
    return _validate_document(content, format)


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
    return not any(
        diagnostic.get("severity") == "error"
        for diagnostic in report.get("diagnostics", [])
    )


__all__ = [
    "UPSTREAM_SPEC_VERSION",
    "__version__",
    "inspect",
    "inspect_summary",
    "is_valid",
    "parse",
    "parse_and_validate",
    "parse_file",
    "quality_rules_count",
    "validate",
    "validate_result",
]

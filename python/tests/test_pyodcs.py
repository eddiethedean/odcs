from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import pyodcs

PACKAGE_ROOT = Path(__file__).resolve().parent
FIXTURES = Path(__file__).resolve().parents[2] / "tests" / "fixtures"
EXAMPLE = Path(__file__).resolve().parents[2] / "examples" / "minimal.odcs.yaml"


def _fixture(name: str) -> bytes:
    return FIXTURES.joinpath(name).read_bytes()


def test_upstream_spec_version() -> None:
    assert pyodcs.UPSTREAM_SPEC_VERSION == "3.1.0"
    assert pyodcs.__version__ == "0.3.0"


def test_parse_valid_yaml_fixture() -> None:
    result = pyodcs.parse(_fixture("minimal.odcs.yaml"), "yaml")
    assert pyodcs.is_valid(result["report"])
    contract = result["contract"]
    assert contract is not None
    assert contract["name"] == "customer_data_contract"


def test_parse_and_validate_repo_example() -> None:
    content = EXAMPLE.read_bytes()
    report = pyodcs.parse_and_validate(content, "yaml")
    assert pyodcs.is_valid(report)


def test_parse_file_repo_example() -> None:
    result = pyodcs.parse_file(str(EXAMPLE))
    assert pyodcs.is_valid(result["report"])
    contract = result["contract"]
    assert contract is not None
    assert contract["kind"] == "DataContract"


def test_validate_result_merges_parse_and_validation_diagnostics() -> None:
    result = pyodcs.parse(
        b"version: '3.1.0'\napiVersion: v9.9.9\nkind: wrong\nid: ''\nstatus: draft\n",
        "yaml",
    )
    report = pyodcs.validate_result(result)
    assert not pyodcs.is_valid(report)


def test_inspect_contract() -> None:
    result = pyodcs.parse(_fixture("minimal.odcs.yaml"), "yaml")
    contract = result["contract"]
    assert contract is not None
    summary = pyodcs.inspect(contract)
    assert "customer_data_contract" in summary
    assert "customer-data-contract" in summary


def test_inspect_summary_matches_rust_fields() -> None:
    result = pyodcs.parse(_fixture("minimal.odcs.yaml"), "yaml")
    contract = result["contract"]
    assert contract is not None
    summary = pyodcs.inspect_summary(contract)
    assert summary["id"] == "customer-data-contract"
    assert summary["apiVersion"] == "v3.1.0"
    assert summary["qualityCount"] == 1
    assert summary["schemaCount"] == 1


def test_quality_rules_count() -> None:
    result = pyodcs.parse(_fixture("minimal.odcs.yaml"), "yaml")
    contract = result["contract"]
    assert contract is not None
    assert pyodcs.quality_rules_count(contract) == 1


def _run_pyodcs_cli(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, "-m", "pyodcs", *args],
        cwd=Path(__file__).resolve().parents[2],
        capture_output=True,
        text=True,
        check=False,
    )


def test_cli_validate_success() -> None:
    result = _run_pyodcs_cli("validate", str(FIXTURES / "minimal.odcs.yaml"))
    assert result.returncode == 0
    assert "valid" in result.stdout


def test_cli_validate_invalid_contract_exits_1() -> None:
    result = _run_pyodcs_cli("validate", str(FIXTURES / "invalid-kind.yaml"))
    assert result.returncode == 1


def test_cli_validate_parse_failure_exits_2() -> None:
    result = _run_pyodcs_cli("validate", str(FIXTURES / "malformed.yaml"))
    assert result.returncode == 2


def test_cli_missing_file_exits_2() -> None:
    result = _run_pyodcs_cli("validate", str(FIXTURES / "does-not-exist.yaml"))
    assert result.returncode == 2


def test_cli_inspect_json_output() -> None:
    result = _run_pyodcs_cli("inspect", "--json", str(FIXTURES / "minimal.odcs.yaml"))
    assert result.returncode == 0
    payload = json.loads(result.stdout)
    assert payload["qualityCount"] == 1
    assert payload["id"] == "customer-data-contract"


def test_cli_schema_command() -> None:
    result = _run_pyodcs_cli("schema")
    assert result.returncode == 0
    assert "Upstream ODCS JSON Schema" in result.stdout

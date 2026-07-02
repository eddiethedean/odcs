from __future__ import annotations

from pathlib import Path

import pyodcs

PACKAGE_ROOT = Path(__file__).resolve().parent
FIXTURES = Path(__file__).resolve().parents[2] / "tests" / "fixtures"
EXAMPLE = Path(__file__).resolve().parents[2] / "examples" / "minimal.odcs.yaml"


def _fixture(name: str) -> bytes:
    return FIXTURES.joinpath(name).read_bytes()


def test_upstream_spec_version() -> None:
    assert pyodcs.UPSTREAM_SPEC_VERSION == "3.1.0"
    assert pyodcs.__version__


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

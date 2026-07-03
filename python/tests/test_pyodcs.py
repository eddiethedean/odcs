from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

import pytest
import pyodcs

PACKAGE_ROOT = Path(__file__).resolve().parent
FIXTURES = Path(__file__).resolve().parents[2] / "tests" / "fixtures"
EXAMPLE = Path(__file__).resolve().parents[2] / "examples" / "minimal.odcs.yaml"


def _fixture(name: str) -> bytes:
    return FIXTURES.joinpath(name).read_bytes()


def test_upstream_spec_version() -> None:
    assert pyodcs.UPSTREAM_SPEC_VERSION == "3.1.0"
    assert re.fullmatch(r"\d+\.\d+\.\d+", pyodcs.__version__)
    assert pyodcs.CODES["INVALID_KIND"] == "odcs:invalid-kind"


def test_validation_phases_constants() -> None:
    assert pyodcs.VALIDATION_PHASES["DOCUMENT"] == "document"
    assert pyodcs.VALIDATION_PHASES["JSON_SCHEMA"] == "jsonSchema"


def test_validation_diagnostics_include_validation_phase() -> None:
    report = pyodcs.parse_and_validate(_fixture("invalid-kind.yaml"), "yaml")
    validation_diagnostics = [
        diagnostic
        for diagnostic in report["diagnostics"]
        if diagnostic.get("stage") == "validation"
    ]
    assert validation_diagnostics
    assert all("validationPhase" in diagnostic for diagnostic in validation_diagnostics)
    assert any(
        diagnostic.get("validationPhase") == pyodcs.VALIDATION_PHASES["DOCUMENT"]
        for diagnostic in validation_diagnostics
    )


def test_parse_diagnostics_omit_validation_phase() -> None:
    result = pyodcs.parse(_fixture("invalid-nested-duplicate-key.yaml"), "yaml")
    for diagnostic in result["report"]["diagnostics"]:
        assert "validationPhase" not in diagnostic


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


def test_validate_result_accepts_validation_report_shape() -> None:
    invalid = pyodcs.parse_and_validate(
        b"version: '3.1.0'\napiVersion: v3.1.0\nkind: wrong\nid: x\nstatus: draft\n",
        "yaml",
    )
    wrapped = pyodcs.validate_result(invalid)
    assert wrapped["diagnostics"] == invalid["diagnostics"]
    assert not pyodcs.is_valid(wrapped)


def test_validate_result_is_idempotent() -> None:
    result = pyodcs.parse(_fixture("invalid-kind.yaml"), "yaml")
    first = pyodcs.validate_result(result)
    second = pyodcs.validate_result(result)
    assert first["diagnostics"] == second["diagnostics"]


def test_validate_result_rejects_wrong_shape() -> None:
    import pytest

    with pytest.raises(TypeError):
        pyodcs.validate_result({"unexpected": True})


def test_parse_file_missing_raises_file_not_found() -> None:
    import pytest

    with pytest.raises(FileNotFoundError):
        pyodcs.parse_file(str(FIXTURES / "does-not-exist.yaml"))


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
    minimal = pyodcs.parse(_fixture("minimal.odcs.yaml"), "yaml")["contract"]
    with_items = pyodcs.parse(_fixture("with-schema-quality-items.yaml"), "yaml")["contract"]
    assert minimal is not None
    assert with_items is not None
    assert pyodcs.quality_rules_count(minimal) == 1
    assert pyodcs.quality_rules_count(with_items) == 1


def test_diff_detects_breaking_property_removal() -> None:
    base = pyodcs.parse_file(str(FIXTURES / "compatibility" / "base.yaml"))["contract"]
    breaking = pyodcs.parse_file(
        str(FIXTURES / "compatibility" / "breaking-removed-column.yaml")
    )["contract"]
    assert base is not None
    assert breaking is not None
    report = pyodcs.diff(base, breaking)
    assert report["hasBreaking"] is True
    assert any(
        change["kind"] == "breaking"
        and change["path"] == "schema[customers].properties[email]"
        for change in report["changes"]
    )


def test_parse_and_validate_paths_with_dep() -> None:
    primary = FIXTURES / "cross-file" / "consumer-valid.yaml"
    provider = FIXTURES / "cross-file" / "provider.yaml"
    report = pyodcs.parse_and_validate_paths(str(primary), deps=[str(provider)])
    assert pyodcs.is_valid(report)


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


def test_cli_validate_text_includes_validation_phase() -> None:
    result = _run_pyodcs_cli(
        "validate", str(FIXTURES / "invalid-structural-duplicate-schema-name.yaml")
    )
    assert result.returncode == 1
    assert "phase: structural" in result.stdout


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
    payload = json.loads(result.stdout)
    assert "$schema" in payload or "title" in payload


def test_cli_schema_url_only() -> None:
    result = _run_pyodcs_cli("schema", "--url-only")
    assert result.returncode == 0
    assert "Upstream ODCS JSON Schema" in result.stdout


def test_cli_validate_json_schema_violation() -> None:
    result = _run_pyodcs_cli(
        "validate",
        str(FIXTURES / "invalid-json-schema-only.yaml"),
    )
    assert result.returncode == 1
    assert "odcs:json-schema-violation" in result.stdout


def test_invalid_json_schema_fixture_fails_default_validation() -> None:
    result = pyodcs.parse(_fixture("invalid-json-schema-only.yaml"), "yaml")
    contract = result["contract"]
    assert contract is not None
    report = pyodcs.validate(contract)
    assert not pyodcs.is_valid(report)


def test_parse_rejects_nested_yaml_duplicate_key() -> None:
    result = pyodcs.parse(_fixture("invalid-nested-duplicate-key.yaml"), "yaml")
    assert not pyodcs.is_valid(result)
    diagnostics = result["report"]["diagnostics"]
    duplicate = next(
        diagnostic
        for diagnostic in diagnostics
        if diagnostic.get("id") == pyodcs.CODES["DUPLICATE_KEY"]
    )
    assert duplicate.get("object_ref") == "schema[0].name"


def test_parse_rejects_nested_json_duplicate_key() -> None:
    result = pyodcs.parse(_fixture("invalid-nested-duplicate-key.json"), "json")
    assert not pyodcs.is_valid(result)
    duplicate = next(
        diagnostic
        for diagnostic in result["report"]["diagnostics"]
        if diagnostic.get("id") == pyodcs.CODES["DUPLICATE_KEY"]
    )
    assert duplicate.get("object_ref") == "schema[0].name"


def test_is_valid_accepts_parse_result_shape() -> None:
    result = pyodcs.parse(_fixture("invalid-nested-duplicate-key.yaml"), "yaml")
    assert not pyodcs.is_valid(result)


def test_parse_file_unsupported_extension_raises_value_error(tmp_path: Path) -> None:
    path = tmp_path / "contract.txt"
    path.write_text("version: 1", encoding="utf-8")
    with pytest.raises(ValueError, match="unsupported file extension"):
        pyodcs.parse_file(str(path))


def test_cli_validate_duplicate_key_exits_2() -> None:
    result = _run_pyodcs_cli(
        "validate",
        str(FIXTURES / "invalid-nested-duplicate-key.yaml"),
    )
    assert result.returncode == 2


def test_pinned_schema_export() -> None:
    schema = pyodcs.pinned_schema()
    assert isinstance(schema, dict)
    metadata = pyodcs.pinned_schema(json_metadata=True)
    assert metadata["schemaVersion"] == "3.1.0"
    assert "schema" in metadata


def test_cli_diff_breaking_exits_1() -> None:
    old = FIXTURES / "compatibility" / "base.yaml"
    new = FIXTURES / "compatibility" / "breaking-removed-column.yaml"
    result = _run_pyodcs_cli("diff", str(old), str(new))
    assert result.returncode == 1
    assert "odcs:compatibility-breaking" in result.stdout


def test_cli_validate_with_dep_succeeds() -> None:
    primary = FIXTURES / "cross-file" / "consumer-valid.yaml"
    provider = FIXTURES / "cross-file" / "provider.yaml"
    result = _run_pyodcs_cli(
        "validate",
        str(primary),
        "--dep",
        str(provider),
    )
    assert result.returncode == 0
    assert "valid" in result.stdout


def test_cli_registry_index_and_validate_with_registry(tmp_path: Path) -> None:
    contracts_root = FIXTURES / "registry" / "contracts"
    isolated = tmp_path / "contracts"
    _copy_tree(contracts_root, isolated)

    index_result = _run_pyodcs_cli("registry", "index", str(isolated))
    assert index_result.returncode == 0

    primary = FIXTURES / "registry" / "consumer.yaml"
    validate_result = _run_pyodcs_cli(
        "validate",
        str(primary),
        "--registry",
        str(isolated),
    )
    assert validate_result.returncode == 0
    assert "valid" in validate_result.stdout


def test_cli_registry_lookup_prefers_highest_semver(tmp_path: Path) -> None:
    contracts_root = FIXTURES / "registry" / "contracts"
    isolated = tmp_path / "contracts"
    _copy_tree(contracts_root, isolated)
    _run_pyodcs_cli("registry", "index", str(isolated))

    result = _run_pyodcs_cli("registry", "lookup", str(isolated), "provider-contract")
    assert result.returncode == 0
    assert "2.0.0" in result.stdout


def test_registry_index_lookup_and_validate(tmp_path: Path) -> None:
    contracts_root = FIXTURES / "registry" / "contracts"
    isolated = tmp_path / "contracts"
    _copy_tree(contracts_root, isolated)

    indexed = pyodcs.registry_index_and_save(str(isolated))
    assert pyodcs.is_valid(indexed["report"])
    entries = indexed["entries"]
    assert any(entry["id"] == "provider-contract" for entry in entries)

    loaded = pyodcs.registry_load(str(isolated))
    assert loaded

    best = pyodcs.registry_lookup(str(isolated), "provider-contract")
    assert best is not None
    assert best["version"] == "2.0.0"

    listed = pyodcs.registry_list(str(isolated))
    assert len(listed) == len(entries)

    primary = FIXTURES / "registry" / "consumer.yaml"
    report = pyodcs.parse_and_validate_paths(
        str(primary),
        registry=str(isolated),
    )
    assert pyodcs.is_valid(report)


def _copy_tree(source: Path, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    for path in source.rglob("*"):
        if path.is_dir():
            continue
        relative = path.relative_to(source)
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(path.read_bytes())

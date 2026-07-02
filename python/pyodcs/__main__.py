"""Command-line interface for the ODCS Python package."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from pyodcs import UPSTREAM_SPEC_VERSION, inspect, is_valid, parse_file, validate_result
from pyodcs._native import inspect_summary as _inspect_summary

PACKAGE_VERSION = "0.3.0"


def _package_version() -> str:
    try:
        from importlib.metadata import version

        return version("pyodcs")
    except Exception:
        return PACKAGE_VERSION


def _has_parse_failure(result: dict, report: dict) -> bool:
    if result.get("contract") is None:
        return True
    return any(
        diagnostic.get("stage") == "parse"
        for diagnostic in report.get("diagnostics", [])
    )


def _exit_code_for_report(result: dict, report: dict) -> int:
    if _has_parse_failure(result, report):
        return 2
    return 0 if is_valid(report) else 1


def _render_report(report: dict, *, json_output: bool, mode: str) -> None:
    if json_output:
        if mode == "validate":
            payload = {"valid": is_valid(report), "diagnostics": report.get("diagnostics", [])}
        else:
            payload = {"diagnostics": report.get("diagnostics", [])}
        print(json.dumps(payload, indent=2))
        return

    if is_valid(report):
        print("valid" if mode == "validate" else "no diagnostics")
        return

    diagnostics = report.get("diagnostics", [])
    for diagnostic in diagnostics:
        severity = diagnostic.get("severity", "error")
        code = diagnostic.get("id", "odcs:unknown")
        message = diagnostic.get("message", "")
        print(f"[{severity}] {code}: {message}")
        if object_ref := diagnostic.get("object_ref"):
            print(f"  at: {object_ref}")
        if remediation := diagnostic.get("remediation"):
            print(f"  hint: {remediation}")


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="pyodcs",
        description="Validate Open Data Contract Standard documents",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    validate_parser = subparsers.add_parser("validate", help="Parse and validate a contract")
    validate_parser.add_argument("path", type=Path)
    validate_parser.add_argument("--json", action="store_true")
    validate_parser.add_argument(
        "--strict",
        action="store_true",
        help="Enable strict validation (reserved for a future release)",
    )

    inspect_parser = subparsers.add_parser("inspect", help="Print a contract summary")
    inspect_parser.add_argument("path", type=Path)
    inspect_parser.add_argument("--json", action="store_true")

    diagnostics_parser = subparsers.add_parser(
        "diagnostics",
        help="Print validation diagnostics",
    )
    diagnostics_parser.add_argument("path", type=Path)
    diagnostics_parser.add_argument("--json", action="store_true")

    schema_parser = subparsers.add_parser("schema", help="Print upstream JSON Schema location")
    schema_parser.add_argument("--json", action="store_true")

    version_parser = subparsers.add_parser("version", help="Print package versions")
    version_parser.add_argument("--json", action="store_true")

    return parser


def main(argv: list[str] | None = None) -> int:
    try:
        return _main_impl(argv)
    except BrokenPipeError:
        return 2


def _main_impl(argv: list[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)

    if args.command == "version":
        if args.json:
            print(
                json.dumps(
                    {
                        "crateVersion": _package_version(),
                        "upstreamSpecVersion": UPSTREAM_SPEC_VERSION,
                    },
                    indent=2,
                )
            )
        else:
            print(
                f"pyodcs {_package_version()} (upstream ODCS {UPSTREAM_SPEC_VERSION})"
            )
        return 0

    if args.command == "schema":
        schema_url = "https://github.com/bitol-io/open-data-contract-standard"
        json_output = getattr(args, "json", False)
        if json_output:
            print(
                json.dumps(
                    {
                        "upstreamRepository": schema_url,
                        "note": "JSON Schema export is planned for a future release",
                    },
                    indent=2,
                )
            )
        else:
            print(f"Upstream ODCS JSON Schema: {schema_url}")
            print("(JSON Schema export planned)")
        return 0

    if getattr(args, "strict", False):
        print(
            "note: --strict validation is reserved for a future release",
            file=sys.stderr,
        )

    try:
        result = parse_file(str(args.path))
    except (FileNotFoundError, OSError, ValueError) as error:
        print(error, file=sys.stderr)
        return 2

    report = validate_result(result)

    if args.command == "validate":
        _render_report(report, json_output=args.json, mode="validate")
        return _exit_code_for_report(result, report)

    if args.command == "diagnostics":
        _render_report(report, json_output=args.json, mode="diagnostics")
        return _exit_code_for_report(result, report)

    if _has_parse_failure(result, report) or not is_valid(report):
        _render_report(report, json_output=args.json, mode="diagnostics")
        return _exit_code_for_report(result, report)

    contract = result.get("contract")
    if contract is None:
        _render_report(report, json_output=args.json, mode="diagnostics")
        return 2

    if args.json:
        print(json.dumps(_inspect_summary(contract), indent=2))
    else:
        print(inspect(contract), end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())

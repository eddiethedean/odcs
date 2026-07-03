"""Command-line interface for the ODCS Python package."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from pyodcs import (
    UPSTREAM_REPOSITORY_URL,
    UPSTREAM_SPEC_VERSION,
    __version__,
    diff,
    inspect,
    is_valid,
    parse_and_validate_paths,
    parse_file,
    pinned_schema,
    registry_index_and_save,
    registry_list,
    registry_lookup,
    validate_result,
)
from pyodcs._native import inspect_summary as _inspect_summary


def _package_version() -> str:
    return __version__


def _has_parse_failure(result: dict, report: dict) -> bool:
    if result.get("contract") is None:
        return True
    return any(
        diagnostic.get("stage") == "parse"
        for diagnostic in report.get("diagnostics", [])
    )


def _exit_code_for_parse_result(result: dict, report: dict) -> int:
    if _has_parse_failure(result, report):
        return 2
    return 0 if is_valid(report) else 1


def _exit_code_for_report(report: dict) -> int:
    if any(diagnostic.get("stage") == "parse" for diagnostic in report.get("diagnostics", [])):
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
        if phase := diagnostic.get("validationPhase"):
            print(f"  phase: {phase}")
        if remediation := diagnostic.get("remediation"):
            print(f"  hint: {remediation}")


def _registry_entry_line(entry: dict) -> str:
    return f"{entry['id']} {entry['version']} ({entry['path']})"


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="pyodcs",
        description="Validate Open Data Contract Standard documents",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    validate_parser = subparsers.add_parser("validate", help="Parse and validate a contract")
    validate_parser.add_argument("path", type=Path)
    validate_parser.add_argument(
        "--dep",
        action="append",
        default=[],
        dest="deps",
        metavar="PATH",
        help="Explicit dependency contract path (repeatable)",
    )
    validate_parser.add_argument(
        "--include",
        action="append",
        default=[],
        dest="includes",
        metavar="DIR",
        help="Directory of dependency contracts (non-recursive scan)",
    )
    validate_parser.add_argument(
        "--registry",
        dest="registry_dir",
        metavar="DIR",
        help="Registry root directory (<dir>/.odcs/registry.json)",
    )
    validate_parser.add_argument("--json", action="store_true")

    inspect_parser = subparsers.add_parser("inspect", help="Print a contract summary")
    inspect_parser.add_argument("path", type=Path)
    inspect_parser.add_argument("--json", action="store_true")

    diagnostics_parser = subparsers.add_parser(
        "diagnostics",
        help="Print validation diagnostics",
    )
    diagnostics_parser.add_argument("path", type=Path)
    diagnostics_parser.add_argument("--json", action="store_true")

    diff_parser = subparsers.add_parser("diff", help="Compare contracts for breaking changes")
    diff_parser.add_argument("old", type=Path)
    diff_parser.add_argument("new", type=Path)
    diff_parser.add_argument("--json", action="store_true")

    schema_parser = subparsers.add_parser("schema", help="Print pinned ODCS JSON Schema")
    schema_parser.add_argument("--json", action="store_true")
    schema_parser.add_argument(
        "--url-only",
        action="store_true",
        help="Print upstream repository URL only",
    )

    version_parser = subparsers.add_parser("version", help="Print package versions")
    version_parser.add_argument("--json", action="store_true")

    registry_parser = subparsers.add_parser(
        "registry",
        help="Local contract registry commands",
    )
    registry_subparsers = registry_parser.add_subparsers(dest="registry_command", required=True)

    index_parser = registry_subparsers.add_parser(
        "index",
        help="Build or overwrite .odcs/registry.json for a directory",
    )
    index_parser.add_argument("dir", type=Path)
    index_parser.add_argument("--json", action="store_true")

    lookup_parser = registry_subparsers.add_parser(
        "lookup",
        help="Look up a contract by id (and optional version)",
    )
    lookup_parser.add_argument("dir", type=Path)
    lookup_parser.add_argument("id")
    lookup_parser.add_argument("--version", dest="contract_version", default=None)
    lookup_parser.add_argument("--json", action="store_true")

    list_parser = registry_subparsers.add_parser("list", help="List all indexed contracts")
    list_parser.add_argument("dir", type=Path)
    list_parser.add_argument("--json", action="store_true")

    return parser


def _run_validate(args: argparse.Namespace) -> int:
    path = str(args.path)
    deps = [str(dep) for dep in args.deps]
    includes = [str(include) for include in args.includes]
    registry = str(args.registry_dir) if args.registry_dir else None

    if deps or includes or registry:
        try:
            report = parse_and_validate_paths(
                path,
                deps=deps or None,
                includes=includes or None,
                registry=registry,
            )
        except ValueError as error:
            print(error, file=sys.stderr)
            return 2
        _render_report(report, json_output=args.json, mode="validate")
        return _exit_code_for_report(report)

    try:
        result = parse_file(path)
    except (FileNotFoundError, OSError, ValueError) as error:
        print(error, file=sys.stderr)
        return 2
    report = validate_result(result)
    _render_report(report, json_output=args.json, mode="validate")
    return _exit_code_for_parse_result(result, report)


def _run_diff(args: argparse.Namespace) -> int:
    try:
        old_result = parse_file(str(args.old))
        new_result = parse_file(str(args.new))
    except (FileNotFoundError, OSError, ValueError) as error:
        print(error, file=sys.stderr)
        return 2

    old_contract = old_result.get("contract")
    new_contract = new_result.get("contract")
    if old_contract is None or new_contract is None:
        print("failed to parse one or both contracts", file=sys.stderr)
        return 2

    report = diff(old_contract, new_contract)
    if args.json:
        payload = {
            "compatible": not report.get("hasBreaking", False),
            "hasBreaking": report.get("hasBreaking", False),
            "changes": report.get("changes", []),
        }
        print(json.dumps(payload, indent=2))
    elif not report.get("changes"):
        print("no changes")
    else:
        for change in report.get("changes", []):
            kind = str(change.get("kind", "")).lower()
            print(
                f"[{kind}] {change.get('code', '')}: "
                f"{change.get('message', '')} ({change.get('path', '')})"
            )

    return 1 if report.get("hasBreaking") else 0


def _run_registry_index(args: argparse.Namespace) -> int:
    directory = str(args.dir)
    try:
        indexed = registry_index_and_save(directory)
    except ValueError as error:
        message = str(error)
        print(message, file=sys.stderr)
        if "duplicate registry entry" in message:
            return 1
        return 2

    report = indexed.get("report", {})
    entries = indexed.get("entries", [])
    if args.json:
        print(
            json.dumps(
                {"entries": entries, "diagnostics": report.get("diagnostics", [])},
                indent=2,
            )
        )
    else:
        for entry in entries:
            print(_registry_entry_line(entry))
        print(f"indexed {len(entries)} contract(s)", file=sys.stderr)
    return 0


def _run_registry_lookup(args: argparse.Namespace) -> int:
    directory = str(args.dir)
    try:
        entry = registry_lookup(directory, args.id, args.contract_version)
    except ValueError as error:
        print(error, file=sys.stderr)
        return 2

    if entry is None:
        if args.json:
            print(json.dumps({"entry": None}, indent=2))
        else:
            print(f"registry entry not found: {args.id}", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps(entry, indent=2))
    else:
        print(f"{entry['id']} {entry['version']} {entry['path']}")
    return 0


def _run_registry_list(args: argparse.Namespace) -> int:
    directory = str(args.dir)
    try:
        entries = registry_list(directory)
    except ValueError as error:
        print(error, file=sys.stderr)
        return 2

    if args.json:
        print(json.dumps({"entries": entries}, indent=2))
    else:
        for entry in entries:
            print(_registry_entry_line(entry))
    return 0


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
        if args.url_only:
            print(f"Upstream ODCS JSON Schema: {UPSTREAM_REPOSITORY_URL}")
            return 0
        if args.json:
            print(json.dumps(pinned_schema(json_metadata=True), indent=2))
        else:
            print(json.dumps(pinned_schema(), indent=2))
        return 0

    if args.command == "validate":
        return _run_validate(args)

    if args.command == "diff":
        return _run_diff(args)

    if args.command == "registry":
        if args.registry_command == "index":
            return _run_registry_index(args)
        if args.registry_command == "lookup":
            return _run_registry_lookup(args)
        if args.registry_command == "list":
            return _run_registry_list(args)

    try:
        result = parse_file(str(args.path))
    except (FileNotFoundError, OSError, ValueError) as error:
        print(error, file=sys.stderr)
        return 2

    report = validate_result(result)

    if args.command == "diagnostics":
        _render_report(report, json_output=args.json, mode="diagnostics")
        return _exit_code_for_parse_result(result, report)

    if _has_parse_failure(result, report) or not is_valid(report):
        _render_report(report, json_output=args.json, mode="diagnostics")
        return _exit_code_for_parse_result(result, report)

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

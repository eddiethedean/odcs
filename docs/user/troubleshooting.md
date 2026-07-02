# Troubleshooting

Symptom → cause → fix for common `odcs` / `pyodcs` issues. For installation-only problems, see [Installation — Troubleshooting](installation.md#troubleshooting).

## Installation

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| `odcs: command not found` | Binary not on `PATH` | `cargo install odcs`; add `~/.cargo/bin` to `PATH` |
| `pyodcs` import fails after clone | Editable install not built | `maturin develop --features python --locked` |
| `PackageNotFoundError: pyodcs` | Importing from source without install | `pip install pyodcs` or `maturin develop` |
| Version mismatch vs docs | Docs track `main`; registries may lag | See [Release status](../project/release-status.md) |
| CI behaves differently than local | Unpinned tool version | Pin `cargo install odcs --version 0.7.0 --locked` |

## Validation failures

| Symptom | Code | Fix |
|---------|------|-----|
| Unknown field at root or nested | `odcs:unknown-field` | Remove typo or use `customProperties`; see [FAQ](faq.md#why-does-my-contract-fail-with-odcsunknown-field) |
| Wrong ODCS spec version | `odcs:unsupported-version` | Set `apiVersion: "v3.1.0"` (not `version`) |
| Empty contract id | `odcs:missing-required-field` | Set non-empty `id` |
| Wrong document kind | `odcs:invalid-kind` | Set `kind: "DataContract"` |
| Legacy metric `not_null` | `odcs:invalid-quality` | Use `nullValues`, `missingValues`, etc. |
| Quality at contract root | `odcs:unknown-field` or schema error | Move `quality` under `schema[]` |
| Duplicate YAML/JSON key | `odcs:duplicate-key` | Remove duplicate; check path in `object_ref` (exit code `2`) |
| JSON Schema constraint | `odcs:json-schema-violation` | Run `odcs diagnostics file.yaml --json`; compare with `odcs schema` |
| Relationship endpoint invalid | `odcs:unresolved-reference` | Fix `from` / `to` under `schema[].relationships` |
| Server typo (e.g. `sever`) | `odcs:unknown-field` | Fix field name; use `server` at object root |

## Exit codes

| Code | Meaning | Typical cause |
|------|---------|---------------|
| `0` | Valid | — |
| `1` | Validation errors | Semantic/structural issues after parse |
| `2` | Parse or I/O failure | Malformed YAML/JSON, duplicate keys, missing file |

## `version` vs `apiVersion`

| Field | Purpose | Example |
|-------|---------|---------|
| `version` | Your contract revision | `"1.0.0"` |
| `apiVersion` | ODCS specification release | `"v3.1.0"` |

Setting `version: "3.1.0"` does **not** select the ODCS spec — use `apiVersion`.

## Duplicate-key limitations (0.5.0+)

Nested duplicates in **block-style** YAML and JSON are detected. Not fully scanned:

- YAML flow mappings (`{key: value}`)
- YAML anchors and aliases (`&anchor`, `*alias`)

Use block-style YAML in CI. See [Diagnostics — Duplicate-key limitations](diagnostics.md#duplicate-key-limitations-050).

## Upgrading from older releases

Contracts that passed 0.2.x or 0.3.x may fail under 0.4.0+ (JSON Schema always on) or 0.5.0 (duplicate keys). See [Migration](migration.md).

## Still stuck?

1. Run `odcs diagnostics contract.yaml --json` for structured output
2. Check [FAQ](faq.md) and [Diagnostics reference](diagnostics.md)
3. Walk through [Fix your first invalid contract](../user/tutorials/fix-invalid-contract.md)
4. Open a [GitHub issue](https://github.com/eddiethedean/odcs/issues) with the diagnostic JSON (redact sensitive data)

## Security

Report vulnerabilities per [SECURITY.md](../../SECURITY.md) — do not file public issues for security-sensitive reports.

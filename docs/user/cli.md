# CLI Reference

The `odcs` binary (Rust) and `pyodcs` command (Python) share the same subcommands and exit codes.

## Commands

```bash
odcs validate <path>    # Parse and validate; print result
odcs inspect <path>     # Print contract summary
odcs diagnostics <path> # Print validation diagnostics
odcs schema             # Print pinned ODCS JSON Schema
odcs version            # Print tool and upstream spec versions
```

## Flags

| Flag | Commands | Description |
|------|----------|-------------|
| `--json` | all | Emit JSON output (`schema --json` includes metadata wrapper) |
| `--strict` | `validate` | Deprecated no-op (JSON Schema always runs in 0.4.0+) |
| `--url-only` | `schema` | Print upstream repository URL only |

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Valid contract (or successful informational command) |
| `1` | Validation errors |
| `2` | Parse failure or I/O error (missing file, malformed YAML/JSON) |

## validate

```bash
odcs validate contract.yaml
odcs validate contract.yaml --json
```

**Text output (valid):**

```text
valid
```

**Text output (invalid):**

```text
[error] odcs:invalid-kind: expected kind 'DataContract', got 'WrongKind'
  at: kind
```

When present, `at:` shows the affected field path and `hint:` shows remediation.

**JSON output:**

```json
{
  "valid": true,
  "diagnostics": []
}
```

## inspect

```bash
odcs inspect contract.yaml
odcs inspect contract.yaml --json
```

**JSON output fields:**

| Field | Description |
|-------|-------------|
| `id` | Contract id |
| `name` | Contract name (if set) |
| `version` | ODCS document version |
| `apiVersion` | ODCS API version |
| `kind` | Document kind |
| `status` | Lifecycle status |
| `schemaCount` | Number of schema objects |
| `qualityCount` | Nested quality rules across schemas |

## diagnostics

Same exit codes as `validate`. JSON output contains only the `diagnostics` array.

## schema

```bash
odcs schema
odcs schema --json
odcs schema --url-only
```

**Default:** full pinned ODCS v3.1.0 JSON Schema to stdout.

**`--json`:** metadata wrapper with `schemaVersion`, `upstreamUrl`, and `schema`.

**`--url-only`:** upstream repository URL (previous default style).

## version

```bash
odcs version
odcs version --json
```

**JSON output:**

```json
{
  "crateVersion": "0.4.0",
  "upstreamSpecVersion": "3.1.0"
}
```

## `--strict` (deprecated)

Since 0.4.0, JSON Schema validation always runs. `--strict` is accepted for backward compatibility but has no additional effect.

## CI integration example

```bash
#!/bin/sh
set -e
for f in contracts/*.yaml; do
  odcs validate "$f" --json > /dev/null
done
```

Or fail the job on any invalid contract:

```bash
odcs validate contract.yaml || exit 1
```

See [diagnostics.md](diagnostics.md) for error code reference.

# CLI Reference

The `odcs` binary (Rust) and `pyodcs` command (Python) share the same subcommands and exit codes.

## Commands

```bash
odcs validate <path>    # Parse and validate; print result
odcs inspect <path>     # Print contract summary
odcs diagnostics <path> # Print validation diagnostics
odcs schema             # Print upstream JSON Schema location
odcs version            # Print tool and upstream spec versions
```

## Flags

| Flag | Commands | Description |
|------|----------|-------------|
| `--json` | all except `schema` | Emit JSON output |
| `--strict` | `validate` | Reserved; no extra validation yet (stderr note) |

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

Prints the upstream ODCS repository URL. JSON Schema export from this tool is planned for a future release.

## version

```bash
odcs version
odcs version --json
```

**JSON output:**

```json
{
  "crateVersion": "0.3.0",
  "upstreamSpecVersion": "3.1.0"
}
```

## `--strict` (reserved)

`--strict` is accepted for forward compatibility but currently performs no additional validation beyond the default pipeline. A note is printed to stderr when the flag is used.

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

# CLI Specification

Binary:

```bash
odcs
```

Commands:

```bash
odcs validate <path>
odcs validate <path> --dep <path> [--dep ...] [--include <dir>]
odcs inspect <path>
odcs diagnostics <path>
odcs diff <old> <new>
odcs schema
odcs version
```

Output modes:

```bash
odcs validate contract.yaml
odcs validate contract.yaml --json
odcs validate contract.yaml --strict   # deprecated no-op since 0.4.0
odcs schema
odcs schema --json
odcs schema --url-only
```

Exit codes:

- `0` valid / no breaking diff changes
- `1` validation errors / breaking diff changes
- `2` parse or IO failure

## Cross-file validation (0.8.0+)

```bash
odcs validate consumer.yaml --dep provider.yaml --include ./contracts/
```

## Compatibility diff (0.8.0+)

```bash
odcs diff old.yaml new.yaml
odcs diff old.yaml new.yaml --json
```

Exit `0` when no breaking changes; `1` when breaking changes are detected.

## Validation (0.4.0+)

Default `validate` runs the Rust pipeline plus pinned ODCS v3.1.0 JSON Schema validation.

`--strict` is retained for backward compatibility but has no additional effect since 0.4.0.

`parse_strict()` (library API) parses and validates in one step, returning `Result<DataContract, DiagnosticReport>`. Unknown fields are rejected during serde deserialization at parse time — this is separate from the deprecated CLI `--strict` flag.

## `odcs schema`

| Flag | Output |
|------|--------|
| (default) | Full pinned JSON Schema to stdout |
| `--json` | `{"schemaVersion","upstreamUrl","schema"}` wrapper |
| `--url-only` | Upstream repository URL line only |

User-facing CLI documentation: [../user/cli.md](../user/cli.md).

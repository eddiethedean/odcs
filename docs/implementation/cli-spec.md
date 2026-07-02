# CLI Specification

Binary:

```bash
odcs
```

Commands:

```bash
odcs validate <path>
odcs inspect <path>
odcs diagnostics <path>
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

- `0` valid
- `1` validation errors
- `2` parse or IO failure

## Validation (0.4.0+)

Default `validate` runs the Rust pipeline plus pinned ODCS v3.1.0 JSON Schema validation.

`--strict` is retained for backward compatibility but has no additional effect since 0.4.0.

`parse_strict()` (library API) rejects unknown fields at parse time — separate from CLI `--strict`.

## `odcs schema`

| Flag | Output |
|------|--------|
| (default) | Full pinned JSON Schema to stdout |
| `--json` | `{"schemaVersion","upstreamUrl","schema"}` wrapper |
| `--url-only` | Upstream repository URL line only |

User-facing CLI documentation: [../user/cli.md](../user/cli.md).

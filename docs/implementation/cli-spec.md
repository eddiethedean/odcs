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
odcs validate contract.yaml --strict
odcs schema
odcs schema --json
odcs schema --url-only
```

Exit codes:

- `0` valid
- `1` validation errors
- `2` parse or IO failure

## `--strict`

`--strict` on `validate` runs the default Rust validation pipeline, then validates the serialized contract against the pinned ODCS v3.1.0 JSON Schema. JSON Schema violations emit `odcs:json-schema-violation` diagnostics.

This is separate from `parse_strict()`, which rejects unknown fields at parse time.

## `odcs schema`

| Flag | Output |
|------|--------|
| (default) | Full pinned JSON Schema to stdout |
| `--json` | `{"schemaVersion","upstreamUrl","schema"}` wrapper |
| `--url-only` | Upstream repository URL line only |

User-facing CLI documentation: [../user/cli.md](../user/cli.md).

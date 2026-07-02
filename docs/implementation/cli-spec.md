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
```

Exit codes:

- `0` valid
- `1` validation errors
- `2` parse or IO failure

## `--strict` (reserved)

`--strict` is accepted on `validate` for forward compatibility but currently performs no additional validation. A note is printed to stderr when the flag is used.

User-facing CLI documentation: [../user/cli.md](../user/cli.md).

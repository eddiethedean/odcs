# CI/CD Integration

Use `odcs` or `pyodcs` in continuous integration to block merges when data contracts are invalid.

## Exit codes

| Code | Meaning | CI action |
|------|---------|-----------|
| `0` | Valid | Pass |
| `1` | Validation errors | Fail |
| `2` | Parse or I/O failure | Fail |

## Cross-file and FQN relationships

**Important:** Single-file `odcs validate contract.yaml` does **not** resolve fully-qualified relationship endpoints (`provider-contract/schema/property`) against other contracts. For consumers with FQN references, use one of:

```bash
# Explicit dependency
odcs validate consumer.yaml --dep provider.yaml

# Include directory (non-recursive scan)
odcs validate consumer.yaml --include ./contracts/

# Local registry (recommended for monorepos)
odcs registry index ./contracts/
odcs validate consumer.yaml --registry ./contracts/
```

See [examples/registry/](../../examples/registry/) and [Local registry](../implementation/registry.md).

## Untrusted input (PR validation)

Treat pull-request YAML as potentially hostile:

- Prefer **JSON** for untrusted contributions when possible
- Enforce a **16 MiB** file size limit at ingress (before `odcs validate`)
- Run validation in an isolated CI job with **memory limits**
- YAML anchors/aliases are not fully bounded — see [SECURITY.md](../../SECURITY.md)

```bash
# Example: cap wall time for large registry index jobs
timeout 300 odcs registry index ./contracts/
```

Set `ODCS_VERBOSE=1` for per-file index progress on stderr (does not affect stdout JSON).

## Validate a single contract

```bash
odcs validate contracts/customer.yaml || exit 1
```

JSON output for structured logging:

```bash
odcs validate contracts/customer.yaml --json
```

## Validate all contracts in a directory

```bash
#!/bin/sh
set -e
failed=0
for f in contracts/*.{yaml,yml,json}; do
  [ -f "$f" ] || continue
  if ! odcs validate "$f" --json > /dev/null; then
    echo "INVALID: $f"
    odcs diagnostics "$f"
    failed=1
  fi
done
exit $failed
```

## GitHub Actions

```yaml
name: Validate ODCS contracts

on:
  pull_request:
    paths:
      - 'contracts/**'
  push:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install odcs
        run: cargo install odcs --version 0.9.0 --locked

      - name: Index contract registry
        run: odcs registry index ./contracts/

      - name: Validate contracts
        run: |
          for f in contracts/*.{yaml,yml,json}; do
            [ -f "$f" ] || continue
            echo "Validating $f"
            odcs validate "$f" --registry ./contracts/ --json
          done
```

### Python alternative

```yaml
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - run: pip install pyodcs==0.9.0

      - run: pyodcs validate contracts/customer.yaml
```

## Pre-commit hook

Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: odcs-validate
        name: Validate ODCS contracts
        entry: odcs validate
        language: system
        files: \.(yaml|yml|json)$
        pass_filenames: true
```

Requires `odcs` on `PATH` (`cargo install odcs`).

## Notes

- Pin the tool version in CI for reproducibility: `cargo install odcs --version 0.9.0 --locked` and `pip install pyodcs==0.9.0`.
- Do **not** run `odcs registry index` in parallel against the same directory — index writes are atomic but concurrent jobs can race.
- See [Release status](../project/release-status.md) for current published versions.
- See [diagnostics.md](diagnostics.md) for routing on `odcs:*` error codes.

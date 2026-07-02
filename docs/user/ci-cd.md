# CI/CD Integration

Use `odcs` or `pyodcs` in continuous integration to block merges when data contracts are invalid.

## Exit codes

| Code | Meaning | CI action |
|------|---------|-----------|
| `0` | Valid | Pass |
| `1` | Validation errors | Fail |
| `2` | Parse or I/O failure | Fail |

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
        run: cargo install odcs --version 0.5.0 --locked

      - name: Validate contracts
        run: |
          for f in contracts/*.{yaml,yml,json}; do
            [ -f "$f" ] || continue
            echo "Validating $f"
            odcs validate "$f" --json
          done
```

### Python alternative

```yaml
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - run: pip install pyodcs==0.5.0

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

- Since 0.4.0, `--strict` is a deprecated no-op — JSON Schema validation always runs in `validate()`.
- Pin the tool version in CI for reproducibility: `cargo install odcs --version 0.5.0 --locked` and `pip install pyodcs==0.5.0`.
- See [Release status](../project/release-status.md) if registries have not yet published 0.5.0 (tag not pushed).
- See [diagnostics.md](diagnostics.md) for routing on `odcs:*` error codes.

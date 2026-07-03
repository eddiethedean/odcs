# Compatibility analysis

Compare two ODCS contract revisions to detect **breaking changes** before you publish or deploy an update.

Breaking changes include removed schema objects, removed properties, type changes, and other contract changes that would break downstream consumers.

## CLI

```bash
odcs diff old.yaml new.yaml
odcs diff old.yaml new.yaml --json
```

| Exit code | Meaning |
|-----------|---------|
| `0` | No breaking changes |
| `1` | Breaking changes detected |
| `2` | Parse or I/O failure |

Text output lists each change with kind, code, message, and path:

```text
[breaking] odcs:compatibility-breaking: removed property 'email' (schema[customers].properties[email])
```

When there are no changes:

```text
no changes
```

### JSON output

```bash
odcs diff examples/compatibility/base.yaml examples/compatibility/breaking-removed-column.yaml --json
```

```json
{
  "compatible": false,
  "hasBreaking": true,
  "changes": [
    {
      "kind": "breaking",
      "code": "odcs:compatibility-breaking",
      "message": "removed property 'email'",
      "path": "schema[customers].properties[email]"
    }
  ]
}
```

## Python

Both contracts must be parsed first:

```python
import pyodcs

old = pyodcs.parse_file("old.yaml")["contract"]
new = pyodcs.parse_file("new.yaml")["contract"]
assert old is not None and new is not None

report = pyodcs.diff(old, new)
if report["hasBreaking"]:
    for change in report["changes"]:
        print(change["kind"], change["path"], change["message"])
```

### Report shape

| Field | Type | Description |
|-------|------|-------------|
| `hasBreaking` | `bool` | `True` when any change has `kind: "breaking"` |
| `changes` | `list` | Each entry has `kind`, `code`, `message`, `path` |

Change kinds include `breaking` and non-breaking variants (for example additions). Treat `hasBreaking` as the CI gate signal.

## Rust

```rust
use odcs::{diff, parse_file};

let old = parse_file("old.yaml")?.into_contract()?;
let new = parse_file("new.yaml")?.into_contract()?;
let report = diff(&old, &new);
if report.has_breaking {
    for change in &report.changes {
        eprintln!("{}: {}", change.path, change.message);
    }
}
```

## CI gate

Fail the job when breaking changes are introduced unintentionally:

```bash
#!/bin/sh
set -e
odcs diff contracts/customer-v1.yaml contracts/customer-v2.yaml
```

Or capture JSON for structured reporting:

```bash
odcs diff old.yaml new.yaml --json | jq -e '.hasBreaking | not'
```

## What diff does not do

- Does not validate either contract (parse errors exit `2`)
- Does not migrate or auto-fix contracts
- Does not compare against live data or runtime behavior

For validation errors, use `odcs validate`. For upgrade steps between tool releases, see [Migration](migration.md).

## Related

- [CLI reference — diff](cli.md#diff)
- [Tutorial: Contract upgrade with diff](tutorials/contract-upgrade-with-diff.md)
- [Examples](../../examples/) — see `tests/fixtures/compatibility/` for sample pairs

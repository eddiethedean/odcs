# Contract upgrade with diff

This tutorial uses `odcs diff` to review breaking changes before publishing a new contract revision.

## Starting point

The repository includes compatibility fixtures under `tests/fixtures/compatibility/`:

| File | Role |
|------|------|
| `base.yaml` | Baseline contract |
| `breaking-removed-column.yaml` | Removes a property (breaking) |

Copy them or reference the fixtures directly from a checkout.

## Step 1 — Compare contracts

```bash
odcs diff tests/fixtures/compatibility/base.yaml \
         tests/fixtures/compatibility/breaking-removed-column.yaml
```

Expected output (exit code `1`):

```text
[breaking] odcs:compatibility-breaking: removed property 'email' (schema[customers].properties[email])
```

## Step 2 — JSON output for CI

```bash
odcs diff tests/fixtures/compatibility/base.yaml \
         tests/fixtures/compatibility/breaking-removed-column.yaml \
         --json
```

Key fields:

```json
{
  "compatible": false,
  "hasBreaking": true,
  "changes": [ ... ]
}
```

Use `hasBreaking` as the gate in scripts:

```bash
odcs diff old.yaml new.yaml || {
  echo "Breaking changes detected — review before merging"
  exit 1
}
```

## Step 3 — Validate both revisions

Diff does not validate. Confirm both files are valid ODCS documents:

```bash
odcs validate tests/fixtures/compatibility/base.yaml
odcs validate tests/fixtures/compatibility/breaking-removed-column.yaml
```

## Step 4 — Python equivalent

```python
import pyodcs

old = pyodcs.parse_file("tests/fixtures/compatibility/base.yaml")["contract"]
new = pyodcs.parse_file("tests/fixtures/compatibility/breaking-removed-column.yaml")["contract"]
assert old is not None and new is not None

report = pyodcs.diff(old, new)
assert report["hasBreaking"] is True
for change in report["changes"]:
    print(change["kind"], change["path"])
```

## Step 5 — Typical upgrade workflow

1. Author the new contract revision (`version` field updated).
2. Run `odcs validate` on the new file.
3. Run `odcs diff old.yaml new.yaml` and review changes with consumers.
4. Merge when breaking changes are intentional and communicated.

## What to read next

| Goal | Document |
|------|----------|
| Full compatibility reference | [Compatibility analysis](../compatibility.md) |
| CLI flags and exit codes | [CLI — diff](../cli.md#diff) |
| Migration between tool releases | [Migration](../migration.md) |

# Fix your first invalid contract

This tutorial walks through diagnosing and fixing a contract that fails validation.

## Starting point

Save this intentionally broken contract as `broken.yaml`:

```yaml
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "tutorial-contract"
status: "draft"

schema:
  - name: customers
    logicalType: object
    properties:
      - name: customer_id
        logicalType: string
        requred: true   # typo — should be "required"
    quality:
      - name: bad-metric
        type: library
        metric: not_null   # legacy name — use nullValues
        mustBe: 0
```

## Step 1 — Validate

```bash
odcs validate broken.yaml
```

You should see multiple errors. Exit code `1` means validation failed (parse succeeded).

For machine-readable output:

```bash
odcs diagnostics broken.yaml --json
```

## Step 2 — Fix the unknown field

Look for `odcs:unknown-field`:

```text
[error] odcs:unknown-field: ...
  at: schema[0].properties[0].requred
```

**Fix:** rename `requred` → `required`.

!!! tip "Extensions"
    If the field is intentional custom metadata, use `customProperties` instead of inventing new keys. See [Authoring contracts](../authoring.md).

## Step 3 — Fix the quality metric

Look for `odcs:invalid-quality`:

```text
[error] odcs:invalid-quality: unsupported library metric 'not_null'
  at: schema[0].quality[0].metric
```

**Fix:** change `metric: not_null` to `metric: nullValues` (v3.1.0 library metric name).

## Step 4 — Re-validate

```bash
odcs validate broken.yaml
```

Expected output:

```text
valid
```

## Step 5 — Inspect the result

```bash
odcs inspect broken.yaml --json
```

Confirms `schemaCount`, `qualityCount`, and contract metadata.

## Common variations

| If you see… | Also check… |
|-------------|-------------|
| Exit code `2` | Parse failure — duplicate keys, malformed YAML; see [Troubleshooting](../troubleshooting.md) |
| `odcs:unsupported-version` | `apiVersion` must be `v3.1.0` |
| `odcs:json-schema-violation` | Compare field against `odcs schema` output |
| `odcs:duplicate-key` | Remove duplicate mapping keys at the path in `object_ref` |

## Next steps

- [Authoring contracts](../authoring.md) — write contracts from scratch
- [CI/CD integration](../ci-cd.md) — enforce validation in pull requests
- [Diagnostics reference](../diagnostics.md) — full error code table

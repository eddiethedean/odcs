# Migrating to 0.3.0

This guide covers breaking changes when upgrading from `odcs` / `pyodcs` 0.2.x to 0.3.0.

## Summary of breaking changes

1. Root-level `quality` removed — quality rules belong under `schema[]`
2. Root `extensions` flatten removed — use `customProperties` arrays
3. Unknown fields rejected at parse time (root and nested objects)
4. Required root fields enforced: `version`, `apiVersion`, `kind`, `id`, `status`
5. Library quality `metric` values must use v3.1.0 enum names

## Root quality removed

**Before (0.2.x):**

```yaml
version: "3.1.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "my-contract"
status: "draft"
quality:
  - name: "row_count"
    type: "library"
    metric: "not_null"
```

**After (0.3.0):**

```yaml
version: "3.1.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "my-contract"
status: "draft"
schema:
  - name: "customers"
    logicalType: "object"
    quality:
      - name: "row_count"
        type: "library"
        metric: "nullValues"
        mustBe: 0
```

## Extensions via customProperties

**Before:** arbitrary root-level fields might be silently accepted or preserved.

**After:** use `customProperties` arrays:

```yaml
customProperties:
  - property: "customDomain"
    value: "finance"
```

Unknown fields at any level now produce `odcs:unknown-field` at parse time.

## Library metric names

| Legacy (informal) | ODCS v3.1.0 |
|-------------------|-------------|
| `not_null` | `nullValues` |
| `unique` | `duplicateValues` |

Valid v3.1.0 metrics: `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, `rowCount`.

Library rules also require a comparison operator (e.g. `mustBe: 0`) per the upstream JSON Schema.

## Required root fields

Ensure all contracts include non-empty values for:

- `version` (e.g. `"3.1.0"`)
- `apiVersion` (e.g. `"v3.1.0"`)
- `kind` (`"DataContract"`)
- `id`
- `status`

## API changes

### Rust

- `ParseResult::into_contract()` now rejects validation-invalid contracts (not just parse errors).
- Use `result.validate()` when you need diagnostics without consuming the contract.

### Python

- CLI exit codes now match Rust: `2` for parse/I/O failures.
- `inspect --json` includes `id`, `apiVersion`, and correct `qualityCount`.

## Validation behavior

0.3.0 adds phase-based validation beyond parsing:

- Required schema and property names
- Quality rule type constraints
- Relationship endpoint checks
- Custom property key validation

Contracts that parsed in 0.2.x may now fail validation with `odcs:invalid-quality` or `odcs:missing-required-field`.

## Getting help

- [faq.md](faq.md) — common questions
- [diagnostics.md](diagnostics.md) — error code reference
- [CHANGELOG.md](../../CHANGELOG.md) — full release notes

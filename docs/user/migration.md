# Migration Guide

This guide covers breaking changes between major pre-1.0 releases of `odcs` and `pyodcs`.

## 0.5.x → 0.6.0

### `validationPhase` on validation diagnostics

**Before (0.5.x):** Validation diagnostics included `stage: validation` but no pipeline phase metadata.

**After (0.6.0):** Every validation-stage diagnostic includes `validationPhase` (camelCase JSON) identifying the validator module (`document`, `schema`, `quality`, `jsonSchema`, …). Parse-stage diagnostics omit the field. CLI text output adds a `phase:` line when set.

**Action:** CI filters that relied on parsing `message` text can use `validationPhase` instead. Python consumers can use `pyodcs.VALIDATION_PHASES`. No changes required for well-formed contracts; diagnostic `id` values are unchanged.

## 0.4.x → 0.5.0

### Nested duplicate keys fail parse

**Before (0.4.x):** Duplicate YAML keys at nested depths were silently overwritten by `serde_yaml` and never reported. JSON duplicate keys reported `object_ref` as the bare key name only.

**After (0.5.0):** Duplicate mapping keys at any depth fail parse with `odcs:duplicate-key` and a path-style `object_ref` (e.g. `schema[0].name`). JSON nested duplicates use the same path format.

**Action:** If CI previously accepted YAML contracts with nested duplicate keys, fix those contracts or expect parse failures (exit code `2`).

No API removals or changes to validation semantics for well-formed documents.

## 0.3.x → 0.4.0

### JSON Schema validation is always on

**Before (0.3.x):** JSON Schema validation ran only with `--strict` (CLI) or `strict=True` (Python).

**After (0.4.0):** `validate()` always runs JSON Schema validation against the pinned ODCS v3.1.0 schema. `--strict` and `strict=True` are deprecated no-ops retained for backward compatibility.

**Action:** Remove `--strict` from CI scripts if you added it only for schema checks. Expect more validation failures on contracts that passed semantic checks but violate JSON Schema.

### `version` vs `apiVersion`

**Before:** Some examples and tooling conflated `version` with the ODCS specification release.

**After:** `version` is your contract document revision (e.g. `1.0.0`). Only `apiVersion` gates spec support (`v3.1.0`).

```yaml
version: "1.0.0"      # your contract semver
apiVersion: "v3.1.0"  # ODCS specification version
```

**Action:** Audit contracts using `version: "3.1.0"` — change to a contract revision and keep `apiVersion: "v3.1.0"`.

### Stricter default validation

0.4.0 adds default checks for:

- Quality rule dimensions and library comparison operators
- `logicalType` enums
- Server `type` enums and type-specific required fields
- Relationship `type` enum (`foreignKey`)

**Action:** Run `odcs validate contract.yaml --json` and fix new `odcs:invalid-quality`, `odcs:invalid-schema`, or `odcs:json-schema-violation` diagnostics.

### `odcs schema` default output

**Before:** Default printed upstream repository URL only.

**After:** Default prints the full pinned JSON Schema. Use `--url-only` for URL-only output.

### Library API changes

| API | 0.4.0 behavior |
|-----|----------------|
| `validate_strict()` | Alias for `validate()` |
| `ValidationOptions::strict()` | No additional effect |
| `validate_with_options()` | `strict` flag ignored for schema gating |

## 0.2.x → 0.3.0

### Unknown fields rejected

**Before (0.2.x):** Extra fields at the root or in nested objects might be ignored.

**After (0.3.0):** Unknown fields produce `odcs:unknown-field`. Use `customProperties` for extensions.

```yaml
customProperties:
  - property: myExtension
    value: value
```

### Quality rules moved under `schema[]`

**Before:** Root-level `quality` may have been accepted.

**After:** Quality rules must be nested under `schema[]` objects or properties per ODCS v3.1.0.

### Library metric names

v3.1.0 library metrics: `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, `rowCount`. Legacy names like `not_null` fail validation.

### Canonical Object Model

0.3.0 introduced the full ODCS v3.1.0 object model (SLA, team, servers, roles, pricing, support, relationships). Parsing uses `deny_unknown_fields` on model types.

## 0.1.x → 0.2.0

0.2.0 added CI/release workflows, PyO3 bindings, and the `pyodcs` package. The CLI gained `validate`, `inspect`, `diagnostics`, `schema`, and `version` commands with standardized exit codes.

## Getting help

- [FAQ](faq.md) — common validation errors
- [Diagnostics](diagnostics.md) — error code reference
- [CHANGELOG](../changelog.md) — full release notes

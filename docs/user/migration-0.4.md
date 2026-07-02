# Migrating to 0.4.0

Version 0.4.0 adds strict JSON Schema validation and schema export. Default validation behavior is unchanged.

## `--strict` vs `parse_strict()`

These are different features:

| Feature | API / flag | When it runs |
|---------|------------|--------------|
| **Parse strict** | `parse_strict()` | Parse time — rejects unknown fields |
| **Validation strict** | `validate(strict=True)`, `odcs validate --strict` | After parse — runs JSON Schema checks |

Use `--strict` when you want conformance against the pinned upstream ODCS v3.1.0 JSON Schema in addition to the Rust validation pipeline.

## CLI changes

### `odcs validate --strict`

```bash
odcs validate contract.yaml --strict
```

Exit codes are unchanged: `0` valid, `1` validation errors (including JSON Schema violations), `2` parse/IO failure.

### `odcs schema`

Default output is now the full pinned JSON Schema document:

```bash
odcs schema              # JSON Schema to stdout
odcs schema --json       # metadata wrapper with schemaVersion, upstreamUrl, schema
odcs schema --url-only   # upstream repository URL only (previous default style)
```

## Python changes

```python
import pyodcs

# Strict validation
report = pyodcs.validate(contract, strict=True)
report = pyodcs.parse_and_validate(content, "yaml", strict=True)
report = pyodcs.validate_result(parse_result, strict=True)

# Pinned schema
schema = pyodcs.pinned_schema()
metadata = pyodcs.pinned_schema(json_metadata=True)
```

CLI:

```bash
pyodcs validate contract.yaml --strict
pyodcs schema
pyodcs schema --json
pyodcs schema --url-only
```

## New diagnostic code

Strict mode may emit:

- `odcs:json-schema-violation` — instance does not conform to the pinned JSON Schema

See [diagnostics.md](diagnostics.md) for the full code list.

## No breaking changes to default validation

`validate()` and `odcs validate` without `--strict` behave as in 0.3.0. Upgrade without code changes if you do not opt into strict mode.

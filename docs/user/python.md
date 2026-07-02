# Python API

The `pyodcs` package wraps the Rust implementation via PyO3. All parsing and validation semantics match the `odcs` crate.

## Installation

```bash
pip install pyodcs
```

## Module overview

```python
import pyodcs

pyodcs.__version__           # package version
pyodcs.UPSTREAM_SPEC_VERSION # "3.1.0"
```

## Parsing

### `parse(content, format="yaml")`

Parse a document from text or bytes. Returns a dict with `contract` and `report` keys.

```python
result = pyodcs.parse(open("contract.yaml", "rb").read(), format="yaml")
contract = result["contract"]   # dict or None
report = result["report"]       # {"diagnostics": [...]}
```

`format` may be `"yaml"`, `"yml"`, or `"json"`.

### `parse_file(path)`

Parse from a file path. Raises `ValueError` on I/O or unsupported extension errors.

```python
result = pyodcs.parse_file("examples/minimal.odcs.yaml")
```

## Validation

### `validate(contract, *, strict=False)`

Validate a parsed contract dict. Returns `{"diagnostics": [...]}`. Since 0.4.0, default validation includes pinned ODCS v3.1.0 JSON Schema checks. `strict=True` is a deprecated no-op alias.

```python
validation = pyodcs.validate(contract)
```

### `validate_result(result, *, strict=False)`

Merge parse-time and validation diagnostics from a `parse()` / `parse_file()` result.

```python
report = pyodcs.validate_result(result)
```

### `parse_and_validate(content, format="yaml", *, strict=False)`

Parse and validate in one step. Returns `{"diagnostics": [...]}`.

```python
report = pyodcs.parse_and_validate(content, format="yaml")
```

### `pinned_schema(*, json_metadata=False)`

Return the pinned ODCS v3.1.0 JSON Schema dict.

```python
schema = pyodcs.pinned_schema()
metadata = pyodcs.pinned_schema(json_metadata=True)
```

### `is_valid(report)`

Returns `True` when no error-level diagnostics are present.

```python
assert pyodcs.is_valid(report)
```

## Inspection

### `inspect(contract)`

Human-readable summary string.

```python
print(pyodcs.inspect(contract))
```

### `inspect_summary(contract)`

Structured summary dict (same fields as `odcs inspect --json`).

```python
summary = pyodcs.inspect_summary(contract)
# {"id", "name", "version", "apiVersion", "kind", "status",
#  "schemaCount", "qualityCount"}
```

### `quality_rules_count(contract)`

Count nested quality rules across all schema objects and properties.

```python
count = pyodcs.quality_rules_count(contract)
```

## CLI

The `pyodcs` console script mirrors the Rust `odcs` CLI:

```bash
pyodcs validate examples/minimal.odcs.yaml
pyodcs inspect examples/minimal.odcs.yaml --json
pyodcs diagnostics examples/minimal.odcs.yaml
pyodcs validate examples/minimal.odcs.yaml  # --strict is deprecated (no-op since 0.4.0)
pyodcs schema
pyodcs schema --json
pyodcs schema --url-only
pyodcs version
```

Exit codes match the Rust CLI: `0` valid, `1` validation error, `2` parse/I/O failure.

## Error handling pattern

```python
import pyodcs
import sys

try:
    result = pyodcs.parse_file("contract.yaml")
except (FileNotFoundError, OSError, ValueError) as e:
    print(e, file=sys.stderr)
    sys.exit(2)

report = pyodcs.validate_result(result)
if not pyodcs.is_valid(report):
    for d in report["diagnostics"]:
        print(f"{d['id']}: {d['message']}")
    sys.exit(1)
```

## Rust parity

| Python | Rust |
|--------|------|
| `parse()` | `parse()` |
| `parse_file()` | `parse_file()` |
| `validate()` | `validate()` |
| `parse_and_validate()` | `parse_and_validate()` |
| `validate(strict=True)` | `validate_strict()` (deprecated no-op since 0.4.0) |
| `diagnostic_codes()` / `CODES` | Diagnostic code constants |
| `pinned_schema()` | `odcs schema` |

See also [cli.md](cli.md) and [diagnostics.md](diagnostics.md).

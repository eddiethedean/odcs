# Diagnostics Reference

Diagnostics are structured error and warning records emitted during parsing and validation. Each diagnostic has a stable `odcs:*` identifier suitable for CI/CD routing.

## Diagnostic shape

```json
{
  "id": "odcs:invalid-kind",
  "severity": "error",
  "stage": "validation",
  "category": "structure",
  "message": "expected kind 'DataContract', got 'WrongKind'",
  "object_ref": "kind",
  "validationPhase": "document"
}
```

| Field | Description |
|-------|-------------|
| `id` | Stable diagnostic code (see table below) |
| `severity` | `error`, `warning`, or `information` |
| `stage` | Processing stage: `parse`, `validation`, etc. |
| `category` | `syntax`, `structure`, `semantic`, `compatibility`, `reference`, `extension`, … |
| `message` | Human-readable description |
| `object_ref` | JSON-path-style reference to the affected field (when known) |
| `remediation` | Suggested fix (when practical) |
| `validationPhase` | Validation pipeline phase (since 0.6.0); present on validation-stage diagnostics only |

A report is **valid** when it contains no `error`-severity diagnostics.

## Diagnostic codes

| Code | When it fires | Typical `object_ref` |
|------|---------------|----------------------|
| `odcs:parse-yaml` | YAML syntax or structure cannot be parsed | — |
| `odcs:parse-json` | JSON syntax or structure cannot be parsed | — |
| `odcs:duplicate-key` | Duplicate mapping key in JSON or YAML (since 0.5.0, nested paths use dotted `object_ref`) | `id` or `schema[0].name` |
| `odcs:document-too-large` | Document exceeds maximum parse size | — |
| `odcs:unknown-field` | Unknown field at root or nested object (deny_unknown_fields) | dotted path (e.g. `schema[0].properties[0].requred`) |
| `odcs:unsupported-version` | Unsupported `apiVersion` (or empty `version`) | `apiVersion`, `version` |
| `odcs:missing-required-field` | Required field missing or empty | field path |
| `odcs:invalid-kind` | `kind` is not `DataContract` | `kind` |
| `odcs:invalid-schema` | Schema object structural error | `schema[n].…` |
| `odcs:invalid-quality` | Quality rule constraint violation | `schema[n].quality[m].…` |
| `odcs:unresolved-reference` | Relationship endpoint empty or invalid | `schema[n].relationships[m].…` |
| `odcs:invalid-extension` | Custom property key invalid | `customProperties[n].property` |
| `odcs:json-schema-violation` | Document fails pinned ODCS v3.1.0 JSON Schema validation (default `validate()`) | JSON instance path |

## Stages

| Stage | Meaning |
|-------|---------|
| `parse` | Error occurred during YAML/JSON deserialization |
| `validation` | Error occurred during post-parse validation |
| `canonicalObjectModel` | Reserved for COM-stage checks |
| `analysis`, `planning`, `compilation`, `runtime` | Reserved for future use |

## Categories

| Category | Used for |
|----------|----------|
| `syntax` | Parse failures |
| `structure` | Required fields, document shape |
| `semantic` | Quality rules, business constraints |
| `compatibility` | Version mismatches |
| `reference` | Relationship and reference integrity |
| `extension` | Custom property validation |

## Validation phases (0.6.0+)

When `stage` is `validation`, diagnostics include `validationPhase` identifying which validator produced the error. Parse-stage diagnostics omit this field.

| `validationPhase` | Validator module | Typical checks |
|-------------------|------------------|----------------|
| `document` | Document | Required root fields, `apiVersion`, `kind` |
| `structural` | Structural | Cross-field rules (reserved) |
| `schema` | Schema | Schema names, `logicalType`, array/object shape |
| `quality` | Quality | Rule types, metrics, dimensions |
| `references` | References | Relationship endpoints |
| `extensions` | Extensions | Custom properties, authoritative definitions |
| `servers` | Servers | Server name, `type`, detail fields |
| `sections` | Sections | Team, roles, support, SLA |
| `ids` | IDs | StableId patterns |
| `jsonSchema` | JSON Schema | Pinned ODCS v3.1.0 schema |

Filter in CI:

```bash
odcs validate contract.yaml --json | jq '.diagnostics[] | select(.validationPhase == "quality")'
```

## Examples

### Unknown nested field

```text
[error] odcs:unknown-field: failed to parse document: unknown field `requred`
  at: schema[0].properties[0].requred
  hint: remove the unknown field or use customProperties for extensions
```

### Invalid kind

```text
[error] odcs:invalid-kind: expected kind 'DataContract', got 'WrongKind'
  at: kind
  phase: document
```

### Unsupported apiVersion

```text
[error] odcs:unsupported-version: unsupported ODCS apiVersion 'v9.9.9'; supported: ["v3.1.0"]
  at: apiVersion
  phase: document
  hint: set apiVersion to a supported ODCS release
```

### Invalid library metric

```text
[error] odcs:invalid-quality: unsupported library metric 'not_null'; expected one of: nullValues, missingValues, ...
  at: schema[0].quality[0].metric
  hint: use a v3.1.0 library metric name
```

## JSON output

Use `--json` with `validate` or `diagnostics`:

```bash
odcs validate contract.yaml --json
```

```json
{
  "valid": false,
  "diagnostics": [
    {
      "id": "odcs:invalid-kind",
      "severity": "error",
      "stage": "validation",
      "category": "structure",
      "message": "expected kind 'DataContract', got 'WrongKind'",
      "object_ref": "kind",
      "validationPhase": "document"
    }
  ]
}
```

## Duplicate-key limitations (0.5.0+)

Nested duplicate keys in **block-style** YAML mappings and JSON objects are detected before deserialization. The following are **not** fully scanned:

- YAML flow-style mappings (e.g. `{key: value}`)
- YAML anchors and aliases (`&anchor`, `*alias`)

Use block-style mappings for CI validation. See [migration.md](migration.md).

## Implementation details

Type definitions live in [`src/diagnostics/`](../../src/diagnostics/). Code constants are in [`src/diagnostics/codes.rs`](../../src/diagnostics/codes.rs).

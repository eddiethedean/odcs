# Examples

Sample ODCS v3.1.0 data contracts demonstrating common contract sections.

## Quick validation

```bash
odcs validate examples/minimal.odcs.yaml
```

## Catalog

| File | Demonstrates |
|------|--------------|
| [minimal.odcs.yaml](minimal.odcs.yaml) | Minimal valid contract: schema, properties, library quality rule |
| [minimal.odcs.json](minimal.odcs.json) | Same contract in JSON |
| [with-sla.yaml](with-sla.yaml) | Service level agreement properties |
| [with-team.yaml](with-team.yaml) | Team object with members (v3.1.0 form) |
| [with-servers.yaml](with-servers.yaml) | Server definitions with type-specific fields |
| [with-relationships.yaml](with-relationships.yaml) | Schema-level foreign key relationships |
| [with-schema-quality.yaml](with-schema-quality.yaml) | Nested quality rules (library, SQL, text) |
| [with-extensions.yaml](with-extensions.yaml) | Root and nested `customProperties` |

## Validate all examples

```bash
for f in examples/*.{yaml,yml,json}; do
  [ -f "$f" ] && odcs validate "$f" && echo "OK $f"
done
```

## More fixtures

Additional valid and invalid fixtures used in integration tests live under [`tests/fixtures/`](../tests/fixtures/), including:

- `with-roles.yaml`, `with-pricing.yaml`, `with-support.yaml`
- `with-schema-array-items.yaml`, `with-custom-quality-object.yaml`
- `invalid-kind.yaml`, `unsupported-version.yaml` (negative cases)

## Authoring tips

- Required root fields: `version`, `apiVersion`, `kind`, `id`, `status`
- Quality rules belong under `schema[]` (not at the contract root)
- Use `customProperties` for extensions; unknown fields are rejected
- Library metrics must use v3.1.0 names: `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, `rowCount`

See [../docs/user/faq.md](../docs/user/faq.md).

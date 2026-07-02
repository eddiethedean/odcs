# Authoring Contracts

This guide helps you write a minimal ODCS v3.1.0 data contract. For the full specification, see the [upstream ODCS documentation](https://github.com/bitol-io/open-data-contract-standard).

## Minimal template

Save as `contract.yaml` and validate with `odcs validate contract.yaml`:

```yaml
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "my-data-contract"
status: "draft"

schema:
  - name: customers
    logicalType: object
    properties:
      - name: customer_id
        logicalType: string
        required: true
```

## Required root fields

| Field | Description | Example |
|-------|-------------|---------|
| `version` | Your contract revision (any non-empty string) | `"1.0.0"` |
| `apiVersion` | ODCS specification release | `"v3.1.0"` |
| `kind` | Document type | `"DataContract"` |
| `id` | Stable contract identifier | `"customer-data-contract"` |
| `status` | Lifecycle status | `"draft"`, `"active"`, … |

## Common sections

| Section | Purpose |
|---------|---------|
| `schema[]` | Tables/objects, columns/properties, nested quality rules |
| `slaProperties` | Service level agreements |
| `team` | Ownership and contacts |
| `servers` | Physical/logical server definitions |
| `roles` | Access roles |
| `customProperties` | Extensions (use instead of unknown fields) |

See the [examples catalog](../examples.md) for contracts with SLA, team, servers, relationships, and quality rules.

## Authoring rules

1. **Quality rules** belong under `schema[]`, not at the contract root.
2. **Extensions** go in `customProperties` — unknown fields are rejected (`odcs:unknown-field`).
3. **Library metrics** use v3.1.0 names: `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, `rowCount`.
4. **Relationships** use `type: foreignKey` with valid `from` / `to` endpoints.
5. **Schema object names** must be unique within `schema[]` (since 0.7.0).
6. **Server identifiers** (`servers[].server`) must be unique within `servers[]` (since 0.7.0).
7. **SLA element references** — `slaProperties[].element` and deprecated `slaDefaultElement` must name existing `schema[].name` values (comma-separated tokens allowed).

## Validate while authoring

```bash
odcs validate contract.yaml
odcs diagnostics contract.yaml --json
odcs inspect contract.yaml
```

## Learn more

- [Getting started](getting-started.md) — install and first validation
- [FAQ](faq.md) — common validation errors
- [Upstream ODCS specification](https://github.com/bitol-io/open-data-contract-standard) — normative standard

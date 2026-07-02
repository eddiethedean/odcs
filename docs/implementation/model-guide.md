# ODCS Canonical Object Model Guide

The root type is [`DataContract`](../../src/model/contract.rs), aligned with upstream ODCS v3.1.0.

## Root document

Required fields: `version`, `apiVersion`, `kind`, `id`, `status`.

Optional sections: `name`, `tenant`, `tags`, `domain`, `description`, `servers`, `schema`, `support`, `price`, `team`, `roles`, `slaProperties`, `authoritativeDefinitions`, `customProperties`, `contractCreatedTs`.

Deprecated fields (parsed but not required): `dataProduct`, `slaDefaultElement`.

Quality rules are nested under `schema[]` objects and properties — not at the contract root.

## Module layout

| Module | Types |
|--------|-------|
| `shared` | `StableId`, `Tags`, `CustomProperty`, `AuthoritativeDefinition`, `ContractDescription`, `SchemaElement` |
| `schema` | `SchemaObject`, `SchemaProperty` |
| `quality` | `DataQuality`, `DataQualityChecks` |
| `sla` | `ServiceLevelAgreementProperty` |
| `servers` | `Server` |
| `team` | `Team`, `TeamMember`, `TeamDeclaration` |
| `roles` | `Role` |
| `pricing` | `Pricing` |
| `support` | `SupportItem`, `Support` |
| `relationships` | `RelationshipSchemaLevel`, `RelationshipPropertyLevel` |

## Design rules

- Prefer explicit structs mapped from the upstream JSON Schema.
- Use `customProperties` arrays for extensions (root `additionalProperties: false`).
- Separate model types from validation logic.
- Apply `#[serde(rename_all = "camelCase")]` on document structs.

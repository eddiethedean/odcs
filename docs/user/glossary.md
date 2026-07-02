# Glossary

Terms used in ODCS documents and this reference implementation.

## Document metadata

| Term | Definition |
|------|------------|
| **Data contract** | A machine-readable ODCS document describing a dataset and its guarantees |
| **`version`** | Your contract's revision identifier (e.g. `1.0.0`); not the ODCS spec version |
| **`apiVersion`** | ODCS specification release (e.g. `v3.1.0`); gates which spec rules apply |
| **`kind`** | Document type; must be `DataContract` for this tool |
| **`id`** | Stable unique identifier for the contract |
| **`status`** | Lifecycle state (`draft`, `active`, deprecated, etc.) |

## Schema and quality

| Term | Definition |
|------|------------|
| **`schema[]`** | List of logical data objects (tables, events, files) in the contract |
| **`properties[]`** | Columns or fields within a schema object |
| **`logicalType`** | Semantic type (`string`, `object`, `array`, …) |
| **`quality[]`** | Data quality rules nested under a schema object or property |
| **Library metric** | Built-in quality metric: `nullValues`, `missingValues`, `invalidValues`, `duplicateValues`, `rowCount` |
| **`customProperties`** | Extension mechanism for vendor- or team-specific metadata |

## Ownership and operations

| Term | Definition |
|------|------------|
| **`team`** | Ownership and contact information (v3.1.0 object form) |
| **`roles[]`** | Access roles defined on the contract or server |
| **`slaProperties[]`** | Service level agreement definitions |
| **`servers[]`** | Physical or logical data source definitions (Snowflake, Kafka, Postgres, …) |
| **`relationships[]`** | Foreign-key-style links between schema objects |

## This implementation

| Term | Definition |
|------|------------|
| **`odcs`** | Rust crate and CLI for parsing and validating ODCS documents |
| **`pyodcs`** | Python package wrapping the same Rust core |
| **Diagnostic** | Structured error record with stable `odcs:*` code, message, and optional `object_ref` |
| **`object_ref`** | Path to the affected field (e.g. `schema[0].properties[0].name`) |
| **Canonical Object Model** | Typed Rust `DataContract` struct graph deserialized from YAML/JSON |
| **Pinned JSON Schema** | Bundled ODCS v3.1.0 schema at `schema/odcs-v3.1.0.json` used in default validation |

## Ecosystem

| Term | Definition |
|------|------------|
| **ODCS** | Open Data Contract Standard — what data *is* |
| **DTCS** | Data Transformation Contract Standard — how data *changes* |
| **DPCS** | Data Pipeline Contract Standard — how transformations *compose* |

See [Relationship to DTCS](../implementation/relationship-to-dtcs.md).

## Related pages

- [What is ODCS?](what-is-odcs.md)
- [Diagnostics reference](diagnostics.md)
- [Authoring contracts](authoring.md)

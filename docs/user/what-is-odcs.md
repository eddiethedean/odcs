# What is ODCS?

The **Open Data Contract Standard (ODCS)** is a machine-readable format for **data contracts** — documents that describe what a dataset is, who owns it, how it should behave, and where it lives.

This repository (`odcs` / `pyodcs`) is a **reference implementation** that parses and validates ODCS documents. It is **not** the specification itself.

## The problem ODCS solves

Data teams need a shared, versionable contract between producers and consumers:

- **Schema** — columns, types, required fields
- **Quality rules** — expectations like “no null customer IDs”
- **SLAs** — freshness, availability, response time
- **Ownership** — team contacts and roles
- **Servers** — where data is physically stored (Snowflake, Kafka, Postgres, …)

Without a standard format, contracts live in wikis, spreadsheets, or tribal knowledge. ODCS makes them **structured YAML or JSON** that tools can validate in CI.

## What this tool does

`odcs` and `pyodcs` check that your contract **document** is well-formed and conforms to **ODCS v3.1.0**. They do **not**:

- Execute quality checks against live data
- Run ETL or pipelines
- Host a contract registry

```text
Your contract file  →  odcs validate  →  valid / structured errors
```

## Key document fields

| Field | Meaning |
|-------|---------|
| `version` | **Your** contract revision (e.g. `1.0.0`) |
| `apiVersion` | **ODCS spec** release (use `v3.1.0` with this tool) |
| `kind` | Always `DataContract` |
| `id` | Stable identifier for this contract |
| `status` | Lifecycle (`draft`, `active`, …) |
| `schema[]` | Tables/objects, properties, nested quality rules |

See the [Glossary](glossary.md) for more terms.

## Two names: `odcs` and `pyodcs`

| Name | What it is |
|------|------------|
| **`odcs`** | Rust crate and CLI |
| **`pyodcs`** | Python package and CLI (same Rust core via PyO3) |

Semantics, diagnostics, and exit codes are aligned.

## Learn the standard

| Resource | Link |
|----------|------|
| Official ODCS repository | [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard) |
| Published specification | [bitol-io.github.io/open-data-contract-standard](https://bitol-io.github.io/open-data-contract-standard/) |
| This repo’s examples | [Examples catalog](../examples.md) |

## Use this tool

1. [Getting started](getting-started.md) — validate your first contract in five minutes
2. [Authoring contracts](authoring.md) — write a minimal template
3. [CI/CD integration](ci-cd.md) — block invalid contracts in pull requests

## How this repo tracks the standard

See [Upstream sync policy](../upstream-sync-policy.md) for how we pin and synchronize with upstream ODCS releases.

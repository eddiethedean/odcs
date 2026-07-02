# Upstream Specification

# ODCS Upstream Specification Policy

**Document Status:** Normative Repository Policy\
**Applies To:** `odcs` Rust Reference Implementation

## Purpose

This document defines the authoritative upstream references that the
`odcs` repository SHALL use when implementing the Open Data Contract
Standard (ODCS).

The goal is to ensure that the Rust implementation remains aligned with
the published ODCS specification while providing an idiomatic,
high-quality Rust API.

------------------------------------------------------------------------

# Normative Upstream Sources

The following resources SHALL be treated as the authoritative sources
for ODCS semantics.

## 1. Official GitHub Repository

https://github.com/bitol-io/open-data-contract-standard

Contains:

-   specification source
-   releases
-   changelog
-   governance
-   examples
-   JSON Schema
-   reference material

------------------------------------------------------------------------

## 2. Official Documentation

https://bitol-io.github.io/open-data-contract-standard/

Provides the published human-readable specification.

------------------------------------------------------------------------

## 3. Official JSON Schema

The JSON Schema published by the ODCS project SHALL be used as the
behavioral parity target for structural validation.

------------------------------------------------------------------------

# Repository Policy

This repository SHALL maintain:

-   SPEC.md
-   Rust source code
-   examples
-   tests
-   conformance fixtures

SPEC.md is the implementation guide for this repository.

The upstream ODCS specification remains the normative standard.

If a conflict exists:

1.  Follow the targeted upstream ODCS release.
2.  Update SPEC.md.
3.  Update tests.
4.  Update implementation.

------------------------------------------------------------------------

# Version Target

Each release of this repository SHALL explicitly identify the supported
upstream ODCS version.

Example:

-   ODCS 3.1.0
-   ODCS 3.2.0

The supported version SHALL appear in:

-   README.md
-   Cargo.toml metadata where appropriate
-   SPEC.md
-   release notes

## Upstream example corpus (0.4.0)

Curated upstream examples are synced from `bitol-io/open-data-contract-standard` @ `main` into `tests/fixtures/upstream/`. See `tests/fixtures/upstream/SOURCE.txt` for the file list. Document `version` fields are preserved from upstream (typically `1.0.0`).

------------------------------------------------------------------------

# Spec parity policy (0.4.0)

Default `validate()` runs the Rust semantic pipeline **and** JSON Schema validation against the pinned ODCS v3.1.0 schema in [`schema/odcs-v3.1.0.json`](schema/odcs-v3.1.0.json).

## Matches upstream JSON Schema

- Root document properties and `additionalProperties: false` (via `deny_unknown_fields` on model types)
- Enum constraints promoted to default validation (`logicalType`, `quality.dimension`, server `type`, relationship `type`)
- Library quality `DataQualityOperators` oneOf
- Server type-specific required fields (Snowflake, Kafka, PostgreSQL)
- SLA `description` and `scheduler` fields

## Intentional extensions (stricter than schema)

Documented deviations that remain in the Rust pipeline:

| Extension | Behavior |
|-----------|----------|
| Relationship shorthand resolution | Shorthand refs (`table.column`) must resolve to a known schema object and property in the same document |
| Composite endpoint length parity | Composite `from`/`to` arrays must have equal length |
| Library `rule` field | Deprecated `rule` alone is rejected; `metric` is required |
| `apiVersion` scope | Only `v3.1.0` accepted (schema allows older API versions) |

## Structural validation (0.7.0)

Cross-field rules enforced in `src/validation/structural.rs` (not covered by JSON Schema or single-section validators):

| Rule | Behavior | Diagnostic |
|------|----------|------------|
| Unique `schema[].name` | Non-empty schema object names must be unique within `schema[]` | `odcs:invalid-schema` |
| `slaProperties[].element` → `schema[].name` | Each comma-separated token must reference an existing schema object name | `odcs:unresolved-reference` |
| `slaDefaultElement` → `schema[].name` | When set, must reference an existing schema object name (deprecated field; same element-path semantics as `slaProperties[].element`) | `odcs:unresolved-reference` |
| Unique `servers[].server` | Non-empty server identifiers must be unique within `servers[]` | `odcs:invalid-schema` |

**Not adopted:** `servers[].schema` in server-type details is a database/catalog schema string (e.g. Snowflake `"public"`), not a reference to an ODCS `schema[]` object name.

## Out of scope

- Cross-file / fully-qualified reference resolution
- Registry server and compatibility analysis (see [docs/implementation/non-goals.md](docs/implementation/non-goals.md))

------------------------------------------------------------------------

# Synchronization Workflow

When a new upstream release is published:

1.  Review the changelog.
2.  Compare specification changes.
3.  Compare JSON Schema changes.
4.  Update SPEC.md.
5.  Update Canonical Object Model.
6.  Update validators.
7.  Update diagnostics.
8.  Update conformance tests.
9.  Publish a new crate release.

------------------------------------------------------------------------

# Implementation Philosophy

This repository is not intended to replace the upstream specification.

Instead it provides:

-   the Rust reference implementation
-   idiomatic Rust APIs
-   deterministic validation
-   diagnostics
-   CLI tooling
-   Python bindings
-   conformance testing

while remaining faithful to the published ODCS standard.

------------------------------------------------------------------------

# Mapping Responsibilities

The implementation SHALL define a stable mapping from upstream ODCS
concepts into Rust types.

Mapping includes:

-   Canonical Object Model
-   parsing
-   validation
-   diagnostics
-   compatibility
-   version handling

Implementation details MAY differ from the specification provided
observable behavior remains equivalent.

------------------------------------------------------------------------

# Non-Goals

This repository SHALL NOT:

-   redefine ODCS semantics
-   invent incompatible extensions by default
-   diverge intentionally from the upstream specification
-   modify published JSON Schema behavior without documentation

------------------------------------------------------------------------

# Long-Term Vision

The long-term objective is to make this repository the canonical
open-source Rust implementation of ODCS.

Future companion repositories are expected to include:

-   contractmodel
-   dtcs
-   dpcs

forming a cohesive contract-first data engineering ecosystem.

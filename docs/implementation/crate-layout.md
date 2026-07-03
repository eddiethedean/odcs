# Crate Layout

Current layout (as of 0.9.1):

```text
odcs/
  Cargo.toml
  pyproject.toml
  README.md
  SPEC.md
  ROADMAP.md
  LICENSE

  src/
    lib.rs
    bin/odcs.rs
    contract_set.rs      # Multi-document loading

    model/
      mod.rs
      contract.rs
      schema.rs          # SchemaObject, SchemaProperty
      quality.rs
      sla.rs
      stakeholders.rs    # Reserved stub (no upstream section in v3.1.0)
      team.rs
      roles.rs
      pricing.rs
      servers.rs
      support.rs
      relationships.rs
      custom.rs
      shared.rs
      fundamentals.rs
      versioning.rs

    parser/
      mod.rs
      yaml.rs
      json.rs
      duplicate_keys.rs

    validation/
      mod.rs
      phases.rs
      document.rs
      structural.rs
      schema.rs
      quality.rs
      references.rs
      extensions.rs
      servers.rs
      sections.rs
      ids.rs
      json_schema.rs
      dedup.rs
      helpers.rs
      schema_index.rs

    diagnostics/
      mod.rs
      diagnostic.rs
      severity.rs
      category.rs
      stage.rs
      report.rs
      codes.rs
      builders.rs
      validation_phase.rs

    compatibility/
      mod.rs             # Contract diff (0.8.0+)

    registry/
      mod.rs
      local.rs
      entry.rs
      scan.rs

    cli/
      mod.rs

    python.rs            # PyO3 bindings (feature-gated)

  tests/
    common/mod.rs        # Shared fixture helpers
    skeleton.rs
    cli.rs
    cross_file.rs
    compatibility.rs
    registry.rs
    json_schema_conformance.rs
    validation_negative.rs
    diagnostic_metadata.rs
    fixtures/

  docs/
    user/
    implementation/
    maintainer/
```

Keep the layout close to `dtcs` so future maintainers recognize the ecosystem pattern.

Internal modules are `#[doc(hidden)]`; use root re-exports documented in [public-api.md](public-api.md).

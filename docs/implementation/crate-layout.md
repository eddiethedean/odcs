# Crate Layout

Current layout (as of 0.7.0):

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

    model/
      mod.rs
      contract.rs
      schema.rs          # SchemaObject, SchemaProperty
      quality.rs
      sla.rs
      stakeholders.rs
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
      mod.rs             # stub

    registry/
      mod.rs             # stub

    cli/
      mod.rs

    python.rs            # PyO3 bindings (feature-gated)

  python/
    pyodcs/
      __init__.py
      __main__.py

  examples/
  tests/
    skeleton.rs
    cli.rs
    json_schema_conformance.rs
    fixtures/

  docs/
    user/                # user-facing guides
    implementation/      # maintainer guides
    maintainer/
```

Keep the layout close to `dtcs` so future maintainers recognize the ecosystem pattern.

Note: `SchemaProperty` lives in `schema.rs` (there is no separate `field.rs` module).

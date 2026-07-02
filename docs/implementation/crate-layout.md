# Proposed Crate Layout

Recommended layout:

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

    model/
      mod.rs
      contract.rs
      fundamentals.rs
      schema.rs
      field.rs
      quality.rs
      sla.rs
      stakeholders.rs
      team.rs
      roles.rs
      pricing.rs
      servers.rs
      support.rs
      custom.rs
      versioning.rs

    parser/
      mod.rs
      yaml.rs
      json.rs

    validation/
      mod.rs
      phases.rs
      document.rs
      structural.rs
      schema.rs
      quality.rs
      references.rs
      extensions.rs

    diagnostics/
      mod.rs
      diagnostic.rs
      severity.rs
      category.rs
      report.rs

    compatibility/
      mod.rs

    registry/
      mod.rs

    cli/
      mod.rs

  python/
    pyodcs/
      __init__.py

  examples/
  tests/
  docs/
```

Keep the layout close to `dtcs` so future maintainers recognize the ecosystem pattern.

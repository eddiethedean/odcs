# ODCS Implementation Guide

Guides for building and maintaining the Rust reference implementation of the Open Data Contract Standard (ODCS).

See the canonical documentation index at [Home](../index.md).

**User documentation** (install, CLI, Python API) lives in [User guides](../user/getting-started.md).

Treat [Specification](../upstream/spec.md) as the authoritative upstream policy. The normative ODCS specification lives in the [upstream repository](https://github.com/bitol-io/open-data-contract-standard).

The implementation pipeline:

```text
parse -> Canonical Object Model -> validate -> diagnostics
```

Do not implement execution, pipelines, or transformation semantics without an agreed milestone.

## Related documents

- [project-goal.md](project-goal.md)
- [architecture.md](architecture.md)
- [crate-layout.md](crate-layout.md)
- [public-api.md](public-api.md)
- [implementation-phases.md](implementation-phases.md)
- [spec-usage.md](spec-usage.md)

Cursor build prompt: [build.md](https://github.com/eddiethedean/odcs/blob/main/.cursor/prompts/build.md)

# Contributing to ODCS

Thank you for contributing to the Open Data Contract Standard reference implementation.

## Upstream specification

Normative ODCS semantics are defined in the [upstream specification](https://github.com/bitol-io/open-data-contract-standard). This repository's [SPEC.md](SPEC.md) documents how we track and synchronize with upstream releases.

When implementing behavior:

1. Read the relevant upstream ODCS specification sections.
2. Consult [docs/implementation/](docs/implementation/) for architecture and API guidance.
3. Preserve spec-aligned naming and behavior.
4. Add tests that exercise spec requirements. See [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md).

### Authority

The upstream ODCS specification is the single source of truth for semantics, terminology, and conformance. Implementation docs in [docs/implementation/](docs/implementation/) are illustrative unless explicitly normative.

## Implementation changes

The Rust reference crate lives in [src/](src/). Before implementing a module:

1. Read the relevant upstream specification sections.
2. Consult [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md) for module boundaries.
3. Preserve spec-aligned naming and behavior.
4. Add tests for behavioral changes.

### Scope

The initial crate targets parsing, the canonical object model, validation, and diagnostics. Do not add execution, pipeline composition, or transformation features without an agreed milestone. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

### Code style

- Run `cargo fmt` and `cargo clippy` before submitting changes.
- Keep modules aligned with [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md).
- Prefer conservative behavior when the spec is ambiguous; document open questions with a `TODO` referencing the spec section.

## Pull requests

1. Describe whether the change is implementation, documentation, or infrastructure.
2. Link related issues or design discussions when available.
3. Include or update tests for behavioral changes.
4. Ensure `cargo test` passes.

## Questions

For ambiguous upstream spec language, open an issue with the relevant section rather than inventing behavior in code.

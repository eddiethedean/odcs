# How to Use the Upstream ODCS Specification

[SPEC.md](../../SPEC.md) defines the authoritative upstream references for this repository.

During implementation:

1. Read the relevant upstream ODCS specification sections before implementing each module.
2. Preserve terminology exactly as defined in the upstream specification.
3. Prefer spec-aligned names over ad hoc names.
4. Treat guides in this directory as illustrative unless explicitly normative.
5. Treat conflicts between this pack and the upstream ODCS specification as resolved in favor of the upstream specification.

Do not invent behavior that is not supported by the upstream ODCS specification.

If the spec is ambiguous, implement the smallest conservative behavior and add a `TODO` referencing the relevant section.

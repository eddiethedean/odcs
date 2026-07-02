# Relationship to DTCS

ODCS and DTCS should mirror implementation architecture but remain conceptually separate.

```text
odcs
  data contracts
  what data is

dtcs
  transformation contracts
  how data changes

dpcs
  pipeline contracts
  how transformations compose
```

Shared design ideas:

- Rust core
- Canonical Object Model
- phase-based validation
- diagnostics
- CLI
- optional Python bindings

Avoid coupling the crates too early.
Shared utilities can be extracted later if duplication becomes painful.

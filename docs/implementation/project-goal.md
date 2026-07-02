# Project Goal

Build `odcs`, a Rust reference-style implementation for the Open Data Contract Standard.

The repo should mirror the `dtcs` architecture where practical.

The crate should implement:

- ODCS Canonical Object Model
- YAML and JSON parsing
- Validation phases
- Diagnostics
- JSON Schema parity targets
- CLI
- Optional Python bindings
- Examples and tests

`odcs` should eventually sit beside:

- `dtcs` — Data Transformation Contract Standard
- `dpcs` — Data Pipeline Contract Standard

Ecosystem framing:

```text
ODCS defines what data is.
DTCS defines how data changes.
DPCS defines how transformations compose.
```

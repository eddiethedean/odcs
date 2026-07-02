# Architecture

Mirror the `dtcs` processing architecture:

```text
ODCS Document
        │
        ▼
Parser
        │
        ▼
Canonical Object Model
        │
        ▼
Validator
        │
        ▼
Diagnostics
```

ODCS is dataset-contract focused. [DTCS](https://github.com/eddiethedean/dtcs) is transformation-contract focused.

Do not introduce transformation semantics into ODCS.

## Module map

| Module | Role |
|--------|------|
| `parser/` | YAML/JSON deserialization into `DataContract` |
| `model/` | Canonical Object Model types |
| `validation/` | Phase-based validation pipeline |
| `diagnostics/` | Structured error records and codes |
| `schema/` | Pinned ODCS JSON Schema asset |
| `cli/` | `odcs` binary (feature `cli`) |
| `registry/` | Reserved — contract registry (not implemented) |
| `compatibility/` | Reserved — cross-version diffing (not implemented) |

See [crate-layout.md](crate-layout.md) for file-level layout and [relationship-to-dtcs.md](relationship-to-dtcs.md) for ecosystem positioning.

# Non-Goals

Do not implement these in the first ODCS repo:

- DTCS transformation semantics
- Pipeline composition
- ETL execution
- Runtime engine
- SQL generation
- Polars/Spark/DuckDB adapters
- Data quality execution engine
- Registry server

The first goal is a correct Rust spec core.

## Reserved modules (not implemented)

The crate includes stub modules for future work. They are **not** part of the public API surface:

| Module | Planned purpose | Status |
|--------|-----------------|--------|
| `src/registry/` | Contract registry / discovery | Stub only |
| `src/compatibility/` | Cross-version contract diffing | Stub only |

Do not document or depend on these until a roadmap milestone ships.

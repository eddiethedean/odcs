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

## Reserved modules

| Module | Planned purpose | Status |
|--------|-----------------|--------|
| `src/registry/` | Local contract registry / discovery | Implemented (0.9.0) |
| `src/compatibility/` | Cross-version contract diffing | Implemented (0.8.0) |

Do not depend on remote registry features until explicitly documented.

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

ODCS is dataset-contract focused.

DTCS is transformation-contract focused.

Do not introduce transformation semantics into ODCS.

# Diagnostics Guide

Diagnostics should mirror DTCS style.

Suggested Rust types:

```rust
pub enum Severity {
    Error,
    Warning,
    Information,
}

pub enum DiagnosticStage {
    Parse,
    CanonicalObjectModel,
    Validation,
    Compatibility,
}

pub struct Diagnostic {
    pub id: String,
    pub severity: Severity,
    pub stage: DiagnosticStage,
    pub category: String,
    pub message: String,
    pub object_ref: Option<String>,
    pub remediation: Option<String>,
}
```

Diagnostics should be stable enough for tests and CLI output.

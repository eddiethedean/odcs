# Diagnostics Guide

Diagnostics mirror DTCS style.

Implemented Rust types in [`src/diagnostics/`](../../src/diagnostics/):

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
    Analysis,
    Planning,
    Compilation,
    Runtime,
}

pub enum DiagnosticCategory {
    Syntax,
    Structure,
    Type,
    Reference,
    Semantic,
    Compatibility,
    Capability,
    Runtime,
    Extension,
}

pub struct Diagnostic {
    pub id: String,
    pub severity: Severity,
    pub stage: DiagnosticStage,
    pub category: DiagnosticCategory,
    pub message: String,
    pub object_ref: Option<String>,
    pub remediation: Option<String>,
    pub validation_phase: Option<ValidationPhase>, // JSON: validationPhase
}
```

Validation-stage diagnostics are built with `validation_error(phase, id, category, message)`, which sets `stage: Validation` and `validation_phase: Some(phase)`. Parse-stage diagnostics omit `validationPhase`.

Standard diagnostic identifiers live in [`src/diagnostics/codes.rs`](../../src/diagnostics/codes.rs).

Diagnostics are stable enough for tests and CLI output. The Rust and Python CLIs include `object_ref` and `remediation` in text output when present.

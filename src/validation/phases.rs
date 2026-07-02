//! Validation phase identifiers.

/// Validation phases for ODCS documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationPhase {
    /// Document-level validation.
    Document,
    /// Canonical Object Model validation.
    CanonicalObjectModel,
    /// Structural validation.
    Structural,
    /// Schema validation.
    Schema,
    /// Quality rule validation.
    Quality,
    /// Reference validation.
    References,
    /// Extension validation.
    Extensions,
}

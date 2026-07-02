//! Validation pipeline phase identifiers attached to validation diagnostics.

/// Validation phases for ODCS documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValidationPhase {
    /// Document-level validation.
    Document,
    /// Structural cross-field validation.
    Structural,
    /// Schema validation.
    Schema,
    /// Quality rule validation.
    Quality,
    /// Reference validation.
    References,
    /// Extension validation.
    Extensions,
    /// Server validation.
    Servers,
    /// Section validation (team, roles, support, SLA, etc.).
    Sections,
    /// Stable ID validation.
    Ids,
    /// Pinned JSON Schema validation.
    JsonSchema,
}

impl ValidationPhase {
    /// Returns the camelCase JSON value for this phase.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Structural => "structural",
            Self::Schema => "schema",
            Self::Quality => "quality",
            Self::References => "references",
            Self::Extensions => "extensions",
            Self::Servers => "servers",
            Self::Sections => "sections",
            Self::Ids => "ids",
            Self::JsonSchema => "jsonSchema",
        }
    }
}

impl std::fmt::Display for ValidationPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

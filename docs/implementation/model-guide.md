# ODCS Canonical Object Model Guide

Root type:

```rust
pub struct DataContract {
    pub version: String,
    pub kind: Option<String>,
    pub api_version: Option<String>,
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub tenant: Option<String>,
    pub domain: Option<String>,
    pub data_product: Option<String>,
    pub schema: Vec<SchemaObject>,
    pub quality: Vec<QualityRule>,
    pub sla: Option<Sla>,
    pub stakeholders: Vec<Stakeholder>,
    pub team: Vec<TeamMember>,
    pub roles: Vec<Role>,
    pub servers: Vec<Server>,
    pub pricing: Option<Pricing>,
    pub custom: IndexMap<String, serde_json::Value>,
}
```

This is only a starting sketch. Cursor should adapt it to the ODCS spec and official schema.

Design rules:

- Prefer explicit structs.
- Preserve extension/custom fields.
- Separate model from validation.
- Avoid DTCS transformation-specific concepts.

//! Team types.

use serde::{Deserialize, Serialize};

use super::shared::{AuthoritativeDefinitions, CustomProperties, StableId, Tags};

/// Team member information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TeamMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_out: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_by_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
}

/// Team information object (v3.1.0+).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Team {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<TeamMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
}

/// Root `team` field supports the object form or deprecated member array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamDeclaration {
    /// Preferred team object form.
    Team(Team),
    /// Deprecated array of team members (v3.0.2 and earlier).
    LegacyMembers(Vec<TeamMember>),
}

impl TeamDeclaration {
    /// Returns team members regardless of representation.
    #[must_use]
    pub fn members(&self) -> &[TeamMember] {
        match self {
            Self::Team(team) => &team.members,
            Self::LegacyMembers(members) => members,
        }
    }
}

//! Root data contract document.

use serde::{Deserialize, Serialize};

use super::fundamentals::ContractDescription;
use super::pricing::Pricing;
use super::quality::DataQuality;
use super::roles::Role;
use super::schema::SchemaObject;
use super::servers::Server;
use super::shared::{AuthoritativeDefinitions, CustomProperties, Tags};
use super::sla::ServiceLevelAgreementProperty;
use super::support::Support;
use super::team::TeamDeclaration;
use super::versioning::is_supported_api_version;

/// Supported upstream ODCS document `version` values for this implementation.
pub const SUPPORTED_ODCS_VERSIONS: &[&str] = &["3.1.0"];

/// An ODCS Data Contract — the canonical root object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataContract {
    /// ODCS document version.
    pub version: String,
    /// ODCS API version.
    pub api_version: String,
    /// Document kind (typically `DataContract`).
    pub kind: String,
    /// Stable contract identifier.
    pub id: String,
    /// Contract lifecycle status.
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// Deprecated data product name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_product: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ContractDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schema: Vec<SchemaObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<Support>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Pricing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamDeclaration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role>,
    /// Deprecated SLA default element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla_default_element: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sla_properties: Vec<ServiceLevelAgreementProperty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authoritative_definitions: Option<AuthoritativeDefinitions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_created_ts: Option<String>,
}

impl DataContract {
    /// Returns `true` when the document version is supported by this crate.
    #[must_use]
    pub fn is_supported_version(&self) -> bool {
        SUPPORTED_ODCS_VERSIONS.contains(&self.version.as_str())
    }

    /// Returns `true` when the API version is supported by this crate.
    #[must_use]
    pub fn is_supported_api_version(&self) -> bool {
        is_supported_api_version(&self.api_version)
    }

    /// Returns all nested quality rules declared on schema objects and properties.
    #[must_use]
    pub fn quality_rules(&self) -> Vec<&DataQuality> {
        let mut rules = Vec::new();
        for schema in &self.schema {
            if let Some(ref quality) = schema.quality {
                rules.extend(quality.iter());
            }
            collect_property_quality(&schema.properties, &mut rules);
        }
        rules
    }
}

fn collect_property_quality<'a>(
    properties: &'a [super::schema::SchemaProperty],
    out: &mut Vec<&'a DataQuality>,
) {
    for property in properties {
        if let Some(ref quality) = property.quality {
            out.extend(quality.iter());
        }
        if !property.properties.is_empty() {
            collect_property_quality(&property.properties, out);
        }
    }
}

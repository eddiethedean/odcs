//! Canonical Object Model types derived from the upstream ODCS specification.

#![allow(missing_docs)]

mod contract;
mod custom;
mod fundamentals;
mod pricing;
mod quality;
mod relationships;
mod roles;
mod schema;
mod servers;
mod shared;
mod sla;
mod stakeholders;
mod support;
mod team;
mod versioning;

pub use contract::{DataContract, SUPPORTED_ODCS_VERSIONS};
pub use custom::{CustomProperties, CustomProperty};
pub use fundamentals::ContractDescription;
pub use pricing::Pricing;
pub use quality::{DataQuality, DataQualityChecks};
pub use relationships::{RelationshipEndpoint, RelationshipPropertyLevel, RelationshipSchemaLevel};
pub use roles::Role;
pub use schema::{Field, SchemaObject, SchemaProperty};
pub use servers::Server;
pub use shared::{
    AuthoritativeDefinition, AuthoritativeDefinitions, SchemaElement, StableId, Tags,
};
pub use sla::ServiceLevelAgreementProperty;
pub use support::{Support, SupportItem};
pub use team::{Team, TeamDeclaration, TeamMember};
pub use versioning::{is_supported_api_version, SUPPORTED_API_VERSIONS};

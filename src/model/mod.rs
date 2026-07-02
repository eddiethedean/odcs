//! Canonical Object Model types derived from the upstream ODCS specification.

mod contract;
mod custom;
mod field;
mod fundamentals;
mod pricing;
mod quality;
mod roles;
mod schema;
mod servers;
mod sla;
mod stakeholders;
mod support;
mod team;
mod versioning;

pub use contract::{DataContract, SUPPORTED_ODCS_VERSIONS};
pub use field::Field;
pub use quality::QualityRule;
pub use schema::SchemaObject;

//! Canonical Object Model types derived from the upstream ODCS specification.

mod contract;
mod field;
mod quality;
mod schema;

pub use contract::{DataContract, SUPPORTED_ODCS_VERSIONS};
pub use field::Field;
pub use quality::QualityRule;
pub use schema::SchemaObject;

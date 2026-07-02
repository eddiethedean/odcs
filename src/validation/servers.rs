//! Server validation.

use indexmap::IndexMap;
use serde_json::Value;

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, Server};

const SERVER_CANONICAL_KEYS: &[&str] = &[
    "id",
    "server",
    "type",
    "description",
    "environment",
    "roles",
    "customProperties",
];

/// Validate server entries and catch typos absorbed into flattened details.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for (index, server) in contract.servers.iter().enumerate() {
        let object_ref = format!("servers[{index}]");
        validate_server_entry(&mut report, server, &object_ref);
    }

    report
}

fn validate_server_entry(report: &mut DiagnosticReport, server: &Server, object_ref: &str) {
    if server.server.as_ref().map_or(true, |name| name.is_empty()) {
        emit(
            report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "server entry requires a non-empty server name",
            )
            .with_object_ref(format!("{object_ref}.server")),
        );
    }

    validate_details_keys(report, &server.details, object_ref);
}

fn validate_details_keys(
    report: &mut DiagnosticReport,
    details: &IndexMap<String, Value>,
    object_ref: &str,
) {
    for key in details.keys() {
        if SERVER_CANONICAL_KEYS
            .iter()
            .any(|canonical| canonical.eq_ignore_ascii_case(key))
        {
            emit(
                report,
                validation_error(
                    codes::UNKNOWN_FIELD,
                    DiagnosticCategory::Structure,
                    format!(
                        "server details contains '{key}' which must be a top-level server field, not nested in server-specific properties"
                    ),
                )
                .with_object_ref(format!("{object_ref}.{key}"))
                .with_remediation("move the field to the server object root or fix the field name spelling"),
            );
        }
    }
}

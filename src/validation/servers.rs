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

const SERVER_TYPES: &[&str] = &[
    "api",
    "athena",
    "azure",
    "bigquery",
    "clickhouse",
    "databricks",
    "denodo",
    "dremio",
    "duckdb",
    "glue",
    "cloudsql",
    "db2",
    "hive",
    "impala",
    "informix",
    "kafka",
    "kinesis",
    "local",
    "mysql",
    "oracle",
    "postgresql",
    "postgres",
    "presto",
    "pubsub",
    "redshift",
    "s3",
    "sftp",
    "snowflake",
    "sqlserver",
    "synapse",
    "trino",
    "vertica",
    "zen",
    "custom",
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

    let Some(server_type) = server.server_type.as_deref() else {
        emit(
            report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "server entry requires a type",
            )
            .with_object_ref(format!("{object_ref}.type")),
        );
        validate_details_keys(report, &server.details, object_ref);
        return;
    };

    if server_type.is_empty() {
        emit(
            report,
            validation_error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "server entry requires a non-empty type",
            )
            .with_object_ref(format!("{object_ref}.type")),
        );
    } else if !SERVER_TYPES.contains(&server_type) {
        emit(
            report,
            validation_error(
                codes::INVALID_SCHEMA,
                DiagnosticCategory::Structure,
                format!(
                    "unsupported server type '{server_type}'; expected one of the ODCS server types"
                ),
            )
            .with_object_ref(format!("{object_ref}.type")),
        );
    } else {
        validate_type_specific_fields(report, server, server_type, object_ref);
    }

    validate_details_keys(report, &server.details, object_ref);
}

fn validate_type_specific_fields(
    report: &mut DiagnosticReport,
    server: &Server,
    server_type: &str,
    object_ref: &str,
) {
    let fields = server_fields(server);
    match server_type {
        "snowflake" => require_fields(
            report,
            &fields,
            &["account", "database", "schema"],
            object_ref,
        ),
        "kafka" => require_fields(report, &fields, &["host"], object_ref),
        "postgresql" | "postgres" => require_fields(
            report,
            &fields,
            &["host", "port", "database", "schema"],
            object_ref,
        ),
        _ => {}
    }
}

fn server_fields(server: &Server) -> IndexMap<String, Value> {
    server.details.clone()
}

fn require_fields(
    report: &mut DiagnosticReport,
    fields: &IndexMap<String, Value>,
    required: &[&str],
    object_ref: &str,
) {
    for field in required {
        if !fields.contains_key(*field) {
            emit(
                report,
                validation_error(
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    format!("{object_ref} requires field '{field}' for this server type"),
                )
                .with_object_ref(format!("{object_ref}.{field}")),
            );
        }
    }
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

//! Structural validation — cross-field constraints.

use std::collections::HashSet;

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::DataContract;

/// Validate cross-field structural constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    let schema_names = collect_schema_names(contract);
    validate_unique_schema_names(&mut report, contract);
    validate_unique_server_names(&mut report, contract);
    validate_sla_element_refs(&mut report, contract, &schema_names);
    validate_sla_default_element(&mut report, contract, &schema_names);
    report
}

fn collect_schema_names(contract: &DataContract) -> HashSet<String> {
    contract
        .schema
        .iter()
        .filter_map(|schema| schema.element.name.as_ref())
        .filter(|name| !name.is_empty())
        .cloned()
        .collect()
}

fn validate_unique_schema_names(report: &mut DiagnosticReport, contract: &DataContract) {
    let mut seen = HashSet::new();

    for (index, schema) in contract.schema.iter().enumerate() {
        let Some(name) = schema.element.name.as_deref() else {
            continue;
        };
        if name.is_empty() {
            continue;
        }

        if !seen.insert(name.to_string()) {
            emit(
                report,
                validation_error(
                    ValidationPhase::Structural,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!("duplicate schema object name '{name}'"),
                )
                .with_object_ref(format!("schema[{index}].name"))
                .with_remediation("use unique non-empty schema object names"),
            );
        }
    }
}

fn validate_unique_server_names(report: &mut DiagnosticReport, contract: &DataContract) {
    let mut seen = HashSet::new();

    for (index, server) in contract.servers.iter().enumerate() {
        let Some(name) = server.server.as_deref() else {
            continue;
        };
        if name.is_empty() {
            continue;
        }

        if !seen.insert(name.to_string()) {
            emit(
                report,
                validation_error(
                    ValidationPhase::Structural,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!("duplicate server identifier '{name}'"),
                )
                .with_object_ref(format!("servers[{index}].server"))
                .with_remediation("use unique non-empty server identifiers"),
            );
        }
    }
}

fn validate_sla_element_refs(
    report: &mut DiagnosticReport,
    contract: &DataContract,
    schema_names: &HashSet<String>,
) {
    for (index, sla) in contract.sla_properties.iter().enumerate() {
        let Some(element) = sla.element.as_deref() else {
            continue;
        };

        validate_sla_element_tokens(
            report,
            element,
            &format!("slaProperties[{index}].element"),
            schema_names,
        );
    }
}

fn validate_sla_default_element(
    report: &mut DiagnosticReport,
    contract: &DataContract,
    schema_names: &HashSet<String>,
) {
    let Some(element) = contract.sla_default_element.as_deref() else {
        return;
    };

    validate_sla_element_tokens(report, element, "slaDefaultElement", schema_names);
}

fn validate_sla_element_tokens(
    report: &mut DiagnosticReport,
    value: &str,
    object_ref: &str,
    schema_names: &HashSet<String>,
) {
    for_each_sla_element_token(value, |token| {
        if !schema_names.contains(token) {
            emit_unresolved_schema_reference(report, token, object_ref);
        }
    });
}

fn for_each_sla_element_token(value: &str, mut f: impl FnMut(&str)) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    for token in trimmed
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        f(token);
    }
}

fn emit_unresolved_schema_reference(
    report: &mut DiagnosticReport,
    reference: &str,
    object_ref: &str,
) {
    emit(
        report,
        validation_error(
            ValidationPhase::Structural,
            codes::UNRESOLVED_REFERENCE,
            DiagnosticCategory::Reference,
            format!("unresolved schema object reference '{reference}'"),
        )
        .with_object_ref(object_ref)
        .with_remediation("reference an existing schema object name"),
    );
}

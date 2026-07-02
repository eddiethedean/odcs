//! Stable identifier validation.

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::{DataContract, SchemaProperty, StableId, TeamDeclaration};

fn is_valid_stable_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn validate_stable_id(report: &mut DiagnosticReport, id: &StableId, object_ref: &str) {
    if !is_valid_stable_id(&id.0) {
        emit(
            report,
            validation_error(
                ValidationPhase::Ids,
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Extension,
                format!("stable id '{}' contains invalid characters", id.0),
            )
            .with_object_ref(object_ref.to_string())
            .with_remediation("use only letters, numbers, underscores, and hyphens"),
        );
    }
}

fn validate_optional_stable_id(
    report: &mut DiagnosticReport,
    id: &Option<StableId>,
    object_ref: &str,
) {
    if let Some(id) = id {
        validate_stable_id(report, id, object_ref);
    }
}

/// Validate stable identifier patterns across the contract.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if !is_valid_stable_id(&contract.id) {
        emit(
            &mut report,
            validation_error(
                ValidationPhase::Ids,
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Extension,
                format!("contract id '{}' contains invalid characters", contract.id),
            )
            .with_object_ref("id".to_string()),
        );
    }

    for (index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{index}]");
        validate_optional_stable_id(&mut report, &schema.element.id, &format!("{base_ref}.id"));
        validate_property_ids(&mut report, &schema.properties, &base_ref);
    }

    for (index, server) in contract.servers.iter().enumerate() {
        validate_optional_stable_id(&mut report, &server.id, &format!("servers[{index}].id"));
    }

    for (index, role) in contract.roles.iter().enumerate() {
        validate_optional_stable_id(&mut report, &role.id, &format!("roles[{index}].id"));
    }

    for (index, sla) in contract.sla_properties.iter().enumerate() {
        validate_optional_stable_id(&mut report, &sla.id, &format!("slaProperties[{index}].id"));
    }

    if let Some(team) = &contract.team {
        validate_team_ids(&mut report, team);
    }

    report
}

fn validate_team_ids(report: &mut DiagnosticReport, team: &TeamDeclaration) {
    match team {
        TeamDeclaration::Team(team) => {
            validate_optional_stable_id(report, &team.id, "team.id");
            for (index, member) in team.members.iter().enumerate() {
                validate_optional_stable_id(
                    report,
                    &member.id,
                    &format!("team.members[{index}].id"),
                );
            }
        }
        TeamDeclaration::LegacyMembers(members) => {
            for (index, member) in members.iter().enumerate() {
                validate_optional_stable_id(report, &member.id, &format!("team[{index}].id"));
            }
        }
    }
}

fn validate_property_ids(report: &mut DiagnosticReport, properties: &[SchemaProperty], base: &str) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        validate_optional_stable_id(report, &property.element.id, &format!("{prop_ref}.id"));
        if !property.properties.is_empty() {
            validate_property_ids(report, &property.properties, &prop_ref);
        }
        if let Some(items) = &property.items {
            validate_property_ids(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

//! Section-level validation for team, roles, support, SLA, and pricing.

use std::collections::HashSet;

use crate::diagnostics::{
    codes, emit, validation_error, DiagnosticCategory, DiagnosticReport, ValidationPhase,
};
use crate::model::{DataContract, TeamDeclaration};

/// Validate non-root document sections.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if let Some(team) = &contract.team {
        validate_team(&mut report, team);
    }

    validate_roles(&mut report, contract);
    validate_support(&mut report, contract);
    validate_sla(&mut report, contract);
    validate_pricing(&mut report, contract);

    report
}

fn validate_team(report: &mut DiagnosticReport, team: &TeamDeclaration) {
    match team {
        TeamDeclaration::Team(team) => {
            for (index, member) in team.members.iter().enumerate() {
                if member.username.is_empty() {
                    emit(
                        report,
                        validation_error(
                            ValidationPhase::Sections,
                            codes::MISSING_REQUIRED_FIELD,
                            DiagnosticCategory::Structure,
                            "team member username must not be empty",
                        )
                        .with_object_ref(format!("team.members[{index}].username")),
                    );
                }
            }
        }
        TeamDeclaration::LegacyMembers(members) => {
            for (index, member) in members.iter().enumerate() {
                if member.username.is_empty() {
                    emit(
                        report,
                        validation_error(
                            ValidationPhase::Sections,
                            codes::MISSING_REQUIRED_FIELD,
                            DiagnosticCategory::Structure,
                            "team member username must not be empty",
                        )
                        .with_object_ref(format!("team[{index}].username")),
                    );
                }
            }
        }
    }
}

fn validate_roles(report: &mut DiagnosticReport, contract: &DataContract) {
    let mut seen = HashSet::new();

    for (index, role) in contract.roles.iter().enumerate() {
        let Some(id) = role.id.as_ref().map(|id| id.0.as_str()) else {
            continue;
        };
        if id.is_empty() {
            continue;
        }

        if !seen.insert(id.to_string()) {
            emit(
                report,
                validation_error(
                    ValidationPhase::Sections,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    format!("duplicate role id '{id}'"),
                )
                .with_object_ref(format!("roles[{index}].id"))
                .with_remediation("use unique non-empty role ids"),
            );
        }
    }
}

fn support_tool_requires_url(tool: &str) -> bool {
    matches!(
        tool,
        "slack" | "teams" | "discord" | "googlechat" | "ticket" | "other"
    )
}

fn validate_support(report: &mut DiagnosticReport, contract: &DataContract) {
    let Some(support) = &contract.support else {
        return;
    };

    for (index, item) in support.iter().enumerate() {
        let requires_url = item.tool.as_deref().is_some_and(support_tool_requires_url);

        let url_missing = item.url.as_ref().map(|url| url.is_empty()).unwrap_or(true);
        if requires_url && url_missing {
            emit(
                report,
                validation_error(
                    ValidationPhase::Sections,
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    format!(
                        "support url is required when tool is '{}'",
                        item.tool.as_deref().unwrap_or("other")
                    ),
                )
                .with_object_ref(format!("support[{index}].url")),
            );
        }
    }
}

fn validate_sla(report: &mut DiagnosticReport, contract: &DataContract) {
    for (index, sla) in contract.sla_properties.iter().enumerate() {
        let schedule_missing = sla
            .schedule
            .as_ref()
            .map(|value| value.is_empty())
            .unwrap_or(true);
        if sla
            .scheduler
            .as_deref()
            .is_some_and(|value| !value.is_empty())
            && schedule_missing
        {
            emit(
                report,
                validation_error(
                    ValidationPhase::Sections,
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "sla schedule is required when scheduler is set",
                )
                .with_object_ref(format!("slaProperties[{index}].schedule")),
            );
        }
    }
}

fn validate_pricing(report: &mut DiagnosticReport, contract: &DataContract) {
    let Some(price) = &contract.price else {
        return;
    };

    if let Some(amount) = price.price_amount {
        if amount < 0.0 {
            emit(
                report,
                validation_error(
                    ValidationPhase::Sections,
                    codes::INVALID_SCHEMA,
                    DiagnosticCategory::Structure,
                    "price amount must not be negative",
                )
                .with_object_ref("price.priceAmount"),
            );
        }

        let currency_missing = price
            .price_currency
            .as_ref()
            .map(|value| value.is_empty())
            .unwrap_or(true);
        if currency_missing {
            emit(
                report,
                validation_error(
                    ValidationPhase::Sections,
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "price currency is required when price amount is set",
                )
                .with_object_ref("price.priceCurrency"),
            );
        }
    }
}

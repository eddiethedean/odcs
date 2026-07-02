//! Section-level validation for team, roles, and related structures.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, TeamDeclaration};

/// Validate non-root document sections.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if let Some(team) = &contract.team {
        validate_team(&mut report, team);
    }

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

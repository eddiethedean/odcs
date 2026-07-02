//! Extension and custom property validation.

use std::collections::HashSet;

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{CustomProperties, DataContract, DataQuality, SchemaProperty, Server};

/// Validate extension and custom property constraints.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    validate_custom_properties(
        &mut report,
        contract.custom_properties.as_ref(),
        "customProperties",
    );

    if let Some(description) = &contract.description {
        validate_custom_properties(
            &mut report,
            description.custom_properties.as_ref(),
            "description.customProperties",
        );
    }

    if let Some(definitions) = &contract.authoritative_definitions {
        for (index, definition) in definitions.iter().enumerate() {
            if definition.url.is_empty() {
                emit(
                    &mut report,
                    validation_error(
                        codes::MISSING_REQUIRED_FIELD,
                        DiagnosticCategory::Structure,
                        "authoritative definition url must not be empty",
                    )
                    .with_object_ref(format!("authoritativeDefinitions[{index}].url")),
                );
            }
            if definition.definition_type.is_empty() {
                emit(
                    &mut report,
                    validation_error(
                        codes::MISSING_REQUIRED_FIELD,
                        DiagnosticCategory::Structure,
                        "authoritative definition type must not be empty",
                    )
                    .with_object_ref(format!("authoritativeDefinitions[{index}].type")),
                );
            }
        }
    }

    for (index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{index}]");
        validate_custom_properties(
            &mut report,
            schema.element.custom_properties.as_ref(),
            &format!("{base_ref}.customProperties"),
        );
        if let Some(rules) = &schema.quality {
            validate_quality_extensions(&mut report, rules, &format!("{base_ref}.quality"));
        }
        for (rel_index, relationship) in schema.relationships.iter().enumerate() {
            validate_custom_properties(
                &mut report,
                relationship.base.custom_properties.as_ref(),
                &format!("{base_ref}.relationships[{rel_index}].customProperties"),
            );
        }
        validate_property_extensions(&mut report, &schema.properties, &base_ref);
    }

    for (index, server) in contract.servers.iter().enumerate() {
        validate_server_extensions(&mut report, server, &format!("servers[{index}]"));
    }

    for (index, role) in contract.roles.iter().enumerate() {
        validate_custom_properties(
            &mut report,
            role.custom_properties.as_ref(),
            &format!("roles[{index}].customProperties"),
        );
        if role.role.is_empty() {
            emit(
                &mut report,
                validation_error(
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "role name must not be empty",
                )
                .with_object_ref(format!("roles[{index}].role")),
            );
        }
    }

    if let Some(team) = &contract.team {
        match team {
            crate::model::TeamDeclaration::Team(team) => {
                validate_custom_properties(
                    &mut report,
                    team.custom_properties.as_ref(),
                    "team.customProperties",
                );
                for (index, member) in team.members.iter().enumerate() {
                    validate_custom_properties(
                        &mut report,
                        member.custom_properties.as_ref(),
                        &format!("team.members[{index}].customProperties"),
                    );
                }
            }
            crate::model::TeamDeclaration::LegacyMembers(members) => {
                for (index, member) in members.iter().enumerate() {
                    validate_custom_properties(
                        &mut report,
                        member.custom_properties.as_ref(),
                        &format!("team[{index}].customProperties"),
                    );
                }
            }
        }
    }

    if let Some(support) = &contract.support {
        for (index, item) in support.iter().enumerate() {
            validate_custom_properties(
                &mut report,
                item.custom_properties.as_ref(),
                &format!("support[{index}].customProperties"),
            );
            if item.channel.is_empty() {
                emit(
                    &mut report,
                    validation_error(
                        codes::MISSING_REQUIRED_FIELD,
                        DiagnosticCategory::Structure,
                        "support channel must not be empty",
                    )
                    .with_object_ref(format!("support[{index}].channel")),
                );
            }
        }
    }

    for (index, sla) in contract.sla_properties.iter().enumerate() {
        if sla.property.is_empty() {
            emit(
                &mut report,
                validation_error(
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    "sla property name must not be empty",
                )
                .with_object_ref(format!("slaProperties[{index}].property")),
            );
        }
    }

    report
}

fn validate_quality_extensions(report: &mut DiagnosticReport, rules: &[DataQuality], base: &str) {
    for (index, rule) in rules.iter().enumerate() {
        validate_custom_properties(
            report,
            rule.custom_properties.as_ref(),
            &format!("{base}[{index}].customProperties"),
        );
    }
}

fn validate_server_extensions(report: &mut DiagnosticReport, server: &Server, object_ref: &str) {
    validate_custom_properties(
        report,
        server.custom_properties.as_ref(),
        &format!("{object_ref}.customProperties"),
    );
    for (index, role) in server.roles.iter().enumerate() {
        validate_custom_properties(
            report,
            role.custom_properties.as_ref(),
            &format!("{object_ref}.roles[{index}].customProperties"),
        );
    }
}

fn validate_property_extensions(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        validate_custom_properties(
            report,
            property.element.custom_properties.as_ref(),
            &format!("{prop_ref}.customProperties"),
        );
        if let Some(rules) = &property.quality {
            validate_quality_extensions(report, rules, &format!("{prop_ref}.quality"));
        }
        for (rel_index, relationship) in property.relationships.iter().enumerate() {
            validate_custom_properties(
                report,
                relationship.base.custom_properties.as_ref(),
                &format!("{prop_ref}.relationships[{rel_index}].customProperties"),
            );
        }
        if !property.properties.is_empty() {
            validate_property_extensions(report, &property.properties, &prop_ref);
        }
        if let Some(items) = &property.items {
            validate_property_extensions(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

fn validate_custom_properties(
    report: &mut DiagnosticReport,
    properties: Option<&CustomProperties>,
    object_ref: &str,
) {
    let Some(properties) = properties else {
        return;
    };

    let mut seen = HashSet::new();
    for (index, property) in properties.iter().enumerate() {
        if property.property.is_empty() {
            emit(
                report,
                validation_error(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Extension,
                    "custom property key must not be empty",
                )
                .with_object_ref(format!("{object_ref}[{index}].property")),
            );
            continue;
        }
        if !seen.insert(property.property.clone()) {
            emit(
                report,
                validation_error(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Extension,
                    format!("duplicate custom property key '{}'", property.property),
                )
                .with_object_ref(format!("{object_ref}[{index}].property")),
            );
        }
    }
}

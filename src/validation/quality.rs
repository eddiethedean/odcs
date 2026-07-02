//! Quality rule validation.

use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, DataQuality, SchemaProperty};

const LIBRARY_METRICS: &[&str] = &[
    "nullValues",
    "missingValues",
    "invalidValues",
    "duplicateValues",
    "rowCount",
];

/// Validate quality rules declared on schema objects and properties.
#[must_use]
pub fn validate(contract: &DataContract) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for (schema_index, schema) in contract.schema.iter().enumerate() {
        let base_ref = format!("schema[{schema_index}]");
        if let Some(rules) = &schema.quality {
            for (rule_index, rule) in rules.iter().enumerate() {
                validate_rule(
                    &mut report,
                    rule,
                    &format!("{base_ref}.quality[{rule_index}]"),
                );
            }
        }
        validate_property_quality(&mut report, &schema.properties, &base_ref);
    }

    report
}

fn validate_property_quality(
    report: &mut DiagnosticReport,
    properties: &[SchemaProperty],
    base: &str,
) {
    for (index, property) in properties.iter().enumerate() {
        let prop_ref = format!("{base}.properties[{index}]");
        if let Some(rules) = &property.quality {
            for (rule_index, rule) in rules.iter().enumerate() {
                validate_rule(report, rule, &format!("{prop_ref}.quality[{rule_index}]"));
            }
        }
        if !property.properties.is_empty() {
            validate_property_quality(report, &property.properties, &prop_ref);
        }
        if let Some(items) = &property.items {
            validate_property_quality(
                report,
                std::slice::from_ref(items),
                &format!("{prop_ref}.items"),
            );
        }
    }
}

fn validate_rule(report: &mut DiagnosticReport, rule: &DataQuality, object_ref: &str) {
    if rule.rule_type.as_deref() == Some("library") {
        if let Some(metric) = &rule.metric {
            if !LIBRARY_METRICS.contains(&metric.as_str()) {
                emit(
                    report,
                    validation_error(
                        codes::INVALID_QUALITY,
                        DiagnosticCategory::Semantic,
                        format!(
                            "unsupported library metric '{metric}'; expected one of: {}",
                            LIBRARY_METRICS.join(", ")
                        ),
                    )
                    .with_object_ref(format!("{object_ref}.metric"))
                    .with_remediation("use a v3.1.0 library metric name"),
                );
            }
        } else if rule.rule.is_none() {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    "library quality rules require metric",
                )
                .with_object_ref(format!("{object_ref}.metric")),
            );
        }
    }

    if rule.rule_type.as_deref() == Some("custom") {
        if rule.engine.is_none() {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    "custom quality rules require engine",
                )
                .with_object_ref(format!("{object_ref}.engine")),
            );
        }
        if rule.implementation.is_none() {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    "custom quality rules require implementation",
                )
                .with_object_ref(format!("{object_ref}.implementation")),
            );
        }
    }

    if rule.rule_type.as_deref() == Some("sql") && rule.query.is_none() {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "sql quality rules require query",
            )
            .with_object_ref(format!("{object_ref}.query")),
        );
    }
}

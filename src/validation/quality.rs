//! Quality rule validation.

use super::helpers::{is_blank, is_empty_value, is_known_rule_type, normalized_rule_type};
use crate::diagnostics::{codes, emit, validation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{DataContract, DataQuality, SchemaProperty};

const LIBRARY_METRICS: &[&str] = &[
    "nullValues",
    "missingValues",
    "invalidValues",
    "duplicateValues",
    "rowCount",
];

const QUALITY_DIMENSIONS: &[&str] = &[
    "accuracy",
    "completeness",
    "conformity",
    "consistency",
    "coverage",
    "timeliness",
    "uniqueness",
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
    if let Some(dimension) = rule.dimension.as_deref() {
        if !QUALITY_DIMENSIONS.contains(&dimension) {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    format!(
                        "unsupported quality dimension '{dimension}'; expected one of: {}",
                        QUALITY_DIMENSIONS.join(", ")
                    ),
                )
                .with_object_ref(format!("{object_ref}.dimension")),
            );
        }
    }

    if let Some(rule_type) = rule.rule_type.as_deref() {
        if !is_known_rule_type(rule_type) {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    format!(
                        "unsupported quality rule type '{rule_type}'; expected one of: text, library, sql, custom"
                    ),
                )
                .with_object_ref(format!("{object_ref}.type")),
            );
            return;
        }
    }

    let Some(normalized) = normalized_rule_type(rule) else {
        return;
    };

    match normalized.as_str() {
        "library" => validate_library_rule(report, rule, object_ref),
        "custom" => validate_custom_rule(report, rule, object_ref),
        "sql" => validate_sql_rule(report, rule, object_ref),
        "text" => {}
        _ => {}
    }

    validate_between_constraints(report, rule, object_ref);
}

fn validate_library_rule(report: &mut DiagnosticReport, rule: &DataQuality, object_ref: &str) {
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
    } else if let Some(legacy_rule) = &rule.rule {
        if !LIBRARY_METRICS.contains(&legacy_rule.as_str()) {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    format!(
                        "deprecated rule '{legacy_rule}' is not a supported library metric; use metric instead"
                    ),
                )
                .with_object_ref(format!("{object_ref}.rule"))
                .with_remediation("set metric to a v3.1.0 library metric name"),
            );
        } else {
            emit(
                report,
                validation_error(
                    codes::INVALID_QUALITY,
                    DiagnosticCategory::Semantic,
                    "library quality rules require metric; deprecated rule field is not sufficient",
                )
                .with_object_ref(format!("{object_ref}.metric"))
                .with_remediation("set metric to the desired library metric name"),
            );
        }
    } else {
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

    if !has_quality_operator(rule) {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "library quality rules require a comparison operator (mustBe, mustBeGreaterThan, mustBeBetween, etc.)",
            )
            .with_object_ref(object_ref.to_string()),
        );
    }
}

fn has_quality_operator(rule: &DataQuality) -> bool {
    rule.must_be.is_some()
        || rule.must_not_be.is_some()
        || rule.must_be_greater_than.is_some()
        || rule.must_be_greater_or_equal_to.is_some()
        || rule.must_be_less_than.is_some()
        || rule.must_be_less_or_equal_to.is_some()
        || rule.must_be_between.is_some()
        || rule.must_not_be_between.is_some()
}

fn validate_custom_rule(report: &mut DiagnosticReport, rule: &DataQuality, object_ref: &str) {
    if is_blank(&rule.engine) {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "custom quality rules require a non-empty engine",
            )
            .with_object_ref(format!("{object_ref}.engine")),
        );
    }
    if is_empty_value(&rule.implementation) {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "custom quality rules require a non-empty implementation",
            )
            .with_object_ref(format!("{object_ref}.implementation")),
        );
    }
}

fn validate_sql_rule(report: &mut DiagnosticReport, rule: &DataQuality, object_ref: &str) {
    if is_blank(&rule.query) {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "sql quality rules require a non-empty query",
            )
            .with_object_ref(format!("{object_ref}.query")),
        );
    }
}

fn validate_between_constraints(
    report: &mut DiagnosticReport,
    rule: &DataQuality,
    object_ref: &str,
) {
    validate_between_range(
        report,
        rule.must_be_between.as_deref(),
        &format!("{object_ref}.mustBeBetween"),
    );
    validate_between_range(
        report,
        rule.must_not_be_between.as_deref(),
        &format!("{object_ref}.mustNotBeBetween"),
    );
}

fn validate_between_range(report: &mut DiagnosticReport, values: Option<&[f64]>, object_ref: &str) {
    let Some(values) = values else {
        return;
    };
    if values.len() != 2 {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                format!(
                    "range constraint must contain exactly 2 values, got {}",
                    values.len()
                ),
            )
            .with_object_ref(object_ref.to_string()),
        );
        return;
    }
    if values[0] > values[1] {
        emit(
            report,
            validation_error(
                codes::INVALID_QUALITY,
                DiagnosticCategory::Semantic,
                "range constraint minimum must be less than or equal to maximum",
            )
            .with_object_ref(object_ref.to_string()),
        );
    }
}

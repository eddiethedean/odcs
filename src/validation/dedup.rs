//! Diagnostic deduplication helpers.

use std::collections::HashSet;

use crate::diagnostics::{codes, DiagnosticReport};

/// Normalize JSON Pointer or dotted object references for comparison.
fn normalize_object_ref(object_ref: &str) -> String {
    let trimmed = object_ref.trim();
    if trimmed.starts_with('/') {
        let mut result = String::new();
        for segment in trimmed.trim_start_matches('/').split('/') {
            if segment.is_empty() {
                continue;
            }
            if segment.chars().all(|c| c.is_ascii_digit()) {
                result.push('[');
                result.push_str(segment);
                result.push(']');
            } else if result.is_empty() {
                result.push_str(segment);
            } else {
                result.push('.');
                result.push_str(segment);
            }
        }
        result
    } else {
        trimmed.to_string()
    }
}

/// Remove JSON Schema diagnostics that duplicate a Rust validation diagnostic on the same field.
pub fn dedup_json_schema_overlap(report: &mut DiagnosticReport) {
    let rust_paths: HashSet<String> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.id != codes::JSON_SCHEMA_VIOLATION)
        .filter_map(|diagnostic| diagnostic.object_ref.as_deref())
        .map(normalize_object_ref)
        .collect();

    report.diagnostics.retain(|diagnostic| {
        if diagnostic.id != codes::JSON_SCHEMA_VIOLATION {
            return true;
        }
        let Some(object_ref) = diagnostic.object_ref.as_deref() else {
            return true;
        };
        let normalized = normalize_object_ref(object_ref);
        !rust_paths.contains(&normalized)
    });
}

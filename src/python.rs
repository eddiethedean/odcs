//! Python bindings exposed through maturin as `pyodcs._native`.

use pyo3::exceptions::{
    PyFileNotFoundError, PyOSError, PyPermissionError, PyTypeError, PyValueError,
};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use serde::Serialize;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::compatibility::diff;
use crate::contract_set::{load_set, validate_set_with_options};
use crate::diagnostics::inspect_contract;
use crate::model::DataContract;
use crate::parser::{parse, parse_file, DocumentFormat, ParseResult};
use crate::schema;
use crate::validation::{validate_with_options, ValidationOptions, ValidationPhase};

fn value_to_py(py: Python<'_>, value: &impl Serialize) -> PyResult<Py<PyAny>> {
    let json = serde_json::to_string(value)
        .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))?;
    let json_mod = py.import("json")?;
    json_mod
        .call_method1("loads", (json,))
        .map(|obj| obj.unbind())
}

fn parse_format(format: &str) -> PyResult<DocumentFormat> {
    match format.to_lowercase().as_str() {
        "yaml" | "yml" => Ok(DocumentFormat::Yaml),
        "json" => Ok(DocumentFormat::Json),
        other => Err(PyValueError::new_err(format!(
            "unsupported format '{other}'; use 'yaml' or 'json'"
        ))),
    }
}

fn content_to_bytes(content: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    if content.is_none() {
        return Err(PyTypeError::new_err("content must be str or bytes"));
    }
    if let Ok(text) = content.extract::<String>() {
        return Ok(text.into_bytes());
    }
    if let Ok(data) = content.extract::<Vec<u8>>() {
        return Ok(data);
    }
    if let Ok(byte_array) = content.downcast::<PyByteArray>() {
        return Ok(unsafe { byte_array.as_bytes().to_vec() });
    }
    Err(PyTypeError::new_err(
        "content must be str, bytes, or bytearray",
    ))
}

fn contract_from_py(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<DataContract> {
    if contract.is_none() {
        return Err(PyTypeError::new_err("contract must be a dict, not None"));
    }
    let json_mod = py.import("json")?;
    let json_str: String = json_mod.call_method1("dumps", (contract,))?.extract()?;
    serde_json::from_str(&json_str)
        .map_err(|e| PyValueError::new_err(format!("invalid contract: {e}")))
}

fn parse_result_to_py(py: Python<'_>, result: ParseResult) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    match result.contract {
        Some(contract) => dict.set_item("contract", value_to_py(py, &contract)?)?,
        None => dict.set_item("contract", py.None())?,
    }
    dict.set_item("report", value_to_py(py, &result.report)?)?;
    Ok(dict.into())
}

/// Upstream ODCS specification version this crate targets.
#[pyfunction]
fn upstream_spec_version() -> &'static str {
    crate::UPSTREAM_SPEC_VERSION
}

/// Parse an ODCS document from text or bytes.
#[pyfunction]
#[pyo3(signature = (content, format="yaml"))]
fn parse_document(py: Python<'_>, content: &Bound<'_, PyAny>, format: &str) -> PyResult<Py<PyAny>> {
    let bytes = content_to_bytes(content)?;
    let doc_format = parse_format(format)?;
    parse_result_to_py(py, parse(&bytes, doc_format))
}

/// Parse an ODCS document from a file path.
#[pyfunction]
fn parse_path(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let path_obj = Path::new(path);
    if let Err(error) = std::fs::metadata(path_obj) {
        return Err(map_io_error(error, path));
    }
    let result = parse_file(path_obj).map_err(|error| PyValueError::new_err(error.to_string()))?;
    parse_result_to_py(py, result)
}

fn map_io_error(error: std::io::Error, path: &str) -> PyErr {
    let message = format!("failed to read {path}: {error}");
    match error.kind() {
        ErrorKind::NotFound => PyFileNotFoundError::new_err(message),
        ErrorKind::PermissionDenied => PyPermissionError::new_err(message),
        _ => PyOSError::new_err(message),
    }
}

/// Validate a parsed data contract.
#[pyfunction]
#[pyo3(signature = (contract, strict=false))]
fn validate_contract(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
    strict: bool,
) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    let options = if strict {
        ValidationOptions::strict()
    } else {
        ValidationOptions::default_options()
    };
    value_to_py(py, &validate_with_options(&contract, options))
}

/// Parse and validate an ODCS document in one step.
#[pyfunction]
#[pyo3(signature = (content, format="yaml", strict=false))]
fn validate_document(
    py: Python<'_>,
    content: &Bound<'_, PyAny>,
    format: &str,
    strict: bool,
) -> PyResult<Py<PyAny>> {
    let bytes = content_to_bytes(content)?;
    let doc_format = parse_format(format)?;
    let result = parse(&bytes, doc_format);
    let mut report = result.report;
    if let Some(contract) = result.contract {
        let options = if strict {
            ValidationOptions::strict()
        } else {
            ValidationOptions::default_options()
        };
        report.merge(validate_with_options(&contract, options));
    }
    value_to_py(py, &report)
}

/// Parse and validate a primary contract with optional dependency paths.
#[pyfunction]
#[pyo3(signature = (primary, deps=None, includes=None, strict=false))]
fn parse_and_validate_paths(
    py: Python<'_>,
    primary: &str,
    deps: Option<Vec<String>>,
    includes: Option<Vec<String>>,
    strict: bool,
) -> PyResult<Py<PyAny>> {
    let deps: Vec<PathBuf> = deps
        .unwrap_or_default()
        .into_iter()
        .map(PathBuf::from)
        .collect();
    let includes: Vec<PathBuf> = includes
        .unwrap_or_default()
        .into_iter()
        .map(PathBuf::from)
        .collect();
    let options = if strict {
        ValidationOptions::strict()
    } else {
        ValidationOptions::default_options()
    };

    let report = if deps.is_empty() && includes.is_empty() {
        let result = parse_file(Path::new(primary))
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        let mut report = result.report;
        if let Some(contract) = result.contract {
            report.merge(validate_with_options(&contract, options));
        }
        report
    } else {
        match load_set(Path::new(primary), &deps, &includes) {
            Ok(set) => validate_set_with_options(&set, options),
            Err(report) => report,
        }
    };

    value_to_py(py, &report)
}

/// Return a short human-readable contract summary.
#[pyfunction]
fn inspect(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<String> {
    let contract = contract_from_py(py, contract)?;
    Ok(inspect_contract(&contract))
}

/// Return the number of nested quality rules in a contract.
#[pyfunction]
fn quality_rules_count(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<usize> {
    let contract = contract_from_py(py, contract)?;
    Ok(contract.quality_rules().len())
}

/// Return inspect summary fields as JSON-compatible dict.
#[pyfunction]
fn inspect_summary(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    let summary = serde_json::json!({
        "id": contract.id,
        "name": contract.name,
        "version": contract.version,
        "apiVersion": contract.api_version,
        "kind": contract.kind,
        "status": contract.status,
        "schemaCount": contract.schema.len(),
        "qualityCount": contract.quality_rules().len(),
    });
    value_to_py(py, &summary)
}

/// Return the pinned ODCS JSON Schema as a JSON-compatible dict.
#[pyfunction]
#[pyo3(signature = (json_metadata=false))]
fn pinned_schema(py: Python<'_>, json_metadata: bool) -> PyResult<Py<PyAny>> {
    if json_metadata {
        let payload = serde_json::json!({
            "schemaVersion": crate::UPSTREAM_SPEC_VERSION,
            "upstreamUrl": schema::UPSTREAM_REPOSITORY_URL,
            "schema": schema::pinned_schema_value(),
        });
        value_to_py(py, &payload)
    } else {
        value_to_py(py, schema::pinned_schema_value())
    }
}

/// Return stable `odcs:` diagnostic code constants.
#[pyfunction]
fn diagnostic_codes(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("PARSE_YAML", crate::diagnostics::codes::PARSE_YAML)?;
    dict.set_item("PARSE_JSON", crate::diagnostics::codes::PARSE_JSON)?;
    dict.set_item(
        "UNSUPPORTED_VERSION",
        crate::diagnostics::codes::UNSUPPORTED_VERSION,
    )?;
    dict.set_item(
        "MISSING_REQUIRED_FIELD",
        crate::diagnostics::codes::MISSING_REQUIRED_FIELD,
    )?;
    dict.set_item("INVALID_KIND", crate::diagnostics::codes::INVALID_KIND)?;
    dict.set_item("INVALID_SCHEMA", crate::diagnostics::codes::INVALID_SCHEMA)?;
    dict.set_item(
        "INVALID_QUALITY",
        crate::diagnostics::codes::INVALID_QUALITY,
    )?;
    dict.set_item("UNKNOWN_FIELD", crate::diagnostics::codes::UNKNOWN_FIELD)?;
    dict.set_item(
        "UNRESOLVED_REFERENCE",
        crate::diagnostics::codes::UNRESOLVED_REFERENCE,
    )?;
    dict.set_item(
        "INVALID_EXTENSION",
        crate::diagnostics::codes::INVALID_EXTENSION,
    )?;
    dict.set_item("DUPLICATE_KEY", crate::diagnostics::codes::DUPLICATE_KEY)?;
    dict.set_item(
        "DOCUMENT_TOO_LARGE",
        crate::diagnostics::codes::DOCUMENT_TOO_LARGE,
    )?;
    dict.set_item(
        "JSON_SCHEMA_VIOLATION",
        crate::diagnostics::codes::JSON_SCHEMA_VIOLATION,
    )?;
    Ok(dict.into())
}

/// Return validation pipeline phase name constants.
#[pyfunction]
fn validation_phases(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("DOCUMENT", ValidationPhase::Document.as_str())?;
    dict.set_item("STRUCTURAL", ValidationPhase::Structural.as_str())?;
    dict.set_item("SCHEMA", ValidationPhase::Schema.as_str())?;
    dict.set_item("QUALITY", ValidationPhase::Quality.as_str())?;
    dict.set_item("REFERENCES", ValidationPhase::References.as_str())?;
    dict.set_item("EXTENSIONS", ValidationPhase::Extensions.as_str())?;
    dict.set_item("SERVERS", ValidationPhase::Servers.as_str())?;
    dict.set_item("SECTIONS", ValidationPhase::Sections.as_str())?;
    dict.set_item("IDS", ValidationPhase::Ids.as_str())?;
    dict.set_item("JSON_SCHEMA", ValidationPhase::JsonSchema.as_str())?;
    Ok(dict.into())
}

/// Compare two parsed contracts for compatibility.
#[pyfunction]
fn diff_contracts(
    py: Python<'_>,
    old: &Bound<'_, PyAny>,
    new: &Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    let old = contract_from_py(py, old)?;
    let new = contract_from_py(py, new)?;
    value_to_py(py, &diff(&old, &new))
}

/// Native extension module for the Python `pyodcs` package.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(upstream_spec_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_document, m)?)?;
    m.add_function(wrap_pyfunction!(parse_path, m)?)?;
    m.add_function(wrap_pyfunction!(validate_contract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_document, m)?)?;
    m.add_function(wrap_pyfunction!(parse_and_validate_paths, m)?)?;
    m.add_function(wrap_pyfunction!(diff_contracts, m)?)?;
    m.add_function(wrap_pyfunction!(pinned_schema, m)?)?;
    m.add_function(wrap_pyfunction!(diagnostic_codes, m)?)?;
    m.add_function(wrap_pyfunction!(validation_phases, m)?)?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    m.add_function(wrap_pyfunction!(quality_rules_count, m)?)?;
    m.add_function(wrap_pyfunction!(inspect_summary, m)?)?;
    Ok(())
}

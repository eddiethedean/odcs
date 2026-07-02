//! Python bindings exposed through maturin as `pyodcs._native`.

use pyo3::exceptions::{
    PyFileNotFoundError, PyOSError, PyPermissionError, PyTypeError, PyValueError,
};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use serde::Serialize;
use std::io::ErrorKind;
use std::path::Path;

use crate::diagnostics::inspect_contract;
use crate::model::DataContract;
use crate::parser::{parse, parse_file, DocumentFormat, ParseResult};
use crate::schema;
use crate::validation::{validate_with_options, ValidationOptions};

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

/// Native extension module for the Python `pyodcs` package.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(upstream_spec_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_document, m)?)?;
    m.add_function(wrap_pyfunction!(parse_path, m)?)?;
    m.add_function(wrap_pyfunction!(validate_contract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_document, m)?)?;
    m.add_function(wrap_pyfunction!(pinned_schema, m)?)?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    m.add_function(wrap_pyfunction!(quality_rules_count, m)?)?;
    m.add_function(wrap_pyfunction!(inspect_summary, m)?)?;
    Ok(())
}

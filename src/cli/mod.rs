//! Command-line interface.

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::diagnostics::{inspect_contract, DiagnosticReport, DiagnosticStage};
use crate::parser::{parse_file, ParseResult};
use crate::schema::{self, UPSTREAM_REPOSITORY_URL};
use crate::validation::ValidationOptions;
use crate::UPSTREAM_SPEC_VERSION;

/// ODCS command-line tool.
#[derive(Debug, Parser)]
#[command(
    name = "odcs",
    version,
    about = "Validate Open Data Contract Standard documents"
)]
pub struct Cli {
    #[command(subcommand)]
    /// Subcommand to execute.
    pub command: Command,
}

/// Supported CLI commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse and validate a contract.
    Validate {
        /// Path to an ODCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
        /// Enable strict validation (Rust pipeline plus JSON Schema).
        #[arg(long)]
        strict: bool,
    },
    /// Print a contract summary.
    Inspect {
        /// Path to an ODCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Print validation diagnostics.
    Diagnostics {
        /// Path to an ODCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Print pinned ODCS JSON Schema.
    Schema {
        /// Emit JSON output with schema metadata.
        #[arg(long)]
        json: bool,
        /// Print upstream repository URL only.
        #[arg(long)]
        url_only: bool,
    },
    /// Print tool and upstream specification versions.
    Version {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

/// Run the CLI application.
pub fn run(cli: Cli) -> i32 {
    match cli.command {
        Command::Validate { path, json, strict } => {
            let result = match parse_file(&path) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("{error}");
                    return 2;
                }
            };
            let options = if strict {
                ValidationOptions::strict()
            } else {
                ValidationOptions::default_options()
            };
            let report = result.validate_with_options(options);
            if let Err(error) = render_report(&report, json, ReportMode::Validate) {
                eprintln!("{error}");
                return 2;
            }
            exit_code_for_report(&report)
        }
        Command::Inspect { path, json } => {
            let result = match parse_file(&path) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("{error}");
                    return 2;
                }
            };
            if has_parse_failure(&result) {
                if let Err(error) = render_report(&result.report, json, ReportMode::Diagnostics) {
                    eprintln!("{error}");
                    return 2;
                }
                return 2;
            }
            let ParseResult {
                contract,
                report: parse_report,
            } = result;
            let mut report = parse_report;
            if let Some(ref contract) = contract {
                report.merge(crate::validate(contract));
            }
            if !report.is_valid() {
                if let Err(error) = render_report(&report, json, ReportMode::Diagnostics) {
                    eprintln!("{error}");
                    return 2;
                }
                return 1;
            }
            let Some(contract) = contract else {
                if let Err(error) = render_report(&report, json, ReportMode::Diagnostics) {
                    eprintln!("{error}");
                    return 2;
                }
                return 2;
            };
            if json {
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
                if let Err(code) = write_json_stdout(&summary) {
                    eprintln!("failed to write JSON output");
                    return code;
                }
            } else if let Err(error) = writeln!(io::stdout(), "{}", inspect_contract(&contract)) {
                eprintln!("{error}");
                return 2;
            }
            0
        }
        Command::Diagnostics { path, json } => {
            let result = match parse_file(&path) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("{error}");
                    return 2;
                }
            };
            let report = result.validate();
            if let Err(error) = render_report(&report, json, ReportMode::Diagnostics) {
                eprintln!("{error}");
                return 2;
            }
            exit_code_for_report(&report)
        }
        Command::Schema { json, url_only } => {
            if url_only {
                if let Err(error) = writeln!(
                    io::stdout(),
                    "Upstream ODCS JSON Schema: {UPSTREAM_REPOSITORY_URL}"
                ) {
                    eprintln!("{error}");
                    return 2;
                }
                return 0;
            }
            if json {
                let payload = serde_json::json!({
                    "schemaVersion": UPSTREAM_SPEC_VERSION,
                    "upstreamUrl": UPSTREAM_REPOSITORY_URL,
                    "schema": schema::pinned_schema_value(),
                });
                if let Err(code) = write_json_stdout(&payload) {
                    eprintln!("failed to write JSON output");
                    return code;
                }
            } else if let Err(error) = write!(io::stdout(), "{}", schema::PINNED_SCHEMA_JSON) {
                eprintln!("{error}");
                return 2;
            }
            0
        }
        Command::Version { json } => {
            if json {
                let payload = serde_json::json!({
                    "crateVersion": env!("CARGO_PKG_VERSION"),
                    "upstreamSpecVersion": UPSTREAM_SPEC_VERSION,
                });
                if let Err(code) = write_json_stdout(&payload) {
                    eprintln!("failed to write JSON output");
                    return code;
                }
            } else if let Err(error) = writeln!(
                io::stdout(),
                "odcs {} (upstream ODCS {})",
                env!("CARGO_PKG_VERSION"),
                UPSTREAM_SPEC_VERSION
            ) {
                eprintln!("{error}");
                return 2;
            }
            0
        }
    }
}

fn write_json_stdout(payload: &serde_json::Value) -> Result<(), i32> {
    let rendered = serde_json::to_string_pretty(payload).map_err(|_| 2)?;
    writeln!(io::stdout(), "{rendered}").map_err(|_| 2)
}

#[derive(Debug, Clone, Copy)]
enum ReportMode {
    Validate,
    Diagnostics,
}

fn has_parse_failure(result: &ParseResult) -> bool {
    result.contract.is_none()
        || result
            .report
            .diagnostics
            .iter()
            .any(|d| d.stage == DiagnosticStage::Parse)
}

fn exit_code_for_report(report: &DiagnosticReport) -> i32 {
    if report
        .diagnostics
        .iter()
        .any(|d| d.stage == DiagnosticStage::Parse)
    {
        return 2;
    }
    if report.is_valid() {
        0
    } else {
        1
    }
}

fn render_report(report: &DiagnosticReport, json: bool, mode: ReportMode) -> io::Result<()> {
    if json {
        let payload = match mode {
            ReportMode::Validate => serde_json::json!({
                "valid": report.is_valid(),
                "diagnostics": report.diagnostics,
            }),
            ReportMode::Diagnostics => serde_json::json!({
                "diagnostics": report.diagnostics,
            }),
        };
        writeln!(
            io::stdout(),
            "{}",
            serde_json::to_string_pretty(&payload)
                .map_err(|e| std::io::Error::other(e.to_string()))?
        )?;
        return Ok(());
    }

    if report.is_valid() {
        match mode {
            ReportMode::Validate => writeln!(io::stdout(), "valid")?,
            ReportMode::Diagnostics => writeln!(io::stdout(), "no diagnostics")?,
        }
        return Ok(());
    }

    for diagnostic in &report.diagnostics {
        writeln!(
            io::stdout(),
            "[{}] {}: {}",
            format!("{:?}", diagnostic.severity).to_lowercase(),
            diagnostic.id,
            diagnostic.message
        )?;
        if let Some(object_ref) = &diagnostic.object_ref {
            writeln!(io::stdout(), "  at: {object_ref}")?;
        }
        if let Some(remediation) = &diagnostic.remediation {
            writeln!(io::stdout(), "  hint: {remediation}")?;
        }
    }
    Ok(())
}

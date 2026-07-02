//! Command-line interface.

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::diagnostics::{inspect_contract, DiagnosticReport, DiagnosticStage};
use crate::parser::{parse_file, ParseResult};
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
        /// Enable strict validation (reserved for Phase 5/6).
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
    /// Print upstream JSON Schema location (reserved).
    Schema {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
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
            if strict {
                eprintln!("note: --strict validation is reserved for a future release");
            }
            let result = match parse_file(&path) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("{error}");
                    return 2;
                }
            };
            let report = result.validate();
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
            let mut report = result.report;
            if let Some(ref contract) = result.contract {
                report.merge(crate::validate(contract));
            }
            if !report.is_valid() {
                if let Err(error) = render_report(&report, json, ReportMode::Diagnostics) {
                    eprintln!("{error}");
                    return 2;
                }
                return 1;
            }
            let Some(contract) = result.contract else {
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
                if let Err(error) = writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&summary).unwrap_or_else(|e| e.to_string())
                ) {
                    eprintln!("{error}");
                    return 2;
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
        Command::Schema { json } => {
            let schema_url = "https://github.com/bitol-io/open-data-contract-standard";
            let write_result = if json {
                let payload = serde_json::json!({
                    "upstreamRepository": schema_url,
                    "note": "JSON Schema export is planned for a future release",
                });
                writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&payload).unwrap_or_else(|e| e.to_string())
                )
            } else {
                writeln!(
                    io::stdout(),
                    "Upstream ODCS JSON Schema: {schema_url}\n(JSON Schema export planned)"
                )
            };
            if write_result.is_err() {
                return 2;
            }
            0
        }
        Command::Version { json } => {
            let write_result = if json {
                let payload = serde_json::json!({
                    "crateVersion": env!("CARGO_PKG_VERSION"),
                    "upstreamSpecVersion": UPSTREAM_SPEC_VERSION,
                });
                writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&payload).unwrap_or_else(|e| e.to_string())
                )
            } else {
                writeln!(
                    io::stdout(),
                    "odcs {} (upstream ODCS {})",
                    env!("CARGO_PKG_VERSION"),
                    UPSTREAM_SPEC_VERSION
                )
            };
            if write_result.is_err() {
                return 2;
            }
            0
        }
    }
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
    }
    Ok(())
}

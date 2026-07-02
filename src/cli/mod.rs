//! Command-line interface.

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::diagnostics::{inspect_contract, DiagnosticReport};
use crate::parser::parse_file;
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
        /// Enable strict validation (reserved).
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
pub fn run(cli: Cli) -> miette::Result<i32> {
    match cli.command {
        Command::Validate {
            path,
            json,
            strict: _,
        } => {
            let result = parse_file(&path)?;
            let report = result.validate();
            render_report(&report, json, ReportMode::Validate)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Inspect { path, json } => {
            let result = parse_file(&path)?;
            let mut report = result.report;
            if let Some(ref contract) = result.contract {
                report.merge(crate::validate(contract));
            }
            if !report.is_valid() {
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }
            let Some(contract) = result.contract else {
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(2);
            };
            if json {
                let summary = serde_json::json!({
                    "name": contract.name,
                    "version": contract.version,
                    "kind": contract.kind,
                    "status": contract.status,
                    "schemaCount": contract.schema.len(),
                    "qualityCount": contract.quality.len(),
                });
                writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&summary).map_err(|e| miette::miette!("{e}"))?
                )
                .map_err(|e| miette::miette!("{e}"))?;
            } else {
                writeln!(io::stdout(), "{}", inspect_contract(&contract))
                    .map_err(|e| miette::miette!("{e}"))?;
            }
            Ok(0)
        }
        Command::Diagnostics { path, json } => {
            let result = parse_file(&path)?;
            let report = result.validate();
            render_report(&report, json, ReportMode::Diagnostics)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Schema { json } => {
            let schema_url = "https://github.com/bitol-io/open-data-contract-standard";
            if json {
                let payload = serde_json::json!({
                    "upstreamRepository": schema_url,
                    "note": "JSON Schema export is planned for a future release",
                });
                writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&payload).map_err(|e| miette::miette!("{e}"))?
                )
                .map_err(|e| miette::miette!("{e}"))?;
            } else {
                writeln!(
                    io::stdout(),
                    "Upstream ODCS JSON Schema: {schema_url}\n(JSON Schema export planned)"
                )
                .map_err(|e| miette::miette!("{e}"))?;
            }
            Ok(0)
        }
        Command::Version { json } => {
            if json {
                let payload = serde_json::json!({
                    "crateVersion": env!("CARGO_PKG_VERSION"),
                    "upstreamSpecVersion": UPSTREAM_SPEC_VERSION,
                });
                writeln!(
                    io::stdout(),
                    "{}",
                    serde_json::to_string_pretty(&payload).map_err(|e| miette::miette!("{e}"))?
                )
                .map_err(|e| miette::miette!("{e}"))?;
            } else {
                writeln!(
                    io::stdout(),
                    "odcs {} (upstream ODCS {})",
                    env!("CARGO_PKG_VERSION"),
                    UPSTREAM_SPEC_VERSION
                )
                .map_err(|e| miette::miette!("{e}"))?;
            }
            Ok(0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ReportMode {
    Validate,
    Diagnostics,
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

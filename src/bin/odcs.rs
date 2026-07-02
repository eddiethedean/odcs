//! ODCS command-line binary.

use clap::Parser;
use odcs::cli::{run, Cli};

fn main() {
    let code = run(Cli::parse());
    if code != 0 {
        std::process::exit(code);
    }
}

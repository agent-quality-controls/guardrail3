//! Command entry point for serialized `g3rs-code-ingestion` fixture output.

use std::io::Write;
use std::process::ExitCode;

use g3_workspace_crawl as _;
use g3rs_code_ingestion as _;
use g3rs_code_ingestion_types as _;
use g3rs_code_types as _;
use serde as _;
use serde_json as _;

/// Run the command and write JSON or an error.
fn main() -> ExitCode {
    match g3rs_code_ingestion_fixture_output::run_from_env() {
        Ok(output) => write_stdout(&output),
        Err(error) => write_stderr(&error.to_string()),
    }
}

/// Write command output to stdout.
fn write_stdout(output: &str) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    match stdout.write_all(format!("{output}\n").as_bytes()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_error) => ExitCode::FAILURE,
    }
}

/// Write command errors to stderr.
fn write_stderr(error: &str) -> ExitCode {
    let mut stderr = std::io::stderr().lock();
    match stderr.write_all(format!("{error}\n").as_bytes()) {
        Ok(()) => ExitCode::FAILURE,
        Err(_error) => ExitCode::FAILURE,
    }
}

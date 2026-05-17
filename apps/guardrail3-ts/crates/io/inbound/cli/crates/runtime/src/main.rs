#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive dep `siphasher` resolves to 0.3.11 (via swc_common in g3ts-arch-ingestion's SWC-based parser) and 1.0.2 (via other dependents); both versions are pinned by upstream crates this app does not own"
)]

use std::io::Write;

use clap::Parser;
use g3ts::{Cli, run_command_with_defaults};

/// Keeps direct crate references visible to `unused_crate_dependencies`.
mod deps {
    use g3_workspace_crawl as _;
    use g3ts as _;
    use g3ts_topology_file_tree_checks as _;
    use g3ts_topology_ingestion as _;
    use guardrail3_check_types as _;
    use guardrail3_ts_app_types as _;
    use guardrail3_ts_family_runner_config as _;
    use guardrail3_ts_family_runner_hooks as _;
    use guardrail3_ts_family_runner_structure as _;
    use guardrail3_ts_packages as _;
    use guardrail3_ts_report as _;
    use guardrail3_ts_validate_command as _;
}

fn main() -> std::process::ExitCode {
    run().map_or(std::process::ExitCode::FAILURE, |exit_code| exit_code)
}

/// Runs the CLI and writes its output streams.
fn run() -> std::io::Result<std::process::ExitCode> {
    let cli = Cli::parse();
    let output = run_command_with_defaults(cli.command);

    if !output.stdout.is_empty() {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(output.stdout.as_bytes())?;
        stdout.flush()?;
    }
    if !output.stderr.is_empty() {
        let mut stderr = std::io::stderr().lock();
        stderr.write_all(output.stderr.as_bytes())?;
        stderr.flush()?;
    }
    Ok(match output.exit_code {
        0 => std::process::ExitCode::SUCCESS,
        _ => std::process::ExitCode::FAILURE,
    })
}

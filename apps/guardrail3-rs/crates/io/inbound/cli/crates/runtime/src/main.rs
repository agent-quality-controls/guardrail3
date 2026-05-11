use std::io::Write;

use clap::Parser;
use guardrail3_rs::{Cli, CliFamilyRunner, PackageRuntime, PlainTextReportRenderer, run_command};

/// Keeps direct crate references visible to `unused_crate_dependencies`.
mod deps {
    use g3_workspace_crawl as _;
    use guardrail3_rs_app_types as _;
    use guardrail3_rs_family_runner_policy as _;
    use guardrail3_rs_family_runner_process as _;
    use guardrail3_rs_family_runner_quality as _;
    use guardrail3_rs_family_runner_structure as _;
    use guardrail3_rs_family_runner_style as _;
    use guardrail3_rs_family_runner_test as _;
    use guardrail3_rs_packages as _;
    use guardrail3_rs_report as _;
    use guardrail3_rs_validate_command as _;

    #[cfg(test)]
    use guardrail3_rs_assertions as _;
}

fn main() -> std::process::ExitCode {
    run().map_or(std::process::ExitCode::FAILURE, |exit_code| exit_code)
}

/// Runs the CLI and writes its output streams.
fn run() -> std::io::Result<std::process::ExitCode> {
    let cli = Cli::parse();
    let crawler = PackageRuntime;
    let family_runner = CliFamilyRunner;
    let renderer = PlainTextReportRenderer;
    let output = run_command(cli.command, &crawler, &family_runner, &renderer);

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

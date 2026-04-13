use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use guardrail3_rs_app_types::{FamilyRunner, SupportedFamily, ValidateRequest, WorkspaceCrawler};
use guardrail3_rs_packages::PackageRuntime;
use guardrail3_rs_report::PlainTextReportRenderer;
use guardrail3_rs_validate_command::execute;

#[derive(Parser, Debug)]
#[command(name = "guardrail3-rs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Validate {
        #[arg(long = "path")]
        path: PathBuf,
        #[arg(long = "family")]
        family: Vec<FamilyArg>,
        #[arg(long = "inventory", default_value_t = false)]
        inventory: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum FamilyArg {
    Apparch,
    Arch,
    Cargo,
    Clippy,
    Code,
    Deps,
    Deny,
    Fmt,
    Garde,
    Hooks,
    Release,
    Test,
    Toolchain,
    Topology,
}

impl From<FamilyArg> for SupportedFamily {
    fn from(value: FamilyArg) -> Self {
        match value {
            FamilyArg::Apparch => Self::Apparch,
            FamilyArg::Arch => Self::Arch,
            FamilyArg::Cargo => Self::Cargo,
            FamilyArg::Clippy => Self::Clippy,
            FamilyArg::Code => Self::Code,
            FamilyArg::Deps => Self::Deps,
            FamilyArg::Deny => Self::Deny,
            FamilyArg::Fmt => Self::Fmt,
            FamilyArg::Garde => Self::Garde,
            FamilyArg::Hooks => Self::Hooks,
            FamilyArg::Release => Self::Release,
            FamilyArg::Test => Self::Test,
            FamilyArg::Toolchain => Self::Toolchain,
            FamilyArg::Topology => Self::Topology,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let package_runtime = PackageRuntime;
    let renderer = PlainTextReportRenderer;
    let output = run_command(cli.command, &package_runtime, &package_runtime, &renderer);

    if !output.stdout.is_empty() {
        let mut stdout = std::io::stdout().lock();
        stdout
            .write_all(output.stdout.as_bytes())
            .expect("stdout write should succeed");
        stdout.flush().expect("stdout flush should succeed");
    }
    if !output.stderr.is_empty() {
        let mut stderr = std::io::stderr().lock();
        stderr
            .write_all(output.stderr.as_bytes())
            .expect("stderr write should succeed");
        stderr.flush().expect("stderr flush should succeed");
    }
    std::process::exit(output.exit_code);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn run_command(
    command: Command,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn guardrail3_rs_app_types::ReportRenderer,
) -> CliOutput {
    match command {
        Command::Validate {
            path,
            family,
            inventory,
        } => {
            let request = ValidateRequest {
                workspace_root: path,
                families: family.into_iter().map(Into::into).collect(),
                include_inventory: inventory,
            };
            match execute(&request, crawler, family_runner, renderer) {
                Ok(outcome) => CliOutput {
                    stdout: outcome.stdout,
                    stderr: outcome.stderr,
                    exit_code: outcome.exit_code,
                },
                Err(error) => CliOutput {
                    stdout: String::new(),
                    stderr: format!("{error}\n"),
                    exit_code: 1,
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::Parser;
    use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
    use guardrail3_check_types::G3CheckResult;
    use guardrail3_rs_app_types::{ReportRenderer, ValidateReport};

    use super::{Cli, CliOutput, Command, FamilyArg, run_command};

    #[derive(Debug)]
    struct StubCrawler;

    impl super::WorkspaceCrawler for StubCrawler {
        fn crawl(&self, _root: &Path) -> Result<G3RsWorkspaceCrawl, String> {
            Err("crawl failed".to_owned())
        }
    }

    #[derive(Debug)]
    struct StubFamilyRunner;

    impl super::FamilyRunner for StubFamilyRunner {
        fn run_family(
            &self,
            _family: super::SupportedFamily,
            _crawl: &G3RsWorkspaceCrawl,
        ) -> Result<Vec<G3CheckResult>, String> {
            Ok(Vec::new())
        }
    }

    #[derive(Debug)]
    struct StubRenderer;

    impl ReportRenderer for StubRenderer {
        fn render(&self, _report: &ValidateReport, _include_inventory: bool) -> String {
            "rendered\n".to_owned()
        }
    }

    #[test]
    fn run_command_sends_failures_to_stderr() {
        let output = run_command(
            Command::Validate {
                path: ".".into(),
                family: Vec::new(),
                inventory: false,
            },
            &StubCrawler,
            &StubFamilyRunner,
            &StubRenderer,
        );

        assert_eq!(
            output,
            CliOutput {
                stdout: String::new(),
                stderr: "crawl failed\n".to_owned(),
                exit_code: 1,
            }
        );
    }

    #[test]
    fn cli_parses_family_and_inventory_flags() {
        let cli = Cli::try_parse_from([
            "guardrail3-rs",
            "validate",
            "--path",
            ".",
            "--family",
            "fmt",
            "--inventory",
        ])
        .expect("cli parse should succeed");

        match cli.command {
            Command::Validate {
                path,
                family,
                inventory,
            } => {
                assert_eq!(path, std::path::PathBuf::from("."));
                assert_eq!(family, vec![FamilyArg::Fmt]);
                assert!(inventory);
            }
        }
    }
}

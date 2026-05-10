use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use guardrail3_rs_app_types::SupportedFamily;

/// Top-level CLI parser for the guardrail3-rs binary.
#[derive(Parser, Debug)]
#[command(name = "guardrail3-rs")]
pub struct Cli {
    /// Parsed subcommand payload.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported CLI subcommands.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Validates one workspace root against the selected families.
    Validate {
        /// Workspace root to validate.
        #[arg(long = "path")]
        path: PathBuf,
        /// Optional family filter.
        #[arg(long = "family")]
        family: Vec<FamilyArg>,
        /// Includes inventory findings in the rendered output.
        #[arg(long = "inventory", default_value_t = false)]
        inventory: bool,
        /// When set, filters cargo gates by staged files. Skips cargo gates if no Rust-relevant staged paths inside `--path`.
        #[arg(long = "staged", default_value_t = false)]
        staged: bool,
        /// When set, runs only static rule families and skips cargo gates entirely.
        #[arg(long = "rules-only", default_value_t = false)]
        rules_only: bool,
    },
    /// Validates the repository as a whole: hook contents, tool presence, repo-wide topology, marker-pair completeness.
    ValidateRepo {
        /// Optional override for the repo root. Defaults to `git rev-parse --show-toplevel`.
        #[arg(long = "repo-root")]
        repo_root: Option<PathBuf>,
        /// Includes inventory findings in the rendered output.
        #[arg(long = "inventory", default_value_t = false)]
        inventory: bool,
    },
}

/// CLI-visible family names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FamilyArg {
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

/// Parses CLI arguments into the validated command payload.
///
/// # Errors
///
/// Returns [`clap::Error`] when the input arguments do not parse as a valid command.
pub fn parse_command_from<I, T>(args: I) -> Result<Command, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    Ok(Cli::try_parse_from(args)?.command)
}

#[cfg(test)]
#[path = "cli_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod cli_tests;

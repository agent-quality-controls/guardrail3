use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use guardrail3_rs_app_types::SupportedFamily;

/// Top-level CLI parser for the guardrail3-rs binary.
#[derive(Parser, Debug)]
#[command(
    name = "g3rs",
    version,
    after_help = "G3RS enforces Rust repo setup and Rust workspace guardrails.

Ecosystem requirement:
  G3RS requires Cargo-managed Rust workspaces or package roots.

Adoption order:
  1. From the Git repo root, run: g3rs init repo
  2. Choose every top-level Cargo.toml root that G3RS should manage.
  3. For each chosen root, run: g3rs init workspace --path <path>
  4. For each chosen root, run: g3rs validate workspace --path <path>
  5. After workspace adoption, run: g3rs validate repo

Workspace path choices:
  Root-only package: use .
  App with I/O: use apps/<name> for CLIs, APIs, servers, UI apps, and other executable surfaces.
  Library without I/O: use packages/<name> for reusable packages consumed by other software.

Concepts:
  repo       Git repository surface: hooks, repo-level topology, marker pairs.
  workspace  One adopted Rust unit: Cargo.toml plus guardrail3-rs.toml.

Rules:
  init writes setup.
  validate only reports.
  validate repo checks that Git will run G3RS.
  validate workspace checks one Rust unit.
  Each top-level Cargo.toml should be managed by G3RS unless it is intentionally outside the Rust guardrail surface."
)]
pub struct Cli {
    /// Parsed subcommand payload.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported CLI subcommands.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Writes repo or workspace setup.
    Init {
        #[command(subcommand)]
        command: InitCommand,
    },
    /// Reports repo or workspace validation findings.
    Validate {
        #[command(subcommand)]
        command: ValidateCommand,
    },
}

/// Supported init subcommands.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum InitCommand {
    /// Bootstraps repo-level hook guardrails.
    Repo {
        /// Path inside the repo to initialize. Defaults to the current directory.
        #[arg(long = "path", default_value = ".")]
        path: PathBuf,
        /// Allows bounded managed-file rewrites and managed-block insertion.
        #[arg(long = "force", default_value_t = false)]
        force: bool,
    },
    /// Bootstraps one adopted Rust workspace or package root.
    Workspace {
        /// Workspace root to initialize.
        #[arg(long = "path")]
        path: PathBuf,
        /// Allows bounded managed-file rewrites.
        #[arg(long = "force", default_value_t = false)]
        force: bool,
    },
}

/// Supported validate subcommands.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum ValidateCommand {
    /// Validates repository-level invariants (hooks, tools, topology, marker pairs).
    Repo {
        /// Optional repo root override; defaults to git rev-parse --show-toplevel.
        #[arg(long = "path", default_value = ".")]
        path: PathBuf,
        /// Includes inventory findings in the rendered output.
        #[arg(long = "inventory", default_value_t = false)]
        inventory: bool,
    },
    /// Validates one workspace root against the selected families.
    Workspace {
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

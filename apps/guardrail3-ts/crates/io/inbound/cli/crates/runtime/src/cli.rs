use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use guardrail3_ts_app_types::SupportedFamily;

/// Top-level CLI parser for the g3ts binary.
#[derive(Parser, Debug)]
#[command(
    name = "g3ts",
    version,
    after_help = "G3TS enforces TypeScript repo setup and TypeScript workspace guardrails.

Package manager:
  G3TS requires pnpm-managed TypeScript workspaces.

Adoption order:
  1. From the Git repo root, run: g3ts init repo
  2. Choose every top-level package.json root that G3TS should manage.
  3. For each chosen root, run: g3ts init workspace --path <path>
  4. For each chosen root, run: g3ts validate workspace --path <path>
  5. After workspace adoption, run: g3ts validate repo

Workspace path choices:
  Root-only package: use .
  App with I/O: use apps/<name> for CLIs, APIs, servers, UI apps, and other executable surfaces.
  Library without I/O: use packages/<name> for reusable packages consumed by other software.

Concepts:
  repo       Git repository surface: hooks, repo-level topology, marker pairs.
  workspace  One adopted TypeScript unit: package.json plus guardrail3-ts.toml.

Rules:
  init writes setup.
  validate only reports.
  validate repo checks that Git will run G3TS.
  validate workspace checks one TypeScript unit.
  Each top-level package.json should be managed by G3TS unless it is intentionally outside the TypeScript guardrail surface."
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
        /// Selected init scope.
        #[command(subcommand)]
        command: InitCommand,
    },
    /// Reports repo or workspace validation findings.
    Validate {
        /// Selected validation scope.
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
    /// Bootstraps one adopted TypeScript workspace or package root.
    Workspace {
        /// Workspace root to initialize.
        #[arg(long = "path")]
        path: PathBuf,
        /// Allows bounded generated rewrites.
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
        #[arg(long = "path")]
        path: Option<PathBuf>,
        /// Includes inventory findings in the rendered output.
        #[arg(long = "inventory", default_value_t = false)]
        inventory: bool,
    },
    /// Validates one TypeScript workspace root against the selected families.
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
        /// Filters toolchain gates by staged files.
        #[arg(long = "staged", default_value_t = false)]
        staged: bool,
        /// Skips toolchain gates and runs only the static rule families.
        #[arg(long = "rules-only", default_value_t = false)]
        rules_only: bool,
    },
}

/// CLI-visible family names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FamilyArg {
    Eslint,
    Astro,
    AstroSetup,
    AstroContent,
    AstroMdx,
    AstroI18n,
    AstroMedia,
    AstroSeo,
    AstroState,
    Arch,
    Apparch,
    Tsconfig,
    Package,
    Npmrc,
    Jscpd,
    Style,
    Fmt,
    Spelling,
    Typecov,
    Hooks,
    Topology,
}

impl FamilyArg {
    #[must_use]
    pub fn expand(self) -> Vec<SupportedFamily> {
        match self {
            Self::Eslint => vec![SupportedFamily::Eslint],
            Self::Astro => vec![
                SupportedFamily::AstroSetup,
                SupportedFamily::AstroContent,
                SupportedFamily::AstroMdx,
                SupportedFamily::AstroI18n,
                SupportedFamily::AstroMedia,
                SupportedFamily::AstroSeo,
                SupportedFamily::AstroState,
            ],
            Self::AstroSetup => vec![SupportedFamily::AstroSetup],
            Self::AstroContent => vec![SupportedFamily::AstroContent],
            Self::AstroMdx => vec![SupportedFamily::AstroMdx],
            Self::AstroI18n => vec![SupportedFamily::AstroI18n],
            Self::AstroMedia => vec![SupportedFamily::AstroMedia],
            Self::AstroSeo => vec![SupportedFamily::AstroSeo],
            Self::AstroState => vec![SupportedFamily::AstroState],
            Self::Arch => vec![SupportedFamily::Arch],
            Self::Apparch => vec![SupportedFamily::Apparch],
            Self::Tsconfig => vec![SupportedFamily::Tsconfig],
            Self::Package => vec![SupportedFamily::Package],
            Self::Npmrc => vec![SupportedFamily::Npmrc],
            Self::Jscpd => vec![SupportedFamily::Jscpd],
            Self::Style => vec![SupportedFamily::Style],
            Self::Fmt => vec![SupportedFamily::Fmt],
            Self::Spelling => vec![SupportedFamily::Spelling],
            Self::Typecov => vec![SupportedFamily::Typecov],
            Self::Hooks => vec![SupportedFamily::Hooks],
            Self::Topology => vec![SupportedFamily::Topology],
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

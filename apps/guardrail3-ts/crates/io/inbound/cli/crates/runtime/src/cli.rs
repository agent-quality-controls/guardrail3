use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use guardrail3_ts_app_types::SupportedFamily;

/// Top-level CLI parser for the g3ts binary.
#[derive(Parser, Debug)]
#[command(name = "g3ts")]
pub struct Cli {
    /// Parsed subcommand payload.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported CLI subcommands.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Validates one TypeScript package root against the selected families.
    Validate {
        /// Package root to validate.
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
    /// Validates repository-level invariants (hooks, tools, topology, marker pairs).
    #[command(name = "validate-repo")]
    ValidateRepo {
        /// Optional repo root override; defaults to git rev-parse --show-toplevel.
        #[arg(long = "path")]
        path: Option<PathBuf>,
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

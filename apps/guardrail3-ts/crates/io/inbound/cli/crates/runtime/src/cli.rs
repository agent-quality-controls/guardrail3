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
    Hooks,
}

impl FamilyArg {
    #[must_use]
    pub fn expand(self) -> Vec<SupportedFamily> {
        match self {
            FamilyArg::Eslint => vec![SupportedFamily::Eslint],
            FamilyArg::Astro => vec![
                SupportedFamily::AstroSetup,
                SupportedFamily::AstroContent,
                SupportedFamily::AstroMdx,
                SupportedFamily::AstroI18n,
                SupportedFamily::AstroMedia,
                SupportedFamily::AstroSeo,
                SupportedFamily::AstroState,
            ],
            FamilyArg::AstroSetup => vec![SupportedFamily::AstroSetup],
            FamilyArg::AstroContent => vec![SupportedFamily::AstroContent],
            FamilyArg::AstroMdx => vec![SupportedFamily::AstroMdx],
            FamilyArg::AstroI18n => vec![SupportedFamily::AstroI18n],
            FamilyArg::AstroMedia => vec![SupportedFamily::AstroMedia],
            FamilyArg::AstroSeo => vec![SupportedFamily::AstroSeo],
            FamilyArg::AstroState => vec![SupportedFamily::AstroState],
            FamilyArg::Arch => vec![SupportedFamily::Arch],
            FamilyArg::Apparch => vec![SupportedFamily::Apparch],
            FamilyArg::Tsconfig => vec![SupportedFamily::Tsconfig],
            FamilyArg::Package => vec![SupportedFamily::Package],
            FamilyArg::Npmrc => vec![SupportedFamily::Npmrc],
            FamilyArg::Jscpd => vec![SupportedFamily::Jscpd],
            FamilyArg::Hooks => vec![SupportedFamily::Hooks],
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

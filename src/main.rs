// These crates are used by the lib, not directly by the binary.
// Suppress false positives from unused_crate_dependencies.
use colored as _;
use glob as _;
use proc_macro2 as _;
use serde as _;
use serde_json as _;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

#[cfg(test)]
use proptest as _;
#[cfg(test)]
use tempfile as _;

use clap::Parser;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::adapters::outbound::tool_runner::RealToolChecker;
use guardrail3::app::discover;
use guardrail3::app::{hooks, rs, ts};
use guardrail3::cli::{Cli, Commands, HooksCommands, RsCommands, TsCommands, ValidateArgs};
use guardrail3::domain::report::ValidateDomains;
use guardrail3::{commands, report};

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI entry point — stderr output and process::exit for error codes are intentional
#[allow(clippy::too_many_lines)] // reason: CLI dispatch for all subcommands
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => {
            commands::validate::run(&args);
        }
        Commands::Generate(args) => {
            commands::generate::run(&args);
        }
        Commands::Check(args) => {
            commands::check::run(&args.path);
        }
        Commands::Diff(args) => {
            commands::diff::run(&args.path);
        }
        Commands::ListModules => {
            commands::modules_cmd::list_modules();
        }
        Commands::ShowModule(args) => {
            commands::modules_cmd::show_module(&args.name);
        }
        Commands::Rs { command } => match command {
            RsCommands::Validate(args) => {
                let path = std::path::Path::new(&args.path);
                let Some(abs_path) = path.canonicalize().ok() else {
                    eprintln!("Error: cannot resolve path '{}'", args.path);
                    std::process::exit(1);
                };
                let fs = RealFileSystem;
                let tc = RealToolChecker;
                let domains = domains_from_args(&args);
                let project = discover::detect_project(&fs, &abs_path);
                let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &abs_path);
                let report = rs::validate::run(
                    &fs,
                    &abs_path,
                    &project,
                    scoped_files.as_deref(),
                    &domains,
                    args.thorough,
                    &tc,
                );
                match args.format.as_str() {
                    "json" => report::json::print_report(&report),
                    "md" | "markdown" => report::markdown::print_report(&report),
                    _ => report::text::print_report(&report),
                }
                if report.error_count() > 0 {
                    std::process::exit(1);
                }
            }
            RsCommands::Generate(args) => {
                commands::generate::run_rs(&args);
            }
            RsCommands::Init {
                profile,
                path,
                force,
            } => {
                commands::init::run_rs(&profile, &path, force);
            }
        },
        Commands::Ts { command } => match command {
            TsCommands::Validate(args) => {
                let path = std::path::Path::new(&args.path);
                let Some(abs_path) = path.canonicalize().ok() else {
                    eprintln!("Error: cannot resolve path '{}'", args.path);
                    std::process::exit(1);
                };
                let fs = RealFileSystem;
                let domains = domains_from_args(&args);
                let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &abs_path);
                let scoped_ref = scoped_files.as_deref();
                let report = ts::validate::run(&fs, &abs_path, scoped_ref, &domains);
                match args.format.as_str() {
                    "json" => report::json::print_report(&report),
                    "md" | "markdown" => report::markdown::print_report(&report),
                    _ => report::text::print_report(&report),
                }
                if report.error_count() > 0 {
                    std::process::exit(1);
                }
            }
            TsCommands::Generate(args) => {
                commands::generate::run_ts(&args);
            }
            TsCommands::Init { path, force } => {
                commands::init::run_ts(&path, force);
            }
        },
        Commands::Hooks { command } => match command {
            HooksCommands::Validate(args) => {
                let path = std::path::Path::new(&args.path);
                let Some(abs_path) = path.canonicalize().ok() else {
                    eprintln!("Error: cannot resolve path '{}'", args.path);
                    std::process::exit(1);
                };
                let fs = RealFileSystem;
                let tc = RealToolChecker;
                let domains = domains_from_args(&args);
                let project = discover::detect_project(&fs, &abs_path);
                let report = hooks::validate::run(
                    &fs,
                    &abs_path,
                    project.has_rust,
                    project.has_typescript,
                    &domains,
                    &tc,
                );
                match args.format.as_str() {
                    "json" => report::json::print_report(&report),
                    "md" | "markdown" => report::markdown::print_report(&report),
                    _ => report::text::print_report(&report),
                }
                if report.error_count() > 0 {
                    std::process::exit(1);
                }
            }
            HooksCommands::Install(args) => {
                commands::generate::run_hooks(&args);
            }
        },
    }
}

const fn domains_from_args(args: &ValidateArgs) -> ValidateDomains {
    let run_all = !args.code && !args.architecture && !args.release && !args.tests;
    ValidateDomains {
        code: run_all || args.code,
        architecture: run_all || args.architecture,
        release: run_all || args.release,
        tests: run_all || args.tests,
    }
}

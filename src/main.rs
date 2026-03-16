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

use clap::{CommandFactory, FromArgMatches};

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::adapters::outbound::tool_runner::RealToolChecker;
use guardrail3::app::discover;
use guardrail3::app::{hooks, rs, ts};
use guardrail3::cli::{Cli, Commands, RsCommands, TsCommands, ValidateArgs};
use guardrail3::domain::report::ValidateDomains;
use guardrail3::help_gen;
use guardrail3::{commands, report};

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI entry point — stderr output and process::exit for error codes are intentional
#[allow(clippy::too_many_lines)] // reason: CLI dispatch for all subcommands
fn main() {
    let cmd = help_gen::inject_help(Cli::command());
    let matches = match cmd.try_get_matches() {
        Ok(m) => m,
        Err(e) => e.exit(),
    };
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    match cli.command {
        Commands::Rs { command } => handle_rs(command),
        Commands::Ts { command } => handle_ts(command),
    }
}

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI dispatch
fn handle_rs(command: RsCommands) {
    match command {
        RsCommands::Init {
            profile,
            path,
            force,
        } => {
            commands::init::run_rs(&profile, &path, force);
        }
        RsCommands::Generate(args) => {
            commands::generate::run_rs(&args);
        }
        RsCommands::Validate(args) => {
            let (report, _) = run_rs_validate(&args);
            print_report(&args, &report);
        }
        RsCommands::Check(args) => {
            commands::check::run(&args.path);
        }
        RsCommands::Diff(args) => {
            commands::diff::run(&args.path);
        }
        RsCommands::HooksInstall(args) => {
            commands::generate::run_hooks(&args);
        }
        RsCommands::HooksValidate(args) => {
            let path = resolve_path(&args.path);
            let fs = RealFileSystem;
            let tc = RealToolChecker;
            let domains = domains_from_args(&args);
            let project = discover::detect_project(&fs, &path);
            let report = hooks::validate::run(
                &fs,
                &path,
                project.has_rust,
                project.has_typescript,
                &domains,
                &tc,
            );
            print_report(&args, &report);
        }
        RsCommands::ListModules => {
            commands::modules_cmd::list_modules();
        }
        RsCommands::ShowModule(args) => {
            commands::modules_cmd::show_module(&args.name);
        }
    }
}

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI dispatch
fn handle_ts(command: TsCommands) {
    match command {
        TsCommands::Init { path, force } => {
            commands::init::run_ts(&path, force);
        }
        TsCommands::Generate(args) => {
            commands::generate::run_ts(&args);
        }
        TsCommands::Validate(args) => {
            let path = resolve_path(&args.path);
            let fs = RealFileSystem;
            let domains = domains_from_args(&args);
            let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &path);
            let report = ts::validate::run(&fs, &path, scoped_files.as_deref(), &domains);
            print_report(&args, &report);
        }
        TsCommands::HooksInstall(args) => {
            commands::generate::run_hooks(&args);
        }
        TsCommands::HooksValidate(args) => {
            let path = resolve_path(&args.path);
            let fs = RealFileSystem;
            let tc = RealToolChecker;
            let domains = domains_from_args(&args);
            let project = discover::detect_project(&fs, &path);
            let report = hooks::validate::run(
                &fs,
                &path,
                project.has_rust,
                project.has_typescript,
                &domains,
                &tc,
            );
            print_report(&args, &report);
        }
    }
}

#[allow(clippy::disallowed_methods)] // reason: CLI — process::exit for error codes
fn run_rs_validate(
    args: &ValidateArgs,
) -> (guardrail3::domain::report::Report, std::path::PathBuf) {
    let path = resolve_path(&args.path);
    let fs = RealFileSystem;
    let tc = RealToolChecker;
    let domains = domains_from_args(args);
    let project = discover::detect_project(&fs, &path);
    let scoped_files = commands::validate::resolve_scoped_files_pub(args, &path);
    let report = rs::validate::run(
        &fs,
        &path,
        &project,
        scoped_files.as_deref(),
        &domains,
        args.thorough,
        &tc,
    );
    (report, path)
}

#[allow(clippy::disallowed_methods)] // reason: CLI — process::exit
fn print_report(args: &ValidateArgs, report: &guardrail3::domain::report::Report) {
    match args.format.as_str() {
        "json" => report::json::print_report(report),
        "md" | "markdown" => report::markdown::print_report(report),
        _ => report::text::print_report(report),
    }
    if report.error_count() > 0 {
        std::process::exit(1);
    }
}

#[allow(clippy::disallowed_methods, clippy::print_stderr)] // reason: CLI — process::exit + error output
fn resolve_path(path_str: &str) -> std::path::PathBuf {
    let path = std::path::Path::new(path_str);
    match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Error: cannot resolve path '{path_str}'");
            std::process::exit(1);
        }
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

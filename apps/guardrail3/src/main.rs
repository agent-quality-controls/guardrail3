// These crates are used by the lib, not directly by the binary.
// Suppress false positives from unused_crate_dependencies.
use colored as _;
use garde as _;
use glob as _;
use proc_macro2 as _;
use quote as _;
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
use garde::Validate;
use guardrail3::{
    adapters::outbound::{fs::RealFileSystem, tool_runner::RealToolChecker},
    app::{discover, hooks, rs, ts},
    cli::{Cli, Commands, RsCommands, TsCommands, ValidateArgs},
    commands,
    domain::{
        config::types::GuardrailConfig,
        report::{RustCheckCategories, TsCheckCategories, ValidateDomains},
    },
    help_gen,
    ports::outbound::FileSystem,
    report,
};

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI entry point — stderr output and process::exit for error codes are intentional
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
        Commands::DumpGuide => handle_guide(),
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI — writes file and prints path
fn handle_guide() {
    let path = std::path::Path::new("GUARDRAIL3_GUIDE.md");
    let content = guardrail3::domain::modules::guide::GUIDE_CONTENT;
    if let Err(e) = guardrail3::fs::write_file(path, content) {
        eprintln!("Error writing GUARDRAIL3_GUIDE.md: {e}");
        std::process::exit(1);
    }
    println!("Generated: {}", path.display());
    println!("Commit this file so agents and contributors can find it.");
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
            validate_or_exit(&args);
            commands::generate::run_rs(&args);
        }
        RsCommands::Validate(args) => {
            validate_or_exit(&args);
            let (report, _) = run_rs_validate(&args);
            print_report(&args, &report);
        }
        RsCommands::Check(args) => {
            validate_or_exit(&args);
            commands::check::run(&args.path);
        }
        RsCommands::Diff(args) => {
            validate_or_exit(&args);
            commands::diff::run(&args.path);
        }
        RsCommands::HooksInstall(args) => {
            validate_or_exit(&args);
            commands::generate::run_hooks(&args);
        }
        RsCommands::HooksValidate(args) => {
            validate_or_exit(&args);
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
            validate_or_exit(&args);
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
            validate_or_exit(&args);
            commands::generate::run_ts(&args);
        }
        TsCommands::Validate(args) => {
            validate_or_exit(&args);
            let path = resolve_path(&args.path);
            let fs = RealFileSystem;
            let categories = build_ts_categories(&args, &fs, &path);
            let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &path);
            let report = ts::validate::run(&fs, &path, scoped_files.as_deref(), &categories);
            print_report(&args, &report);
        }
        TsCommands::HooksInstall(args) => {
            validate_or_exit(&args);
            commands::generate::run_hooks(&args);
        }
        TsCommands::HooksValidate(args) => {
            validate_or_exit(&args);
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

fn run_rs_validate(
    args: &ValidateArgs,
) -> (guardrail3::domain::report::Report, std::path::PathBuf) {
    let path = resolve_path(&args.path);
    let fs = RealFileSystem;
    let tc = RealToolChecker;
    let categories = build_rs_categories(args, &fs, &path);
    let project = discover::detect_project(&fs, &path);
    let scoped_files = commands::validate::resolve_scoped_files_pub(args, &path);
    let report = rs::validate::run(
        &fs,
        &path,
        &project,
        scoped_files.as_deref(),
        &categories,
        args.thorough,
        &tc,
    );
    (report, path)
}

#[allow(clippy::disallowed_methods)] // reason: CLI — process::exit
fn print_report(args: &ValidateArgs, report: &guardrail3::domain::report::Report) {
    match args.format.as_str() {
        "json" => report::json::print_report(report, args.inventory),
        "md" | "markdown" => report::markdown::print_report(report, args.inventory, args.verbose),
        _ => report::text::print_report(report, args.inventory, args.verbose),
    }
    if report.error_count() > 0 {
        std::process::exit(1);
    }
}

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI — validation error output + exit
fn validate_or_exit<T: Validate<Context = ()>>(args: &T) {
    if let Err(e) = args.validate() {
        eprintln!("Validation error: {e}");
        std::process::exit(2);
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
    let run_all = !args.code && !args.architecture && !args.release && !args.tests && !args.garde;
    ValidateDomains {
        code: run_all || args.code,
        architecture: run_all || args.architecture,
        release: run_all || args.release,
        tests: run_all || args.tests,
    }
}

/// Load guardrail3.toml config, if present.
#[allow(clippy::disallowed_methods)] // reason: guardrail3 config parsing
fn load_config(fs: &RealFileSystem, path: &std::path::Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = fs.read_file(&config_path)?;
    toml::from_str(&content).ok()
}

/// Build `RustCheckCategories` by merging config defaults with CLI flags.
fn build_rs_categories(
    args: &ValidateArgs,
    fs: &RealFileSystem,
    path: &std::path::Path,
) -> RustCheckCategories {
    let cfg = load_config(fs, path);
    let checks = cfg
        .as_ref()
        .and_then(|c| c.rust.as_ref())
        .and_then(|r| r.checks.as_ref());

    let rs_defaults = RustCheckCategories::default();
    let cfg_arch = checks
        .and_then(|c| c.architecture)
        .unwrap_or(rs_defaults.architecture);
    let cfg_garde = checks.and_then(|c| c.garde).unwrap_or(rs_defaults.garde);
    let cfg_tests = checks.and_then(|c| c.tests).unwrap_or(rs_defaults.tests);
    let cfg_release = checks
        .and_then(|c| c.release)
        .unwrap_or(rs_defaults.release);

    // If any CLI domain flag is set, it acts as a filter (only run those)
    let any_cli = args.code || args.architecture || args.tests || args.release || args.garde;
    if any_cli {
        RustCheckCategories {
            architecture: args.architecture,
            garde: args.garde,
            tests: args.tests,
            release: args.release,
        }
    } else {
        RustCheckCategories {
            architecture: cfg_arch,
            garde: cfg_garde,
            tests: cfg_tests,
            release: cfg_release,
        }
    }
}

/// Build `TsCheckCategories` by merging config defaults with CLI flags.
fn build_ts_categories(
    args: &ValidateArgs,
    fs: &RealFileSystem,
    path: &std::path::Path,
) -> TsCheckCategories {
    let cfg = load_config(fs, path);
    let checks = cfg
        .as_ref()
        .and_then(|c| c.typescript.as_ref())
        .and_then(|t| t.checks.as_ref());

    let ts_defaults = TsCheckCategories::default();
    let cfg_arch = checks
        .and_then(|c| c.architecture)
        .unwrap_or(ts_defaults.architecture);
    let cfg_tests = checks.and_then(|c| c.tests).unwrap_or(ts_defaults.tests);

    let any_cli = args.code || args.architecture || args.tests || args.release || args.garde;
    if any_cli {
        TsCheckCategories {
            architecture: args.architecture,
            tests: args.tests,
        }
    } else {
        TsCheckCategories {
            architecture: cfg_arch,
            tests: cfg_tests,
        }
    }
}

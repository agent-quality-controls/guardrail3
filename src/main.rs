mod cli;
mod commands;
mod config;
mod discover;
mod fs;
mod hooks;
mod modules;
mod report;
mod rs;
mod ts;

use clap::Parser;

use cli::{Cli, Commands, HooksCommands, RsCommands, TsCommands};

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI entry point — stderr output and process::exit for error codes are intentional
#[allow(clippy::too_many_lines)] // reason: CLI dispatch for all subcommands
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => {
            commands::validate::run(&args);
        }
        Commands::Init(args) => {
            commands::init::run(&args);
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
                let project = discover::detect_project(&abs_path);
                let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &abs_path);
                let report = rs::validate::run(&abs_path, &project, scoped_files.as_deref());
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
        },
        Commands::Ts { command } => match command {
            TsCommands::Validate(args) => {
                let path = std::path::Path::new(&args.path);
                let Some(abs_path) = path.canonicalize().ok() else {
                    eprintln!("Error: cannot resolve path '{}'", args.path);
                    std::process::exit(1);
                };
                let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &abs_path);
                let scoped_ref = scoped_files.as_deref();
                let report = ts::validate::run(&abs_path, scoped_ref);
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
        },
        Commands::Hooks { command } => match command {
            HooksCommands::Validate(args) => {
                let path = std::path::Path::new(&args.path);
                let Some(abs_path) = path.canonicalize().ok() else {
                    eprintln!("Error: cannot resolve path '{}'", args.path);
                    std::process::exit(1);
                };
                let project = discover::detect_project(&abs_path);
                let report =
                    hooks::validate::run(&abs_path, project.has_rust, project.has_typescript);
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

use clap::{CommandFactory, FromArgMatches};
use garde::Validate;
use guardrail3_adapters_inbound_cli::{
    self as commands,
    cli::{Cli, Commands, RsCommands, RsValidateArgs, TsCommands, TsValidateArgs},
    help_gen,
};
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_adapters_outbound_report as report;
use guardrail3_adapters_outbound_tool_runner::RealToolChecker;
use guardrail3_app_commands::messages::GUIDE_CONTENT;
use guardrail3_app_core::{crawl, discover, project_walker};
use guardrail3_app_hooks as hooks;
use guardrail3_app_rs_runtime as rs;
use guardrail3_app_ts as ts;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_report::{Report, TsCheckCategories, ValidateDomains};
use guardrail3_outbound_traits::FileSystem;

struct CoverageSelection {
    clippy: bool,
    deny: bool,
    rustfmt: bool,
    eslint: bool,
    stylelint: bool,
    prettier: bool,
    cspell: bool,
    jscpd: bool,
    tsconfig: bool,
    rust_toolchain: bool,
    npmrc: bool,
}

impl CoverageSelection {
    const fn any(&self) -> bool {
        self.clippy
            || self.deny
            || self.rustfmt
            || self.eslint
            || self.stylelint
            || self.prettier
            || self.cspell
            || self.jscpd
            || self.tsconfig
            || self.rust_toolchain
            || self.npmrc
    }
}

#[allow(clippy::print_stderr)] // reason: CLI entry point — stderr for error output
#[allow(clippy::disallowed_methods)] // reason: CLI entry point — process::exit for error codes
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
        Commands::DumpTree { path } => handle_dump_tree(&path),
        Commands::Map {
            path,
            clippy,
            deny,
            rustfmt,
            eslint,
            stylelint,
            prettier,
            cspell,
            jscpd,
            tsconfig,
            rust_toolchain,
            npmrc,
        } => {
            let selection = CoverageSelection {
                clippy,
                deny,
                rustfmt,
                eslint,
                stylelint,
                prettier,
                cspell,
                jscpd,
                tsconfig,
                rust_toolchain,
                npmrc,
            };
            handle_map_command(&path, &selection);
        }
    }
}

fn handle_map_command(path: &str, selection: &CoverageSelection) {
    if selection.any() {
        let project_path = std::path::Path::new(path);
        let crawl_result = crawl::crawl(project_path);
        run_coverage_maps(project_path, &crawl_result, selection);
    } else {
        commands::map::run(path);
    }
}

fn run_coverage_maps(
    project_path: &std::path::Path,
    crawl_result: &crawl::CrawlResult,
    selection: &CoverageSelection,
) {
    use commands::coverage;
    if selection.clippy {
        coverage::clippy::print(project_path, crawl_result);
    }
    if selection.deny {
        coverage::deny::print(project_path, crawl_result);
    }
    if selection.rustfmt {
        coverage::rustfmt::print(project_path, crawl_result);
    }
    if selection.eslint {
        coverage::eslint::print(project_path, crawl_result);
    }
    if selection.stylelint {
        coverage::stylelint::print(project_path, crawl_result);
    }
    if selection.prettier {
        coverage::prettier::print(project_path, crawl_result);
    }
    if selection.cspell {
        coverage::cspell::print(project_path, crawl_result);
    }
    if selection.jscpd {
        coverage::jscpd::print(project_path, crawl_result);
    }
    if selection.tsconfig {
        coverage::tsconfig::print(project_path, crawl_result);
    }
    if selection.rust_toolchain {
        coverage::rust_toolchain::print(project_path, crawl_result);
    }
    if selection.npmrc {
        coverage::npmrc::print(project_path, crawl_result);
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI — dumps JSON to stdout
fn handle_dump_tree(path_str: &str) {
    let path = std::path::Path::new(path_str);
    let resolved = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: cannot resolve path '{path_str}': {e}");
            std::process::exit(1);
        }
    };
    let fs = RealFileSystem;
    let tree = project_walker::walk_project(&fs, &resolved);
    match serde_json::to_string_pretty(&tree) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("Error serializing tree: {e}");
            std::process::exit(1);
        }
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI — writes file and prints path
fn handle_guide() {
    let path = std::path::Path::new("GUARDRAIL3_GUIDE.md");
    if let Err(e) = guardrail3_shared_fs::write_file(path, GUIDE_CONTENT) {
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
            dry_run,
        } => {
            commands::init::run_rs(&profile, &path, force, dry_run);
        }
        RsCommands::Generate(args) => {
            validate_or_exit(&args);
            if args.dry_run {
                commands::diff::run(&args.path, args.dump_dir.as_deref());
            } else {
                commands::generate::run_rs(&args);
            }
        }
        RsCommands::Validate(args) => {
            validate_or_exit(&args);
            let report = run_rs_validate(&args);
            print_report(&args.format, args.inventory, args.verbose, &report);
        }
        RsCommands::Check(args) => {
            validate_or_exit(&args);
            commands::check::run(&args.path);
        }
        RsCommands::HooksInstall(args) => {
            validate_or_exit(&args);
            commands::generate::run_rs_hooks(&args);
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
        TsCommands::Init {
            path,
            force,
            dry_run,
        } => {
            commands::init::run_ts(&path, force, dry_run);
        }
        TsCommands::Generate(args) => {
            validate_or_exit(&args);
            if args.dry_run {
                commands::diff::run_ts(&args.path, args.dump_dir.as_deref());
            } else {
                commands::generate::run_ts(&args);
            }
        }
        TsCommands::Validate(args) => {
            validate_or_exit(&args);
            let path = resolve_path(&args.path);
            let fs = RealFileSystem;
            let cfg = load_config(&fs, &path);
            let categories = build_ts_categories(&args, &fs, &path);
            let crawl = crawl::crawl(&path);
            let scoped_files = commands::validate::resolve_scoped_files_pub(&args, &path);
            let report = ts::validate::run(
                &fs,
                &path,
                scoped_files.as_deref(),
                &categories,
                cfg.as_ref(),
                &crawl,
            );
            print_report(&args.format, args.inventory, args.verbose, &report);
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
            let crawl = crawl::crawl(&path);
            let report = hooks::validate::run(
                &fs,
                &path,
                false,
                project.has_typescript,
                &domains,
                &tc,
                &crawl,
            );
            print_report(&args.format, args.inventory, args.verbose, &report);
        }
    }
}

fn run_rs_validate(args: &RsValidateArgs) -> Report {
    let path = resolve_path(&args.path);
    let fs = RealFileSystem;
    let tc = RealToolChecker;
    let families: Vec<_> = args.families.iter().copied().map(Into::into).collect();
    let scoped_files = commands::validate::resolve_scoped_files_pub(args, &path);
    let normalized_scope =
        commands::validate::normalize_scoped_files(&path, scoped_files.as_deref());
    match rs::run(
        &fs,
        &path,
        normalized_scope.as_ref(),
        &families,
        args.thorough,
        &tc,
    ) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

#[allow(clippy::disallowed_methods)] // reason: CLI — process::exit
fn print_report(format: &str, inventory: bool, verbose: bool, report: &Report) {
    match format {
        "json" => report::json::print_report(report, inventory),
        "md" | "markdown" => report::markdown::print_report(report, inventory, verbose),
        _ => report::text::print_report(report, inventory, verbose),
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

const fn domains_from_args(_args: &TsValidateArgs) -> ValidateDomains {
    ValidateDomains {
        code: true,
        architecture: true,
        release: true,
        tests: true,
    }
}

/// Load guardrail3.toml config, if present.
fn load_config(fs: &RealFileSystem, path: &std::path::Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = fs.read_file(&config_path)?;
    toml::from_str(&content).ok()
}

/// Build `TsCheckCategories` by merging config defaults with CLI flags.
fn build_ts_categories(
    _args: &TsValidateArgs,
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
    let cfg_content = checks
        .and_then(|c| c.content)
        .unwrap_or(ts_defaults.content);
    let cfg_tests = checks.and_then(|c| c.tests).unwrap_or(ts_defaults.tests);

    TsCheckCategories {
        architecture: cfg_arch,
        content: cfg_content,
        tests: cfg_tests,
    }
}

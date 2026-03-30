use std::path::Path;

use crate::cli::GenerateArgs;
#[cfg(feature = "product-rs-generate")]
use guardrail3_app_commands::command_ids::{RS_INIT, RS_SHOW_MODULE};
use guardrail3_domain_config as config;
#[cfg(feature = "product-ts")]
use guardrail3_domain_modules::{canonical, cspell, eslint, stylelint};

#[cfg(feature = "product-rs-generate")]
use guardrail3_app_rs_generate as rs_generate;

/// A (`relative_path`, `content`) pair for a generated file.
type GeneratedPair = (String, String);

struct GeneratedFile {
    path: String,
    content: String,
}

impl GeneratedFile {
    #[must_use]
    fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    fn content(&self) -> &str {
        &self.content
    }
}

/// Load guardrail3.toml configuration from a project path.
fn load_config(path: &Path) -> Result<Option<config::types::GuardrailConfig>, String> {
    let config_path = path.join("guardrail3.toml");
    let Some(content) = guardrail3_shared_fs::read_file(&config_path) else {
        return Ok(None);
    };
    toml::from_str(&content)
        .map(Some)
        .map_err(|error| format!("Error parsing guardrail3.toml: {error}"))
}

#[cfg(feature = "product-rs-generate")]
/// Main generate command -- generates all config files from guardrail3.toml.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and process::exit for error codes
pub fn run(args: &GenerateArgs) {
    let project_path = Path::new(args.path());
    let cfg = match load_config(project_path) {
        Ok(Some(cfg)) => cfg,
        Ok(None) => {
            eprintln!(
                "Error: guardrail3.toml not found or invalid at {}",
                project_path.display()
            );
            eprintln!("Run '{RS_INIT}' to create one.");
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{error}");
            eprintln!("Run '{RS_INIT}' to create one.");
            std::process::exit(1);
        }
    };

    let profile = cfg.profile().map_or("service", |p| p.name()).to_owned();

    let files = generate_all_files(project_path, &cfg, &profile);

    let mut written = 0usize;
    for gf in &files {
        let target = project_path.join(gf.path());
        if let Some(parent) = target.parent() {
            if let Err(e) = guardrail3_shared_fs::create_dir_all(parent) {
                eprintln!("Error creating directory {}: {e}", parent.display());
                continue;
            }
        }
        warn_if_overwriting(&target, gf.path(), gf.content());
        if let Err(e) = guardrail3_shared_fs::write_file(&target, gf.content()) {
            eprintln!("Error writing {}: {e}", gf.path());
            continue;
        }
        println!("  wrote: {}", gf.path());
        written = written.saturating_add(1);
    }

    println!();
    println!("Generated {written} files (profile: {profile}).");

    // Print cargo-lints reference
    if cfg.rust().is_some() {
        println!();
        println!("NOTE: Add these workspace lints to your Cargo.toml manually");
        println!("(guardrail3 does not modify Cargo.toml):");
        println!("  {RS_SHOW_MODULE} canonical/cargo-lints");
    }
}

#[cfg(feature = "product-rs-generate")]
/// Generate only Rust config files.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_rs(args: &GenerateArgs) {
    let project_path = Path::new(args.path());
    let cfg = match load_config(project_path) {
        Ok(Some(cfg)) => cfg,
        Ok(None) => {
            eprintln!(
                "Error: guardrail3.toml not found or invalid at {}",
                project_path.display()
            );
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    let files = rs_generate::generate_rust_owned_artifacts(project_path, &cfg);

    for gf in &files {
        let target = project_path.join(gf.path());
        if let Some(parent) = target.parent() {
            let _ = guardrail3_shared_fs::create_dir_all(parent);
        }
        warn_if_overwriting(&target, gf.path(), gf.content());
        if let Err(e) = guardrail3_shared_fs::write_file(&target, gf.content()) {
            eprintln!("Error writing {}: {e}", gf.path());
            continue;
        }
        println!("  wrote: {}", gf.path());
    }
}

#[cfg(feature = "product-ts")]
/// Generate only TypeScript config files.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_ts(args: &GenerateArgs) {
    let project_path = Path::new(args.path());
    let cfg = match load_config(project_path) {
        Ok(Some(cfg)) => cfg,
        Ok(None) => {
            eprintln!(
                "Error: guardrail3.toml not found at {}",
                project_path.display()
            );
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    let files = generate_ts_files(&cfg);

    if files.is_empty() {
        println!(
            "No TypeScript files to generate (check [typescript.canonical] in guardrail3.toml)."
        );
        return;
    }

    let mut written = 0usize;
    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            let _ = guardrail3_shared_fs::create_dir_all(parent);
        }
        warn_if_overwriting(&target, &gf.path, &gf.content);
        if let Err(e) = guardrail3_shared_fs::write_file(&target, &gf.content) {
            eprintln!("Error writing {}: {e}", gf.path);
            continue;
        }
        println!("  wrote: {}", gf.path);
        written = written.saturating_add(1);
    }

    println!();
    println!("Generated {written} TypeScript config files.");

    // Also generate pre-commit hook
    let has_rust = cfg.rust().is_some();
    let hook_content = build_hook_content(Some(&cfg), has_rust, true);
    generate_and_install_hooks(project_path, &hook_content);
}

#[cfg(feature = "product-ts")]
/// Generate and install a pre-commit hook.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI output
fn generate_and_install_hooks(project_path: &Path, hook_content: &str) {
    let hooks_dir = project_path.join(".githooks");
    let _ = guardrail3_shared_fs::create_dir_all(&hooks_dir);
    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = guardrail3_shared_fs::write_file(&hook_path, hook_content) {
        eprintln!("Error writing pre-commit hook: {e}");
        return;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(meta) = guardrail3_shared_fs::metadata(&hook_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = guardrail3_shared_fs::set_permissions(&hook_path, perms);
        }
    }
    println!("  wrote: .githooks/pre-commit");

    // Configure git to use .githooks/
    let _ = std::process::Command::new("git")
        .args(["config", "core.hooksPath", ".githooks"])
        .current_dir(project_path)
        .output();
}

#[cfg(feature = "product-rs-generate")]
/// Install pre-commit hooks (standalone command).
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_rs_hooks(args: &GenerateArgs) {
    let project_path = Path::new(args.path());
    let cfg = match load_config(project_path) {
        Ok(cfg) => cfg,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let hook = rs_generate::generate_rust_hook_artifact(cfg.as_ref());

    let hooks_dir = project_path.join(".githooks");
    if let Err(e) = guardrail3_shared_fs::create_dir_all(&hooks_dir) {
        eprintln!("Error creating .githooks/ directory: {e}");
        std::process::exit(1);
    }

    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = guardrail3_shared_fs::write_file(&hook_path, hook.content()) {
        eprintln!("Error writing pre-commit hook: {e}");
        std::process::exit(1);
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(meta) = guardrail3_shared_fs::metadata(&hook_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            if let Err(e) = guardrail3_shared_fs::set_permissions(&hook_path, perms) {
                eprintln!("Warning: could not set executable permission: {e}");
            }
        }
    }

    println!("  wrote: .githooks/pre-commit");
    println!();
    println!("Configure git to use hooks: git config core.hooksPath .githooks");
}

/// Install mixed-stack pre-commit hooks for the current project config.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_hooks(args: &GenerateArgs) {
    let project_path = Path::new(args.path());
    let cfg = match load_config(project_path) {
        Ok(cfg) => cfg,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let has_rust = cfg
        .as_ref()
        .and_then(config::types::GuardrailConfig::rust)
        .is_some();
    let has_typescript = cfg
        .as_ref()
        .and_then(config::types::GuardrailConfig::typescript)
        .is_some();
    let hook_content = build_hook_content(cfg.as_ref(), has_rust, has_typescript);

    let hooks_dir = project_path.join(".githooks");
    if let Err(e) = guardrail3_shared_fs::create_dir_all(&hooks_dir) {
        eprintln!("Error creating .githooks/ directory: {e}");
        std::process::exit(1);
    }

    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = guardrail3_shared_fs::write_file(&hook_path, &hook_content) {
        eprintln!("Error writing pre-commit hook: {e}");
        std::process::exit(1);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(meta) = guardrail3_shared_fs::metadata(&hook_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            if let Err(e) = guardrail3_shared_fs::set_permissions(&hook_path, perms) {
                eprintln!("Warning: could not set executable permission: {e}");
            }
        }
    }

    println!("  wrote: .githooks/pre-commit");
    println!();
    println!("Configure git to use hooks: git config core.hooksPath .githooks");
}

/// Warn if an existing file has content that differs from what we'd generate.
#[allow(clippy::print_stderr)] // reason: CLI tool — overwrite warnings reported to stderr
fn warn_if_overwriting(target: &Path, relative_path: &str, new_content: &str) {
    if let Some(existing) = guardrail3_shared_fs::read_file(target) {
        if existing != new_content {
            eprintln!(
                "  warning: Overwriting {relative_path} — manual edits will be lost. Use .guardrail3/overrides/ for project-specific customization."
            );
        }
    }
}

fn generate_all_files(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
    _profile: &str,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    #[cfg(feature = "product-rs-generate")]
    if cfg.rust().is_some() {
        files.extend(
            rs_generate::generate_rust_owned_artifacts(project_path, cfg)
                .into_iter()
                .map(|file| GeneratedFile {
                    path: file.path().to_owned(),
                    content: file.content().to_owned(),
                }),
        );
    }

    #[cfg(feature = "product-ts")]
    if cfg.typescript().is_some() {
        let ts_files = generate_ts_files(cfg);
        files.extend(ts_files);
    }

    files
}

#[cfg(feature = "product-ts")]
fn generate_ts_files(cfg: &config::types::GuardrailConfig) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let Some(ts_cfg) = cfg.typescript() else {
        return files;
    };

    let canonical_cfg = ts_cfg.canonical();

    // Determine app types from config
    let (has_content_app, has_service_app) = detect_ts_app_types(ts_cfg);

    // .npmrc -- generate unless explicitly disabled
    let gen_npmrc = canonical_cfg
        .and_then(config::types::CanonicalConfig::npmrc)
        .unwrap_or(true);
    if gen_npmrc {
        files.push(GeneratedFile {
            path: ".npmrc".to_owned(),
            content: canonical::NPMRC.content().to_owned(),
        });
    }

    // tsconfig.base.json -- generate unless explicitly disabled
    let gen_tsconfig = canonical_cfg
        .and_then(config::types::CanonicalConfig::tsconfig_base)
        .unwrap_or(true);
    if gen_tsconfig {
        files.push(GeneratedFile {
            path: "tsconfig.base.json".to_owned(),
            content: canonical::TSCONFIG_BASE.content().to_owned(),
        });
    }

    // .jscpd.json -- generate unless explicitly disabled
    let gen_jscpd = canonical_cfg
        .and_then(config::types::CanonicalConfig::jscpd)
        .unwrap_or(true);
    if gen_jscpd {
        files.push(GeneratedFile {
            path: ".jscpd.json".to_owned(),
            content: canonical::JSCPD.content().to_owned(),
        });
    }

    // eslint.config.mjs -- always generated with full plugin config
    files.push(GeneratedFile {
        path: "eslint.config.mjs".to_owned(),
        content: eslint::build_eslint_config(has_content_app, has_service_app),
    });

    // cspell.json -- always generated
    files.push(GeneratedFile {
        path: "cspell.json".to_owned(),
        content: cspell::build_cspell_config(),
    });

    // .stylelintrc.mjs -- only if content app exists
    if has_content_app {
        files.push(GeneratedFile {
            path: ".stylelintrc.mjs".to_owned(),
            content: stylelint::build_stylelint_config(),
        });
    }

    files
}

#[cfg(feature = "product-ts")]
/// Generate expected TS file contents without writing -- used by ts diff.
pub fn generate_expected_ts(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path).ok()??;

    let files = generate_ts_files(&cfg);
    if files.is_empty() {
        return Some(Vec::new());
    }

    let mut pairs: Vec<GeneratedPair> = files.into_iter().map(|gf| (gf.path, gf.content)).collect();

    // Include pre-commit hook (same as run_ts)
    let has_rust = cfg.rust().is_some();
    let hook_content = build_hook_content(Some(&cfg), has_rust, true);
    pairs.push((".githooks/pre-commit".to_owned(), hook_content));

    Some(pairs)
}

/// Generate expected file contents without writing -- used by check and diff.
pub fn generate_expected(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path).ok()??;
    let profile = cfg.profile().map_or("service", |p| p.name()).to_owned();

    let files = generate_all_files(project_path, &cfg, &profile);
    Some(files.into_iter().map(|gf| (gf.path, gf.content)).collect())
}

#[cfg(feature = "product-ts")]
/// Determine which app types exist in the TypeScript config.
fn detect_ts_app_types(ts_cfg: &config::types::TypeScriptConfig) -> (bool, bool) {
    let Some(apps) = ts_cfg.apps() else {
        return (false, true);
    };

    if apps.is_empty() {
        return (false, true);
    }

    let mut has_content = false;
    let mut has_service = false;

    for app_cfg in apps.values() {
        match app_cfg.type_() {
            Some(t) if t.eq_ignore_ascii_case("content") => has_content = true,
            Some(t) if t.eq_ignore_ascii_case("service") => has_service = true,
            Some(t) if t.eq_ignore_ascii_case("library") => {}
            _ => has_service = true,
        }
    }

    if !has_content && !has_service {
        has_service = true;
    }

    (has_content, has_service)
}

fn build_hook_content(
    cfg: Option<&config::types::GuardrailConfig>,
    has_rust: bool,
    has_typescript: bool,
) -> String {
    let rust_workspace_root = cfg.map_or_else(
        || ".".to_owned(),
        |config| {
            config
                .rust()
                .and_then(|rust| rust.workspace_root())
                .unwrap_or(".")
                .to_owned()
        },
    );
    guardrail3_domain_modules::pre_commit::build_pre_commit_script(has_rust, has_typescript)
        .replace(
            "GUARDRAIL3_RUST_WORKSPACE:-.}",
            &format!("GUARDRAIL3_RUST_WORKSPACE:-{rust_workspace_root}}}"),
        )
}

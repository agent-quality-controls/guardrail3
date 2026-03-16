use std::collections::BTreeMap;
use std::path::Path;

use crate::cli::GenerateArgs;
use crate::domain::config;
use crate::domain::modules::{canonical, clippy, deny, release};

/// Load guardrail3.toml configuration from a project path.
#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI tool — config parse errors reported to stderr; guardrail3 config parsing — no garde validation needed for own config
fn load_config(path: &Path) -> Option<config::types::GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = crate::fs::read_file(&config_path)?;
    match toml::from_str(&content) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            eprintln!("Error parsing guardrail3.toml: {e}");
            None
        }
    }
}

struct GeneratedFile {
    path: String,
    content: String,
}

/// Main generate command -- generates all config files from guardrail3.toml.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and process::exit for error codes
pub fn run(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let Some(cfg) = load_config(project_path) else {
        eprintln!(
            "Error: guardrail3.toml not found or invalid at {}",
            project_path.display()
        );
        eprintln!("Run 'guardrail3 init' to create one.");
        std::process::exit(1);
    };

    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_owned();

    let files = generate_all_files(project_path, &cfg, &profile);

    let mut written = 0usize;
    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            if let Err(e) = crate::fs::create_dir_all(parent) {
                eprintln!("Error creating directory {}: {e}", parent.display());
                continue;
            }
        }
        if let Err(e) = crate::fs::write_file(&target, &gf.content) {
            eprintln!("Error writing {}: {e}", gf.path);
            continue;
        }
        println!("  wrote: {}", gf.path);
        written = written.saturating_add(1);
    }

    println!();
    println!("Generated {written} files (profile: {profile}).");

    // Print cargo-lints reference
    if cfg.rust.is_some() {
        println!();
        println!("NOTE: Add these workspace lints to your Cargo.toml manually");
        println!("(guardrail3 does not modify Cargo.toml):");
        println!("  guardrail3 show-module canonical/cargo-lints");
    }
}

/// Generate only Rust config files.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_rs(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let Some(cfg) = load_config(project_path) else {
        eprintln!(
            "Error: guardrail3.toml not found or invalid at {}",
            project_path.display()
        );
        std::process::exit(1);
    };

    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_owned();

    let local = load_local_overrides(project_path, &cfg);
    let rust_root = resolve_rust_root(&cfg);

    let files = generate_rust_files(&rust_root, &cfg, &profile, &local);

    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            let _ = crate::fs::create_dir_all(parent);
        }
        if let Err(e) = crate::fs::write_file(&target, &gf.content) {
            eprintln!("Error writing {}: {e}", gf.path);
            continue;
        }
        println!("  wrote: {}", gf.path);
    }

    // Also generate pre-commit hook
    let has_typescript = cfg.typescript.is_some();
    generate_and_install_hooks(project_path, true, has_typescript);
}

/// Generate only TypeScript config files.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_ts(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let Some(cfg) = load_config(project_path) else {
        eprintln!(
            "Error: guardrail3.toml not found at {}",
            project_path.display()
        );
        std::process::exit(1);
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
            let _ = crate::fs::create_dir_all(parent);
        }
        if let Err(e) = crate::fs::write_file(&target, &gf.content) {
            eprintln!("Error writing {}: {e}", gf.path);
            continue;
        }
        println!("  wrote: {}", gf.path);
        written = written.saturating_add(1);
    }

    println!();
    println!("Generated {written} TypeScript config files.");

    // Also generate pre-commit hook
    let has_rust = cfg.rust.is_some();
    generate_and_install_hooks(project_path, has_rust, true);
}

/// Generate and install pre-commit hooks for the detected stacks.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI output
fn generate_and_install_hooks(project_path: &Path, has_rust: bool, has_typescript: bool) {
    let hook_content =
        crate::domain::modules::pre_commit::build_pre_commit_script(has_rust, has_typescript);

    let hooks_dir = project_path.join(".githooks");
    let _ = crate::fs::create_dir_all(&hooks_dir);
    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = crate::fs::write_file(&hook_path, &hook_content) {
        eprintln!("Error writing pre-commit hook: {e}");
        return;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(meta) = crate::fs::metadata(&hook_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = crate::fs::set_permissions(&hook_path, perms);
        }
    }
    println!("  wrote: .githooks/pre-commit");

    // Configure git to use .githooks/
    let _ = std::process::Command::new("git")
        .args(["config", "core.hooksPath", ".githooks"])
        .current_dir(project_path)
        .output();
}

/// Install pre-commit hooks (standalone command).
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_hooks(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let cfg = load_config(project_path);

    let has_rust = cfg.as_ref().and_then(|c| c.rust.as_ref()).is_some();
    let has_typescript = cfg.as_ref().and_then(|c| c.typescript.as_ref()).is_some();
    let hook_content =
        crate::domain::modules::pre_commit::build_pre_commit_script(has_rust, has_typescript);

    let hooks_dir = project_path.join(".githooks");
    if let Err(e) = crate::fs::create_dir_all(&hooks_dir) {
        eprintln!("Error creating .githooks/ directory: {e}");
        std::process::exit(1);
    }

    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = crate::fs::write_file(&hook_path, &hook_content) {
        eprintln!("Error writing pre-commit hook: {e}");
        std::process::exit(1);
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        if let Err(e) = crate::fs::set_permissions(&hook_path, perms) {
            eprintln!("Warning: could not set executable permission: {e}");
        }
    }

    println!("  wrote: .githooks/pre-commit");
    println!();
    println!("Configure git to use hooks: git config core.hooksPath .githooks");
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

struct LocalOverrides {
    clippy_methods: String,
    clippy_types: String,
    deny_bans: String,
    deny_skip: String,
    deny_feature_bans: String,
}

fn load_local_overrides(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
) -> LocalOverrides {
    let local = cfg.local.as_ref();

    let read_local = |field: Option<&String>| -> String {
        match field {
            Some(rel) => {
                let path = project_path.join(rel);
                crate::fs::read_file(&path).unwrap_or_default()
            }
            None => String::new(),
        }
    };

    LocalOverrides {
        clippy_methods: read_local(local.and_then(|l| l.clippy_methods.as_ref())),
        clippy_types: read_local(local.and_then(|l| l.clippy_types.as_ref())),
        deny_bans: read_local(local.and_then(|l| l.deny_bans.as_ref())),
        deny_skip: read_local(local.and_then(|l| l.deny_skip.as_ref())),
        deny_feature_bans: read_local(local.and_then(|l| l.deny_feature_bans.as_ref())),
    }
}

fn resolve_rust_root(cfg: &config::types::GuardrailConfig) -> String {
    cfg.rust
        .as_ref()
        .and_then(|r| r.workspace_root.as_deref())
        .unwrap_or(".")
        .to_owned()
}

fn generate_all_files(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
    profile: &str,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let local = load_local_overrides(project_path, cfg);

    if cfg.rust.is_some() {
        let rust_root = resolve_rust_root(cfg);
        let rust_files = generate_rust_files(&rust_root, cfg, profile, &local);
        files.extend(rust_files);
    }

    if cfg.typescript.is_some() {
        let ts_files = generate_ts_files(cfg);
        files.extend(ts_files);
    }

    // Hooks — build script with appropriate duplication sections and workspace root
    let has_rust = cfg.rust.is_some();
    let has_typescript = cfg.typescript.is_some();
    let rust_workspace_root = cfg
        .rust
        .as_ref()
        .and_then(|r| r.workspace_root.as_deref())
        .unwrap_or(".");
    let hook_content =
        crate::domain::modules::pre_commit::build_pre_commit_script(has_rust, has_typescript)
            .replace(
                "GUARDRAIL3_RUST_WORKSPACE:-.}",
                &format!("GUARDRAIL3_RUST_WORKSPACE:-{rust_workspace_root}}}"),
            );
    files.push(GeneratedFile {
        path: ".githooks/pre-commit".to_owned(),
        content: hook_content,
    });

    files
}

fn generate_ts_files(cfg: &config::types::GuardrailConfig) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let Some(ts_cfg) = cfg.typescript.as_ref() else {
        return files;
    };

    let canonical_cfg = ts_cfg.canonical.as_ref();

    // .npmrc -- generate unless explicitly disabled
    let gen_npmrc = canonical_cfg.and_then(|c| c.npmrc).unwrap_or(true);
    if gen_npmrc {
        files.push(GeneratedFile {
            path: ".npmrc".to_owned(),
            content: canonical::NPMRC.content.to_owned(),
        });
    }

    // tsconfig.base.json -- generate unless explicitly disabled
    let gen_tsconfig = canonical_cfg.and_then(|c| c.tsconfig_base).unwrap_or(true);
    if gen_tsconfig {
        files.push(GeneratedFile {
            path: "tsconfig.base.json".to_owned(),
            content: canonical::TSCONFIG_BASE.content.to_owned(),
        });
    }

    // .jscpd.json -- generate unless explicitly disabled
    let gen_jscpd = canonical_cfg.and_then(|c| c.jscpd).unwrap_or(true);
    if gen_jscpd {
        files.push(GeneratedFile {
            path: ".jscpd.json".to_owned(),
            content: canonical::JSCPD.content.to_owned(),
        });
    }

    // eslint.config.mjs -- only generate if eslint mode is "generate" or "starter"
    let gen_eslint = ts_cfg
        .eslint
        .as_ref()
        .and_then(|e| e.mode.as_deref())
        .is_some_and(|m| m == "generate" || m == "starter");
    if gen_eslint {
        files.push(GeneratedFile {
            path: "eslint.config.mjs".to_owned(),
            content: canonical::ESLINT_STARTER.content.to_owned(),
        });
    }

    files
}

fn generate_rust_files(
    rust_root: &str,
    cfg: &config::types::GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();
    let root_prefix = if rust_root == "." {
        String::new()
    } else {
        format!("{rust_root}/")
    };

    // For library profile, root clippy is always "pure" (includes global-state bans)
    let root_is_pure = profile == "library";

    // Workspace-root clippy.toml
    let root_clippy = clippy::build_clippy_toml(
        profile,
        root_is_pure,
        &local.clippy_methods,
        &local.clippy_types,
    );
    files.push(GeneratedFile {
        path: format!("{root_prefix}clippy.toml"),
        content: root_clippy,
    });

    // Per-crate clippy.toml for crates with layer config
    let crate_configs: BTreeMap<String, &crate::domain::config::types::CrateConfig> = cfg
        .rust
        .as_ref()
        .and_then(|r| r.crates.as_ref())
        .map(|c| c.iter().map(|(k, v)| (k.clone(), v)).collect())
        .unwrap_or_default();

    for (crate_path, crate_cfg) in &crate_configs {
        // Per-crate profile overrides workspace profile
        let effective_profile = crate_cfg.profile.as_deref().unwrap_or(profile);

        // Library profile (workspace or per-crate): all crates are pure.
        // Otherwise check layer config.
        let is_pure = effective_profile == "library"
            || crate_cfg.layer.as_deref().is_some_and(|l| l == "pure");

        let crate_clippy = clippy::build_clippy_toml(
            effective_profile,
            is_pure,
            &local.clippy_methods,
            &local.clippy_types,
        );
        files.push(GeneratedFile {
            path: format!("{root_prefix}{crate_path}/clippy.toml"),
            content: crate_clippy,
        });
    }

    // deny.toml -- profile-aware
    let deny_content = build_deny_for_profile(
        profile,
        &local.deny_bans,
        &local.deny_skip,
        &local.deny_feature_bans,
    );
    files.push(GeneratedFile {
        path: format!("{root_prefix}deny.toml"),
        content: deny_content,
    });

    // rustfmt.toml
    files.push(GeneratedFile {
        path: format!("{root_prefix}rustfmt.toml"),
        content: canonical::RUSTFMT.content.to_owned(),
    });

    // rust-toolchain.toml
    files.push(GeneratedFile {
        path: format!("{root_prefix}rust-toolchain.toml"),
        content: canonical::RUST_TOOLCHAIN.content.to_owned(),
    });

    // release-plz.toml and cliff.toml — service profile only
    if profile == "service" {
        files.push(GeneratedFile {
            path: "release-plz.toml".to_owned(),
            content: release::RELEASE_PLZ_TOML.content.to_owned(),
        });
        files.push(GeneratedFile {
            path: "cliff.toml".to_owned(),
            content: release::CLIFF_TOML.content.to_owned(),
        });
    }

    files
}

fn build_deny_for_profile(
    profile: &str,
    extra_bans: &str,
    extra_skip: &str,
    extra_feature_bans: &str,
) -> String {
    match profile {
        "library" => deny::build_deny_toml_with_entries(
            profile,
            &deny::library_profile_ban_entries(),
            None, // no tokio feature ban (tokio banned entirely)
            extra_bans,
            extra_skip,
            extra_feature_bans,
        ),
        _ => deny::build_deny_toml(profile, extra_bans, extra_skip, extra_feature_bans),
    }
}

/// Generate expected file contents without writing -- used by check and diff.
#[allow(clippy::type_complexity)] // reason: legitimate complex type
pub fn generate_expected(project_path: &Path) -> Option<Vec<(String, String)>> {
    let cfg = load_config(project_path)?;
    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_owned();

    let files = generate_all_files(project_path, &cfg, &profile);
    Some(files.into_iter().map(|gf| (gf.path, gf.content)).collect())
}

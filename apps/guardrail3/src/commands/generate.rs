use std::collections::BTreeMap;
use std::path::Path;

use crate::cli::GenerateArgs;
use crate::domain::config;
use crate::domain::modules::{canonical, clippy, deny, release};

/// A (`relative_path`, `content`) pair for a generated file.
type GeneratedPair = (String, String);

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
        warn_if_overwriting(&target, &gf.path, &gf.content);
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

    let local = load_local_overrides(project_path);
    let rust_root = resolve_rust_root(&cfg);

    let files = generate_rust_files(&rust_root, &cfg, &profile, &local);

    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            let _ = crate::fs::create_dir_all(parent);
        }
        warn_if_overwriting(&target, &gf.path, &gf.content);
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
        warn_if_overwriting(&target, &gf.path, &gf.content);
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
        if let Some(meta) = crate::fs::metadata(&hook_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            if let Err(e) = crate::fs::set_permissions(&hook_path, perms) {
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
    if let Some(existing) = crate::fs::read_file(target) {
        if existing != new_content {
            eprintln!(
                "  warning: Overwriting {relative_path} — manual edits will be lost. Use .guardrail3/overrides/ for project-specific customization."
            );
        }
    }
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

fn load_local_overrides(project_path: &Path) -> LocalOverrides {
    let overrides_dir = project_path.join(".guardrail3/overrides");

    let read_and_validate = |name: &str| -> String {
        let path = overrides_dir.join(name);
        let raw = crate::fs::read_file(&path).unwrap_or_default();
        // Strip UTF-8 BOM if present
        let clean = raw.strip_prefix('\u{FEFF}').unwrap_or(&raw);
        if clean.trim().is_empty() {
            return String::new();
        }
        validate_override_content(clean, name)
    };

    LocalOverrides {
        clippy_methods: read_and_validate("clippy-methods.toml"),
        clippy_types: read_and_validate("clippy-types.toml"),
        deny_bans: read_and_validate("deny-bans.toml"),
        deny_skip: read_and_validate("deny-skip.toml"),
        deny_feature_bans: read_and_validate("deny-feature-bans.toml"),
    }
}

/// Validate override content — skip lines that don't match expected TOML entry patterns.
/// Valid patterns: `{ path = "..." }`, `{ name = "..." }`, `[[...]]` section headers.
/// Comments and blank lines are silently stripped (not injected).
#[allow(clippy::print_stderr)] // reason: CLI tool — malformed override warnings reported to stderr
fn validate_override_content(content: &str, file_name: &str) -> String {
    let mut valid = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let normalized = trimmed.replace(' ', "");
        if normalized.starts_with("{path=")
            || normalized.starts_with("{name=")
            || normalized.starts_with("[[")
        {
            valid.push_str(line);
            valid.push('\n');
        } else {
            eprintln!("  warning: skipping invalid line in {file_name}: {trimmed}");
        }
    }
    valid
}

/// Remove override entries that already exist in the base content.
/// For TOML array entries, compares by the trimmed line (minus trailing comma).
pub(crate) fn deduplicated_override(base: &str, override_content: &str) -> String {
    if override_content.trim().is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for line in override_content.lines() {
        let trimmed = line.trim();
        // Skip empty lines and comments (already stripped by validation, but be defensive)
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // Check if this exact entry already exists in base (ignore trailing comma differences)
        let key = trimmed.trim_end_matches(',');
        if base.contains(key) {
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

#[allow(clippy::print_stderr)] // reason: workspace_root traversal warning reported to stderr
fn resolve_rust_root(cfg: &config::types::GuardrailConfig) -> String {
    let root = cfg
        .rust
        .as_ref()
        .and_then(|r| r.workspace_root.as_deref())
        .unwrap_or(".");

    // Reject path traversal attempts — workspace_root must not escape the project
    if root.contains("..") {
        eprintln!(
            "Error: workspace_root contains '..', which could write files outside the project. Use a relative path within the project."
        );
        return ".".to_owned();
    }

    root.to_owned()
}

fn generate_all_files(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
    profile: &str,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let local = load_local_overrides(project_path);

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
        .and_then(|r| r.apps.as_ref())
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

/// Generate expected TS file contents without writing -- used by ts diff.
pub fn generate_expected_ts(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path)?;

    let files = generate_ts_files(&cfg);
    if files.is_empty() {
        return Some(Vec::new());
    }

    let mut pairs: Vec<GeneratedPair> = files.into_iter().map(|gf| (gf.path, gf.content)).collect();

    // Include pre-commit hook (same as run_ts)
    let has_rust = cfg.rust.is_some();
    let hook_content = crate::domain::modules::pre_commit::build_pre_commit_script(has_rust, true);
    pairs.push((".githooks/pre-commit".to_owned(), hook_content));

    Some(pairs)
}

/// Generate expected file contents without writing -- used by check and diff.
pub fn generate_expected(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path)?;
    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_owned();

    let files = generate_all_files(project_path, &cfg, &profile);
    Some(files.into_iter().map(|gf| (gf.path, gf.content)).collect())
}

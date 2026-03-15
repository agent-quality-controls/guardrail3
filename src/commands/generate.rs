use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::cli::GenerateArgs;
use crate::config;
use crate::modules::{canonical, clippy, deny};

struct GeneratedFile {
    path: String,
    content: String,
}

/// Main generate command -- generates all config files from guardrail3.toml.
pub fn run(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let cfg = match config::load_config(project_path) {
        Some(c) => c,
        None => {
            eprintln!(
                "Error: guardrail3.toml not found or invalid at {}",
                project_path.display()
            );
            eprintln!("Run 'guardrail3 init' to create one.");
            std::process::exit(1);
        }
    };

    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_string();

    let files = generate_all_files(project_path, &cfg, &profile);

    let mut written = 0usize;
    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Error creating directory {}: {e}", parent.display());
                continue;
            }
        }
        if let Err(e) = fs::write(&target, &gf.content) {
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
pub fn run_rs(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let cfg = match config::load_config(project_path) {
        Some(c) => c,
        None => {
            eprintln!(
                "Error: guardrail3.toml not found or invalid at {}",
                project_path.display()
            );
            std::process::exit(1);
        }
    };

    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_string();

    let local = load_local_overrides(project_path, &cfg);
    let rust_root = resolve_rust_root(project_path, &cfg);

    let files = generate_rust_files(&rust_root, &cfg, &profile, &local);

    for gf in &files {
        let target = project_path.join(&gf.path);
        if let Some(parent) = target.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(e) = fs::write(&target, &gf.content) {
            eprintln!("Error writing {}: {e}", gf.path);
            continue;
        }
        println!("  wrote: {}", gf.path);
    }
}

/// Generate only TypeScript config files (placeholder for now).
pub fn run_ts(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let cfg = config::load_config(project_path);
    if cfg.is_none() {
        eprintln!("Error: guardrail3.toml not found at {}", project_path.display());
        std::process::exit(1);
    }
    println!("TypeScript config generation not yet implemented.");
}

/// Install pre-commit hooks.
pub fn run_hooks(args: &GenerateArgs) {
    let project_path = Path::new(&args.path);
    let hook_content = crate::modules::pre_commit::PRE_COMMIT_SCRIPT.content;

    let hooks_dir = project_path.join(".githooks");
    if let Err(e) = fs::create_dir_all(&hooks_dir) {
        eprintln!("Error creating .githooks/ directory: {e}");
        std::process::exit(1);
    }

    let hook_path = hooks_dir.join("pre-commit");
    if let Err(e) = fs::write(&hook_path, hook_content) {
        eprintln!("Error writing pre-commit hook: {e}");
        std::process::exit(1);
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        if let Err(e) = fs::set_permissions(&hook_path, perms) {
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
                fs::read_to_string(&path).unwrap_or_default()
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

fn resolve_rust_root(
    _project_path: &Path,
    cfg: &config::types::GuardrailConfig,
) -> String {
    cfg.rust
        .as_ref()
        .and_then(|r| r.workspace_root.as_deref())
        .unwrap_or(".")
        .to_string()
}

/// Discover workspace member crates from Cargo.toml.
/// Used during generate to auto-discover crates when not explicitly listed in config.
pub fn discover_workspace_crates(project_path: &Path, rust_root: &str) -> Vec<String> {
    let cargo_path = project_path.join(rust_root).join("Cargo.toml");
    let content = match fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let parsed: toml::Value = match toml::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut crates = Vec::new();

    if let Some(members) = parsed
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
    {
        for member in members {
            if let Some(s) = member.as_str() {
                // Expand globs
                if s.contains('*') {
                    let pattern = project_path
                        .join(rust_root)
                        .join(s)
                        .display()
                        .to_string();
                    if let Ok(paths) = glob::glob(&pattern) {
                        for entry in paths.flatten() {
                            if entry.join("Cargo.toml").exists() {
                                if let Ok(rel) = entry.strip_prefix(project_path.join(rust_root)) {
                                    crates.push(rel.display().to_string());
                                }
                            }
                        }
                    }
                } else {
                    crates.push(s.to_string());
                }
            }
        }
    }

    crates
}

fn generate_all_files(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
    profile: &str,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let local = load_local_overrides(project_path, cfg);

    if cfg.rust.is_some() {
        let rust_root = resolve_rust_root(project_path, cfg);
        let rust_files = generate_rust_files(&rust_root, cfg, profile, &local);
        files.extend(rust_files);
    }

    // Hooks
    files.push(GeneratedFile {
        path: ".githooks/pre-commit".to_string(),
        content: crate::modules::pre_commit::PRE_COMMIT_SCRIPT.content.to_string(),
    });

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

    // Workspace-root clippy.toml (not pure layer -- composition root level)
    let root_clippy = clippy::build_clippy_toml(
        profile,
        false,
        &local.clippy_methods,
        &local.clippy_types,
    );
    files.push(GeneratedFile {
        path: format!("{root_prefix}clippy.toml"),
        content: root_clippy,
    });

    // Per-crate clippy.toml for crates with layer config
    let crate_configs: BTreeMap<String, &crate::config::types::CrateConfig> = cfg
        .rust
        .as_ref()
        .and_then(|r| r.crates.as_ref())
        .map(|c| c.iter().map(|(k, v)| (k.clone(), v)).collect())
        .unwrap_or_default();

    for (crate_path, crate_cfg) in &crate_configs {
        let is_pure = crate_cfg
            .layer
            .as_deref()
            .is_some_and(|l| l == "pure");

        let crate_clippy = clippy::build_clippy_toml(
            profile,
            is_pure,
            &local.clippy_methods,
            &local.clippy_types,
        );
        files.push(GeneratedFile {
            path: format!("{root_prefix}{crate_path}/clippy.toml"),
            content: crate_clippy,
        });
    }

    // deny.toml
    let deny_content = deny::build_deny_toml(
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
        content: canonical::RUSTFMT.content.to_string(),
    });

    // rust-toolchain.toml
    files.push(GeneratedFile {
        path: format!("{root_prefix}rust-toolchain.toml"),
        content: canonical::RUST_TOOLCHAIN.content.to_string(),
    });

    files
}

/// Generate expected file contents without writing -- used by check and diff.
pub fn generate_expected(
    project_path: &Path,
) -> Option<Vec<(String, String)>> {
    let cfg = config::load_config(project_path)?;
    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |p| p.name.as_str())
        .to_string();

    let files = generate_all_files(project_path, &cfg, &profile);
    Some(
        files
            .into_iter()
            .map(|gf| (gf.path, gf.content))
            .collect(),
    )
}

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::discover::{detect_project, resolve_app_paths_from_member_dirs};
use guardrail3_domain_config::types::{CrateConfig, GuardrailConfig};
use guardrail3_domain_modules::{canonical, clippy, deny, pre_commit, release};

use crate::overrides::{LocalOverrides, load_local_overrides};

/// One generated artifact owned by Rust generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

/// A (`relative_path`, `content`) pair for dry-run/check output.
pub type GeneratedPair = (String, String);

/// Generate the full Rust write set: Rust config files plus the Rust-only hook artifact.
pub fn generate_rust_owned_artifacts(
    project_path: &Path,
    cfg: &GuardrailConfig,
) -> Vec<GeneratedFile> {
    let mut files = generate_rust_config_files(project_path, cfg);
    files.push(generate_rust_hook_artifact(Some(cfg)));
    files
}

/// Generate the standalone Rust pre-commit hook artifact.
pub fn generate_rust_hook_artifact(cfg: Option<&GuardrailConfig>) -> GeneratedFile {
    GeneratedFile {
        path: ".githooks/pre-commit".to_owned(),
        content: build_rust_hook_content(cfg),
    }
}

/// Generate expected Rust-owned files without writing them.
pub fn generate_rust_expected(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path)?;
    Some(
        generate_rust_owned_artifacts(project_path, &cfg)
            .into_iter()
            .map(|file| (file.path, file.content))
            .collect(),
    )
}

fn load_config(path: &Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = guardrail3_shared_fs::read_file(&config_path)?;
    toml::from_str(&content).ok()
}

fn generate_rust_config_files(project_path: &Path, cfg: &GuardrailConfig) -> Vec<GeneratedFile> {
    let profile = cfg
        .profile
        .as_ref()
        .map_or("service", |profile| profile.name.as_str());
    let local = load_local_overrides(project_path);
    generate_rust_files(project_path, cfg, profile, &local)
}

#[allow(clippy::print_stderr)] // reason: invalid workspace_root should produce direct CLI-visible guidance
fn resolve_rust_root(cfg: &GuardrailConfig) -> String {
    let root = cfg
        .rust
        .as_ref()
        .and_then(|rust| rust.workspace_root.as_deref())
        .unwrap_or(".");

    if root.contains("..") {
        eprintln!(
            "Error: workspace_root contains '..', which could write files outside the project. Use a relative path within the project."
        );
        return ".".to_owned();
    }

    root.to_owned()
}

fn resolve_app_paths(project_path: &Path, cfg: &GuardrailConfig) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let Some(rust_cfg) = cfg.rust.as_ref() else {
        return map;
    };
    let Some(apps) = rust_cfg.apps.as_ref() else {
        return map;
    };

    let fs = RealFileSystem;
    let project = detect_project(&fs, project_path);
    let resolved = resolve_app_paths_from_member_dirs(project.all_member_dirs());

    for app_name in apps.keys() {
        if let Some(app_dir) = resolved.get(app_name) {
            let _ = map.insert(app_name.clone(), app_dir.clone());
        }
    }

    map
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
            None,
            extra_bans,
            extra_skip,
            extra_feature_bans,
        ),
        _ => deny::build_deny_toml(profile, extra_bans, extra_skip, extra_feature_bans),
    }
}

#[allow(clippy::too_many_lines)] // reason: the Rust write set is one ownership boundary and is easier to audit as one artifact list
fn generate_rust_files(
    project_path: &Path,
    cfg: &GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();
    let app_path_map = resolve_app_paths(project_path, cfg);
    let root_is_pure = profile == "library";

    let crate_configs: BTreeMap<String, &CrateConfig> = cfg
        .rust
        .as_ref()
        .and_then(|rust| rust.apps.as_ref())
        .map(|apps| {
            apps.iter()
                .map(|(name, config)| (name.clone(), config))
                .collect()
        })
        .unwrap_or_default();

    let mut generated_dirs = BTreeSet::new();

    for (app_name, crate_cfg) in &crate_configs {
        let app_dir = app_path_map
            .get(app_name.as_str())
            .map_or_else(|| app_name.clone(), Clone::clone);

        let effective_profile = crate_cfg
            .type_
            .as_deref()
            .or(crate_cfg.profile.as_deref())
            .unwrap_or(profile);
        let effective_garde = crate_cfg
            .checks
            .as_ref()
            .and_then(|checks| checks.garde)
            .or_else(|| {
                cfg.rust
                    .as_ref()
                    .and_then(|rust| rust.checks.as_ref())
                    .and_then(|checks| checks.garde)
            })
            .unwrap_or(true);

        let is_pure = effective_profile == "library"
            || crate_cfg
                .layer
                .as_deref()
                .is_some_and(|layer| layer == "pure");
        let prefix = format!("{app_dir}/");

        files.push(GeneratedFile {
            path: format!("{prefix}clippy.toml"),
            content: clippy::build_clippy_toml(
                effective_profile,
                is_pure,
                effective_garde,
                &local.clippy_methods,
                &local.clippy_types,
            ),
        });
        files.push(GeneratedFile {
            path: format!("{prefix}deny.toml"),
            content: build_deny_for_profile(
                effective_profile,
                &local.deny_bans,
                &local.deny_skip,
                &local.deny_feature_bans,
            ),
        });
        files.push(GeneratedFile {
            path: format!("{prefix}rustfmt.toml"),
            content: canonical::RUSTFMT.content.to_owned(),
        });

        let _ = generated_dirs.insert(app_dir);
    }

    let has_packages = cfg
        .rust
        .as_ref()
        .and_then(|rust| rust.packages.as_ref())
        .is_some();
    if has_packages && !generated_dirs.contains(".") {
        let pkg_cfg = cfg.rust.as_ref().and_then(|rust| rust.packages.as_ref());
        let pkg_profile = pkg_cfg
            .and_then(|config| config.type_.as_deref().or(config.profile.as_deref()))
            .unwrap_or("library");
        let pkg_garde = pkg_cfg
            .and_then(|config| config.checks.as_ref())
            .and_then(|checks| checks.garde)
            .or_else(|| {
                cfg.rust
                    .as_ref()
                    .and_then(|rust| rust.checks.as_ref())
                    .and_then(|checks| checks.garde)
            })
            .unwrap_or(true);
        let pkg_is_pure = pkg_profile == "library";

        files.push(GeneratedFile {
            path: "clippy.toml".to_owned(),
            content: clippy::build_clippy_toml(
                pkg_profile,
                pkg_is_pure,
                pkg_garde,
                &local.clippy_methods,
                &local.clippy_types,
            ),
        });
        files.push(GeneratedFile {
            path: "deny.toml".to_owned(),
            content: build_deny_for_profile(
                pkg_profile,
                &local.deny_bans,
                &local.deny_skip,
                &local.deny_feature_bans,
            ),
        });
        files.push(GeneratedFile {
            path: "rustfmt.toml".to_owned(),
            content: canonical::RUSTFMT.content.to_owned(),
        });
    } else if crate_configs.is_empty() {
        let rust_root = resolve_rust_root(cfg);
        let prefix = if rust_root == "." {
            String::new()
        } else {
            format!("{rust_root}/")
        };

        files.push(GeneratedFile {
            path: format!("{prefix}clippy.toml"),
            content: clippy::build_clippy_toml(
                profile,
                root_is_pure,
                cfg.rust
                    .as_ref()
                    .and_then(|rust| rust.checks.as_ref())
                    .and_then(|checks| checks.garde)
                    .unwrap_or(true),
                &local.clippy_methods,
                &local.clippy_types,
            ),
        });
        files.push(GeneratedFile {
            path: format!("{prefix}deny.toml"),
            content: build_deny_for_profile(
                profile,
                &local.deny_bans,
                &local.deny_skip,
                &local.deny_feature_bans,
            ),
        });
        files.push(GeneratedFile {
            path: format!("{prefix}rustfmt.toml"),
            content: canonical::RUSTFMT.content.to_owned(),
        });
    }

    files.push(GeneratedFile {
        path: "rust-toolchain.toml".to_owned(),
        content: canonical::RUST_TOOLCHAIN.content.to_owned(),
    });

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

fn build_rust_hook_content(cfg: Option<&GuardrailConfig>) -> String {
    let rust_only_base = strip_typescript_steps(pre_commit::PRE_COMMIT_BASE);
    let rust_workspace_root = cfg.map_or_else(|| ".".to_owned(), resolve_rust_root);
    let script = format!(
        "{rust_only_base}{}{footer}",
        pre_commit::DUPLICATION_CARGO_DUPES,
        footer = pre_commit::PRE_COMMIT_FOOTER
    );
    script.replace(
        "GUARDRAIL3_RUST_WORKSPACE:-.}",
        &format!("GUARDRAIL3_RUST_WORKSPACE:-{rust_workspace_root}}}"),
    )
}

fn strip_typescript_steps(base: &str) -> String {
    let without_migration = strip_section(
        base,
        "# --- Migration consistency ---\n",
        "# --- Lockfile integrity ---\n",
    );
    let without_lockfile = strip_section(
        &without_migration,
        "# --- Lockfile integrity ---\n",
        "# --- Detect which stacks changed ---\n",
    );
    let without_ts_validation = without_lockfile.replace(
        r#"    if [ "$TS_CHANGED" -gt 0 ]; then
        echo "Running guardrail3 TypeScript validation..."
        if ! guardrail3 ts validate --staged .; then
            echo "guardrail3 TypeScript validation failed. Fix issues before committing."
            exit 1
        fi
    fi
"#,
        "",
    );
    let without_ts_checks = strip_section(
        &without_ts_validation,
        "# --- TypeScript checks (only if TS files changed) ---\n",
        "# --- CSS checks (only if CSS files changed) ---\n",
    );
    let without_css_checks = strip_section(
        &without_ts_checks,
        "# --- CSS checks (only if CSS files changed) ---\n",
        "# --- Rust checks (only if Rust or Cargo files changed) ---\n",
    );

    without_css_checks
        .replace(
            "TS_CHANGED=$(echo \"$STAGED_FILES\" | grep -cE '\\.(ts|tsx|mjs)$' || true)\n",
            "",
        )
        .replace(
            "CSS_CHANGED=$(echo \"$STAGED_FILES\" | grep -cE '\\.css$' || true)\n",
            "",
        )
}

fn strip_section(script: &str, start_marker: &str, end_marker: &str) -> String {
    let Some(start) = script.find(start_marker) else {
        return script.to_owned();
    };
    let Some(end_relative) = script[start..].find(end_marker) else {
        return script.to_owned();
    };
    let end = start + end_relative;
    let mut result = String::with_capacity(script.len());
    result.push_str(&script[..start]);
    result.push_str(&script[end..]);
    result
}

#[cfg(test)]
#[path = "owned_artifacts_tests.rs"]
mod tests;

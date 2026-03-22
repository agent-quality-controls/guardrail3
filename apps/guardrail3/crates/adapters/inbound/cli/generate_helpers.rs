use std::collections::BTreeMap;
use std::path::Path;

use crate::domain::config;
use crate::domain::modules::deny;

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

pub(super) struct GeneratedFile {
    pub(super) path: String,
    pub(super) content: String,
}

// ---------------------------------------------------------------------------
// Local override loading
// ---------------------------------------------------------------------------

pub struct LocalOverrides {
    pub clippy_methods: String,
    pub clippy_types: String,
    pub deny_bans: String,
    pub deny_skip: String,
    pub deny_feature_bans: String,
}

pub fn load_local_overrides(project_path: &Path) -> LocalOverrides {
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

/// Validate override content -- skip lines that don't match expected TOML entry patterns.
/// Valid patterns: `{ path = "..." }`, `{ name = "..." }`.
/// `[[...]]` section headers are ONLY valid in `deny-feature-bans.toml`.
/// Comments and blank lines are silently stripped (not injected).
#[allow(clippy::print_stderr)] // reason: CLI tool — malformed override warnings reported to stderr
fn validate_override_content(content: &str, file_name: &str) -> String {
    let mut valid = String::new();
    let is_feature_bans = file_name.contains("feature-bans");
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let normalized = trimmed.replace(' ', "");
        let is_entry = normalized.starts_with("{path=") || normalized.starts_with("{name=");
        let is_section_header = is_feature_bans && normalized.starts_with("[[");
        if is_entry || is_section_header {
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
pub fn deduplicated_override(base: &str, override_content: &str) -> String {
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

// ---------------------------------------------------------------------------
// Path and profile resolution
// ---------------------------------------------------------------------------

#[allow(clippy::print_stderr)] // reason: workspace_root traversal warning reported to stderr
pub fn resolve_rust_root(cfg: &config::types::GuardrailConfig) -> String {
    let root = cfg
        .rust
        .as_ref()
        .and_then(|r| r.workspace_root.as_deref())
        .unwrap_or(".");

    // Reject path traversal attempts -- workspace_root must not escape the project
    if root.contains("..") {
        eprintln!(
            "Error: workspace_root contains '..', which could write files outside the project. Use a relative path within the project."
        );
        return ".".to_owned();
    }

    root.to_owned()
}

/// Resolve app config names to actual filesystem paths using discovery.
///
/// Init strips `apps/` from directory names to create clean config keys.
/// This function reverses that: discovers workspace members and maps
/// app names back to their real relative paths.
pub fn resolve_app_paths(
    project: &crate::app::core::discover::ProjectInfo,
    cfg: &config::types::GuardrailConfig,
) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    let Some(rust_cfg) = cfg.rust.as_ref() else {
        return map;
    };
    let Some(apps) = rust_cfg.apps.as_ref() else {
        return map;
    };

    // Build a set of all known member dirs from discovery
    // Member dirs are relative to project root (e.g., "apps/validator-rust/crates/app")
    // Extract the top-level app directory (e.g., "apps/validator-rust")
    let mut app_dirs: Vec<String> = Vec::new();
    for ws in &project.workspaces {
        for member in &ws.members {
            let dir = &member.dir;
            // Extract top-level app path: "apps/X/..." -> "apps/X"
            let parts: Vec<&str> = dir.split('/').collect();
            let app_path = if parts.len() >= 2 && parts.first() == Some(&"apps") {
                format!(
                    "{}/{}",
                    parts.first().unwrap_or(&""),
                    parts.get(1).unwrap_or(&"")
                )
            } else {
                dir.clone()
            };
            if !app_dirs.contains(&app_path) {
                app_dirs.push(app_path);
            }
        }
    }

    for app_name in apps.keys() {
        // Try to find this app in discovered member directories
        for app_dir in &app_dirs {
            if app_dir.split('/').next_back() == Some(app_name.as_str()) {
                let _ = map.insert(app_name.clone(), app_dir.clone());
                break;
            }
        }
    }

    map
}

/// Determine which app types exist in the TypeScript config.
///
/// Scans `[typescript.apps.*]` entries for `type = "content"` and `type = "service"`.
/// If no apps are configured, defaults to `has_service_app = true`.
pub fn detect_ts_app_types(ts_cfg: &config::types::TypeScriptConfig) -> (bool, bool) {
    let Some(apps) = ts_cfg.apps.as_ref() else {
        // No apps configured -- default to service
        return (false, true);
    };

    if apps.is_empty() {
        return (false, true);
    }

    let mut has_content = false;
    let mut has_service = false;

    for app_cfg in apps.values() {
        match app_cfg.type_.as_deref() {
            Some(t) if t.eq_ignore_ascii_case("content") => has_content = true,
            Some(t) if t.eq_ignore_ascii_case("service") => has_service = true,
            Some(t) if t.eq_ignore_ascii_case("library") => {} // neither content nor service
            _ => has_service = true,                           // default unknown types to service
        }
    }

    // If only library apps exist, still default service to true
    if !has_content && !has_service {
        has_service = true;
    }

    (has_content, has_service)
}

pub fn build_deny_for_profile(
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

// ---------------------------------------------------------------------------
// Rust file generation
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_lines)] // reason: generates config files for multiple locations — splitting would fragment the generation logic
pub(super) fn generate_rust_files(
    project_path: &Path,
    cfg: &config::types::GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) -> Vec<GeneratedFile> {
    use crate::domain::modules::{canonical, clippy, release};

    let mut files = Vec::new();

    // Discover actual workspace structure to resolve app names → real paths
    let fs = crate::adapters::outbound::fs::RealFileSystem;
    let project = crate::app::core::discover::detect_project(&fs, project_path);
    let app_path_map = resolve_app_paths(&project, cfg);

    // For library profile, root clippy is always "pure" (includes global-state bans)
    let root_is_pure = profile == "library";

    // Per-app: clippy.toml + deny.toml + rustfmt.toml at each app's resolved path
    let crate_configs: BTreeMap<String, &crate::domain::config::types::CrateConfig> = cfg
        .rust
        .as_ref()
        .and_then(|r| r.apps.as_ref())
        .map(|c| c.iter().map(|(k, v)| (k.clone(), v)).collect())
        .unwrap_or_default();

    let mut generated_dirs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

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
            || crate_cfg.layer.as_deref().is_some_and(|l| l == "pure");

        let prefix = format!("{app_dir}/");

        let crate_clippy = clippy::build_clippy_toml(
            effective_profile,
            is_pure,
            effective_garde,
            &local.clippy_methods,
            &local.clippy_types,
        );
        files.push(GeneratedFile {
            path: format!("{prefix}clippy.toml"),
            content: crate_clippy,
        });

        let deny_content = build_deny_for_profile(
            profile,
            &local.deny_bans,
            &local.deny_skip,
            &local.deny_feature_bans,
        );
        files.push(GeneratedFile {
            path: format!("{prefix}deny.toml"),
            content: deny_content,
        });

        files.push(GeneratedFile {
            path: format!("{prefix}rustfmt.toml"),
            content: canonical::RUSTFMT.content.to_owned(),
        });

        let _ = generated_dirs.insert(app_dir);
    }

    // Root-level configs for packages (if [rust.packages] exists and root isn't already an app)
    let has_packages = cfg
        .rust
        .as_ref()
        .and_then(|r| r.packages.as_ref())
        .is_some();
    if has_packages && !generated_dirs.contains(".") {
        let pkg_cfg = cfg.rust.as_ref().and_then(|r| r.packages.as_ref());
        let pkg_profile = pkg_cfg
            .and_then(|c| c.type_.as_deref().or(c.profile.as_deref()))
            .unwrap_or("library");
        let pkg_garde = pkg_cfg
            .and_then(|c| c.checks.as_ref())
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
        // No apps configured — generate at workspace root (single-crate project)
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

    // Project-root only: rust-toolchain.toml, release-plz.toml, cliff.toml
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

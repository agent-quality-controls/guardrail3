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
    path: String,
    content: String,
}

impl GeneratedFile {
    #[must_use]
    pub const fn new(path: String, content: String) -> Self {
        Self { path, content }
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub fn into_pair(self) -> GeneratedPair {
        (self.path, self.content)
    }
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
    GeneratedFile::new(
        ".githooks/pre-commit".to_owned(),
        build_rust_hook_content(cfg),
    )
}

/// Generate expected Rust-owned files without writing them.
pub fn generate_rust_expected(project_path: &Path) -> Option<Vec<GeneratedPair>> {
    let cfg = load_config(project_path)?;
    Some(
        generate_rust_owned_artifacts(project_path, &cfg)
            .into_iter()
            .map(GeneratedFile::into_pair)
            .collect(),
    )
}

fn load_config(path: &Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = guardrail3_shared_fs::read_file(&config_path)?;
    toml::from_str(&content).ok()
}

fn generate_rust_config_files(project_path: &Path, cfg: &GuardrailConfig) -> Vec<GeneratedFile> {
    let profile = cfg.profile().map_or("service", |profile| profile.name());
    let local = load_local_overrides(project_path);
    generate_rust_files(project_path, cfg, profile, &local)
}

#[allow(clippy::print_stderr)] // reason: invalid workspace_root should produce direct CLI-visible guidance
fn resolve_rust_root(cfg: &GuardrailConfig) -> String {
    let root = cfg
        .rust()
        .and_then(|rust| rust.workspace_root())
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
    let Some(rust_cfg) = cfg.rust() else {
        return map;
    };
    let Some(apps) = rust_cfg.apps() else {
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

fn generate_rust_files(
    project_path: &Path,
    cfg: &GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();
    let app_path_map = resolve_app_paths(project_path, cfg);

    let crate_configs: BTreeMap<String, &CrateConfig> = cfg
        .rust()
        .and_then(|rust| rust.apps())
        .map(|apps| {
            apps.iter()
                .map(|(name, config)| (name.clone(), config))
                .collect()
        })
        .unwrap_or_default();

    let mut generated_dirs = BTreeSet::new();

    add_app_owned_rust_files(
        &mut files,
        &mut generated_dirs,
        &crate_configs,
        &app_path_map,
        cfg,
        profile,
        local,
    );
    add_root_owned_rust_files(
        &mut files,
        &generated_dirs,
        &crate_configs,
        cfg,
        profile,
        local,
    );
    add_shared_rust_files(&mut files, profile);

    files
}

fn add_app_owned_rust_files(
    files: &mut Vec<GeneratedFile>,
    generated_dirs: &mut BTreeSet<String>,
    crate_configs: &BTreeMap<String, &CrateConfig>,
    app_path_map: &BTreeMap<String, String>,
    cfg: &GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) {
    for (app_name, crate_cfg) in crate_configs {
        let app_dir = app_path_map
            .get(app_name.as_str())
            .map_or_else(|| app_name.clone(), Clone::clone);
        let effective_profile = crate_effective_profile(crate_cfg, profile);
        let effective_garde = crate_effective_garde(cfg, Some(*crate_cfg));
        let is_pure = crate_is_pure(crate_cfg, effective_profile);
        let prefix = format!("{app_dir}/");

        push_standard_rust_files(
            files,
            &prefix,
            effective_profile,
            is_pure,
            effective_garde,
            local,
        );
        let _ = generated_dirs.insert(app_dir);
    }
}

fn add_root_owned_rust_files(
    files: &mut Vec<GeneratedFile>,
    generated_dirs: &BTreeSet<String>,
    crate_configs: &BTreeMap<String, &CrateConfig>,
    cfg: &GuardrailConfig,
    profile: &str,
    local: &LocalOverrides,
) {
    let has_packages = cfg.rust().and_then(|rust| rust.packages()).is_some();

    if has_packages && !generated_dirs.contains(".") {
        let pkg_cfg = cfg.rust().and_then(|rust| rust.packages());
        let pkg_profile = pkg_cfg
            .and_then(|config| config.type_().or(config.profile()))
            .unwrap_or("library");
        let pkg_garde = crate_effective_garde(cfg, pkg_cfg);
        push_standard_rust_files(
            files,
            "",
            pkg_profile,
            pkg_profile == "library",
            pkg_garde,
            local,
        );
        return;
    }

    if crate_configs.is_empty() {
        let rust_root = resolve_rust_root(cfg);
        let prefix = if rust_root == "." {
            String::new()
        } else {
            format!("{rust_root}/")
        };
        push_standard_rust_files(
            files,
            &prefix,
            profile,
            profile == "library",
            crate_effective_garde(cfg, None),
            local,
        );
    }
}

fn add_shared_rust_files(files: &mut Vec<GeneratedFile>, profile: &str) {
    files.push(GeneratedFile::new(
        "rust-toolchain.toml".to_owned(),
        canonical::RUST_TOOLCHAIN.content().to_owned(),
    ));

    if profile == "service" {
        files.push(GeneratedFile::new(
            "release-plz.toml".to_owned(),
            release::RELEASE_PLZ_TOML.content().to_owned(),
        ));
        files.push(GeneratedFile::new(
            "cliff.toml".to_owned(),
            release::CLIFF_TOML.content().to_owned(),
        ));
    }
}

fn push_standard_rust_files(
    files: &mut Vec<GeneratedFile>,
    prefix: &str,
    profile: &str,
    is_pure: bool,
    garde_enabled: bool,
    local: &LocalOverrides,
) {
    files.push(GeneratedFile::new(
        format!("{prefix}clippy.toml"),
        clippy::build_clippy_toml(
            profile,
            is_pure,
            garde_enabled,
            &local.clippy_methods,
            &local.clippy_types,
        ),
    ));
    files.push(GeneratedFile::new(
        format!("{prefix}deny.toml"),
        build_deny_for_profile(
            profile,
            &local.deny_bans,
            &local.deny_skip,
            &local.deny_feature_bans,
        ),
    ));
    files.push(GeneratedFile::new(
        format!("{prefix}rustfmt.toml"),
        canonical::RUSTFMT.content().to_owned(),
    ));
}

fn crate_effective_profile<'a>(crate_cfg: &'a CrateConfig, fallback: &'a str) -> &'a str {
    crate_cfg
        .type_()
        .or(crate_cfg.profile())
        .unwrap_or(fallback)
}

fn crate_effective_garde(cfg: &GuardrailConfig, crate_cfg: Option<&CrateConfig>) -> bool {
    crate_cfg
        .and_then(|config| config.checks())
        .and_then(|checks| checks.garde())
        .or_else(|| {
            cfg.rust()
                .and_then(|rust| rust.checks())
                .and_then(|checks| checks.garde())
        })
        .unwrap_or(true)
}

fn crate_is_pure(crate_cfg: &CrateConfig, effective_profile: &str) -> bool {
    effective_profile == "library" || crate_cfg.layer().is_some_and(|layer| layer == "pure")
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

mod owned_artifacts_tests;

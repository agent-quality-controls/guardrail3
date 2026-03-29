pub mod allow_checks;
pub mod arch;
pub mod ast_helpers;
pub mod ast_visitors;
pub mod cargo_lints;
pub mod clippy_coverage;
pub mod code_quality_checks;
pub mod config_files;
pub mod deny_audit;
mod deny_bans;
pub mod deny_inventory;
mod deny_licenses;
pub mod dependency_allowlist;
pub mod dependency_scan;
pub mod extra_visitors;
pub mod garde_checks;
pub mod hex_arch_checks;
pub mod hex_arch_structure;
pub mod release_bin_checks;
pub mod release_checks;
pub mod release_crate_checks;
pub mod release_crate_deps;
pub mod release_repo_checks;
mod rustfmt_check;
pub mod source_scan;
pub mod structure_checks;
pub mod test_checks;
pub mod test_quality_checks;
mod toolchain_check;
mod workspace_metadata;

use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;
use guardrail3_app_core::discover::ProjectInfo;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_report::{Report, RustCheckCategories, Section};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

pub struct RunInput<'a> {
    pub fs: &'a dyn FileSystem,
    pub path: &'a Path,
    pub project: &'a ProjectInfo,
    pub scoped_files: Option<&'a [String]>,
    pub categories: &'a RustCheckCategories,
    pub thorough: bool,
    pub tc: &'a dyn ToolChecker,
    pub crawl: &'a CrawlResult,
}

struct CodeCheckInput<'a> {
    fs: &'a dyn FileSystem,
    tc: &'a dyn ToolChecker,
    workspace_root: &'a Path,
    project: &'a ProjectInfo,
    scoped_files: Option<&'a [String]>,
    profile_ref: Option<&'a str>,
    garde_enabled: bool,
    clippy_tomls: &'a [PathBuf],
    rustfmt_tomls: &'a [PathBuf],
    rust_toolchains: &'a [PathBuf],
    deny_tomls: &'a [PathBuf],
    cargo_tomls: &'a [PathBuf],
}

/// Try to load guardrail3.toml as a typed config.
fn load_guardrail_config(fs: &dyn FileSystem, path: &Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = fs.read_file(&config_path)?;
    toml::from_str(&content).ok()
}

/// Extract profile name from the loaded config.
fn extract_profile(cfg: &GuardrailConfig) -> Option<String> {
    cfg.profile.as_ref().map(|p| p.name.clone())
}

pub fn run(input: RunInput<'_>) -> Report {
    let workspace_root = input.project.primary_workspace_root().unwrap_or(input.path);

    let guardrail_cfg = load_guardrail_config(input.fs, input.path);
    let profile = guardrail_cfg.as_ref().and_then(extract_profile);

    // Filter crawler data to files within this workspace
    let ws_clippy_tomls = filter_by_workspace(&input.crawl.clippy_tomls, workspace_root);
    let ws_rustfmt_tomls = filter_by_workspace(&input.crawl.rustfmt_tomls, workspace_root);
    let ws_rust_toolchains = filter_by_workspace(&input.crawl.rust_toolchains, workspace_root);
    let ws_deny_tomls = filter_by_workspace(&input.crawl.deny_tomls, workspace_root);
    let ws_cargo_tomls = filter_by_workspace(&input.crawl.cargo_tomls, workspace_root);

    let mut report = Report::new(input.path.display().to_string(), vec!["Rust".to_owned()]);

    run_code_checks(
        CodeCheckInput {
            fs: input.fs,
            tc: input.tc,
            workspace_root,
            project: input.project,
            scoped_files: input.scoped_files,
            profile_ref: profile.as_deref(),
            garde_enabled: input.categories.garde,
            clippy_tomls: &ws_clippy_tomls,
            rustfmt_tomls: &ws_rustfmt_tomls,
            rust_toolchains: &ws_rust_toolchains,
            deny_tomls: &ws_deny_tomls,
            cargo_tomls: &ws_cargo_tomls,
        },
        &mut report,
    );

    if input.categories.architecture {
        run_architecture_checks(
            input.fs,
            workspace_root,
            input.project,
            guardrail_cfg.as_ref(),
            profile.as_ref(),
            &mut report,
        );
    }

    if input.categories.garde {
        let garde_results = garde_checks::check(input.fs, workspace_root);
        report.add_section(Section {
            name: "Garde boundary validation".to_owned(),
            results: garde_results,
        });
    }

    if input.categories.hooks {
        let tree = guardrail3_app_core::project_walker::walk_project(input.fs, input.path);
        let hook_results = guardrail3_app_hooks::check(input.fs, input.path, &tree, input.tc);
        report.add_section(Section {
            name: "Hook checks".to_owned(),
            results: hook_results,
        });
    }

    if input.categories.tests {
        let test_results = test_checks::check(input.fs, input.tc, workspace_root);
        report.add_section(Section {
            name: "Test quality".to_owned(),
            results: test_results,
        });
    }

    if input.categories.release {
        let release_results = release_checks::check(
            input.fs,
            input.tc,
            workspace_root,
            input.project,
            input.thorough,
        );
        report.add_section(Section {
            name: "Release readiness".to_owned(),
            results: release_results,
        });
    }

    report
}

pub fn run_hook_report(fs: &dyn FileSystem, path: &Path, tc: &dyn ToolChecker) -> Report {
    let tree = guardrail3_app_core::project_walker::walk_project(fs, path);
    let hook_results = guardrail3_app_hooks::check(fs, path, &tree, tc);
    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);
    report.add_section(Section {
        name: "Hook checks".to_owned(),
        results: hook_results,
    });
    report
}

/// Filter a list of paths to only those within the given workspace root.
fn filter_by_workspace(paths: &[PathBuf], workspace_root: &Path) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|p| p.starts_with(workspace_root))
        .cloned()
        .collect()
}

fn run_code_checks(input: CodeCheckInput<'_>, report: &mut Report) {
    let config_results = config_files::check(
        input.fs,
        input.workspace_root,
        input.clippy_tomls,
        input.rustfmt_tomls,
        input.rust_toolchains,
    );
    let member_dirs = input.project.all_member_dirs();
    let per_crate_results = config_files::check_per_crate_clippy(
        input.fs,
        input.workspace_root,
        &member_dirs,
        input.clippy_tomls,
    );
    let mut config_section_results = config_results;
    config_section_results.extend(per_crate_results);
    report.add_section(Section {
        name: "Config files".to_owned(),
        results: config_section_results,
    });

    let clippy_results = clippy_coverage::check(
        input.fs,
        input.workspace_root,
        input.profile_ref,
        input.clippy_tomls,
    );
    report.add_section(Section {
        name: "Clippy ban coverage".to_owned(),
        results: clippy_results,
    });

    let deny_results = deny_audit::check(
        input.fs,
        input.workspace_root,
        input.profile_ref,
        input.deny_tomls,
    );
    report.add_section(Section {
        name: "deny.toml audit".to_owned(),
        results: deny_results,
    });

    let lint_results = cargo_lints::check(input.fs, input.workspace_root, input.cargo_tomls);
    let inheritance_results =
        cargo_lints::check_workspace_inheritance(input.fs, input.workspace_root, &member_dirs);
    let mut lint_section = lint_results;
    lint_section.extend(inheritance_results);
    report.add_section(Section {
        name: "Cargo workspace lints".to_owned(),
        results: lint_section,
    });

    let source_results = source_scan::check(
        input.fs,
        input.workspace_root,
        input.scoped_files,
        input.garde_enabled,
    );
    report.add_section(Section {
        name: "Source code scan".to_owned(),
        results: source_results,
    });

    let dep_results = dependency_scan::check(input.tc);
    report.add_section(Section {
        name: "Dependency & tool checks".to_owned(),
        results: dep_results,
    });
}

fn run_architecture_checks(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    project: &ProjectInfo,
    guardrail_cfg: Option<&GuardrailConfig>,
    profile: Option<&String>,
    report: &mut Report,
) {
    let mut arch_results = Vec::new();
    let crate_configs = merged_crate_configs(project, guardrail_cfg);
    {
        hex_arch_structure::check_hex_arch_structure(fs, workspace_root, &mut arch_results);
        hex_arch_checks::check_dependency_flow(
            fs,
            workspace_root,
            project,
            &crate_configs,
            &mut arch_results,
        );
        hex_arch_checks::check_library_service_boundary(
            fs,
            workspace_root,
            project,
            &crate_configs,
            &mut arch_results,
        );
        hex_arch_checks::check_unconfigured_members(
            fs,
            workspace_root,
            project,
            &crate_configs,
            profile.map_or("service", String::as_str),
            &mut arch_results,
        );
    }

    run_dependency_allowlist_checks(fs, workspace_root, &crate_configs, &mut arch_results);

    report.add_section(Section {
        name: "Architecture checks".to_owned(),
        results: arch_results,
    });
}

fn merged_crate_configs(
    project: &ProjectInfo,
    guardrail_cfg: Option<&GuardrailConfig>,
) -> std::collections::BTreeMap<String, guardrail3_domain_config::types::RustAppConfig> {
    let empty = std::collections::BTreeMap::new();
    let app_configs = guardrail_cfg
        .and_then(|c| c.rust.as_ref())
        .and_then(|r| r.apps.as_ref())
        .unwrap_or(&empty);

    let mut crate_configs = app_configs.clone();

    if let Some(pkg_cfg) = guardrail_cfg
        .and_then(|c| c.rust.as_ref())
        .and_then(|r| r.packages.as_ref())
    {
        for ws in &project.workspaces {
            for member in &ws.members {
                if (member.dir.starts_with("packages/") || member.dir.contains("/packages/"))
                    && !crate_configs.contains_key(member.name.as_str())
                    && !crate_configs.contains_key(member.dir.as_str())
                {
                    let _ = crate_configs.insert(member.name.clone(), pkg_cfg.clone());
                }
            }
        }
    }

    crate_configs
}

fn run_dependency_allowlist_checks(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    crate_configs: &std::collections::BTreeMap<
        String,
        guardrail3_domain_config::types::RustAppConfig,
    >,
    arch_results: &mut Vec<guardrail3_domain_report::CheckResult>,
) {
    for (crate_name, crate_cfg) in crate_configs {
        if let Some(allowed) = &crate_cfg.allowed_deps {
            let cargo_path = workspace_root.join(crate_name).join("Cargo.toml");
            dependency_allowlist::check_dependency_allowlist(
                &cargo_path,
                crate_name,
                allowed,
                fs,
                arch_results,
            );
        }
        dependency_allowlist::check_library_has_allowlist(crate_name, crate_cfg, arch_results);
    }
}

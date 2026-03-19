pub mod allow_checks;
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

use std::path::Path;

use crate::app::crawl::CrawlResult;
use crate::app::discover::ProjectInfo;
use crate::domain::config::types::GuardrailConfig;
use crate::domain::report::{Report, RustCheckCategories, Section};
use crate::ports::outbound::{FileSystem, ToolChecker};

/// Try to load guardrail3.toml as a typed config.
#[allow(clippy::disallowed_methods)] // reason: guardrail3 config parsing — no garde validation needed for own config
fn load_guardrail_config(fs: &dyn FileSystem, path: &Path) -> Option<GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = fs.read_file(&config_path)?;
    toml::from_str(&content).ok()
}

/// Extract profile name from the loaded config.
fn extract_profile(cfg: &GuardrailConfig) -> Option<String> {
    cfg.profile.as_ref().map(|p| p.name.clone())
}

#[allow(clippy::too_many_arguments)] // reason: orchestrator needs all context — will consolidate into struct in Phase 2
pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    project: &ProjectInfo,
    scoped_files: Option<&[String]>,
    categories: &RustCheckCategories,
    thorough: bool,
    tc: &dyn ToolChecker,
    _crawl: &CrawlResult,
) -> Report {
    let workspace_root = project.primary_workspace_root().unwrap_or(path);

    let guardrail_cfg = load_guardrail_config(fs, path);
    let profile = guardrail_cfg.as_ref().and_then(extract_profile);

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    run_code_checks(
        fs,
        tc,
        workspace_root,
        project,
        scoped_files,
        profile.as_deref(),
        categories.garde,
        &mut report,
    );

    if categories.architecture {
        run_architecture_checks(
            fs,
            workspace_root,
            project,
            guardrail_cfg.as_ref(),
            profile.as_ref(),
            &mut report,
        );
    }

    if categories.garde {
        let garde_results = garde_checks::check(fs, workspace_root);
        report.add_section(Section {
            name: "Garde boundary validation".to_owned(),
            results: garde_results,
        });
    }

    if categories.tests {
        let test_results = test_checks::check(fs, tc, workspace_root);
        report.add_section(Section {
            name: "Test quality".to_owned(),
            results: test_results,
        });
    }

    if categories.release {
        let release_results = release_checks::check(fs, tc, workspace_root, project, thorough);
        report.add_section(Section {
            name: "Release readiness".to_owned(),
            results: release_results,
        });
    }

    report
}

#[allow(clippy::too_many_arguments)] // reason: validation orchestrator passes all context to sub-checks
fn run_code_checks(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    workspace_root: &Path,
    project: &ProjectInfo,
    scoped_files: Option<&[String]>,
    profile_ref: Option<&str>,
    garde_enabled: bool,
    report: &mut Report,
) {
    let config_results = config_files::check(fs, workspace_root);
    let member_dirs = project.all_member_dirs();
    let per_crate_results = config_files::check_per_crate_clippy(fs, workspace_root, &member_dirs);
    let mut config_section_results = config_results;
    config_section_results.extend(per_crate_results);
    report.add_section(Section {
        name: "Config files".to_owned(),
        results: config_section_results,
    });

    let clippy_results = clippy_coverage::check(fs, workspace_root, profile_ref);
    report.add_section(Section {
        name: "Clippy ban coverage".to_owned(),
        results: clippy_results,
    });

    let deny_results = deny_audit::check(fs, workspace_root, profile_ref);
    report.add_section(Section {
        name: "deny.toml audit".to_owned(),
        results: deny_results,
    });

    let lint_results = cargo_lints::check(fs, workspace_root);
    let inheritance_results =
        cargo_lints::check_workspace_inheritance(fs, workspace_root, &member_dirs);
    let mut lint_section = lint_results;
    lint_section.extend(inheritance_results);
    report.add_section(Section {
        name: "Cargo workspace lints".to_owned(),
        results: lint_section,
    });

    let source_results = source_scan::check(fs, workspace_root, scoped_files, garde_enabled);
    report.add_section(Section {
        name: "Source code scan".to_owned(),
        results: source_results,
    });

    let dep_results = dependency_scan::check(tc);
    report.add_section(Section {
        name: "Dependency & tool checks".to_owned(),
        results: dep_results,
    });
}

#[allow(clippy::too_many_lines)] // reason: architecture check orchestrator builds merged config and runs multiple checks
fn run_architecture_checks(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    project: &ProjectInfo,
    guardrail_cfg: Option<&GuardrailConfig>,
    profile: Option<&String>,
    report: &mut Report,
) {
    let mut arch_results = Vec::new();

    // Build merged config: apps + packages (packages apply to all members in packages/)
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

    {
        hex_arch_checks::check_hex_arch_structure(
            fs,
            workspace_root,
            project,
            &crate_configs,
            &mut arch_results,
        );
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

    {
        for (crate_name, crate_cfg) in &crate_configs {
            if let Some(allowed) = &crate_cfg.allowed_deps {
                let cargo_path = workspace_root.join(crate_name).join("Cargo.toml");
                dependency_allowlist::check_dependency_allowlist(
                    &cargo_path,
                    crate_name,
                    allowed,
                    fs,
                    &mut arch_results,
                );
            }
            dependency_allowlist::check_library_has_allowlist(
                crate_name,
                crate_cfg,
                &mut arch_results,
            );
        }
    }

    report.add_section(Section {
        name: "Architecture checks".to_owned(),
        results: arch_results,
    });
}

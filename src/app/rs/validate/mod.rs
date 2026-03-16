pub mod allow_checks;
pub mod ast_helpers;
mod ast_visitors;
pub mod cargo_lints;
pub mod clippy_coverage;
pub mod code_quality_checks;
pub mod config_files;
pub mod deny_audit;
mod deny_bans;
mod deny_inventory;
mod deny_licenses;
pub mod dependency_allowlist;
mod dependency_direction;
mod hex_arch_checks;
pub mod dependency_scan;
pub mod garde_checks;
pub mod release_bin_checks;
pub mod release_checks;
pub mod release_crate_checks;
pub mod release_crate_deps;
pub mod release_repo_checks;
mod rustfmt_check;
pub mod source_scan;
pub mod structure_checks;
pub mod test_checks;
mod test_quality_checks;
mod toolchain_check;
mod workspace_metadata;

use std::path::Path;

use crate::app::discover::ProjectInfo;
use crate::domain::config::types::GuardrailConfig;
use crate::domain::report::{Report, Section, ValidateDomains};
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

#[allow(clippy::too_many_lines)] // reason: validation orchestrator — each section is a simple block, splitting would reduce readability
pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    project: &ProjectInfo,
    scoped_files: Option<&[String]>,
    domains: &ValidateDomains,
    thorough: bool,
    tc: &dyn ToolChecker,
) -> Report {
    let workspace_root = project.cargo_workspace_root.as_deref().unwrap_or(path);

    // Load guardrail3.toml config (used for profile + per-crate settings)
    let guardrail_cfg = load_guardrail_config(fs, path);
    let profile = guardrail_cfg.as_ref().and_then(extract_profile);
    let profile_ref = profile.as_deref();

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    if domains.code {
        // Config file checks
        let config_results = config_files::check(fs, workspace_root);
        let per_crate_results = config_files::check_per_crate_clippy(
            fs,
            workspace_root,
            &project.workspace_member_dirs,
        );
        let mut config_section_results = config_results;
        config_section_results.extend(per_crate_results);
        report.add_section(Section {
            name: "Config files".to_owned(),
            results: config_section_results,
        });

        // Clippy ban coverage
        let clippy_results = clippy_coverage::check(fs, workspace_root, profile_ref);
        report.add_section(Section {
            name: "Clippy ban coverage".to_owned(),
            results: clippy_results,
        });

        // deny.toml audit
        let deny_results = deny_audit::check(fs, workspace_root, profile_ref);
        report.add_section(Section {
            name: "deny.toml audit".to_owned(),
            results: deny_results,
        });

        // Cargo workspace lints
        let lint_results = cargo_lints::check(fs, workspace_root);
        let inheritance_results = cargo_lints::check_workspace_inheritance(
            fs,
            workspace_root,
            &project.workspace_member_dirs,
        );
        let mut lint_section = lint_results;
        lint_section.extend(inheritance_results);
        report.add_section(Section {
            name: "Cargo workspace lints".to_owned(),
            results: lint_section,
        });

        // Source code scan (respects scope flags)
        let source_results = source_scan::check(fs, workspace_root, scoped_files);
        report.add_section(Section {
            name: "Source code scan".to_owned(),
            results: source_results,
        });

        // Dependency / tool checks
        let dep_results = dependency_scan::check(fs, tc, workspace_root);
        report.add_section(Section {
            name: "Dependency & tool checks".to_owned(),
            results: dep_results,
        });
    }

    if domains.architecture {
        // Architecture checks: dependency direction + graph + unsafe_code forbid
        let mut arch_results = Vec::new();
        dependency_direction::check_all_dependency_directions(
            fs,
            workspace_root,
            project,
            &mut arch_results,
        );
        dependency_direction::check_dependency_graph(
            fs,
            workspace_root,
            project,
            &mut arch_results,
        );
        structure_checks::check_unsafe_code_forbid(fs, workspace_root, &mut arch_results);

        // Hex arch enforcement (R-ARCH-01, R-ARCH-02, R-ARCH-03)
        {
            let empty = std::collections::BTreeMap::new();
            let crate_configs = guardrail_cfg
                .as_ref()
                .and_then(|c| c.rust.as_ref())
                .and_then(|r| r.crates.as_ref())
                .unwrap_or(&empty);

            hex_arch_checks::check_hex_arch_structure(
                fs,
                workspace_root,
                crate_configs,
                &mut arch_results,
            );
            hex_arch_checks::check_dependency_flow(
                fs,
                workspace_root,
                project,
                crate_configs,
                &mut arch_results,
            );
            hex_arch_checks::check_library_service_boundary(
                fs,
                workspace_root,
                project,
                crate_configs,
                &mut arch_results,
            );
        }

        // Dependency allowlist checks (R-DEPS-01, R-DEPS-02)
        if let Some(crate_map) = guardrail_cfg
            .as_ref()
            .and_then(|c| c.rust.as_ref())
            .and_then(|r| r.crates.as_ref())
        {
            for (crate_name, crate_cfg) in crate_map {
                // R-DEPS-01: check allowed_deps against actual Cargo.toml dependencies
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

                // R-DEPS-02: library crate without allowlist
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

        // Garde boundary validation (R-GARDE-01 through R-GARDE-05)
        let garde_results = garde_checks::check(fs, workspace_root);
        report.add_section(Section {
            name: "Garde boundary validation".to_owned(),
            results: garde_results,
        });
    }

    if domains.tests {
        // Test quality checks (R-TEST-01 through R-TEST-08)
        let test_results = test_checks::check(fs, tc, workspace_root);
        report.add_section(Section {
            name: "Test quality".to_owned(),
            results: test_results,
        });
    }

    if domains.release {
        // Release readiness checks (R-PUB, R-REL, R-BIN)
        let release_results = release_checks::check(fs, tc, workspace_root, project, thorough);
        report.add_section(Section {
            name: "Release readiness".to_owned(),
            results: release_results,
        });
    }

    report
}

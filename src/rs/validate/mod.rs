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
mod dependency_direction;
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

use crate::discover::ProjectInfo;
use crate::report::types::{Report, Section};

/// Controls which validation domains are active.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: domain flags are independent toggles, not a state machine
pub struct ValidateDomains {
    pub code: bool,
    pub architecture: bool,
    pub release: bool,
    pub tests: bool,
}

/// Try to load the profile name from guardrail3.toml if it exists.
fn detect_profile(path: &Path) -> Option<String> {
    let config_path = path.join("guardrail3.toml");
    let content = crate::fs::read_file(&config_path)?;
    let table: toml::Value = content.parse().ok()?;
    table
        .get("profile")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(std::borrow::ToOwned::to_owned)
}

#[allow(clippy::too_many_lines)] // reason: validation orchestrator — each section is a simple block, splitting would reduce readability
pub fn run(
    path: &Path,
    project: &ProjectInfo,
    scoped_files: Option<&[String]>,
    domains: &ValidateDomains,
    thorough: bool,
) -> Report {
    let workspace_root = project.cargo_workspace_root.as_deref().unwrap_or(path);

    // Try to detect the profile from guardrail3.toml at the project root
    let profile = detect_profile(path);
    let profile_ref = profile.as_deref();

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    if domains.code {
        // Config file checks
        let config_results = config_files::check(workspace_root);
        let per_crate_results =
            config_files::check_per_crate_clippy(workspace_root, &project.workspace_member_dirs);
        let mut config_section_results = config_results;
        config_section_results.extend(per_crate_results);
        report.add_section(Section {
            name: "Config files".to_owned(),
            results: config_section_results,
        });

        // Clippy ban coverage
        let clippy_results = clippy_coverage::check(workspace_root, profile_ref);
        report.add_section(Section {
            name: "Clippy ban coverage".to_owned(),
            results: clippy_results,
        });

        // deny.toml audit
        let deny_results = deny_audit::check(workspace_root, profile_ref);
        report.add_section(Section {
            name: "deny.toml audit".to_owned(),
            results: deny_results,
        });

        // Cargo workspace lints
        let lint_results = cargo_lints::check(workspace_root);
        let inheritance_results = cargo_lints::check_workspace_inheritance(
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
        let source_results = source_scan::check(workspace_root, scoped_files);
        report.add_section(Section {
            name: "Source code scan".to_owned(),
            results: source_results,
        });

        // Dependency / tool checks
        let dep_results = dependency_scan::check(workspace_root);
        report.add_section(Section {
            name: "Dependency & tool checks".to_owned(),
            results: dep_results,
        });
    }

    if domains.architecture {
        // Architecture checks: dependency direction + graph + unsafe_code forbid
        let mut arch_results = Vec::new();
        dependency_direction::check_all_dependency_directions(
            workspace_root,
            project,
            &mut arch_results,
        );
        dependency_direction::check_dependency_graph(workspace_root, project, &mut arch_results);
        structure_checks::check_unsafe_code_forbid(workspace_root, &mut arch_results);
        report.add_section(Section {
            name: "Architecture checks".to_owned(),
            results: arch_results,
        });

        // Garde boundary validation (R-GARDE-01 through R-GARDE-05)
        let garde_results = garde_checks::check(workspace_root);
        report.add_section(Section {
            name: "Garde boundary validation".to_owned(),
            results: garde_results,
        });
    }

    if domains.tests {
        // Test quality checks (R-TEST-01 through R-TEST-08)
        let test_results = test_checks::check(workspace_root);
        report.add_section(Section {
            name: "Test quality".to_owned(),
            results: test_results,
        });
    }

    if domains.release {
        // Release readiness checks (R-PUB, R-REL, R-BIN)
        let release_results = release_checks::check(workspace_root, project, thorough);
        report.add_section(Section {
            name: "Release readiness".to_owned(),
            results: release_results,
        });
    }

    report
}

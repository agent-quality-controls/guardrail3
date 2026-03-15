mod allow_checks;
pub mod cargo_lints;
pub mod clippy_coverage;
mod code_quality_checks;
pub mod config_files;
pub mod deny_audit;
mod deny_bans;
mod deny_inventory;
mod deny_licenses;
mod dependency_direction;
pub mod dependency_scan;
mod rustfmt_check;
pub mod source_scan;
mod structure_checks;
mod toolchain_check;
mod workspace_metadata;

use std::path::Path;

use crate::discover::ProjectInfo;
use crate::report::types::{Report, Section};

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

pub fn run(path: &Path, project: &ProjectInfo, scoped_files: Option<&[String]>) -> Report {
    let workspace_root = project.cargo_workspace_root.as_deref().unwrap_or(path);

    // Try to detect the profile from guardrail3.toml at the project root
    let profile = detect_profile(path);
    let profile_ref = profile.as_deref();

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    // Config file checks (always run fully)
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
    let inheritance_results =
        cargo_lints::check_workspace_inheritance(workspace_root, &project.workspace_member_dirs);
    let mut lint_section = lint_results;
    lint_section.extend(inheritance_results);
    report.add_section(Section {
        name: "Cargo workspace lints".to_owned(),
        results: lint_section,
    });

    // Source code scan (respects scope flags)
    let source_results = source_scan::check(workspace_root, scoped_files, project);
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

    report
}

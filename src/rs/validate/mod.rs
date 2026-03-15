pub mod cargo_lints;
pub mod clippy_coverage;
pub mod config_files;
pub mod deny_audit;
pub mod dependency_scan;
pub mod source_scan;

use std::path::Path;

use crate::discover::ProjectInfo;
use crate::report::types::{Report, Section};

pub fn run(path: &Path, project: &ProjectInfo, scoped_files: Option<&[String]>) -> Report {
    let workspace_root = project
        .cargo_workspace_root
        .as_deref()
        .unwrap_or(path);

    let mut report = Report::new(
        path.display().to_string(),
        vec!["Rust".to_string()],
    );

    // Config file checks (always run fully)
    let config_results = config_files::check(workspace_root);
    let per_crate_results = config_files::check_per_crate_clippy(
        workspace_root,
        &project.workspace_member_dirs,
    );
    let mut config_section_results = config_results;
    config_section_results.extend(per_crate_results);
    report.add_section(Section {
        name: "Config files".to_string(),
        results: config_section_results,
    });

    // Clippy ban coverage
    let clippy_results = clippy_coverage::check(workspace_root);
    report.add_section(Section {
        name: "Clippy ban coverage".to_string(),
        results: clippy_results,
    });

    // deny.toml audit
    let deny_results = deny_audit::check(workspace_root);
    report.add_section(Section {
        name: "deny.toml audit".to_string(),
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
        name: "Cargo workspace lints".to_string(),
        results: lint_section,
    });

    // Source code scan (respects scope flags)
    let source_results = source_scan::check(workspace_root, scoped_files, project);
    report.add_section(Section {
        name: "Source code scan".to_string(),
        results: source_results,
    });

    // Dependency / tool checks
    let dep_results = dependency_scan::check(workspace_root);
    report.add_section(Section {
        name: "Dependency & tool checks".to_string(),
        results: dep_results,
    });

    report
}

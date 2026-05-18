use guardrail3_check_types::G3Severity;
use guardrail3_rs_app_types::{
    FamilyRun, FamilyRunner, ReportRenderer, SupportedFamily, ValidateRepoRequest, ValidateReport,
    ValidateWorkspaceRequest, WorkspaceCrawlError, WorkspaceCrawler,
};

use crate::{
    cargo_gates, family_opt_out, marker_pairs,
    outcome::ExecutionOutcome,
    selection::{REPO_LEVEL_FAMILIES, family_cli_name, selected_families_with_opt_out},
    staged,
};

/// Executes one validated request through crawl, family run, and render steps.
///
/// # Errors
///
/// Returns [`WorkspaceCrawlError`] when the workspace cannot be crawled.
pub fn execute(
    request: &ValidateWorkspaceRequest,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> Result<ExecutionOutcome, WorkspaceCrawlError> {
    validate_workspace_root(&request.workspace_root)?;
    let disabled = match family_opt_out::disabled_families(&request.workspace_root) {
        Ok(disabled) => disabled,
        Err(error) => {
            return Ok(ExecutionOutcome::new(
                String::new(),
                format!("{error}\n"),
                1,
            ));
        }
    };
    let crawl = crawler.crawl(&request.workspace_root)?;
    let mut report = ValidateReport::scoped("workspace", request.workspace_root.clone());
    let mut family_errors = Vec::new();

    let families = selected_families_with_opt_out(request, &disabled);

    for family in &families {
        match family_runner.run_family(*family, &crawl) {
            Ok(results) => report.runs.push(FamilyRun {
                family: *family,
                results,
            }),
            Err(error) => {
                family_errors.push(format!("{}: {}", family_cli_name(*family), error));
            }
        }
    }

    let stdout = renderer.render(&report, request.include_inventory);

    let mut gate_failures: Vec<String> = Vec::new();

    if !request.rules_only {
        let repo_root = staged::resolve_repo_root(&request.workspace_root);
        let staged_paths = if request.staged {
            staged::read_staged_files(&repo_root)
        } else {
            Vec::new()
        };

        let workspace_rel = workspace_rel_to_repo(&repo_root, &request.workspace_root);
        let in_scope = if request.staged {
            cargo_gates::paths_under_workspace(&staged_paths, &workspace_rel)
        } else {
            Vec::new()
        };

        let should_run_gates = if request.staged {
            cargo_gates::any_rust_relevant(&in_scope)
        } else {
            true
        };

        if should_run_gates {
            let rust_source_present = cargo_gates::any_rust_source(&in_scope);
            let commands = cargo_gates::cargo_gate_commands(
                &request.workspace_root,
                &families,
                request.staged,
                rust_source_present,
            );
            let target_dir = repo_root.join(".cargo-target");
            let outcomes =
                cargo_gates::run_cargo_gates(&request.workspace_root, &target_dir, &commands);
            collect_gate_failures(&outcomes, &mut gate_failures);
        }
    }

    let stderr = build_stderr(&family_errors, &gate_failures);
    let exit_code = match (
        highest_severity(&report, request.include_inventory),
        family_errors.is_empty() && gate_failures.is_empty(),
    ) {
        (Some(G3Severity::Error), _) | (_, false) => 1,
        (Some(_) | None, true) => 0,
    };

    Ok(ExecutionOutcome::new(stdout, stderr, exit_code))
}

/// Validates only the explicit workspace root marker before config parsing.
fn validate_workspace_root(workspace_root: &std::path::Path) -> Result<(), WorkspaceCrawlError> {
    if !crate::fs::is_dir(workspace_root) {
        return Err(WorkspaceCrawlError {
            message: format!("path is not a directory: {}", workspace_root.display()),
        });
    }
    if !crate::fs::is_file(&workspace_root.join("Cargo.toml")) {
        return Err(WorkspaceCrawlError {
            message: format!(
                "g3rs validates one Rust workspace or package root at a time. Target path \"{}\" has no root Cargo.toml. Run g3rs with --path pointing at a directory that contains the Rust workspace Cargo.toml.",
                workspace_root.display()
            ),
        });
    }
    Ok(())
}

/// Executes one repo-level validate request (validate-repo).
///
/// Runs only the repo-level family set (Hooks + repo-wide Topology) and adds
/// marker-pair completeness findings.
///
/// # Errors
///
/// Returns [`WorkspaceCrawlError`] when the repo cannot be crawled.
pub fn execute_repo(
    request: &ValidateRepoRequest,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> Result<ExecutionOutcome, WorkspaceCrawlError> {
    let crawl = crawler.crawl_any(&request.repo_root)?;
    let mut report = ValidateReport::scoped("repo", request.repo_root.clone());
    let mut family_errors = Vec::new();

    let has_root_cargo_toml = request.repo_root.join("Cargo.toml").is_file();

    for family in REPO_LEVEL_FAMILIES {
        // Topology ingestion currently requires a Cargo.toml at the root.
        // Repo-wide topology over the file tree is handled by `check_marker_pairs`
        // for now; per-workspace topology is handled by per-workspace validate.
        if matches!(family, SupportedFamily::Topology) && !has_root_cargo_toml {
            continue;
        }
        match family_runner.run_family(*family, &crawl) {
            Ok(results) => report.runs.push(FamilyRun {
                family: *family,
                results,
            }),
            Err(error) => {
                family_errors.push(format!("{}: {}", family_cli_name(*family), error));
            }
        }
    }

    let marker_pair_results = marker_pairs::check_repo(&request.repo_root);
    if !marker_pair_results.is_empty() {
        report.runs.push(FamilyRun {
            family: SupportedFamily::Topology,
            results: marker_pair_results,
        });
    }

    let stdout = renderer.render(&report, request.include_inventory);
    let stderr = build_stderr(&family_errors, &[]);
    let exit_code = match (
        highest_severity(&report, request.include_inventory),
        family_errors.is_empty(),
    ) {
        (Some(G3Severity::Error), _) | (_, false) => 1,
        (Some(_) | None, true) => 0,
    };
    Ok(ExecutionOutcome::new(stdout, stderr, exit_code))
}

/// Appends a formatted failure line for each non-zero gate outcome.
fn collect_gate_failures(
    outcomes: &[cargo_gates::CargoGateOutcome],
    gate_failures: &mut Vec<String>,
) {
    for outcome in outcomes {
        if !outcome.ok() {
            gate_failures.push(format!(
                "cargo gate failed (exit {}): {}",
                outcome.exit_code(),
                outcome.command().join(" ")
            ));
        }
    }
}

/// Concatenates family-runner errors and gate failures into a single stderr blob.
fn build_stderr(family_errors: &[String], gate_failures: &[String]) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.extend(family_errors.iter().cloned());
    lines.extend(gate_failures.iter().cloned());
    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

/// Returns the workspace path relative to `repo_root` as a display string, or
/// an empty string when the workspace is not under the repo.
fn workspace_rel_to_repo(repo_root: &std::path::Path, workspace: &std::path::Path) -> String {
    workspace
        .strip_prefix(repo_root)
        .ok()
        .and_then(|p| p.to_str())
        .map_or_else(String::new, str::to_owned)
}

/// Returns the highest visible severity in the current report.
fn highest_severity(report: &ValidateReport, include_inventory: bool) -> Option<G3Severity> {
    report
        .runs
        .iter()
        .flat_map(|run| run.results.iter())
        .filter(|result| include_inventory || !result.inventory())
        .map(guardrail3_check_types::G3CheckResult::severity)
        .max_by_key(|severity| match severity {
            G3Severity::Info => 0_u8,
            G3Severity::Warn => 1_u8,
            G3Severity::Error => 2_u8,
        })
}

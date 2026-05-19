use guardrail3_ts_app_types::{
    FamilyRun, FamilyRunner, ReportRenderer, SUPPORTED_FAMILIES, SupportedFamily,
    ValidateRepoRequest, ValidateReport, ValidateWorkspaceRequest, WorkspaceCrawlError,
    WorkspaceCrawler,
};

use crate::{
    ExecutionOutcome, family_cli_name, family_opt_out, marker_pairs,
    selection::{REPO_LEVEL_FAMILIES, selected_families_with_opt_out},
    staged, tool_presence, toolchain_gates, workspace_adoption,
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
    if request.staged && !staged::has_relevant_staged_files(&request.workspace_root) {
        return Ok(ExecutionOutcome::new(String::new(), String::new(), 0));
    }

    let crawl = crawler.crawl(&request.workspace_root)?;
    let waivers = load_workspace_waivers(&request.workspace_root);
    let mut report = ValidateReport::scoped("workspace", request.workspace_root.clone());
    let mut family_errors = Vec::new();

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
    let families = selected_families_with_opt_out(request, &disabled);
    let enabled_contract_families = enabled_hook_contract_families(&families, &disabled);

    for family in &families {
        match family_runner.run_family(*family, &crawl, &enabled_contract_families) {
            Ok(mut results) => {
                guardrail3_waivers::apply_waivers(&mut results, &waivers);
                report.runs.push(FamilyRun {
                    family: *family,
                    results,
                });
            }
            Err(error) => family_errors.push(format!("{}: {}", family_cli_name(*family), error)),
        }
    }

    let mut stdout = render_stdout(
        &report,
        request.include_inventory,
        !family_errors.is_empty(),
        renderer,
    );
    let mut stderr = build_stderr(&family_errors);
    let gate_failed = if request.rules_only {
        false
    } else {
        let toolchain = toolchain_gates::run_toolchain_gates(
            &request.workspace_root,
            &enabled_contract_families,
            request.include_inventory,
        );
        if !toolchain.stdout.is_empty() && stdout.trim() == "No findings." {
            stdout.clear();
        }
        if !toolchain.stdout.is_empty() {
            stdout.push_str(&toolchain.stdout);
        }
        if !toolchain.stderr.is_empty() {
            stderr.push_str(&toolchain.stderr);
        }
        toolchain.exit_code != 0
    };

    let exit_code = match (
        highest_severity(&report, false),
        family_errors.is_empty() && !gate_failed,
    ) {
        (Some(guardrail3_check_types::G3Severity::Error), _) | (_, false) => 1,
        (Some(_) | None, true) => 0,
    };

    Ok(ExecutionOutcome::new(stdout, stderr, exit_code))
}

/// Executes one repo-level validate request.
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
    let crawl = crawler.crawl(&request.repo_root)?;
    let waivers = load_workspace_waivers(&request.repo_root);
    let mut report = ValidateReport::scoped("repo", request.repo_root.clone());
    let mut family_errors = Vec::new();

    for family in REPO_LEVEL_FAMILIES {
        match family_runner.run_family(*family, &crawl, &SUPPORTED_FAMILIES) {
            Ok(mut results) => {
                guardrail3_waivers::apply_waivers(&mut results, &waivers);
                report.runs.push(FamilyRun {
                    family: *family,
                    results,
                });
            }
            Err(error) => family_errors.push(format!("{}: {}", family_cli_name(*family), error)),
        }
    }

    let mut adoption_results =
        workspace_adoption::check_repo(&request.repo_root, request.include_inventory);
    let mut marker_pair_results = marker_pairs::check_repo(&request.repo_root);
    adoption_results.append(&mut marker_pair_results);
    if !adoption_results.is_empty() {
        guardrail3_waivers::apply_waivers(&mut adoption_results, &waivers);
        report.runs.push(FamilyRun {
            family: SupportedFamily::Topology,
            results: adoption_results,
        });
    }

    let mut tool_results = tool_presence::check_required_tools_present();
    if !tool_results.is_empty() {
        guardrail3_waivers::apply_waivers(&mut tool_results, &waivers);
        report.runs.push(FamilyRun {
            family: SupportedFamily::Hooks,
            results: tool_results,
        });
    }

    let stdout = render_stdout(
        &report,
        request.include_inventory,
        !family_errors.is_empty(),
        renderer,
    );
    let stderr = build_stderr(&family_errors);
    let exit_code = match (highest_severity(&report, false), family_errors.is_empty()) {
        (Some(guardrail3_check_types::G3Severity::Error), _) | (_, false) => 1,
        (Some(_) | None, true) => 0,
    };

    Ok(ExecutionOutcome::new(stdout, stderr, exit_code))
}

/// Returns the family set whose hook contracts should be enforced for this run.
fn enabled_hook_contract_families(
    families: &[SupportedFamily],
    disabled: &[SupportedFamily],
) -> Vec<SupportedFamily> {
    if families.contains(&SupportedFamily::Hooks) {
        return SUPPORTED_FAMILIES
            .into_iter()
            .filter(|family| !disabled.contains(family))
            .collect();
    }
    families.to_vec()
}

/// Loads central waivers from the workspace config when it can be parsed.
fn load_workspace_waivers(
    workspace_root: &std::path::Path,
) -> Vec<guardrail3_waivers::WaiverConfig> {
    g3ts_toml_parser_runtime::from_path(workspace_root.join("guardrail3-ts.toml"))
        .map(|config| config.waivers)
        .unwrap_or_default()
}

/// Builds one stderr blob from family-runner failures.
fn build_stderr(family_errors: &[String]) -> String {
    if family_errors.is_empty() {
        String::new()
    } else {
        format!("{}\n", family_errors.join("\n"))
    }
}

/// Returns the highest visible severity in the current report.
fn highest_severity(
    report: &ValidateReport,
    include_inventory: bool,
) -> Option<guardrail3_check_types::G3Severity> {
    report
        .runs
        .iter()
        .flat_map(|run| run.results.iter())
        .filter(|result| include_inventory || !result.inventory())
        .map(guardrail3_check_types::G3CheckResult::severity)
        .max_by_key(|severity| match severity {
            guardrail3_check_types::G3Severity::Info => 0_u8,
            guardrail3_check_types::G3Severity::Warn => 1_u8,
            guardrail3_check_types::G3Severity::Error => 2_u8,
        })
}

/// Renders stdout only when the visible report should be printed.
fn render_stdout(
    report: &ValidateReport,
    include_inventory: bool,
    has_family_errors: bool,
    renderer: &dyn ReportRenderer,
) -> String {
    if has_family_errors && highest_severity(report, include_inventory).is_none() {
        return String::new();
    }

    renderer.render(report, include_inventory)
}

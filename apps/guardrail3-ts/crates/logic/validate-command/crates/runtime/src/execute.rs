use guardrail3_check_types::G3Severity;
use guardrail3_ts_app_types::{
    FamilyRun, FamilyRunner, ReportRenderer, SUPPORTED_FAMILIES, SupportedFamily, ValidateReport,
    ValidateRequest, WorkspaceCrawlError, WorkspaceCrawler,
};
use guardrail3_waivers::WaiverConfig;

use crate::{ExecutionOutcome, family_cli_name, family_opt_out, selected_families_with_opt_out};

/// Executes one validated request through crawl, family run, and render steps.
///
/// # Errors
///
/// Returns [`WorkspaceCrawlError`] when the workspace cannot be crawled.
pub fn execute(
    request: &ValidateRequest,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> Result<ExecutionOutcome, WorkspaceCrawlError> {
    let crawl = crawler.crawl(&request.workspace_root)?;
    let waivers = load_workspace_waivers(&request.workspace_root);
    let mut report = ValidateReport::default();
    let mut family_errors = Vec::new();

    let disabled = if is_repo_only_request(request) {
        Vec::new()
    } else {
        match family_opt_out::disabled_families(&request.workspace_root) {
            Ok(disabled) => disabled,
            Err(error) => {
                return Ok(ExecutionOutcome::new(
                    String::new(),
                    format!("{error}\n"),
                    1,
                ));
            }
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

    let stdout = render_stdout(
        &report,
        request.include_inventory,
        !family_errors.is_empty(),
        renderer,
    );
    let stderr = if family_errors.is_empty() {
        String::new()
    } else {
        format!("{}\n", family_errors.join("\n"))
    };
    let exit_code = match (
        highest_severity(&report, request.include_inventory),
        family_errors.is_empty(),
    ) {
        (Some(G3Severity::Error), _) | (_, false) => 1,
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

/// Returns true for repo-level requests routed through the shared TS executor.
fn is_repo_only_request(request: &ValidateRequest) -> bool {
    !request.families.is_empty()
        && request
            .families
            .iter()
            .all(|family| matches!(family, SupportedFamily::Hooks | SupportedFamily::Topology))
}

/// Loads central waivers from the workspace config when it can be parsed.
fn load_workspace_waivers(workspace_root: &std::path::Path) -> Vec<WaiverConfig> {
    g3ts_toml_parser_runtime::from_path(workspace_root.join("guardrail3-ts.toml"))
        .map(|config| config.waivers)
        .unwrap_or_default()
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

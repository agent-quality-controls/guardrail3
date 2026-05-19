use guardrail3_check_types::G3Severity;
use guardrail3_ts_app_types::{
    FamilyRun, FamilyRunner, ReportRenderer, SupportedFamily, ValidateReport, ValidateRequest,
    WorkspaceCrawlError, WorkspaceCrawler,
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

    let disabled = default_disabled_families(request);
    let families = selected_families_with_opt_out(request, &disabled);

    for family in families {
        match family_runner.run_family(family, &crawl) {
            Ok(mut results) => {
                guardrail3_waivers::apply_waivers(&mut results, &waivers);
                report.runs.push(FamilyRun { family, results });
            }
            Err(error) => family_errors.push(format!("{}: {}", family_cli_name(family), error)),
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

/// Returns disabled families from user opt-outs plus default framework
/// suppression. Astro families are default-on only when the workspace declares
/// Astro; explicit `--family astro-*` still runs those families.
fn default_disabled_families(request: &ValidateRequest) -> Vec<SupportedFamily> {
    let mut disabled = family_opt_out::disabled_families(&request.workspace_root);
    if request.families.is_empty() && !is_astro_workspace(&request.workspace_root) {
        disabled.extend(astro_families());
    }
    disabled
}

/// Returns every Astro-specific family.
const fn astro_families() -> [SupportedFamily; 7] {
    [
        SupportedFamily::AstroSetup,
        SupportedFamily::AstroContent,
        SupportedFamily::AstroMdx,
        SupportedFamily::AstroI18n,
        SupportedFamily::AstroMedia,
        SupportedFamily::AstroSeo,
        SupportedFamily::AstroState,
    ]
}

/// Returns true when a workspace declares Astro by config, package metadata, or
/// G3TS Astro policy.
fn is_astro_workspace(workspace_root: &std::path::Path) -> bool {
    has_astro_config_file(workspace_root)
        || package_json_declares_astro(workspace_root)
        || guardrail_config_declares_astro(workspace_root)
}

/// Returns true when an Astro config file exists at the workspace root.
fn has_astro_config_file(workspace_root: &std::path::Path) -> bool {
    [
        "astro.config.mjs",
        "astro.config.js",
        "astro.config.ts",
        "astro.config.cjs",
    ]
    .iter()
    .any(|filename| workspace_root.join(filename).is_file())
}

/// Returns true when `package.json` declares the `astro` package.
fn package_json_declares_astro(workspace_root: &std::path::Path) -> bool {
    let Ok(package_json) = package_json_parser::from_path(workspace_root.join("package.json"))
    else {
        return false;
    };
    package_json
        .dependencies
        .iter()
        .chain(&package_json.dev_dependencies)
        .chain(&package_json.peer_dependencies)
        .any(|dependency| dependency == "astro")
}

/// Returns true when `guardrail3-ts.toml` contains an `[astro]` policy.
fn guardrail_config_declares_astro(workspace_root: &std::path::Path) -> bool {
    g3ts_toml_parser_runtime::from_path(workspace_root.join("guardrail3-ts.toml"))
        .map(
            |config: g3ts_toml_parser_types::guardrail3_ts_toml::Guardrail3TsToml| {
                config.astro.is_some()
            },
        )
        .unwrap_or(false)
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
